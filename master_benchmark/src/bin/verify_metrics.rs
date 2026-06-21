use ars_master_benchmark::{algorithms, gen_ints, HardwareMetrics, Profiler};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

fn main() {
    let mut profiler = match Profiler::new() {
        Ok(p) => p,
        Err(e) => {
            println!(
                "Error initializing profiler: {}. Are you on Linux with perf permissions?",
                e
            );
            return;
        }
    };

    let n = 10_000_000;
    let mut rng = ChaCha8Rng::seed_from_u64(42);
    let mut data = gen_ints("Random", n, &mut rng);

    println!("Starting validation benchmark (N={})...", n);

    // Benchmark a parallel algorithm (ARSAero)
    let (_, metrics, duration) = profiler.profile(|| {
        algorithms::ars_gen6_aero(&mut data);
    });

    println!("--- Validation Results ---");
    println!("Duration: {:?}", duration);
    println!("Instructions: {}", metrics.instructions);
    println!("CPU Cycles: {}", metrics.cpu_cycles);
    println!("Cache Misses: {}", metrics.cache_misses);
    println!("Cache References: {}", metrics.cache_references);

    if metrics.cpu_cycles > 0 {
        println!(
            "IPC: {:.4}",
            metrics.instructions as f64 / metrics.cpu_cycles as f64
        );
    }

    if metrics.cache_references > 0 {
        println!(
            "LLC Miss Rate: {:.2}%",
            (metrics.cache_misses as f64 / metrics.cache_references as f64) * 100.0
        );
    }

    if metrics.instructions < 1_000_000 {
        println!("WARNING: Instruction count looks too low for N=10M. PMC might not be inheriting correctly.");
    } else {
        println!("SUCCESS: captured significant instruction volume.");
    }
}
