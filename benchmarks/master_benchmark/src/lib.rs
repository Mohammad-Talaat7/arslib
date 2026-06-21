#![allow(warnings)]
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, Exp, Normal};
use std::fmt::Debug;
use std::sync::atomic::Ordering;

pub use arslib::{ARSValue, COMPARISONS, MOVES};

pub mod hardware_profiler;
pub use hardware_profiler::{HardwareMetrics, Profiler};

// --- DATA GENERATION HELPERS ---

pub fn gen_ints(dist: &str, n: usize, rng: &mut ChaCha8Rng) -> Vec<i64> {
    match dist {
        "Random" => (0..n).map(|_| rng.gen()).collect(),
        "Gaussian" => {
            let normal = Normal::new(0.0, 1000.0).unwrap();
            (0..n).map(|_| normal.sample(rng) as i64).collect()
        }
        "NearlySorted" => {
            let mut v: Vec<i64> = (0..n as i64).collect();
            for _ in 0..n / 10 {
                let i = rng.gen_range(0..n);
                let j = rng.gen_range(0..n);
                v.swap(i, j);
            }
            v
        }
        "Duplicates" => {
            let choices = [10, 20, 30, 40, 50];
            (0..n).map(|_| *choices.choose(rng).unwrap()).collect()
        }
        "Zipfian" => {
            let s = 0.75;
            (0..n)
                .map(|_| {
                    let rank = (rng.gen::<f64>().powf(-1.0 / s)).floor() as i64;
                    rank % (n as i64)
                })
                .collect()
        }
        "Skewed" => {
            let exp = Exp::new(1.0).unwrap();
            (0..n).map(|_| (exp.sample(rng) * 1000.0) as i64).collect()
        }
        "Clustered" => {
            let centers: Vec<f64> = (0..10).map(|_| rng.gen_range(-10000.0..10000.0)).collect();
            let normal = Normal::new(0.0, 10.0).unwrap();
            (0..n)
                .map(|_| {
                    let center = centers.choose(rng).unwrap();
                    (center + normal.sample(rng)) as i64
                })
                .collect()
        }
        "BucketCollapse" | "PrefixCollision" => {
            let base = rng.gen::<i64>() & 0xFFFFFFFF00000000u64 as i64;
            (0..n)
                .map(|_| base | (rng.gen::<i32>() as i64 & 0xFFFFFFFF))
                .collect()
        }
        "LowCardinality" => {
            let pool: Vec<i64> = (0..16).map(|_| rng.gen()).collect();
            (0..n).map(|_| *pool.choose(rng).unwrap()).collect()
        }
        _ => (0..n).map(|_| rng.gen()).collect(),
    }
}

pub fn gen_floats(dist: &str, n: usize, rng: &mut ChaCha8Rng) -> Vec<f64> {
    match dist {
        "Random" => (0..n).map(|_| rng.gen()).collect(),
        "Gaussian" => {
            let normal = Normal::new(0.0, 1.0).unwrap();
            (0..n).map(|_| normal.sample(rng)).collect()
        }
        "NearlySorted" => {
            let mut v: Vec<f64> = (0..n).map(|i| i as f64).collect();
            for _ in 0..n / 10 {
                let i = rng.gen_range(0..n);
                let j = rng.gen_range(0..n);
                v.swap(i, j);
            }
            v
        }
        "Duplicates" => {
            let choices = [1.1, 2.2, 3.3, 4.4, 5.5];
            (0..n).map(|_| *choices.choose(rng).unwrap()).collect()
        }
        "Zipfian" => {
            let s = 0.75;
            (0..n)
                .map(|_| {
                    let rank = (rng.gen::<f64>().powf(-1.0 / s)).floor();
                    rank % (n as f64)
                })
                .collect()
        }
        "Skewed" => {
            let exp = Exp::new(1.0).unwrap();
            (0..n).map(|_| exp.sample(rng)).collect()
        }
        "Clustered" => {
            let centers: Vec<f64> = (0..10).map(|_| rng.gen_range(-100.0..100.0)).collect();
            let normal = Normal::new(0.0, 0.1).unwrap();
            (0..n)
                .map(|_| {
                    let center = centers.choose(rng).unwrap();
                    center + normal.sample(rng)
                })
                .collect()
        }
        "BucketCollapse" | "PrefixCollision" => {
            let base = rng.gen_range(-1000.0..1000.0f64).floor();
            (0..n)
                .map(|_| base + (rng.gen::<f64>() * 0.0000001))
                .collect()
        }
        "LowCardinality" => {
            let pool: Vec<f64> = (0..16).map(|_| rng.gen()).collect();
            (0..n).map(|_| *pool.choose(rng).unwrap()).collect()
        }
        _ => (0..n).map(|_| rng.gen()).collect(),
    }
}

pub fn gen_strings(dist: &str, n: usize, rng: &mut ChaCha8Rng) -> Vec<String> {
    // String Pool Strategy: Pre-allocate a pool of strings to reduce total heap allocations.
    // This allows 10M+ strings on limited RAM without changing the algorithm's work.
    let pool_size = if n >= 1_000_000 { 100_000 } else { 16 };
    let mut pool: Vec<String> = (0..pool_size)
        .map(|_| {
            (0..10)
                .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
                .collect()
        })
        .collect();

    match dist {
        "Random" => (0..n).map(|_| pool.choose(rng).unwrap().clone()).collect(),
        "NearlySorted" => {
            // Sort the pool first to provide a nearly sorted source
            pool.sort_unstable();
            let mut res: Vec<String> = (0..n).map(|i| pool[i % pool_size].clone()).collect();
            for _ in 0..n / 10 {
                let i = rng.gen_range(0..n);
                let j = rng.gen_range(0..n);
                res.swap(i, j);
            }
            res
        }
        "PrefixCollision" => {
            let prefix: String = (0..50)
                .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
                .collect();
            let p_collision_pool: Vec<String> = (0..pool_size)
                .map(|_| {
                    let suffix: String = (0..10)
                        .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
                        .collect();
                    format!("{}{}", prefix, suffix)
                })
                .collect();
            (0..n)
                .map(|_| p_collision_pool.choose(rng).unwrap().clone())
                .collect()
        }
        "LowCardinality" | "Duplicates" => {
            let mini_pool: Vec<String> = (0..16)
                .map(|_| {
                    (0..10)
                        .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
                        .collect()
                })
                .collect();
            (0..n)
                .map(|_| mini_pool.choose(rng).unwrap().clone())
                .collect()
        }
        _ => (0..n).map(|_| pool.choose(rng).unwrap().clone()).collect(),
    }
}

pub fn gen_custom(dist: &str, n: usize, rng: &mut ChaCha8Rng) -> Vec<CustomRecord> {
    let scores = gen_ints(dist, n, rng);
    (0..n)
        .zip(scores)
        .map(|(i, score)| CustomRecord {
            id: i as i64,
            score,
            metadata: rng.gen(),
        })
        .collect()
}

// --- SCIENTIFIC INSTRUMENTATION CORE ---

pub fn reset_metrics() {
    COMPARISONS.store(0, Ordering::SeqCst);
    MOVES.store(0, Ordering::SeqCst);
}

pub fn get_metrics() -> (u64, u64) {
    (
        COMPARISONS.load(Ordering::SeqCst),
        MOVES.load(Ordering::SeqCst),
    )
}

#[derive(Debug)]
pub struct Tracked<T: PartialOrd + Clone + Debug + Send + Sync> {
    pub inner: T,
    pub original_index: usize,
}

impl<T: PartialOrd + Clone + Debug + Send + Sync> Tracked<T> {
    pub fn new(inner: T, original_index: usize) -> Self {
        Self {
            inner,
            original_index,
        }
    }
}

impl<T: PartialOrd + Clone + Debug + Send + Sync> Clone for Tracked<T> {
    fn clone(&self) -> Self {
        MOVES.fetch_add(1, Ordering::Relaxed);
        Self {
            inner: self.inner.clone(),
            original_index: self.original_index,
        }
    }
}

impl<T: PartialOrd + Clone + Debug + Send + Sync> PartialEq for Tracked<T> {
    fn eq(&self, other: &Self) -> bool {
        COMPARISONS.fetch_add(1, Ordering::Relaxed);
        self.inner.partial_cmp(&other.inner) == Some(std::cmp::Ordering::Equal)
    }
}

impl<T: PartialOrd + Clone + Debug + Send + Sync> Eq for Tracked<T> {}

impl<T: PartialOrd + Clone + Debug + Send + Sync> PartialOrd for Tracked<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        COMPARISONS.fetch_add(1, Ordering::Relaxed);
        self.inner.partial_cmp(&other.inner)
    }
}

impl<T: PartialOrd + Clone + Debug + Send + Sync> Ord for Tracked<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        COMPARISONS.fetch_add(1, Ordering::Relaxed);
        self.inner
            .partial_cmp(&other.inner)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl<T: ARSValue + PartialOrd + Clone + Debug + Send + Sync> ARSValue for Tracked<T> {
    fn to_spatial_u64(&self) -> u64 {
        self.inner.to_spatial_u64()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CustomRecord {
    pub id: i64,
    pub score: i64,
    pub metadata: u64,
}

impl PartialOrd for CustomRecord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.score.cmp(&other.score))
    }
}

impl Ord for CustomRecord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl ARSValue for CustomRecord {
    fn to_spatial_u64(&self) -> u64 {
        self.score.to_spatial_u64()
    }
}

// --- ALGORITHMS PHYLOGENY ---

pub mod algorithms {
    use super::*;

    pub fn quicksort<T: PartialOrd>(data: &mut [T]) {
        data.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    }
    pub fn timsort<T: PartialOrd>(data: &mut [T]) {
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    pub fn std_parallel_sort<T: PartialOrd + Send>(data: &mut [T]) {
        use rayon::prelude::*;
        data.par_sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    }

    pub fn ars_gen1_foundation<T: ARSValue + PartialOrd + Send + Sync + Clone>(data: &mut [T]) {
        let mut s = ars_recursive::ARSGeneric::new(1);
        let res = s.sort_parallel(data.to_vec());
        data.clone_from_slice(&res);
    }

    pub fn ars_gen2_grid_mapping<T: ARSValue + PartialOrd + Send + Sync + Clone>(data: &mut [T]) {
        let mut s = ars_spatial_grid::ARSGeneric::new(1);
        let res = s.sort_parallel(data.to_vec());
        data.clone_from_slice(&res);
    }

    pub fn ars_gen3_apex_baseline_unstable<T: ARSValue + PartialOrd + Send + Clone>(
        data: &mut [T],
    ) {
        ars_native::ARSApex::sort(data);
    }

    pub fn ars_gen4_parallel_apex<T: ARSValue + PartialOrd + Send + Clone>(data: &mut [T]) {
        ars_parallel_apex::ARSMassivelyParallel::sort(data);
    }

    pub fn ars_gen5_optimized_apex<T: ARSValue + PartialOrd + Send + Clone>(data: &mut [T]) {
        ars_optimized_apex::ARSOptimizedApex::sort(data);
    }

    pub fn ars_gen5_optimized_apex_stable<T: ARSValue + PartialOrd + Send + Clone>(data: &mut [T]) {
        ars_optimized_apex::ARSOptimizedApex::sort_stable(data);
    }

    pub fn ars_gen6_aero<T: ARSValue + PartialOrd + Send + Sync + Clone>(data: &mut [T]) {
        ars_gen_6_aero::ARSAero::sort(data);
    }

    pub fn ars_gen6_aero_stable<T: ARSValue + PartialOrd + Send + Sync + Clone>(data: &mut [T]) {
        ars_gen_6_aero::ARSAero::sort_stable(data);
    }

    pub fn ars_exp_a_recursive_parallel<T: ARSValue + PartialOrd + Send + Clone>(data: &mut [T]) {
        ars_recursive_parallel::ARSRecursiveParallel::sort(data);
    }

    pub fn ars_exp_b_hierarchical_staging<T: ARSValue + PartialOrd + Send + Clone>(data: &mut [T]) {
        ars_hierarchical_staging::ARSHierarchicalParallel::sort(data);
    }

    pub fn ars_exp_c_adaptive_hierarchical<T: ARSValue + PartialOrd + Send + Clone>(
        data: &mut [T],
    ) {
        ars_adaptive_hierarchical::ARSAdaptiveHierarchical::sort(data);
    }

    pub fn ars_exp_d_stream_microbatch<
        T: ARSValue + PartialOrd + Send + Sync + Clone + 'static + std::fmt::Debug,
    >(
        data: &mut [T],
    ) {
        let mut streamer = ars_stream_microbatch::ARSStreamer::new();
        streamer.push_batch(data);
        let res = streamer.collect();
        data.clone_from_slice(&res);
    }
}
