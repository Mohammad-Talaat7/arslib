use ars_native::{ARSApex, ARSValue};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rayon::prelude::*;
use std::fmt::Debug;
use std::time::{Duration, Instant};
use sys_info;

// --- CONFIGURATION ---
const REPETITIONS: usize = 3;
const SEED: u64 = 42;

// --- CUSTOM TYPE ---
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct HeavyRecord {
    id: i64,
    payload: [u8; 32],
}
impl ARSValue for HeavyRecord {
    fn to_spatial_u64(&self) -> u64 {
        (self.id as u64).wrapping_add(i64::MIN as u64)
    }
}

// --- LEGACY SIMULATIONS ---
struct Gen1Legacy;
impl Gen1Legacy {
    fn sort<T: Ord + Clone>(data: &mut [T]) {
        let mut v = Vec::with_capacity(data.len());
        for x in data.iter() {
            let pos = v.binary_search(x).unwrap_or_else(|e| e);
            v.insert(pos, x.clone());
        }
        for (i, x) in v.into_iter().enumerate() {
            data[i] = x;
        }
    }
}

struct Gen3Grid;
impl Gen3Grid {
    fn sort<T: ARSValue + PartialOrd + Clone>(data: &mut [T]) {
        let n = data.len();
        if n < 128 {
            data.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            return;
        }
        let (mut min, mut max) = (data[0].to_spatial_u64(), data[0].to_spatial_u64());
        for x in data.iter() {
            let v = x.to_spatial_u64();
            if v < min {
                min = v;
            }
            if v > max {
                max = v;
            }
        }
        let range = (max - min).max(1) as f64;
        let num_bins = (n / 100).max(16).min(1024);
        let mut bins = vec![Vec::new(); num_bins];
        for x in data.iter() {
            let v = x.to_spatial_u64();
            let b = (((v.wrapping_sub(min)) as f64 / range) * (num_bins as f64 - 1.0)) as usize;
            bins[b.min(num_bins - 1)].push(x.clone());
        }
        let mut i = 0;
        for mut b in bins {
            b.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            for x in b {
                data[i] = x;
                i += 1;
            }
        }
    }
}

// --- BENCHMARK DRIVER ---
fn main() {
    println!("=== ARS RESEARCH ATLAS v2.0 ===");
    let cpu_num = sys_info::cpu_num().unwrap_or(0);
    let mem_total = sys_info::mem_info().map(|m| m.total / 1024).unwrap_or(0);
    println!("System: {} Cores, {} MB RAM", cpu_num, mem_total);
    println!("---------------------------------\n");

    let sizes = [10_000, 1_000_000, 10_000_000];
    let dists = ["Random", "Sorted", "Clustered"];

    for &n in &sizes {
        println!("🚀 N = {}", n);
        println!(
            "{:<20} | {:<12} | {:<15}",
            "Algorithm", "Distribution", "Avg Time"
        );
        println!("{:-<55}", "");

        for &dist in &dists {
            let mut rng = ChaCha8Rng::seed_from_u64(SEED);
            let algs = ["Std Unstable", "Std Stable", "ARS Gen 3", "ARS Apex"];

            for name in algs {
                let mut total_dur = Duration::ZERO;
                for _ in 0..REPETITIONS {
                    let mut data: Vec<i64> = match dist {
                        "Random" => (0..n).map(|_| rng.gen()).collect(),
                        "Sorted" => (0..n as i64).collect(),
                        "Clustered" => {
                            let mut v = Vec::with_capacity(n);
                            for _ in 0..10 {
                                let c = rng.gen_range(0..i64::MAX - 10000);
                                for _ in 0..(n / 10) {
                                    v.push(rng.gen_range(c..c + 10000));
                                }
                            }
                            v
                        }
                        _ => vec![],
                    };

                    let start = Instant::now();
                    match name {
                        "Std Unstable" => data.sort_unstable(),
                        "Std Stable" => data.sort(),
                        "ARS Gen 3" => Gen3Grid::sort(&mut data),
                        "ARS Apex" => ARSApex::sort(&mut data),
                        _ => {}
                    }
                    total_dur += start.elapsed();
                }
                println!(
                    "{:<20} | {:<12} | {:<15?}",
                    name,
                    dist,
                    total_dur / REPETITIONS as u32
                );
            }
        }
        println!("{:-<55}", "");
    }
}
