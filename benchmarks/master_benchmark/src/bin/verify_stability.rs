#![allow(warnings)]
use ars_master_benchmark::{algorithms, gen_strings, Tracked};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::time::Instant;

extern "C" {
    fn malloc_trim(pad: usize) -> i32;
}

const N: usize = 10_000_000;
const REPS: usize = 3;

fn main() {
    let mut rng = ChaCha8Rng::seed_from_u64(42);

    println!("🧪 STARTING STABILITY PRE-FLIGHT (N={} Strings)", N);

    let algs = [
        (
            "Quicksort",
            algorithms::quicksort as fn(&mut [Tracked<String>]),
        ),
        (
            "ARS Gen 6: Aero Architecture",
            algorithms::ars_gen6_aero as fn(&mut [Tracked<String>]),
        ),
    ];

    for (name, func) in algs {
        println!("\n>>> Testing Algorithm: {}", name);

        for r in 0..REPS {
            print!("  Rep {}/{}... ", r + 1, REPS);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();

            // 1. Memory Safe Move Ingestion
            let data_raw = gen_strings("Uniform", N, &mut rng);
            let mut data_tracked: Vec<Tracked<String>> = data_raw
                .into_iter()
                .enumerate()
                .map(|(idx, x)| Tracked::new(x, idx))
                .collect();

            unsafe {
                malloc_trim(0);
            }

            // 2. Timed Run
            let start = Instant::now();
            func(&mut data_tracked);
            let dur = start.elapsed();

            println!("PASS ({:?})", dur);

            // 3. Hard Cleanup
            drop(data_tracked);
            unsafe {
                malloc_trim(0);
            }
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }

    println!("\n✅ STABILITY VERIFIED. The OS did not kill the process.");
}
