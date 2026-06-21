#![allow(warnings)]
pub use ars_native::{ARSValue, COMPARISONS, MOVES};
use rayon::prelude::*;
use std::mem::MaybeUninit;
use std::ptr;

pub struct ARSAero;

const BUFFER_SIZE: usize = 8;
const TABLE_SIZE: usize = 1024;

struct AeroMapper {
    min: u64,
    multiplier: u128,
    table: [u16; TABLE_SIZE],
    num_bins: usize,
    is_uniform: bool,
}

impl AeroMapper {
    #[inline(always)]
    fn map(&self, k: u64) -> usize {
        let diff = k.wrapping_sub(self.min);
        let idx = ((diff as u128 * self.multiplier) >> 64) as usize;
        if self.is_uniform {
            if idx >= self.num_bins {
                self.num_bins - 1
            } else {
                idx
            }
        } else {
            let t_idx = if idx >= TABLE_SIZE {
                TABLE_SIZE - 1
            } else {
                idx
            };
            self.table[t_idx] as usize
        }
    }
}

impl ARSAero {
    /// ARS Gen 6: Aero Architecture (Unstable)
    pub fn sort<T: ARSValue + PartialOrd + Send + Sync + Clone>(data: &mut [T]) {
        Self::sort_internal(data, false);
    }

    /// ARS Gen 6: Aero Architecture (Stable)
    pub fn sort_stable<T: ARSValue + PartialOrd + Send + Sync + Clone>(data: &mut [T]) {
        Self::sort_internal(data, true);
    }

    fn sort_internal<T: ARSValue + PartialOrd + Send + Sync + Clone>(data: &mut [T], stable: bool) {
        let n = data.len();
        if n < 2048 {
            if stable {
                data.sort_by(|a, b| a.partial_cmp(b).unwrap());
            } else {
                data.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            }
            return;
        }

        let (min_v, max_v, sorted, reversed) = Self::analyze_parallel(data);
        if sorted {
            return;
        }
        if reversed {
            data.reverse();
            return;
        }

        let num_threads = rayon::current_num_threads();
        let num_bins = 256;
        let mapper = Self::create_mapper(data, num_bins, min_v, max_v);

        let chunk_size = (n + num_threads - 1) / num_threads;
        let thread_hists: Vec<Vec<usize>> = (0..num_threads)
            .into_par_iter()
            .map(|t_idx| {
                let start = t_idx * chunk_size;
                let mut local_hist = vec![0usize; num_bins];
                if start < n {
                    let end = (start + chunk_size).min(n);
                    let slice = &data[start..end];
                    let mut i = 0;
                    while i + 8 <= slice.len() {
                        local_hist[mapper.map(slice[i].to_spatial_u64())] += 1;
                        local_hist[mapper.map(slice[i + 1].to_spatial_u64())] += 1;
                        local_hist[mapper.map(slice[i + 2].to_spatial_u64())] += 1;
                        local_hist[mapper.map(slice[i + 3].to_spatial_u64())] += 1;
                        local_hist[mapper.map(slice[i + 4].to_spatial_u64())] += 1;
                        local_hist[mapper.map(slice[i + 5].to_spatial_u64())] += 1;
                        local_hist[mapper.map(slice[i + 6].to_spatial_u64())] += 1;
                        local_hist[mapper.map(slice[i + 7].to_spatial_u64())] += 1;
                        i += 8;
                    }
                    while i < slice.len() {
                        local_hist[mapper.map(slice[i].to_spatial_u64())] += 1;
                        i += 1;
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

        Self::parallel_move(
            data,
            &mapper,
            &thread_starts,
            n,
            num_bins,
            num_threads,
            chunk_size,
        );

        let data_ptr = data.as_mut_ptr() as usize;
        (0..num_bins).into_par_iter().for_each(|b_idx| {
            let start = bin_offsets[b_idx];
            let end = bin_offsets[b_idx + 1];
            if end > start + 1 {
                let s_slice = unsafe {
                    std::slice::from_raw_parts_mut((data_ptr as *mut T).add(start), end - start)
                };
                if stable {
                    s_slice.sort_by(|a, b| a.partial_cmp(b).unwrap());
                } else {
                    s_slice.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
                }
            }
        });
    }

    fn parallel_move<T: ARSValue + Clone + Send + Sync>(
        data: &mut [T],
        mapper: &AeroMapper,
        thread_starts: &Vec<Vec<usize>>,
        n: usize,
        num_bins: usize,
        num_threads: usize,
        chunk_size: usize,
    ) {
        let mut scratch: Vec<MaybeUninit<T>> = Vec::with_capacity(n);
        unsafe {
            scratch.set_len(n);
        }
        let scratch_ptr = scratch.as_mut_ptr() as usize;
        let data_ptr = data.as_mut_ptr() as usize;

        (0..num_threads).into_par_iter().for_each(|t_idx| {
            let start = t_idx * chunk_size;
            if start < n {
                let end = (start + chunk_size).min(n);
                let mut local_pos = thread_starts[t_idx].clone();
                let s_raw = scratch_ptr as *mut T;
                let d_raw = data_ptr as *mut T;

                let mut buffer_counts = vec![0u8; num_bins];
                let mut buffer_data: Vec<MaybeUninit<T>> =
                    Vec::with_capacity(num_bins * BUFFER_SIZE);
                unsafe {
                    buffer_data.set_len(num_bins * BUFFER_SIZE);
                }
                let b_raw = buffer_data.as_mut_ptr() as *mut T;

                for i in start..end {
                    unsafe {
                        let val_ptr = d_raw.add(i);
                        let k = (*val_ptr).to_spatial_u64();
                        let b = mapper.map(k);
                        let b_count = buffer_counts[b] as usize;

                        let val = ptr::read(val_ptr);
                        ptr::write(b_raw.add(b * BUFFER_SIZE + b_count), val);
                        buffer_counts[b] += 1;

                        if buffer_counts[b] as usize == BUFFER_SIZE {
                            let pos = local_pos[b];
                            ptr::copy_nonoverlapping(
                                b_raw.add(b * BUFFER_SIZE),
                                s_raw.add(pos),
                                BUFFER_SIZE,
                            );
                            local_pos[b] += BUFFER_SIZE;
                            buffer_counts[b] = 0;
                        }
                    }
                }

                for b in 0..num_bins {
                    let rem = buffer_counts[b] as usize;
                    if rem > 0 {
                        let pos = local_pos[b];
                        unsafe {
                            ptr::copy_nonoverlapping(
                                b_raw.add(b * BUFFER_SIZE),
                                s_raw.add(pos),
                                rem,
                            );
                        }
                    }
                }
            }
        });

        let s_ptr = scratch_ptr as *const T;
        let d_ptr = data_ptr as *mut T;
        unsafe {
            ptr::copy_nonoverlapping(s_ptr, d_ptr, n);
            scratch.set_len(0);
        }
    }

    fn create_mapper<T: ARSValue + Sync>(
        data: &[T],
        num_bins: usize,
        min_v: u64,
        max_v: u64,
    ) -> AeroMapper {
        let n = data.len();
        let range = (max_v - min_v).max(1);

        let mut sample = Vec::with_capacity(256);
        let mut rng = 42u64;
        for _ in 0..256 {
            rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
            sample.push(data[(rng as usize) % n].to_spatial_u64());
        }
        sample.sort_unstable();

        let mut is_uniform = true;
        for i in 1..16 {
            let idx = (i * 256) / 16;
            let expected = min_v + ((range as u128 * i as u128) / 16) as u64;
            if (sample[idx] as i128 - expected as i128).abs() as u64 > range / 32 {
                is_uniform = false;
                break;
            }
        }

        let mut table = [0u16; TABLE_SIZE];
        let multiplier = if is_uniform {
            ((num_bins as u128 - 1) << 64) / range as u128
        } else {
            let mult = ((TABLE_SIZE as u128) << 64) / range as u128;
            for i in 0..TABLE_SIZE {
                let val = min_v + (((range as u128 * i as u128) / TABLE_SIZE as u128) as u64);
                let mut s_idx = 0;
                while s_idx < 256 && sample[s_idx] < val {
                    s_idx += 1;
                }
                table[i] = ((s_idx * num_bins) / 256).min(num_bins - 1) as u16;
            }
            mult
        };

        AeroMapper {
            min: min_v,
            multiplier,
            table,
            num_bins,
            is_uniform,
        }
    }

    fn analyze_parallel<T: ARSValue + PartialOrd + Send + Sync>(
        data: &[T],
    ) -> (u64, u64, bool, bool) {
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
