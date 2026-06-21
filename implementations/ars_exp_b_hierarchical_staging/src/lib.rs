#![allow(warnings)]
pub use arslib::{ARSValue, COMPARISONS, MOVES};
use rayon::prelude::*;
use std::mem::MaybeUninit;
use std::ptr;
use std::sync::atomic::Ordering;

pub struct ARSHierarchicalParallel;

// Constants for hierarchical control
const PARALLEL_THRESHOLD: usize = 16384;
const RECURSION_THRESHOLD: usize = 4096;
const MAX_PARALLEL_DEPTH: u32 = 2;
const BINS_PER_LEVEL: usize = 512;

impl ARSHierarchicalParallel {
    /// ARS Gen 11: Hierarchical Parallel ARS
    pub fn sort<T: ARSValue + PartialOrd + Send + Clone>(data: &mut [T]) {
        let n = data.len();
        if n < PARALLEL_THRESHOLD {
            ars_optimized_apex::ARSOptimizedApex::sort(data);
            return;
        }

        let mut scratch: Vec<MaybeUninit<T>> = Vec::with_capacity(n);
        unsafe {
            scratch.set_len(n);
        }

        {
            let scratch_slice =
                unsafe { std::slice::from_raw_parts_mut(scratch.as_mut_ptr() as *mut T, n) };
            Self::sort_hierarchical(data, scratch_slice, 0);
        }

        unsafe {
            scratch.set_len(0);
        }
    }

    fn sort_hierarchical<T: ARSValue + PartialOrd + Send + Clone>(
        src: &mut [T],
        dest: &mut [T],
        depth: u32,
    ) {
        let n = src.len();
        if n < RECURSION_THRESHOLD {
            src.sort_unstable_by(|a, b| {
                // Removed manual atomic increments as Tracked<T> handles this.
                a.partial_cmp(b).unwrap()
            });
            return;
        }

        // 1. Pass 1: Find Min/Max (Parallel)
        let (min_v, max_v, sorted, reversed) = Self::find_min_max(src);
        if sorted {
            return;
        }
        if reversed {
            src.reverse();
            // Removed manual atomic increments as Tracked<T> handles this.
            return;
        }

        if min_v == max_v {
            src.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            return;
        }

        let num_threads = if depth < MAX_PARALLEL_DEPTH {
            rayon::current_num_threads()
        } else {
            1
        };
        let num_bins = BINS_PER_LEVEL.min(n / 64).max(1).next_power_of_two();
        let range = (max_v - min_v).max(1);
        let shift_bits = 64;
        let multiplier = (((num_bins - 1) as u128) << shift_bits) / range as u128;

        // 2. Pass 2: Extract Keys + Histogram (Parallel)
        let chunk_size = (n + num_threads - 1) / num_threads;

        let (keys, thread_hists): (Vec<u64>, Vec<Vec<usize>>) = if num_threads > 1 {
            let results: Vec<(Vec<u64>, Vec<usize>)> = src
                .par_chunks(chunk_size)
                .map(|chunk| {
                    let mut local_keys = Vec::with_capacity(chunk.len());
                    let mut local_hist = vec![0usize; num_bins];
                    for x in chunk {
                        let k = x.to_spatial_u64();
                        local_keys.push(k);
                        let b =
                            (((k.wrapping_sub(min_v) as u128) * multiplier) >> shift_bits) as usize;
                        local_hist[b.min(num_bins - 1)] += 1;
                    }
                    (local_keys, local_hist)
                })
                .collect();

            let mut all_keys = Vec::with_capacity(n);
            let mut all_hists = Vec::with_capacity(results.len());
            for (mut k, h) in results {
                all_keys.append(&mut k);
                all_hists.push(h);
            }
            (all_keys, all_hists)
        } else {
            let mut local_keys = Vec::with_capacity(n);
            let mut local_hist = vec![0usize; num_bins];
            for x in src.iter() {
                let k = x.to_spatial_u64();
                local_keys.push(k);
                let b = (((k.wrapping_sub(min_v) as u128) * multiplier) >> shift_bits) as usize;
                local_hist[b.min(num_bins - 1)] += 1;
            }
            (local_keys, vec![local_hist])
        };

        // 3. Global Prefix Sum
        let mut bin_offsets = vec![0usize; num_bins + 1];
        let mut thread_starts = vec![vec![0usize; num_bins]; num_threads];
        let mut total = 0;
        for b in 0..num_bins {
            bin_offsets[b] = total;
            for t in 0..num_threads {
                thread_starts[t][b] = total;
                total += thread_hists[t][b];
            }
        }
        bin_offsets[num_bins] = n;

        // 4. Pass 3: CHUNK-BASED SCATTER with Move Semantics
        let s_ptr_usize = src.as_ptr() as usize;
        let d_ptr_usize = dest.as_mut_ptr() as usize;

        let scatter_op = |t_idx: usize| {
            let start = t_idx * chunk_size;
            if start < n {
                let end = (start + chunk_size).min(n);
                let mut local_pos = thread_starts[t_idx].clone();
                let s_ptr = s_ptr_usize as *const T;
                let d_ptr = d_ptr_usize as *mut T;

                for i in start..end {
                    let k = keys[i];
                    let b = (((k.wrapping_sub(min_v) as u128) * multiplier) >> shift_bits) as usize;
                    let b = b.min(num_bins - 1);
                    let pos = local_pos[b];
                    unsafe {
                        let val = ptr::read(s_ptr.add(i));
                        ptr::write(d_ptr.add(pos), val);
                    }
                    local_pos[b] += 1;
                }
            }
        };

        if num_threads > 1 {
            (0..num_threads).into_par_iter().for_each(scatter_op);
        } else {
            scatter_op(0);
        }
        // Removed manual atomic increments as Tracked<T> handles this.

        // 5. STAGED REFINEMENT
        let d_ptr_usize = dest.as_mut_ptr() as usize;
        let s_ptr_usize = src.as_mut_ptr() as usize;

        if depth < MAX_PARALLEL_DEPTH {
            bin_offsets.par_windows(2).for_each(|range| {
                let b_start = range[0];
                let b_end = range[1];
                if b_end > b_start {
                    let bin_len = b_end - b_start;
                    unsafe {
                        let d_ptr = d_ptr_usize as *mut T;
                        let s_ptr = s_ptr_usize as *mut T;
                        let sub_src = std::slice::from_raw_parts_mut(d_ptr.add(b_start), bin_len);
                        let sub_dest = std::slice::from_raw_parts_mut(s_ptr.add(b_start), bin_len);
                        Self::sort_hierarchical(sub_src, sub_dest, depth + 1);
                    }
                }
            });
        } else {
            let d_ptr = d_ptr_usize as *mut T;
            let s_ptr = s_ptr_usize as *mut T;
            for b in 0..num_bins {
                let b_start = bin_offsets[b];
                let b_end = bin_offsets[b + 1];
                if b_end > b_start {
                    let bin_len = b_end - b_start;
                    unsafe {
                        let sub_src = std::slice::from_raw_parts_mut(d_ptr.add(b_start), bin_len);
                        let sub_dest = std::slice::from_raw_parts_mut(s_ptr.add(b_start), bin_len);
                        Self::sort_hierarchical(sub_src, sub_dest, depth + 1);
                    }
                }
            }
        }

        unsafe {
            ptr::copy_nonoverlapping(dest.as_ptr(), src.as_mut_ptr(), n);
        }
        // Removed manual atomic increments as Tracked<T> handles this.
    }

    fn find_min_max<T: ARSValue + PartialOrd + Send + Sync>(data: &[T]) -> (u64, u64, bool, bool) {
        let n = data.len();
        if n == 0 {
            return (0, 0, true, true);
        }

        let (min_v, max_v) = data
            .par_iter()
            .map(|x| {
                let k = x.to_spatial_u64();
                (k, k)
            })
            .reduce(|| (u64::MAX, 0), |a, b| (a.0.min(b.0), a.1.max(b.1)));

        let mut sorted = true;
        let mut reversed = true;
        for i in 1..n {
            if data[i] < data[i - 1] {
                sorted = false;
            }
            if data[i] > data[i - 1] {
                reversed = false;
            }
            if !sorted && !reversed {
                break;
            }
        }

        (min_v, max_v, sorted, reversed)
    }
}
