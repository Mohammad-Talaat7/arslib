use std::env;
use std::fs::File;
use std::io::Write;
use std::slice;
use rand_chacha::ChaCha8Rng;
use rand::SeedableRng;
use ars_master_benchmark::gen_floats;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: dataset_generator <size> <distribution> <output_file>");
        std::process::exit(1);
    }
    
    let n: usize = args[1].parse().unwrap();
    let dist = &args[2];
    let output_file = &args[3];
    
    let mut rng = ChaCha8Rng::seed_from_u64(42);
    let data = gen_floats(dist, n, &mut rng);
    
    let mut file = File::create(output_file).unwrap();
    let byte_slice = unsafe {
        slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * 8)
    };
    file.write_all(byte_slice).unwrap();
}
