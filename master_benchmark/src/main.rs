use ars_master_benchmark::{
    algorithms, gen_custom, gen_floats, gen_ints, gen_strings, get_metrics, reset_metrics,
    ARSValue, CustomRecord, HardwareMetrics, Profiler, Tracked,
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

extern "C" {
    fn malloc_trim(pad: usize) -> i32;
}

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

fn calculate_stats(durations: &[std::time::Duration]) -> (f64, f64, f64) {
    if durations.is_empty() {
        return (0.0, 0.0, 0.0);
    }
    let mut ms: Vec<f64> = durations.iter().map(|d| d.as_secs_f64() * 1000.0).collect();
    ms.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let min = ms[0];
    let median = if ms.len() % 2 == 0 {
        (ms[ms.len() / 2 - 1] + ms[ms.len() / 2]) / 2.0
    } else {
        ms[ms.len() / 2]
    };

    let mean = ms.iter().sum::<f64>() / ms.len() as f64;
    let variance = ms.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / ms.len() as f64;
    let std_dev = variance.sqrt();
    let cov = if mean > 0.0 {
        (std_dev / mean) * 100.0
    } else {
        0.0
    };

    (min, median, cov)
}

struct BenchResult {
    category: String,
    dist: String,
    algorithm: String,
    n: usize,
    min_ms: f64,
    median_ms: f64,
    cov: f64,
    comparisons: u64,
    moves: u64,
    llc_miss_rate: f64,
    branch_misses: u64,
    ipc: f64,
    bandwidth_mbs: f64,
}

fn run_alg<T: ARSValue + PartialOrd + Clone + Debug + Send + Sync + 'static>(
    name: &str,
    data: &mut [T],
) {
    match name {
        "Quicksort" => algorithms::quicksort(data),
        "Timsort" => algorithms::timsort(data),
        "ARS Gen 1: Foundation" => algorithms::ars_gen1_foundation(data),
        "ARS Gen 2: Grid Mapping" => algorithms::ars_gen2_grid_mapping(data),
        "ARS Gen 3: Apex Baseline" => algorithms::ars_gen3_apex_baseline_unstable(data),
        "ARS Gen 4: Parallel Apex" => algorithms::ars_gen4_parallel_apex(data),
        "ARS Gen 5: Optimized Apex (MAIN)" => algorithms::ars_gen5_optimized_apex(data),
        "ARS Gen 5: Optimized Apex (Stable)" => algorithms::ars_gen5_optimized_apex_stable(data),
        "ARS Gen 6: Aero Architecture" => algorithms::ars_gen6_aero(data),
        "ARS Gen 6: Aero (Stable)" => algorithms::ars_gen6_aero_stable(data),
        "ARS Exp A: Recursive Parallel" => algorithms::ars_exp_a_recursive_parallel(data),
        "ARS Exp B: Hierarchical Staging" => algorithms::ars_exp_b_hierarchical_staging(data),
        "ARS Exp C: Adaptive Hierarchical" => algorithms::ars_exp_c_adaptive_hierarchical(data),
        "ARS Exp D: Stream Micro-Batch" => algorithms::ars_exp_d_stream_microbatch(data),
        _ => {}
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("benchmarks")?;
    let mut report_md = File::create("benchmarks/Definitive_Scientific_Report.md")?;
    let mut workbook = Workbook::new();
    let mut all_results: Vec<BenchResult> = Vec::new();

    let mut profiler = Profiler::new().ok();

    let cpu = sys_info::cpu_num().unwrap_or(0);
    let ram = sys_info::mem_info().map(|m| m.total / 1024).unwrap_or(0);

    writeln!(report_md, "# ARS Evolution Atlas: Final Research Study")?;
    writeln!(report_md, "\n## 1. Experimental Setup\n- **Cores:** {} | **RAM:** {} MB\n- **PMC Instrumentation:** {} (Multi-thread Inherit: Enabled)\n- **Statistical Setup:** Reps={}, Seed={}", cpu, ram, profiler.is_some(), REPETITIONS, SEED)?;

    let sizes = [1_000, 10_000, 100_000, 1_000_000, 10_000_000];
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
        ("Quicksort", "Standard"),
        ("Timsort", "Standard"),
        ("ARS Gen 1: Foundation", "Core Lineage"),
        ("ARS Gen 2: Grid Mapping", "Core Lineage"),
        ("ARS Gen 3: Apex Baseline", "Core Lineage"),
        ("ARS Gen 4: Parallel Apex", "Core Lineage"),
        ("ARS Gen 5: Optimized Apex (MAIN)", "MAIN Lineage"),
        ("ARS Gen 5: Optimized Apex (Stable)", "MAIN Lineage"),
        ("ARS Gen 6: Aero Architecture", "MAIN Lineage"),
        ("ARS Gen 6: Aero (Stable)", "MAIN Lineage"),
        ("ARS Exp A: Recursive Parallel", "Experimental"),
        ("ARS Exp B: Hierarchical Staging", "Experimental"),
        ("ARS Exp C: Adaptive Hierarchical", "Experimental"),
        ("ARS Exp D: Stream Micro-Batch", "Experimental"),
    ];

    let total_steps = sizes.len() * dists.len() * alg_mapping.len() * 4;
    let pb = ProgressBar::new(total_steps as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")?
        .progress_chars("#>-"));

    for cat in ["i64", "f64", "String", "Custom"] {
        let type_size = match cat {
            "i64" | "f64" => 8,
            "String" => 32, // Approx
            "Custom" => 24,
            _ => 8,
        };
        writeln!(report_md, "\n## Category: {}", cat)?;
        for dist in dists {
            writeln!(report_md, "\n### Distribution: {}\n", dist)?;
            writeln!(report_md, "| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |")?;
            writeln!(
                report_md,
                "| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |"
            )?;
            for &n in &sizes {
                for (name, _) in alg_mapping {
                    pb.set_message(format!("{}: {} N={}", cat, name, n));

                    // 1. Scientific Skip Guards
                    if (name.contains("Gen 1") || name.contains("Gen 2")) && n > 100_000 {
                        pb.inc(1);
                        continue;
                    }

                    // String Memory Wall: Gen 4/5 require auxiliary copies that exceed 16GB RAM for 10M Strings
                    if (name.contains("Gen 4") || name.contains("Gen 5"))
                        && cat == "String"
                        && n >= 10_000_000
                    {
                        writeln!(
                            report_md,
                            "| {} | {} | **OOM** | **OOM** | - | - | - | - | - | - |",
                            name, n
                        )?;
                        pb.inc(1);
                        continue;
                    }

                    let mut durations = Vec::with_capacity(REPETITIONS);
                    let mut cache_misses = Vec::with_capacity(REPETITIONS);
                    let mut cache_refs = Vec::with_capacity(REPETITIONS);
                    let mut branch_misses = Vec::with_capacity(REPETITIONS);
                    let mut instructions = Vec::with_capacity(REPETITIONS);
                    let mut cpu_cycles = Vec::with_capacity(REPETITIONS);

                    let (mut final_cmp, mut final_mov) = (0u64, 0u64);
                    let mut rng = ChaCha8Rng::seed_from_u64(SEED);

                    // 1. Warmup Runs
                    for _ in 0..WARMUP_RUNS {
                        match cat {
                            "i64" => {
                                let mut data = gen_ints(dist, n, &mut rng);
                                run_alg(name, &mut data);
                                std::hint::black_box(&mut data);
                            }
                            "f64" => {
                                let mut data = gen_floats(dist, n, &mut rng);
                                run_alg(name, &mut data);
                                std::hint::black_box(&mut data);
                            }
                            "String" => {
                                let mut data = gen_strings(dist, n, &mut rng);
                                run_alg(name, &mut data);
                                std::hint::black_box(&mut data);
                            }
                            "Custom" => {
                                let mut data = gen_custom(dist, n, &mut rng);
                                run_alg(name, &mut data);
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
                                drop(data_raw);
                                unsafe {
                                    malloc_trim(0);
                                }
                                std::thread::sleep(std::time::Duration::from_millis(500));

                                reset_metrics();
                                run_alg(name, &mut data_tracked);
                                std::hint::black_box(&mut data_tracked);
                                let (c, m) = get_metrics();
                                final_cmp = c;
                                final_mov = m;
                                drop(data_tracked);
                                unsafe {
                                    malloc_trim(0);
                                }
                                std::thread::sleep(std::time::Duration::from_millis(500));

                                let mut data_raw_prof = gen_ints(dist, n, &mut rng);
                                let (metrics, duration) = if let Some(ref mut p) = profiler {
                                    let (_, m, d) = p.profile(|| {
                                        run_alg(name, &mut data_raw_prof);
                                        std::hint::black_box(&mut data_raw_prof);
                                    });
                                    (m, d)
                                } else {
                                    let start = Instant::now();
                                    run_alg(name, &mut data_raw_prof);
                                    std::hint::black_box(&mut data_raw_prof);
                                    (
                                        HardwareMetrics {
                                            cache_misses: 0,
                                            cache_references: 0,
                                            branch_misses: 0,
                                            instructions: 0,
                                            cpu_cycles: 0,
                                        },
                                        start.elapsed(),
                                    )
                                };
                                durations.push(duration);
                                cache_misses.push(metrics.cache_misses);
                                cache_refs.push(metrics.cache_references);
                                branch_misses.push(metrics.branch_misses);
                                instructions.push(metrics.instructions);
                                cpu_cycles.push(metrics.cpu_cycles);
                                drop(data_raw_prof);
                                unsafe {
                                    malloc_trim(0);
                                }
                                std::thread::sleep(std::time::Duration::from_millis(500));
                            }
                            "f64" => {
                                let mut data_raw = gen_floats(dist, n, &mut rng);
                                let mut data_tracked: Vec<Tracked<f64>> = data_raw
                                    .iter()
                                    .enumerate()
                                    .map(|(idx, &x)| Tracked::new(x, idx))
                                    .collect();
                                drop(data_raw);
                                unsafe {
                                    malloc_trim(0);
                                }
                                std::thread::sleep(std::time::Duration::from_millis(500));

                                reset_metrics();
                                run_alg(name, &mut data_tracked);
                                std::hint::black_box(&mut data_tracked);
                                let (c, m) = get_metrics();
                                final_cmp = c;
                                final_mov = m;
                                drop(data_tracked);
                                unsafe {
                                    malloc_trim(0);
                                }
                                std::thread::sleep(std::time::Duration::from_millis(500));

                                let mut data_raw_prof = gen_floats(dist, n, &mut rng);
                                let (metrics, duration) = if let Some(ref mut p) = profiler {
                                    let (_, m, d) = p.profile(|| {
                                        run_alg(name, &mut data_raw_prof);
                                        std::hint::black_box(&mut data_raw_prof);
                                    });
                                    (m, d)
                                } else {
                                    let start = Instant::now();
                                    run_alg(name, &mut data_raw_prof);
                                    std::hint::black_box(&mut data_raw_prof);
                                    (
                                        HardwareMetrics {
                                            cache_misses: 0,
                                            cache_references: 0,
                                            branch_misses: 0,
                                            instructions: 0,
                                            cpu_cycles: 0,
                                        },
                                        start.elapsed(),
                                    )
                                };
                                durations.push(duration);
                                cache_misses.push(metrics.cache_misses);
                                cache_refs.push(metrics.cache_references);
                                branch_misses.push(metrics.branch_misses);
                                instructions.push(metrics.instructions);
                                cpu_cycles.push(metrics.cpu_cycles);
                                drop(data_raw_prof);
                                unsafe {
                                    malloc_trim(0);
                                }
                                std::thread::sleep(std::time::Duration::from_millis(500));
                            }
                            "String" => {
                                let data_raw = gen_strings(dist, n, &mut rng);
                                let mut data_tracked: Vec<Tracked<String>> = data_raw
                                    .into_iter()
                                    .enumerate()
                                    .map(|(idx, x)| Tracked::new(x, idx))
                                    .collect();
                                unsafe {
                                    malloc_trim(0);
                                }
                                std::thread::sleep(std::time::Duration::from_millis(1000));

                                reset_metrics();
                                run_alg(name, &mut data_tracked);
                                std::hint::black_box(&mut data_tracked);
                                let (c, m) = get_metrics();
                                final_cmp = c;
                                final_mov = m;
                                drop(data_tracked);
                                unsafe {
                                    malloc_trim(0);
                                }
                                std::thread::sleep(std::time::Duration::from_millis(1000));

                                let data_raw_prof = gen_strings(dist, n, &mut rng);
                                let (metrics, duration) = if let Some(ref mut p) = profiler {
                                    let mut data = data_raw_prof;
                                    let (_, m, d) = p.profile(|| {
                                        run_alg(name, &mut data);
                                        std::hint::black_box(&mut data);
                                    });
                                    drop(data);
                                    unsafe {
                                        malloc_trim(0);
                                    }
                                    (m, d)
                                } else {
                                    let mut data = data_raw_prof;
                                    let start = Instant::now();
                                    run_alg(name, &mut data);
                                    std::hint::black_box(&mut data);
                                    drop(data);
                                    unsafe {
                                        malloc_trim(0);
                                    }
                                    (
                                        HardwareMetrics {
                                            cache_misses: 0,
                                            cache_references: 0,
                                            branch_misses: 0,
                                            instructions: 0,
                                            cpu_cycles: 0,
                                        },
                                        start.elapsed(),
                                    )
                                };
                                durations.push(duration);
                                cache_misses.push(metrics.cache_misses);
                                cache_refs.push(metrics.cache_references);
                                branch_misses.push(metrics.branch_misses);
                                instructions.push(metrics.instructions);
                                cpu_cycles.push(metrics.cpu_cycles);
                                std::thread::sleep(std::time::Duration::from_millis(1000));
                            }
                            "Custom" => {
                                let mut data_raw = gen_custom(dist, n, &mut rng);
                                let mut data_tracked: Vec<Tracked<CustomRecord>> = data_raw
                                    .into_iter()
                                    .enumerate()
                                    .map(|(idx, x)| Tracked::new(x, idx))
                                    .collect();
                                unsafe {
                                    malloc_trim(0);
                                }
                                std::thread::sleep(std::time::Duration::from_millis(500));

                                reset_metrics();
                                run_alg(name, &mut data_tracked);
                                std::hint::black_box(&mut data_tracked);
                                let (c, m) = get_metrics();
                                final_cmp = c;
                                final_mov = m;
                                drop(data_tracked);
                                unsafe {
                                    malloc_trim(0);
                                }
                                std::thread::sleep(std::time::Duration::from_millis(500));

                                let mut data_raw_prof = gen_custom(dist, n, &mut rng);
                                let (metrics, duration) = if let Some(ref mut p) = profiler {
                                    let (_, m, d) = p.profile(|| {
                                        run_alg(name, &mut data_raw_prof);
                                        std::hint::black_box(&mut data_raw_prof);
                                    });
                                    (m, d)
                                } else {
                                    let start = Instant::now();
                                    run_alg(name, &mut data_raw_prof);
                                    std::hint::black_box(&mut data_raw_prof);
                                    (
                                        HardwareMetrics {
                                            cache_misses: 0,
                                            cache_references: 0,
                                            branch_misses: 0,
                                            instructions: 0,
                                            cpu_cycles: 0,
                                        },
                                        start.elapsed(),
                                    )
                                };
                                durations.push(duration);
                                cache_misses.push(metrics.cache_misses);
                                cache_refs.push(metrics.cache_references);
                                branch_misses.push(metrics.branch_misses);
                                instructions.push(metrics.instructions);
                                cpu_cycles.push(metrics.cpu_cycles);
                                drop(data_raw_prof);
                                unsafe {
                                    malloc_trim(0);
                                }
                                std::thread::sleep(std::time::Duration::from_millis(500));
                            }
                            _ => unreachable!(),
                        }
                    }
                    let (min_ms, median_ms, cov) = calculate_stats(&durations);
                    let min_dur = durations.into_iter().min().unwrap();
                    let min_cm = cache_misses.into_iter().min().unwrap();
                    let min_cr = cache_refs.into_iter().min().unwrap();
                    let min_bm = branch_misses.into_iter().min().unwrap();
                    let min_inst = instructions.into_iter().min().unwrap();
                    let min_cyc = cpu_cycles.into_iter().min().unwrap();

                    let llc_miss_rate = if min_cr > 0 {
                        min_cm as f64 / min_cr as f64
                    } else {
                        0.0
                    };
                    let ipc = if min_cyc > 0 {
                        min_inst as f64 / min_cyc as f64
                    } else {
                        0.0
                    };
                    let bandwidth =
                        (2 * n * type_size) as f64 / (min_dur.as_secs_f64() * 1024.0 * 1024.0);

                    writeln!(report_md, "| {} | {} | {:.4}ms | {:.4}ms | {:.2}% | {} | {} | {:.2}% | {:.2} | {:.2} MB/s |", 
                             name, n, min_ms, median_ms, cov, final_cmp, final_mov, llc_miss_rate * 100.0, ipc, bandwidth)?;

                    all_results.push(BenchResult {
                        category: cat.to_string(),
                        dist: dist.to_string(),
                        algorithm: name.to_string(),
                        n,
                        min_ms,
                        median_ms,
                        cov,
                        comparisons: final_cmp,
                        moves: final_mov,
                        llc_miss_rate,
                        branch_misses: min_bm,
                        ipc,
                        bandwidth_mbs: bandwidth,
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
    writeln!(csv_file, "Category,Distribution,Algorithm,N,MinTime(ms),MedianTime(ms),CoV(%),Comparisons,Moves,LLC Miss Rate,Branch Misses,IPC,Throughput (MB/s)")?;
    for res in &all_results {
        writeln!(
            csv_file,
            "{},{},{},{},{:.4},{:.4},{:.2},{},{},{:.4},{},{:.4},{:.4}",
            res.category,
            res.dist,
            res.algorithm,
            res.n,
            res.min_ms,
            res.median_ms,
            res.cov,
            res.comparisons,
            res.moves,
            res.llc_miss_rate,
            res.branch_misses,
            res.ipc,
            res.bandwidth_mbs
        )?;
    }
    println!("\n✅ Research Study Complete. Reports saved to benchmarks/");
    Ok(())
}
