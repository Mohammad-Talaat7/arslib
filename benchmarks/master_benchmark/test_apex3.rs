#![allow(warnings)]
use std::time::Instant;
use ars_master_benchmark::{algorithms, Tracked};
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use rand::SeedableRng;

fn main() {
    let n = 1_000_000;
    let mut rng = ChaCha8Rng::seed_from_u64(42);
    let raw: Vec<i64> = (0..n).map(|_| rng.gen()).collect();
    let mut data: Vec<Tracked<i64>> = raw.iter().map(|&x| Tracked::new(x)).collect();
    
    let start = Instant::now();
    algorithms::ars_apex_unstable_gen14(&mut data);
    println!("Time: {:?}", start.elapsed());
}
