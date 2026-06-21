#![allow(warnings)]
pub use ars_native::{ARSValue, COMPARISONS, MOVES};
use ars_optimized_apex::ARSOptimizedApex;
use crossbeam_channel::Sender;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

const ADJACENCY_THRESHOLD_BITS: u32 = 44;
const L0_COMPACTION_THRESHOLD: usize = 8;
const NUM_WORKERS: usize = 4; // Multi-worker compaction

#[derive(Clone, Debug)]
pub struct SpatialRun<T> {
    pub min: u64,
    pub max: u64,
    pub data: Vec<T>,
}

impl<T: ARSValue + PartialOrd + Clone + std::fmt::Debug> SpatialRun<T> {
    pub fn new(data: Vec<T>) -> Self {
        if data.is_empty() {
            return Self {
                min: u64::MAX,
                max: 0,
                data,
            };
        }
        let min = data[0].to_spatial_u64();
        let max = data.last().unwrap().to_spatial_u64();
        Self { min, max, data }
    }

    #[inline]
    pub fn spatial_key(&self) -> u64 {
        self.min >> ADJACENCY_THRESHOLD_BITS
    }

    pub fn is_adjacent(&self, other: &Self) -> bool {
        self.spatial_key() == other.spatial_key()
            || (self.max >> ADJACENCY_THRESHOLD_BITS) == other.spatial_key()
    }

    pub fn merge(self, other: Self) -> Self {
        let mut combined = Vec::with_capacity(self.data.len() + other.data.len());
        let mut a_iter = self.data.into_iter();
        let mut b_iter = other.data.into_iter();

        let mut a_next = a_iter.next();
        let mut b_next = b_iter.next();

        loop {
            match (a_next.take(), b_next.take()) {
                (Some(a), Some(b)) => {
                    if a.partial_cmp(&b).unwrap() <= std::cmp::Ordering::Equal {
                        combined.push(a);
                        b_next = Some(b);
                        a_next = a_iter.next();
                    } else {
                        combined.push(b);
                        a_next = Some(a);
                        b_next = b_iter.next();
                    }
                }
                (Some(a), None) => {
                    combined.push(a);
                    combined.extend(a_iter);
                    break;
                }
                (None, Some(b)) => {
                    combined.push(b);
                    combined.extend(b_iter);
                    break;
                }
                (None, None) => break,
            }
        }

        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
            data: combined,
        }
    }
}

#[derive(Debug)]
struct TieredSpatialBucket<T> {
    l0: Vec<SpatialRun<T>>,
    l1: Option<SpatialRun<T>>,
}

impl<T: ARSValue + PartialOrd + Clone + std::fmt::Debug> TieredSpatialBucket<T> {
    fn new() -> Self {
        Self {
            l0: Vec::with_capacity(L0_COMPACTION_THRESHOLD),
            l1: None,
        }
    }

    fn insert(&mut self, run: SpatialRun<T>) {
        self.l0.push(run);
        if self.l0.len() >= L0_COMPACTION_THRESHOLD {
            let mut runs = std::mem::take(&mut self.l0);
            let mut merged = runs.remove(0);
            for next in runs {
                merged = merged.merge(next);
            }

            if let Some(existing_l1) = self.l1.take() {
                self.l1 = Some(existing_l1.merge(merged));
            } else {
                self.l1 = Some(merged);
            }
        }
    }
}

/// ARS Gen 13.3 (Concurrent Tiered): Parallel Streaming via Granular Spatial Locking
pub struct ARSStreamer<T: ARSValue + PartialOrd + Send + Sync + Clone + 'static + std::fmt::Debug> {
    sender: Sender<Vec<T>>,
    active_chunk: Vec<T>,
    chunk_threshold: usize,
    buckets: Arc<Mutex<HashMap<u64, Arc<Mutex<TieredSpatialBucket<T>>>>>>,
    handles: Vec<std::thread::JoinHandle<()>>,
}

impl<T: ARSValue + PartialOrd + Send + Sync + Clone + 'static + std::fmt::Debug> ARSStreamer<T> {
    pub fn new() -> Self {
        Self::new_bounded(131072, 128)
    }

    pub fn new_bounded(threshold: usize, cap: usize) -> Self {
        let (tx, rx) = crossbeam_channel::bounded::<Vec<T>>(cap);
        let buckets = Arc::new(Mutex::new(
            HashMap::<u64, Arc<Mutex<TieredSpatialBucket<T>>>>::new(),
        ));
        let mut handles = Vec::new();

        for _ in 0..NUM_WORKERS {
            let rx_worker = rx.clone();
            let buckets_worker = Arc::clone(&buckets);

            handles.push(std::thread::spawn(move || {
                while let Ok(mut chunk) = rx_worker.recv() {
                    ARSOptimizedApex::sort(&mut chunk);
                    let new_run = SpatialRun::new(chunk);
                    let key = new_run.spatial_key();
                    let bucket_arc = {
                        let mut map = buckets_worker.lock().unwrap();
                        map.entry(key)
                            .or_insert_with(|| Arc::new(Mutex::new(TieredSpatialBucket::new())))
                            .clone()
                    };
                    let mut bucket = bucket_arc.lock().unwrap();
                    bucket.insert(new_run);
                }
            }));
        }

        Self {
            sender: tx,
            active_chunk: Vec::with_capacity(threshold),
            chunk_threshold: threshold,
            buckets,
            handles,
        }
    }

    pub fn push(&mut self, val: T) {
        self.active_chunk.push(val);
        if self.active_chunk.len() >= self.chunk_threshold {
            let chunk = std::mem::replace(
                &mut self.active_chunk,
                Vec::with_capacity(self.chunk_threshold),
            );
            self.sender.send(chunk).unwrap();
        }
    }

    pub fn push_batch(&mut self, data: &[T]) {
        for val in data {
            self.push(val.clone());
        }
    }

    pub fn collect(mut self) -> Vec<T> {
        if !self.active_chunk.is_empty() {
            let mut chunk = std::mem::take(&mut self.active_chunk);
            ARSOptimizedApex::sort(&mut chunk);
            let last_run = SpatialRun::new(chunk);
            let key = last_run.spatial_key();
            let bucket_arc = {
                let mut map = self.buckets.lock().unwrap();
                map.entry(key)
                    .or_insert_with(|| Arc::new(Mutex::new(TieredSpatialBucket::new())))
                    .clone()
            };
            bucket_arc.lock().unwrap().insert(last_run);
        }

        drop(self.sender);
        for h in self.handles {
            h.join().unwrap();
        }

        let mut map = std::mem::take(&mut *self.buckets.lock().unwrap());
        let mut raw_data = Vec::new();
        for (_, bucket_mutex) in map.drain() {
            let bucket = Arc::try_unwrap(bucket_mutex).unwrap().into_inner().unwrap();
            for r in bucket.l0 {
                raw_data.push(r.data);
            }
            if let Some(r) = bucket.l1 {
                raw_data.push(r.data);
            }
        }

        if raw_data.is_empty() {
            return Vec::new();
        }
        if raw_data.len() == 1 {
            return raw_data.remove(0);
        }
        Self::parallel_merge_all(raw_data)
    }

    fn parallel_merge_all(mut runs: Vec<Vec<T>>) -> Vec<T> {
        while runs.len() > 1 {
            let mut pairs = Vec::with_capacity((runs.len() + 1) / 2);
            let mut iter = runs.into_iter();
            while let Some(r1) = iter.next() {
                if let Some(r2) = iter.next() {
                    pairs.push((Some(r1), Some(r2)));
                } else {
                    pairs.push((Some(r1), None));
                }
            }
            runs = pairs
                .into_par_iter()
                .map(|(r1, r2)| match (r1, r2) {
                    (Some(a), Some(b)) => Self::merge_two_owned(a, b),
                    (Some(a), None) => a,
                    _ => unreachable!(),
                })
                .collect();
        }
        if runs.is_empty() {
            Vec::new()
        } else {
            runs.remove(0)
        }
    }

    fn merge_two_owned(a_vec: Vec<T>, b_vec: Vec<T>) -> Vec<T> {
        let mut res = Vec::with_capacity(a_vec.len() + b_vec.len());
        let mut a_iter = a_vec.into_iter();
        let mut b_iter = b_vec.into_iter();
        let mut a_next = a_iter.next();
        let mut b_next = b_iter.next();
        loop {
            match (a_next.take(), b_next.take()) {
                (Some(a), Some(b)) => {
                    if a.partial_cmp(&b).unwrap() <= std::cmp::Ordering::Equal {
                        res.push(a);
                        b_next = Some(b);
                        a_next = a_iter.next();
                    } else {
                        res.push(b);
                        a_next = Some(a);
                        b_next = b_iter.next();
                    }
                }
                (Some(a), None) => {
                    res.push(a);
                    res.extend(a_iter);
                    break;
                }
                (None, Some(b)) => {
                    res.push(b);
                    res.extend(b_iter);
                    break;
                }
                (None, None) => break,
            }
        }
        res
    }

    fn merge_two(a: &[T], b: &[T]) -> Vec<T> {
        Self::merge_two_owned(a.to_vec(), b.to_vec())
    }
}
