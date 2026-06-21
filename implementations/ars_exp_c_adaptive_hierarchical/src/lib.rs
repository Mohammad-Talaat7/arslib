#![allow(warnings)]
pub use arslib::{ARSValue, COMPARISONS, MOVES};
use rayon::prelude::*;
use std::mem::MaybeUninit;
use std::ptr;
use std::sync::atomic::Ordering;

pub struct ARSAdaptiveHierarchical;

// Tuning parameters
const PARALLEL_THRESHOLD: usize = 16384;
const RECURSION_THRESHOLD: usize = 8192;
const MAX_PARALLEL_DEPTH: u32 = 2;
const TARGET_BIN_SIZE: usize = 16384; // Target large enough for further recursion
const BINS_MAX: usize = 1024;
const BINS_MIN: usize = 256;

impl ARSAdaptiveHierarchical {
    /// ARS Gen 12: Adaptive Hierarchical Parallel
    pub fn sort<T: ARSValue + PartialOrd + Send + Clone>(data: &mut [T]) {
        let n = data.len();
        if n < PARALLEL_THRESHOLD {
            ars_optimized_apex::ARSOptimizedApex::sort(data);
            return;
        }

        // 1. Allocate scratch once
        let mut scratch: Vec<MaybeUninit<T>> = Vec::with_capacity(n);
        unsafe {
            scratch.set_len(n);
        }

        let in_scratch = {
            let scratch_slice =
                unsafe { std::slice::from_raw_parts_mut(scratch.as_mut_ptr() as *mut T, n) };
            // Returns true if sorted data ended up in scratch, false if in data
            Self::sort_recursive(data, scratch_slice, 0)
        };

        if in_scratch {
            // Only copy back once at the very end if necessary
            unsafe {
                ptr::copy_nonoverlapping(scratch.as_ptr() as *const T, data.as_mut_ptr(), n);
            }
            // Removed manual atomic increments as Tracked<T> handles this.
        }

        unsafe {
            scratch.set_len(0);
        }
    }

    /// Sorts elements. Returns true if result is in 'dest', false if in 'src'.
    fn sort_recursive<T: ARSValue + PartialOrd + Send + Clone>(
        src: &mut [T],
        dest: &mut [T],
        depth: u32,
    ) -> bool {
        let n = src.len();

        // Base case: Sort in place
        if n < RECURSION_THRESHOLD {
            src.sort_unstable_by(|a, b| {
                // Removed manual atomic increments as Tracked<T> handles this.
                a.partial_cmp(b).unwrap()
            });
            return false; // Result in src
        }

        // 1. FUSED PASS: Analyze (Min/Max/Sorted) + Key Extraction
        let (keys, min_v, max_v, sorted, reversed) = Self::analyze_fused(src);

        if sorted {
            return false;
        }
        if reversed {
            src.reverse();
            // Removed manual atomic increments as Tracked<T> handles this.
            return false;
        }

        if min_v == max_v {
            src.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            return false;
        }

        // 2. Adaptive Binning
        let num_bins = (n / TARGET_BIN_SIZE)
            .max(BINS_MIN)
            .min(BINS_MAX)
            .next_power_of_two();
        let range = (max_v - min_v).max(1);
        let shift_bits = 64;
        let multiplier = (((num_bins - 1) as u128) << shift_bits) / range as u128;

        // 3. Parallel Histogram
        let num_threads = if depth < MAX_PARALLEL_DEPTH {
            rayon::current_num_threads()
        } else {
            1
        };
        let chunk_size = (n + num_threads - 1) / num_threads;

        let thread_hists: Vec<Vec<usize>> = if num_threads > 1 {
            (0..num_threads)
                .into_par_iter()
                .map(|t_idx| {
                    let start = t_idx * chunk_size;
                    let mut local_hist = vec![0usize; num_bins];
                    if start < n {
                        let end = (start + chunk_size).min(n);
                        for &k in &keys[start..end] {
                            let b = (((k.wrapping_sub(min_v) as u128) * multiplier) >> shift_bits)
                                as usize;
                            local_hist[b.min(num_bins - 1)] += 1;
                        }
                    }
                    local_hist
                })
                .collect()
        } else {
            let mut local_hist = vec![0usize; num_bins];
            for &k in &keys {
                let b = (((k.wrapping_sub(min_v) as u128) * multiplier) >> shift_bits) as usize;
                local_hist[b.min(num_bins - 1)] += 1;
            }
            vec![local_hist]
        };

        // 4. Prefix Sum
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

        // 5. Partitioned Scatter: src -> dest
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

        // 6. Staged Refinement: Sort bins in 'dest', using 'src' sub-slices as workspace.
        if num_threads > 1 {
            bin_offsets.par_windows(2).for_each(|range| {
                let b_start = range[0];
                let b_end = range[1];
                if b_end > b_start {
                    let bin_len = b_end - b_start;
                    unsafe {
                        let d_ptr = d_ptr_usize as *mut T;
                        let s_ptr = s_ptr_usize as *mut T;
                        let sub_dest = std::slice::from_raw_parts_mut(d_ptr.add(b_start), bin_len);
                        let sub_src = std::slice::from_raw_parts_mut(s_ptr.add(b_start), bin_len);

                        // Recursive call: sorts sub_dest into sub_dest (or sub_src)
                        let in_src_sub = Self::sort_recursive(sub_dest, sub_src, depth + 1);
                        if in_src_sub {
                            // If it ended up in src_sub, we must move it back to dest_sub
                            // to ensure 'dest' is fully sorted at the end of this loop.
                            ptr::copy_nonoverlapping(
                                sub_src.as_ptr(),
                                sub_dest.as_mut_ptr(),
                                bin_len,
                            );
                            // Removed manual atomic increments as Tracked<T> handles this.
                        }
                    }
                }
            });
        } else {
            for range in bin_offsets.windows(2) {
                let b_start = range[0];
                let b_end = range[1];
                if b_end > b_start {
                    let bin_len = b_end - b_start;
                    unsafe {
                        let d_ptr = d_ptr_usize as *mut T;
                        let s_ptr = s_ptr_usize as *mut T;
                        let sub_dest = std::slice::from_raw_parts_mut(d_ptr.add(b_start), bin_len);
                        let sub_src = std::slice::from_raw_parts_mut(s_ptr.add(b_start), bin_len);

                        let in_src_sub = Self::sort_recursive(sub_dest, sub_src, depth + 1);
                        if in_src_sub {
                            ptr::copy_nonoverlapping(
                                sub_src.as_ptr(),
                                sub_dest.as_mut_ptr(),
                                bin_len,
                            );
                            // Removed manual atomic increments as Tracked<T> handles this.
                        }
                    }
                }
            }
        }

        true // Result in dest
    }

    fn analyze_fused<T: ARSValue + PartialOrd + Send + Sync>(
        data: &[T],
    ) -> (Vec<u64>, u64, u64, bool, bool) {
        let n = data.len();
        let keys: Vec<u64> = data.par_iter().map(|x| x.to_spatial_u64()).collect();

        let (min_v, max_v) = keys
            .par_iter()
            .fold(|| (u64::MAX, 0), |acc, &k| (acc.0.min(k), acc.1.max(k)))
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

        (keys, min_v, max_v, sorted, reversed)
    }
}
