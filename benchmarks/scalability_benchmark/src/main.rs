#![allow(warnings)]
use ars_master_benchmark::{algorithms, gen_ints};
use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rayon::ThreadPoolBuilder;
use std::fs::{self, File};
use std::io::Write;
use std::time::Instant;

const SEED: u64 = 42;
const REPETITIONS: usize = 3;
const SCALE_N: usize = 10_000_000;

struct ScalabilityResult {
    threads: usize,
    algorithm: String,
    dist: String,
    avg_time_ms: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("benchmarks/reports")?;
    let mut results = Vec::new();

    let max_threads = sys_info::cpu_num().unwrap_or(8) as usize;
    let thread_counts: Vec<usize> = vec![1, 2, 4, 8, 16, 32]
        .into_iter()
        .filter(|&t| t <= max_threads || t == 1) // Always include 1, and others up to max
        .collect();

    let algs = vec![
        "Rayon ParSort",
        "ARSApex (Unstable)",
        "ARSApex (Stable)",
        "ARSAero (Unstable)",
        "ARSAero (Stable)",
    ];

    let dists = vec![
        "Random",
        "Gaussian",
        "NearlySorted",
        "Duplicates",
        "Zipfian",
    ];

    let total_steps = thread_counts.len() * algs.len() * dists.len();
    let pb = ProgressBar::new(total_steps as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")?
        .progress_chars("#>-"));

    for &threads in &thread_counts {
        let pool = ThreadPoolBuilder::new().num_threads(threads).build()?;

        for &alg in &algs {
            for &dist in &dists {
                pb.set_message(format!("T={} {}: {}", threads, alg, dist));

                let mut durations = Vec::with_capacity(REPETITIONS);
                for _ in 0..REPETITIONS {
                    let mut rng = ChaCha8Rng::seed_from_u64(SEED);
                    let mut data = gen_ints(dist, SCALE_N, &mut rng);

                    let start = Instant::now();
                    pool.install(|| match alg {
                        "Rayon ParSort" => algorithms::std_parallel_sort(&mut data),
                        "ARSApex (Unstable)" => {
                            algorithms::ars_gen3_apex_baseline_unstable(&mut data)
                        }
                        "ARSApex (Stable)" => algorithms::ars_gen5_optimized_apex_stable(&mut data),
                        "ARSAero (Unstable)" => algorithms::ars_gen6_aero(&mut data),
                        "ARSAero (Stable)" => algorithms::ars_gen6_aero_stable(&mut data),
                        _ => {}
                    });
                    durations.push(start.elapsed());
                }

                let min_dur = durations.into_iter().min().unwrap();
                results.push(ScalabilityResult {
                    threads,
                    algorithm: alg.to_string(),
                    dist: dist.to_string(),
                    avg_time_ms: min_dur.as_secs_f64() * 1000.0,
                });
                pb.inc(1);
            }
        }
    }

    pb.finish_with_message("Scalability Study Complete");

    let mut csv_file = File::create("benchmarks/reports/scalability_results.csv")?;
    writeln!(csv_file, "Threads,Algorithm,Distribution,Time (ms)")?;
    for res in &results {
        writeln!(
            csv_file,
            "{},{},{},{:.4}",
            res.threads, res.algorithm, res.dist, res.avg_time_ms
        )?;
    }

    println!("\n✅ Scalability results saved to benchmarks/reports/scalability_results.csv");
    Ok(())
}
