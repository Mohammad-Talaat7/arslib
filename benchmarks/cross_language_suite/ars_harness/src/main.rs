use std::env;
use std::fs::File;
use std::io::Read;
use std::slice;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: ars_harness <dataset_file.bin>");
        std::process::exit(1);
    }
    
    let filename = &args[1];
    
    // Load data
    let mut file = File::open(filename).unwrap();
    let metadata = file.metadata().unwrap();
    let num_bytes = metadata.len() as usize;
    let num_elements = num_bytes / 8;
    
    let mut data: Vec<f64> = Vec::with_capacity(num_elements);
    unsafe {
        data.set_len(num_elements);
        let byte_slice = slice::from_raw_parts_mut(data.as_mut_ptr() as *mut u8, num_bytes);
        file.read_exact(byte_slice).unwrap();
    }
    
    // Time the sort
    let start = Instant::now();
    arslib::sort(&mut data);
    let duration = start.elapsed();
    
    // Verify
    for i in 1..data.len() {
        if data[i] < data[i-1] {
            eprintln!("Verification failed at index {}", i);
            std::process::exit(1);
        }
    }
    
    println!("{}", duration.as_secs_f64());
}
