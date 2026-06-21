pub use ars_native::{ARSValue, COMPARISONS, MOVES};
use rayon::prelude::*;
use std::mem::MaybeUninit;
use std::ptr;

pub struct ARSOptimizedApex;

impl ARSOptimizedApex {
    /// ARS Gen 9: Optimized Apex (Unstable)
    pub fn sort<T: ARSValue + PartialOrd + Send + Clone>(data: &mut [T]) {
        Self::sort_internal(data, false);
    }

    /// ARS Gen 9: Optimized Apex (Stable)
    pub fn sort_stable<T: ARSValue + PartialOrd + Send + Clone>(data: &mut [T]) {
        Self::sort_internal(data, true);
    }

    fn sort_internal<T: ARSValue + PartialOrd + Send + Clone>(data: &mut [T], stable: bool) {
        let n = data.len();
        if n < 1024 {
            if stable {
                data.sort_by(|a, b| a.partial_cmp(b).unwrap());
            } else {
                data.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            }
            return;
        }

        // 1. Parallel Analyze
        let (keys, min_v, max_v, sorted, reversed) = Self::analyze_parallel(data);
        if sorted {
            return;
        }
        if reversed {
            data.reverse();
            // Removed manual atomic increments as Tracked<T> handles this.
            return;
        }

        let num_threads = rayon::current_num_threads();
        let num_bins = (n / 512).max(256).min(1024).next_power_of_two();
        let range = (max_v - min_v).max(1);

        let shift_bits = 64;
        let multiplier = (((num_bins - 1) as u128) << shift_bits) / range as u128;

        // 2. Parallel Histogram
        let chunk_size = (n + num_threads - 1) / num_threads;
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

        // 4. Move to Scratch (Stable Shuffle)
        let mut scratch: Vec<MaybeUninit<T>> = Vec::with_capacity(n);
        unsafe {
            scratch.set_len(n);
        }
        let scratch_ptr_usize = scratch.as_mut_ptr() as usize;

        (0..num_threads).into_par_iter().for_each(|t_idx| {
            let start = t_idx * chunk_size;
            if start < n {
                let end = (start + chunk_size).min(n);
                let mut local_pos = thread_starts[t_idx].clone();
                let s_ptr = scratch_ptr_usize as *mut T;

                for i in start..end {
                    let k = keys[i];
                    let b = (((k.wrapping_sub(min_v) as u128) * multiplier) >> shift_bits) as usize;
                    let b = b.min(num_bins - 1);
                    let pos = local_pos[b];
                    unsafe {
                        let val = ptr::read(&data[i]);
                        ptr::write(s_ptr.add(pos), val);
                    }
                    local_pos[b] += 1;
                }
            }
        });
        // Removed manual atomic increments as Tracked<T> handles this.

        // 5. Parallel Bin Refinement
        let data_ptr_usize = data.as_mut_ptr() as usize;

        (0..num_bins).into_par_iter().for_each(|b_idx| {
            let start = bin_offsets[b_idx];
            let end = bin_offsets[b_idx + 1];
            if end > start {
                let bin_len = end - start;
                let s_slice: &mut [T] = unsafe {
                    let s_ptr = scratch_ptr_usize as *mut T;
                    std::slice::from_raw_parts_mut(s_ptr.add(start), bin_len)
                };

                if bin_len > 1 {
                    if stable {
                        s_slice.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    } else {
                        s_slice.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
                    }
                }

                unsafe {
                    let d_ptr = data_ptr_usize as *mut T;
                    ptr::copy_nonoverlapping(s_slice.as_ptr(), d_ptr.add(start), bin_len);
                }
            }
        });
        // Removed manual atomic increments as Tracked<T> handles this.

        unsafe {
            scratch.set_len(0);
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_stable() {
        // Track original index to verify stability
        #[derive(Debug, Clone, PartialEq)]
        struct Item {
            val: i64,
            id: usize,
        }
        impl PartialOrd for Item {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.val.partial_cmp(&other.val)
            }
        }
        impl ARSValue for Item {
            fn to_spatial_u64(&self) -> u64 {
                self.val.to_spatial_u64()
            }
        }

        let mut data = vec![
            Item { val: 5, id: 0 },
            Item { val: 2, id: 1 },
            Item { val: 5, id: 2 },
            Item { val: 1, id: 3 },
            Item { val: 2, id: 4 },
        ];

        ARSOptimizedApex::sort_stable(&mut data);

        assert_eq!(
            data,
            vec![
                Item { val: 1, id: 3 },
                Item { val: 2, id: 1 },
                Item { val: 2, id: 4 },
                Item { val: 5, id: 0 },
                Item { val: 5, id: 2 },
            ]
        );
    }
}
