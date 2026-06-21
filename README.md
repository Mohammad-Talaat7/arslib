# ARS: Adjacency Run Sort 🚀

[![Crates.io](https://img.shields.io/crates/v/arslib.svg)](https://crates.io/crates/arslib)
[![Documentation](https://docs.rs/arslib/badge.svg)](https://docs.rs/arslib)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

`arslib` is a blazing-fast, cache-friendly, and highly parallel sorting library written in Rust. It implements the **Adjacency Run Sort (ARS)** algorithm—specifically the 6th Generation "Aero" Architecture. ARS leverages spatial adjacency, cache-line buffering, and multi-threading to achieve extreme performance on modern CPU architectures.

## Features

- **Extreme Performance**: Outperforms traditional sorting algorithms (like Introsort and PDQsort) and often beats highly optimized Radix sorts (like RDST and Voracious) on large datasets.
- **Cache-Locality Optimized**: Uses stack-allocated micro-buffers to sequentialize writes and maximize L1/L2 cache utilization.
- **Parallel by Default**: Fully utilizes multi-core processors using `rayon`.
- **Stable & Unstable Variants**: Choose between `arslib::sort` (unstable) and `arslib::sort_stable` (stable).
- **Generic Support**: Easily sort any type by implementing the `ARSValue` trait to map your data to a spatial `u64` representation. Out-of-the-box support for primitives like `i32`, `i64`, `u64`, `f64`, and `String`.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
arslib = "0.4.0"
```

## Quick Start

```rust
use arslib;

fn main() {
    let mut data = vec![5.2, 1.1, 9.8, 3.4, 7.6];
    
    // Unstable sort (faster)
    arslib::sort(&mut data);
    assert_eq!(data, vec![1.1, 3.4, 5.2, 7.6, 9.8]);

    // Stable sort
    let mut more_data = vec![5, 2, 8, 1, 9, 3];
    arslib::sort_stable(&mut more_data);
    assert_eq!(more_data, vec![1, 2, 3, 5, 8, 9]);
}
```

## Custom Types

To sort your own custom structs, simply implement the `ARSValue` trait. This trait requires a single method, `to_spatial_u64()`, which projects your type into a 1D uniform numeric space for histogram analysis.

```rust
use arslib::ARSValue;

#[derive(Clone, PartialEq, PartialOrd)]
struct Record {
    score: f64,
    id: u32,
}

impl ARSValue for Record {
    fn to_spatial_u64(&self) -> u64 {
        // Map f64 to u64 while preserving sorting order
        self.score.to_spatial_u64() 
    }
}

fn main() {
    let mut records = vec![
        Record { score: 95.5, id: 1 },
        Record { score: 42.0, id: 2 },
    ];
    
    arslib::sort(&mut records);
}
```

## How It Works

ARS (Aero Architecture) operates by performing an initial lightweight parallel analysis to determine dataset boundaries and characteristics. It then projects the data into a mapped spatial domain, dynamically allocates bins, and processes elements in chunks. By buffering elements locally per-thread and flushing them in cache-aligned blocks, it drastically reduces cache misses and main memory latency compared to traditional scattered writes.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
