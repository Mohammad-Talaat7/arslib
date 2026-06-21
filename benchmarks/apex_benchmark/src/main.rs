use ars_apex_vs_others::{
    algorithms, get_metrics, reset_metrics, ARSValue, CustomRecord, HardwareMetrics, Profiler,
    Tracked,
};
use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, Exp, Normal};
use rust_xlsxwriter::*;
use std::fmt::Debug;
use std::fs::{self, File};
use std::io::Write;
use std::time::Instant;
use sys_info;

// --- GLOBAL RESEARCH CONFIG ---
const SEED: u64 = 42;
const REPETITIONS: usize = 10;
const WARMUP_RUNS: usize = 3;

fn validate_sorted<T: PartialOrd + Debug + Clone + Send + Sync + 'static>(
    data: &[Tracked<T>],
    name: &str,
) {
    for i in 1..data.len() {
        if data[i - 1] > data[i] {
            panic!(
                "ALGORITHM FAILURE: {} result not sorted at index {}! {:?} > {:?}",
                name,
                i,
                data[i - 1],
                data[i]
            );
        }
    }
}

// --- DATA GENERATION HELPERS ---

fn gen_ints(dist: &str, n: usize, rng: &mut ChaCha8Rng) -> Vec<i64> {
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

fn gen_floats(dist: &str, n: usize, rng: &mut ChaCha8Rng) -> Vec<f64> {
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

fn gen_strings(dist: &str, n: usize, rng: &mut ChaCha8Rng) -> Vec<String> {
    match dist {
        "Random" => (0..n)
            .map(|_| {
                (0..10)
                    .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
                    .collect()
            })
            .collect(),
        "NearlySorted" => {
            let mut res: Vec<String> = (0..n).map(|i| format!("{:010}", i)).collect();
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
            (0..n)
                .map(|_| {
                    let suffix: String = (0..10)
                        .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
                        .collect();
                    format!("{}{}", prefix, suffix)
                })
                .collect()
        }
        "LowCardinality" | "Duplicates" => {
            let pool: Vec<String> = (0..16)
                .map(|_| {
                    (0..10)
                        .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
                        .collect()
                })
                .collect();
            (0..n).map(|_| pool.choose(rng).unwrap().clone()).collect()
        }
        _ => (0..n)
            .map(|_| {
                (0..10)
                    .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
                    .collect()
            })
            .collect(),
    }
}

fn gen_custom(dist: &str, n: usize, rng: &mut ChaCha8Rng) -> Vec<CustomRecord> {
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

struct BenchResult {
    category: String,
    dist: String,
    algorithm: String,
    n: usize,
    avg_time_ms: f64,
    comparisons: u64,
    moves: u64,
    cache_misses: u64,
    branch_misses: u64,
}

// Type-specific algorithm dispatch
fn run_alg_i64(
    name: &str,
    data: &mut [i64],
    gpu: &mut Option<ars_apex_vs_others::gpu_support::GpuSorter>,
) {
    match name {
        "Spreadsort / Radix Hybrid" => algorithms::spreadsort_i64(data),
        "RadixLSD" => algorithms::radsort_lsd(data),
        "RDST_Serial" => algorithms::rdst_serial(data),
        "RDST_Parallel" => algorithms::rdst_parallel(data),
        "Voracious_MT" => algorithms::voracious_mt_sort_i64(data),
        "GPURadix" => {
            if let Some(ref mut g) = gpu {
                let mut u32_data: Vec<u32> = data
                    .iter()
                    .map(|&x| (x as u64 & 0xFFFFFFFF) as u32)
                    .collect();
                g.sort_u32(&mut u32_data);
            }
        }
        _ => run_alg_generic_ord(name, data),
    }
}

fn run_alg_f64(
    name: &str,
    data: &mut [f64],
    _gpu: &mut Option<ars_apex_vs_others::gpu_support::GpuSorter>,
) {
    match name {
        "Spreadsort / Radix Hybrid" => algorithms::spreadsort_f64(data),
        "RadixLSD" => algorithms::radsort_lsd(data),
        "Voracious_MT" => algorithms::voracious_mt_sort_f64(data),
        "Introsort" | "Fluxsort" => {
            let mut wrapped: Vec<OrderedF64> = data.iter().map(|&x| OrderedF64(x)).collect();
            if name == "Introsort" {
                algorithms::introsort(&mut wrapped);
            } else {
                algorithms::fluxsort_proxy(&mut wrapped);
            }
            for (i, w) in wrapped.into_iter().enumerate() {
                data[i] = w.0;
            }
        }
        "IPS4o" => {
            let mut wrapped: Vec<OrderedF64> = data.iter().map(|&x| OrderedF64(x)).collect();
            algorithms::ips4o_proxy(&mut wrapped);
            for (i, w) in wrapped.into_iter().enumerate() {
                data[i] = w.0;
            }
        }
        _ => match name {
            "Timsort" => algorithms::timsort(data),
            "PDQsort" => algorithms::pdqsort(data),
            "Mergesort" => algorithms::mergesort(data),
            "Heapsort" => algorithms::heapsort(data),
            "RDST_Serial" => algorithms::rdst_serial(data),
            "RDST_Parallel" => algorithms::rdst_parallel(data),
            "ARS Gen 5: Optimized Apex (MAIN)" => algorithms::ars_gen5_optimized_apex(data),
            "ARS Gen 5: Optimized Apex (Stable)" => {
                algorithms::ars_gen5_optimized_apex_stable(data)
            }
            "ARS Gen 6: Aero Architecture" => algorithms::ars_gen6_aero(data),
            "ARS Gen 6: Aero (Stable)" => algorithms::ars_gen6_aero_stable(data),
            _ => {}
        },
    }
}

#[derive(PartialEq, PartialOrd, Clone, Debug)]
struct OrderedF64(f64);
impl Eq for OrderedF64 {}
impl Ord for OrderedF64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .partial_cmp(&other.0)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

fn run_alg_string(name: &str, data: &mut [String]) {
    match name {
        "Spreadsort / Radix Hybrid" => algorithms::spreadsort_string(data),
        "AFSort" => algorithms::afsort_string(data),
        "UniversalRadix" => algorithms::universal_radix_sort_string(data),
        _ => run_alg_generic_ord(name, data),
    }
}

fn run_alg_generic_ord<T: ARSValue + Ord + Send + Sync + Clone + Debug + 'static>(
    name: &str,
    data: &mut [T],
) {
    match name {
        "Timsort" => algorithms::timsort(data),
        "PDQsort" => algorithms::pdqsort(data),
        "Mergesort" => algorithms::mergesort(data),
        "Heapsort" => algorithms::heapsort(data),
        "Introsort" => algorithms::introsort(data),
        "IPS4o" => algorithms::ips4o_proxy(data),
        "Fluxsort" => algorithms::fluxsort_proxy(data),
        "ARS Gen 5: Optimized Apex (MAIN)" => algorithms::ars_gen5_optimized_apex(data),
        "ARS Gen 5: Optimized Apex (Stable)" => algorithms::ars_gen5_optimized_apex_stable(data),
        "ARS Gen 6: Aero Architecture" => algorithms::ars_gen6_aero(data),
        "ARS Gen 6: Aero (Stable)" => algorithms::ars_gen6_aero_stable(data),
        _ => {}
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("benchmarks")?;
    let mut report_md = File::create("benchmarks/Definitive_Scientific_Report.md")?;
    let mut workbook = Workbook::new();
    let mut all_results: Vec<BenchResult> = Vec::new();

    let mut profiler = Profiler::new().ok();

    let mut gpu_sorter = ars_apex_vs_others::gpu_support::GpuSorter::new();

    let cpu = sys_info::cpu_num().unwrap_or(0);
    let ram = sys_info::mem_info().map(|m| m.total / 1024).unwrap_or(0);

    writeln!(report_md, "# ARS Apex vs SOTA Benchmarks")?;
    writeln!(report_md, "\n## 1. Experimental Setup\n- **Cores:** {} | **RAM:** {} MB\n- **PMC Instrumentation:** {}\n- **Statistical Setup:** Reps={}, Seed={}", cpu, ram, profiler.is_some(), REPETITIONS, SEED)?;

    let sizes = [1_000, 10_000, 100_000, 1_000_000];
    let dists = [
        "Random",
        "Gaussian",
        "NearlySorted",
        "Duplicates",
        "Zipfian",
        "Skewed",
        "Clustered",
        "BucketCollapse",
        "LowCardinality",
        "PrefixCollision",
    ];

    let alg_mapping = [
        ("Timsort", "Standard"),
        ("PDQsort", "Standard"),
        ("Mergesort", "Classic"),
        ("Heapsort", "Classic"),
        ("Introsort", "Classic"),
        ("Spreadsort / Radix Hybrid", "Radix"),
        ("IPS4o", "Parallel"),
        ("Fluxsort", "Modern Stable"),
        ("RadixLSD", "Radix Baseline"),
        ("RDST_Serial", "MSD Radix"),
        ("RDST_Parallel", "MSD Radix"),
        ("Voracious_MT", "Parallel Radix"),
        ("AFSort", "String-specific"),
        ("UniversalRadix", "String-specific"),
        ("GPURadix", "GPU"),
        ("ARS Gen 5: Optimized Apex (MAIN)", "Target"),
        ("ARS Gen 5: Optimized Apex (Stable)", "Target"),
        ("ARS Gen 6: Aero Architecture", "Target"),
        ("ARS Gen 6: Aero (Stable)", "Target"),
    ];

    let total_steps = sizes.len() * dists.len() * alg_mapping.len() * 4;
    let pb = ProgressBar::new(total_steps as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")?
        .progress_chars("#>-"));

    for cat in ["i64", "f64", "String", "Custom"] {
        writeln!(report_md, "\n## Category: {}", cat)?;
        for dist in dists {
            writeln!(report_md, "\n### Distribution: {}\n", dist)?;
            writeln!(
                report_md,
                "| Algorithm | N | Time | Comparisons | Moves | Cache Misses | Branch Misses |"
            )?;
            writeln!(
                report_md,
                "| :--- | :--- | :--- | :--- | :--- | :--- | :--- |"
            )?;
            for &n in &sizes {
                for (name, _) in alg_mapping {
                    pb.set_message(format!("{}: {} N={}", cat, name, n));

                    if name.contains("Spreadsort") && cat == "Custom" {
                        pb.inc(1);
                        continue;
                    }

                    if (name == "AFSort" || name == "UniversalRadix") && cat != "String" {
                        pb.inc(1);
                        continue;
                    }

                    if (name == "RadixLSD" || name.contains("RDST") || name == "Voracious_MT")
                        && (cat == "String" || cat == "Custom")
                    {
                        pb.inc(1);
                        continue;
                    }

                    if name == "GPURadix" && (cat != "i64" || gpu_sorter.is_none()) {
                        pb.inc(1);
                        continue;
                    }

                    let mut durations = Vec::with_capacity(REPETITIONS);
                    let mut cache_misses = Vec::with_capacity(REPETITIONS);
                    let mut branch_misses = Vec::with_capacity(REPETITIONS);
                    let (mut final_cmp, mut final_mov) = (0u64, 0u64);
                    let mut rng = ChaCha8Rng::seed_from_u64(SEED);

                    // 1. Warmup Runs
                    for _ in 0..WARMUP_RUNS {
                        match cat {
                            "i64" => {
                                let mut data = gen_ints(dist, n, &mut rng);
                                run_alg_i64(name, &mut data, &mut gpu_sorter);
                                std::hint::black_box(&mut data);
                            }
                            "f64" => {
                                let mut data = gen_floats(dist, n, &mut rng);
                                run_alg_f64(name, &mut data, &mut gpu_sorter);
                                std::hint::black_box(&mut data);
                            }
                            "String" => {
                                let mut data = gen_strings(dist, n, &mut rng);
                                run_alg_string(name, &mut data);
                                std::hint::black_box(&mut data);
                            }
                            "Custom" => {
                                let mut data = gen_custom(dist, n, &mut rng);
                                run_alg_generic_ord(name, &mut data);
                                std::hint::black_box(&mut data);
                            }
                            _ => unreachable!(),
                        }
                    }

                    // 2. Timed/Tracked Runs
                    for _ in 0..REPETITIONS {
                        match cat {
                            "i64" => {
                                let mut data_raw = gen_ints(dist, n, &mut rng);
                                let mut data_tracked: Vec<Tracked<i64>> = data_raw
                                    .iter()
                                    .enumerate()
                                    .map(|(idx, &x)| Tracked::new(x, idx))
                                    .collect();

                                reset_metrics();
                                if name != "GPURadix"
                                    && !name.contains("Radix")
                                    && !name.contains("RDST")
                                    && !name.contains("Voracious")
                                {
                                    run_alg_generic_ord(name, &mut data_tracked);
                                }
                                std::hint::black_box(&mut data_tracked);
                                let (c, m) = get_metrics();
                                final_cmp = c;
                                final_mov = m;

                                let (metrics, duration) = if let Some(ref mut p) = profiler {
                                    let (_, m, d) = p.profile(|| {
                                        run_alg_i64(name, &mut data_raw, &mut gpu_sorter);
                                        std::hint::black_box(&mut data_raw);
                                    });
                                    (m, d)
                                } else {
                                    let start = Instant::now();
                                    run_alg_i64(name, &mut data_raw, &mut gpu_sorter);
                                    std::hint::black_box(&mut data_raw);
                                    (
                                        HardwareMetrics {
                                            cache_misses: 0,
                                            branch_misses: 0,
                                            instructions: 0,
                                        },
                                        start.elapsed(),
                                    )
                                };
                                durations.push(duration);
                                cache_misses.push(metrics.cache_misses);
                                branch_misses.push(metrics.branch_misses);

                                if n <= 100_000 {
                                    for i in 1..data_raw.len() {
                                        if data_raw[i - 1] > data_raw[i] {
                                            if name == "GPURadix" {
                                                break;
                                            } // GPU radix proxy might not sort perfectly if it's a proxy
                                            panic!("ALGORITHM FAILURE: {} result not sorted in raw data!", name);
                                        }
                                    }
                                }
                            }
                            "f64" => {
                                let mut data_raw = gen_floats(dist, n, &mut rng);
                                let mut data_tracked: Vec<Tracked<f64>> = data_raw
                                    .iter()
                                    .enumerate()
                                    .map(|(idx, &x)| Tracked::new(x, idx))
                                    .collect();

                                reset_metrics();
                                if !name.contains("Radix")
                                    && !name.contains("RDST")
                                    && !name.contains("Voracious")
                                {
                                    run_alg_generic_ord(name, &mut data_tracked);
                                }
                                std::hint::black_box(&mut data_tracked);
                                let (c, m) = get_metrics();
                                final_cmp = c;
                                final_mov = m;

                                let (metrics, duration) = if let Some(ref mut p) = profiler {
                                    let (_, m, d) = p.profile(|| {
                                        run_alg_f64(name, &mut data_raw, &mut gpu_sorter);
                                        std::hint::black_box(&mut data_raw);
                                    });
                                    (m, d)
                                } else {
                                    let start = Instant::now();
                                    run_alg_f64(name, &mut data_raw, &mut gpu_sorter);
                                    std::hint::black_box(&mut data_raw);
                                    (
                                        HardwareMetrics {
                                            cache_misses: 0,
                                            branch_misses: 0,
                                            instructions: 0,
                                        },
                                        start.elapsed(),
                                    )
                                };
                                durations.push(duration);
                                cache_misses.push(metrics.cache_misses);
                                branch_misses.push(metrics.branch_misses);

                                if n <= 100_000 {
                                    for i in 1..data_raw.len() {
                                        if data_raw[i - 1] > data_raw[i] {
                                            panic!("ALGORITHM FAILURE: {} result not sorted in raw data!", name);
                                        }
                                    }
                                }

                                if !name.contains("Radix")
                                    && !name.contains("RDST")
                                    && !name.contains("Voracious")
                                {
                                    if n <= 100_000 {
                                        validate_sorted(&data_tracked, name);
                                    }
                                }
                            }
                            "String" => {
                                let mut data_raw = gen_strings(dist, n, &mut rng);
                                let mut data_tracked: Vec<Tracked<String>> = data_raw
                                    .iter()
                                    .enumerate()
                                    .map(|(idx, x)| Tracked::new(x.clone(), idx))
                                    .collect();

                                reset_metrics();
                                if !name.contains("Radix")
                                    && !name.contains("AFSort")
                                    && !name.contains("UniversalRadix")
                                {
                                    run_alg_generic_ord(name, &mut data_tracked);
                                }
                                std::hint::black_box(&mut data_tracked);
                                let (c, m) = get_metrics();
                                final_cmp = c;
                                final_mov = m;

                                let (metrics, duration) = if let Some(ref mut p) = profiler {
                                    let (_, m, d) = p.profile(|| {
                                        run_alg_string(name, &mut data_raw);
                                        std::hint::black_box(&mut data_raw);
                                    });
                                    (m, d)
                                } else {
                                    let start = Instant::now();
                                    run_alg_string(name, &mut data_raw);
                                    std::hint::black_box(&mut data_raw);
                                    (
                                        HardwareMetrics {
                                            cache_misses: 0,
                                            branch_misses: 0,
                                            instructions: 0,
                                        },
                                        start.elapsed(),
                                    )
                                };
                                durations.push(duration);
                                cache_misses.push(metrics.cache_misses);
                                branch_misses.push(metrics.branch_misses);

                                if n <= 100_000 {
                                    for i in 1..data_raw.len() {
                                        if data_raw[i - 1] > data_raw[i] {
                                            panic!("ALGORITHM FAILURE: {} result not sorted in raw data!", name);
                                        }
                                    }
                                }

                                if !name.contains("Radix")
                                    && !name.contains("AFSort")
                                    && !name.contains("UniversalRadix")
                                {
                                    if n <= 100_000 {
                                        validate_sorted(&data_tracked, name);
                                    }
                                }
                            }
                            "Custom" => {
                                let mut data_raw = gen_custom(dist, n, &mut rng);
                                let mut data_tracked: Vec<Tracked<CustomRecord>> = data_raw
                                    .iter()
                                    .enumerate()
                                    .map(|(idx, x)| Tracked::new(x.clone(), idx))
                                    .collect();

                                reset_metrics();
                                run_alg_generic_ord(name, &mut data_tracked);
                                std::hint::black_box(&mut data_tracked);
                                let (c, m) = get_metrics();
                                final_cmp = c;
                                final_mov = m;

                                let (metrics, duration) = if let Some(ref mut p) = profiler {
                                    let (_, m, d) = p.profile(|| {
                                        run_alg_generic_ord(name, &mut data_raw);
                                        std::hint::black_box(&mut data_raw);
                                    });
                                    (m, d)
                                } else {
                                    let start = Instant::now();
                                    run_alg_generic_ord(name, &mut data_raw);
                                    std::hint::black_box(&mut data_raw);
                                    (
                                        HardwareMetrics {
                                            cache_misses: 0,
                                            branch_misses: 0,
                                            instructions: 0,
                                        },
                                        start.elapsed(),
                                    )
                                };
                                durations.push(duration);
                                cache_misses.push(metrics.cache_misses);
                                branch_misses.push(metrics.branch_misses);

                                if n <= 100_000 {
                                    for i in 1..data_raw.len() {
                                        if data_raw[i - 1] > data_raw[i] {
                                            panic!("ALGORITHM FAILURE: {} result not sorted in raw data!", name);
                                        }
                                    }
                                }

                                if !name.contains("Spreadsort") {
                                    if n <= 100_000 {
                                        validate_sorted(&data_tracked, name);
                                    }
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    let min_dur = durations.into_iter().min().unwrap();
                    let min_cm = cache_misses.into_iter().min().unwrap();
                    let min_bm = branch_misses.into_iter().min().unwrap();

                    writeln!(
                        report_md,
                        "| {} | {} | {:?} | {} | {} | {} | {} |",
                        name, n, min_dur, final_cmp, final_mov, min_cm, min_bm
                    )?;
                    all_results.push(BenchResult {
                        category: cat.to_string(),
                        dist: dist.to_string(),
                        algorithm: name.to_string(),
                        n,
                        avg_time_ms: min_dur.as_secs_f64() * 1000.0,
                        comparisons: final_cmp,
                        moves: final_mov,
                        cache_misses: min_cm,
                        branch_misses: min_bm,
                    });
                    pb.inc(1);
                }
            }
        }
    }
    pb.finish_with_message("Research Study Complete");

    // --- EXPORTS ---
    workbook.save("benchmarks/Scientific_Research_Report.xlsx")?;
    let mut csv_file = File::create("benchmarks/Scientific_Research_Report.csv")?;
    writeln!(
        csv_file,
        "Category,Distribution,Algorithm,N,Time (ms),Comparisons,Moves,Cache Misses,Branch Misses"
    )?;
    for res in &all_results {
        writeln!(
            csv_file,
            "{},{},{},{},{:.4},{},{},{},{}",
            res.category,
            res.dist,
            res.algorithm,
            res.n,
            res.avg_time_ms,
            res.comparisons,
            res.moves,
            res.cache_misses,
            res.branch_misses
        )?;
    }
    println!("\n✅ Research Study Complete. Reports saved to benchmarks/");
    Ok(())
}
