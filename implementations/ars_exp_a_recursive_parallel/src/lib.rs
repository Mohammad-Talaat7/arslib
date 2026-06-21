#![allow(warnings)]
pub use arslib::{ARSValue, COMPARISONS, MOVES};
use rayon::prelude::*;
use std::mem::MaybeUninit;
use std::ptr;

pub struct ARSRecursiveParallel;

const RECURSION_THRESHOLD: usize = 4096;

impl ARSRecursiveParallel {
    pub fn sort<T: ARSValue + PartialOrd + Send + Clone>(data: &mut [T]) {
        let n = data.len();
        if n < 1024 {
            data.sort_unstable_by(|a, b| {
                // Removed manual atomic increments as Tracked<T> handles this.
                a.partial_cmp(b).unwrap()
            });
            return;
        }

        let mut scratch: Vec<MaybeUninit<T>> = Vec::with_capacity(n);
        unsafe {
            scratch.set_len(n);
        }

        {
            let scratch_slice =
                unsafe { std::slice::from_raw_parts_mut(scratch.as_mut_ptr() as *mut T, n) };
            Self::sort_recursive(data, scratch_slice);
        }

        unsafe {
            scratch.set_len(0);
        }
    }

    /// Sorts elements from 'src' into 'src'. Uses 'dest' as temporary workspace.
    fn sort_recursive<T: ARSValue + PartialOrd + Send + Clone>(src: &mut [T], dest: &mut [T]) {
        let n = src.len();
        if n < RECURSION_THRESHOLD {
            src.sort_unstable_by(|a, b| {
                // Removed manual atomic increments as Tracked<T> handles this.
                a.partial_cmp(b).unwrap()
            });
            return;
        }

        let (keys, min_v, max_v, sorted, reversed) = Self::analyze_and_extract(src);

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

        let num_threads = rayon::current_num_threads();
        let num_bins = (n / 1024).max(256).min(2048).next_power_of_two();
        let range = (max_v - min_v).max(1);
        let shift_bits = 64;
        let multiplier = (((num_bins - 1) as u128) << shift_bits) / range as u128;

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

        // MOVE 1: src -> dest (partitioned)
        let s_ptr_usize = src.as_ptr() as usize;
        let d_ptr_usize = dest.as_mut_ptr() as usize;

        (0..num_threads).into_par_iter().for_each(|t_idx| {
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
        });
        // Removed manual atomic increments as Tracked<T> handles this.

        // RECURSIVE REFINEMENT on bins in 'dest', sorting them into 'dest'.
        // Then we move them back to 'src'.
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

                    // Recursive call sorts sub_src (which is part of 'dest') using sub_dest (part of 'src')
                    Self::sort_recursive(sub_src, sub_dest);
                }
            }
        });

        // MOVE 2: dest (now sorted) -> src
        unsafe {
            ptr::copy_nonoverlapping(dest.as_ptr(), src.as_mut_ptr(), n);
        }
        // Removed manual atomic increments as Tracked<T> handles this.
    }

    fn analyze_and_extract<T: ARSValue + PartialOrd + Send + Sync>(
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
