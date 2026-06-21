#![allow(warnings)]
pub use ars_native::ARSValue;
use rayon::prelude::*;
use std::mem::MaybeUninit;
use std::ptr;

// Metrics linkage
pub use ars_native::{COMPARISONS, MOVES};

pub struct ARSMassivelyParallel;

impl ARSMassivelyParallel {
    /// ARS Gen 8.1: Memory-Efficient Parallel Optimum
    /// Innovation: Fused Refine-and-Return (Reducing Move Latency)
    pub fn sort<T: ARSValue + PartialOrd + Send + Clone>(data: &mut [T]) {
        let n = data.len();
        if n < 2048 {
            data.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            return;
        }

        // 1. Parallel Analyze
        let (keys, min_v, max_v, sorted, reversed) = Self::analyze_parallel(data);
        if sorted {
            return;
        }
        if reversed {
            data.reverse();
            return;
        }

        let num_threads = rayon::current_num_threads();
        let num_bins = (n / 512).max(256).min(1024).next_power_of_two();
        let range = (max_v - min_v).max(1);
        let shift_bits = 64;
        let multiplier = (((num_bins - 1) as u128) << shift_bits) / range as u128;

        // 2. Parallel Histogram
        let chunk_size = (n / num_threads).max(32768);
        let thread_hists: Vec<Vec<usize>> = (0..num_threads)
            .into_par_iter()
            .map(|t_idx| {
                let start = t_idx * chunk_size;
                let mut local_hist = vec![0usize; num_bins];
                if start < n {
                    let end = (start + chunk_size).min(n);
                    for &k in &keys[start..end] {
                        let b =
                            (((k.wrapping_sub(min_v) as u128) * multiplier) >> shift_bits) as usize;
                        local_hist[b.min(num_bins - 1)] += 1;
                    }
                }
                local_hist
            })
            .collect();

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

        // 4. Parallel Shuffle: Original -> Scratch (Move 1)
        let mut scratch: Vec<MaybeUninit<T>> = Vec::with_capacity(n);
        unsafe {
            scratch.set_len(n);
        }
        let scratch_ptr = scratch.as_mut_ptr() as usize;

        (0..num_threads).into_par_iter().for_each(|t_idx| {
            let start = t_idx * chunk_size;
            if start < n {
                let end = (start + chunk_size).min(n);
                let mut local_pos = thread_starts[t_idx].clone();
                let s_ptr = scratch_ptr as *mut T;
                for i in start..end {
                    let k = keys[i];
                    let b = (((k.wrapping_sub(min_v) as u128) * multiplier) >> shift_bits) as usize;
                    let b = b.min(num_bins - 1);
                    let pos = local_pos[b];
                    unsafe {
                        ptr::write(s_ptr.add(pos), data[i].clone());
                    }
                    local_pos[b] += 1;
                }
            }
        });
        // Removed manual atomic increments as Tracked<T> handles this.

        // 5. BREAKTHROUGH: Fused Refine-and-Original (Move 2 + Refine)
        // We sort in scratch and write directly to the final original location
        // This is done in parallel using raw pointers for the target.
        let data_ptr = data.as_mut_ptr() as usize;

        (0..num_bins).into_par_iter().for_each(|b_idx| {
            let start = bin_offsets[b_idx];
            let end = bin_offsets[b_idx + 1];
            if end > start {
                let s_slice = unsafe {
                    let ptr = (scratch_ptr as *mut T).add(start);
                    std::slice::from_raw_parts_mut(ptr, end - start)
                };

                // Sort the data while it is hot in cache
                let mut local_cmp = 0u64;
                if s_slice.len() > 1 {
                    s_slice.sort_unstable_by(|a, b| {
                        local_cmp += 1;
                        a.partial_cmp(b).unwrap()
                    });
                }
                // Removed manual atomic increments as Tracked<T> handles this.

                // Immediate copy back to original array (Cache-Friendly write)
                unsafe {
                    let target_ptr = (data_ptr as *mut T).add(start);
                    ptr::copy_nonoverlapping(s_slice.as_ptr(), target_ptr, s_slice.len());
                }
            }
        });
        // Removed manual atomic increments as Tracked<T> handles this.

        unsafe {
            scratch.set_len(0);
        }
    }

    fn parallel_refine_recursive<T: ARSValue + PartialOrd + Send>(
        _data: &mut [T],
        _offsets: &[usize],
    ) {
        // Implementation logic moved into the fused loop above
        let _n = _data.len();
    }

    fn analyze_parallel<T: ARSValue + PartialOrd + Send + Sync>(
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
