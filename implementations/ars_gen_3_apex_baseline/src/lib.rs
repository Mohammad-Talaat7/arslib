#![allow(warnings)]
use rayon::prelude::*;
use std::sync::atomic::AtomicU64;

pub static COMPARISONS: AtomicU64 = AtomicU64::new(0);
pub static MOVES: AtomicU64 = AtomicU64::new(0);

pub trait ARSValue: Clone + Send + Sync {
    fn to_spatial_u64(&self) -> u64;
}

impl ARSValue for i64 {
    #[inline(always)]
    fn to_spatial_u64(&self) -> u64 {
        (*self as u64).wrapping_add(i64::MIN as u64)
    }
}
impl ARSValue for u64 {
    #[inline(always)]
    fn to_spatial_u64(&self) -> u64 {
        *self
    }
}
impl ARSValue for i32 {
    #[inline(always)]
    fn to_spatial_u64(&self) -> u64 {
        (*self as u64).wrapping_add(i32::MIN as u64)
    }
}
impl ARSValue for f64 {
    #[inline(always)]
    fn to_spatial_u64(&self) -> u64 {
        let u = self.to_bits();
        if u & 0x8000_0000_0000_0000 != 0 {
            !u
        } else {
            u | 0x8000_0000_0000_0000
        }
    }
}

impl ARSValue for String {
    #[inline(always)]
    fn to_spatial_u64(&self) -> u64 {
        let b = self.as_bytes();
        let mut res = 0u64;
        let len = b.len().min(8);
        for (i, &val) in b.iter().enumerate().take(len) {
            res |= (val as u64) << (56 - i * 8);
        }
        res
    }
}

pub struct ARSApex;

// Hardware constant: size of an L1 cache line (usually 64 bytes)
// We want to buffer elements before pushing to Vec to allow sequential write combining.
const BLOCK_SIZE: usize = 16;

impl ARSApex {
    pub fn sort<T: ARSValue + PartialOrd + Send + Clone>(data: &mut [T]) {
        let n = data.len();
        if n < 1024 {
            data.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            return;
        }

        let (_keys, sorted, reversed) = Self::analyze_fast(data);
        if sorted {
            return;
        }
        if reversed {
            data.reverse();
            // Removed manual atomic increments as Tracked<T> handles this.
            return;
        }

        let num_bins = (n / 256).clamp(256, 1024).next_power_of_two();
        let mut sample = Vec::with_capacity(num_bins * 16);
        let step = n / (num_bins * 16).min(n);
        for i in 0..((num_bins * 16).min(n)) {
            sample.push(data[i * step].clone());
        }
        sample.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

        let mut boundaries = Vec::with_capacity(num_bins - 1);
        let b_step = sample.len() / num_bins;
        for i in 1..num_bins {
            boundaries.push(sample[i * b_step].clone());
        }

        // --- RESTORING LOCALITY VIA BLOCK BUFFERING ---
        // We collect elements in small stack-allocated blocks before pushing to the heap.
        let mut bins: Vec<Vec<T>> = (0..num_bins)
            .map(|_| Vec::with_capacity(n / num_bins + 64))
            .collect();
        let mut local_buffers: Vec<Vec<T>> = (0..num_bins)
            .map(|_| Vec::with_capacity(BLOCK_SIZE))
            .collect();

        for val in data.iter() {
            let b = boundaries.partition_point(|x| x < val);
            let lbuf = &mut local_buffers[b];
            lbuf.push(val.clone());

            if lbuf.len() == BLOCK_SIZE {
                // Bulk move to bin (Sequentializing the write)
                bins[b].append(lbuf);
            }
        }
        // Flush remaining
        for b in 0..num_bins {
            if !local_buffers[b].is_empty() {
                bins[b].append(&mut local_buffers[b]);
            }
        }
        // Removed manual atomic increments as Tracked<T> handles this.

        let sorted_chunks: Vec<(Vec<T>, u64)> = bins
            .into_par_iter()
            .map(|mut bin| {
                let mut local_cmp = 0u64;
                if bin.len() > 1 {
                    bin.sort_unstable_by(|a, b| {
                        local_cmp += 1;
                        a.partial_cmp(b).unwrap()
                    });
                }
                (bin, local_cmp)
            })
            .collect();

        let mut curr = 0;
        for (bin, _cmp) in sorted_chunks {
            // Removed manual atomic increments as Tracked<T> handles this.
            let _blen = bin.len();
            for val in bin {
                data[curr] = val;
                curr += 1;
            }
            // Removed manual atomic increments as Tracked<T> handles this.
        }
    }

    pub fn sort_stable<T: ARSValue + PartialOrd + Send + Clone>(data: &mut [T]) {
        let n = data.len();
        if n < 1024 {
            data.sort_by(|a, b| a.partial_cmp(b).unwrap());
            return;
        }

        let (_keys, sorted, reversed) = Self::analyze_fast(data);
        if sorted {
            return;
        }
        if reversed {
            data.reverse();
            return;
        }

        let num_bins = (n / 256).clamp(256, 1024).next_power_of_two();
        let mut sample = Vec::with_capacity(num_bins * 16);
        let step = n / (num_bins * 16).min(n);
        for i in 0..((num_bins * 16).min(n)) {
            sample.push(data[i * step].clone());
        }
        sample.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut boundaries = Vec::with_capacity(num_bins - 1);
        let b_step = sample.len() / num_bins;
        for i in 1..num_bins {
            boundaries.push(sample[i * b_step].clone());
        }

        let mut bins: Vec<Vec<T>> = (0..num_bins)
            .map(|_| Vec::with_capacity(n / num_bins + 64))
            .collect();
        let mut local_buffers: Vec<Vec<T>> = (0..num_bins)
            .map(|_| Vec::with_capacity(BLOCK_SIZE))
            .collect();

        for val in data.iter() {
            let b = boundaries.partition_point(|x| x < val);
            let lbuf = &mut local_buffers[b];
            lbuf.push(val.clone());
            if lbuf.len() == BLOCK_SIZE {
                bins[b].append(lbuf);
            }
        }
        for b in 0..num_bins {
            if !local_buffers[b].is_empty() {
                bins[b].append(&mut local_buffers[b]);
            }
        }

        let sorted_chunks: Vec<(Vec<T>, u64)> = bins
            .into_par_iter()
            .map(|mut bin| {
                let mut local_cmp = 0u64;
                if bin.len() > 1 {
                    bin.sort_by(|a, b| {
                        local_cmp += 1;
                        a.partial_cmp(b).unwrap()
                    });
                }
                (bin, local_cmp)
            })
            .collect();

        let mut curr = 0;
        for (bin, _cmp) in sorted_chunks {
            // Removed manual atomic increments as Tracked<T> handles this.
            let _blen = bin.len();
            for val in bin {
                data[curr] = val;
                curr += 1;
            }
            // Removed manual atomic increments as Tracked<T> handles this.
        }
    }

    fn analyze_fast<T: ARSValue + PartialOrd + Send + Sync>(data: &[T]) -> (Vec<u64>, bool, bool) {
        let n = data.len();
        if n < 2 {
            return (vec![], true, false);
        }

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

        (vec![], sorted, reversed)
    }
}
