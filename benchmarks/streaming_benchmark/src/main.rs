#![allow(warnings)]
use ars_stream_microbatch::ARSStreamer;
use arslib::ARSValue;
use crossbeam_channel::{bounded, Sender};
use hdrhistogram::Histogram;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Debug;
use std::fs::{self, File};
use std::io::Write;
use std::time::Instant;

mod hardware_profiler;
use hardware_profiler::Profiler;

// --- CONFIG ---
const STREAM_SIZE: usize = 100_000_000; // Extreme Scale (100M)
const BATCH_SIZE: usize = 131_072;
const PIPELINE_CAPACITY: usize = 16;

// --- STREAM GENERATORS ---

struct StreamGenerator<T> {
    rng: ChaCha8Rng,
    pattern: StreamPattern,
    current_step: usize,
    _phantom: std::marker::PhantomData<T>,
}

#[derive(Debug, Clone, Copy)]
enum StreamPattern {
    Uniform,
    Zipfian,
    Bursty,
}

impl StreamGenerator<i64> {
    fn new(pattern: StreamPattern, seed: u64) -> Self {
        Self {
            rng: ChaCha8Rng::seed_from_u64(seed),
            pattern,
            current_step: 0,
            _phantom: std::marker::PhantomData,
        }
    }
    fn next_batch(&mut self, n: usize) -> Vec<i64> {
        let mut batch = Vec::with_capacity(n);
        match self.pattern {
            StreamPattern::Uniform => {
                for _ in 0..n {
                    batch.push(self.rng.gen());
                }
            }
            StreamPattern::Zipfian => {
                for _ in 0..n {
                    let rank = (self.rng.gen::<f64>().powf(-1.0 / 0.75)).floor() as i64;
                    batch.push(rank % 1_000_000);
                }
            }
            StreamPattern::Bursty => {
                if self.current_step.is_multiple_of(10) {
                    for _ in 0..n {
                        batch.push(self.rng.gen());
                    }
                } else {
                    for _ in 0..(n / 20) {
                        batch.push(self.rng.gen());
                    }
                }
            }
        }
        self.current_step += 1;
        batch
    }
}

impl StreamGenerator<String> {
    fn new(pattern: StreamPattern, seed: u64) -> Self {
        Self {
            rng: ChaCha8Rng::seed_from_u64(seed),
            pattern,
            current_step: 0,
            _phantom: std::marker::PhantomData,
        }
    }
    fn next_batch(&mut self, n: usize) -> Vec<String> {
        let mut batch = Vec::with_capacity(n);
        let pool: Vec<String> = (0..100)
            .map(|_| {
                (0..16)
                    .map(|_| self.rng.sample(rand::distributions::Alphanumeric) as char)
                    .collect()
            })
            .collect();

        for _ in 0..n {
            match self.pattern {
                StreamPattern::Uniform => {
                    batch.push(
                        (0..16)
                            .map(|_| self.rng.sample(rand::distributions::Alphanumeric) as char)
                            .collect(),
                    );
                }
                _ => {
                    batch.push(pool[self.rng.gen_range(0..100)].clone());
                }
            }
        }
        self.current_step += 1;
        batch
    }
}

// --- PIPELINE ARCHITECTURE ---

trait SorterWorker<T>: Send {
    fn process_batch(&mut self, batch: Vec<T>);
    fn collect(self: Box<Self>) -> Vec<T>;
}

struct Pipeline<T> {
    tx: Sender<Vec<T>>,
    handle: Option<std::thread::JoinHandle<Vec<T>>>,
    pub name: String,
}

impl<T: Send + 'static> Pipeline<T> {
    fn new<W: SorterWorker<T> + 'static>(name: &str, mut worker: W) -> Self {
        let (tx, rx) = bounded::<Vec<T>>(PIPELINE_CAPACITY);
        let handle = std::thread::spawn(move || {
            while let Ok(batch) = rx.recv() {
                worker.process_batch(batch);
            }
            Box::new(worker).collect()
        });
        Self {
            tx,
            handle: Some(handle),
            name: name.to_string(),
        }
    }
    fn ingest(&self, batch: Vec<T>) -> Result<(), crossbeam_channel::SendError<Vec<T>>> {
        self.tx.send(batch)
    }
    fn finalize(mut self) -> Vec<T> {
        drop(self.tx);
        self.handle.take().unwrap().join().unwrap()
    }
}

// --- WORKER IMPLEMENTATIONS ---

struct ArsWorker<T: ARSValue + PartialOrd + Send + Sync + Clone + 'static + Debug> {
    streamer: ARSStreamer<T>,
}
impl<T: ARSValue + PartialOrd + Send + Sync + Clone + 'static + Debug> SorterWorker<T>
    for ArsWorker<T>
{
    fn process_batch(&mut self, batch: Vec<T>) {
        self.streamer.push_batch(&batch);
    }
    fn collect(self: Box<Self>) -> Vec<T> {
        self.streamer.collect()
    }
}

struct LsmWorker<T: Ord + Send + 'static> {
    active: Vec<T>,
    flushed: Vec<Vec<T>>,
    threshold: usize,
}
impl<T: Ord + Send + 'static> SorterWorker<T> for LsmWorker<T> {
    fn process_batch(&mut self, mut batch: Vec<T>) {
        self.active.append(&mut batch);
        if self.active.len() >= self.threshold {
            let mut to_flush = std::mem::take(&mut self.active);
            to_flush.sort_unstable();
            self.flushed.push(to_flush);
        }
    }
    fn collect(mut self: Box<Self>) -> Vec<T> {
        let mut all = std::mem::take(&mut self.flushed)
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        all.append(&mut self.active);
        all.sort_unstable();
        all
    }
}

// --- MAIN RUNNER ---

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("benchmarks/reports")?;
    let mut csv_file = File::create("benchmarks/reports/extreme_scale_audit.csv")?;
    writeln!(
        csv_file,
        "Type,Algorithm,Pattern,P99(us),Throughput(ops/s),IPC,CacheMissRate,Consolidation(ms)"
    )?;

    let mut profiler = Profiler::new().ok();
    println!("🚀 STARTING EXTREME SCALE AUDIT (N=100 Million)");

    run_int_suite(&mut csv_file, &mut profiler)?;
    run_string_suite(&mut csv_file, &mut profiler)?;

    println!(
        "\n✅ Extreme Scale Audit complete. Results in benchmarks/reports/extreme_scale_audit.csv"
    );
    Ok(())
}

fn run_int_suite(
    csv: &mut File,
    profiler: &mut Option<Profiler>,
) -> Result<(), Box<dyn std::error::Error>> {
    let patterns = vec![
        StreamPattern::Uniform,
        StreamPattern::Zipfian,
        StreamPattern::Bursty,
    ];
    for &pattern in &patterns {
        let pattern_name = format!("{:?}", pattern);
        for alg_idx in 0..2 {
            let mut hist = Histogram::<u64>::new_with_bounds(1, 100_000_000, 3)?;
            let mut metrics_acc = (0, 0, 0, 0);

            let pipeline = if alg_idx == 0 {
                Pipeline::new(
                    "ARS Stream",
                    ArsWorker {
                        streamer: ARSStreamer::new_bounded(BATCH_SIZE, PIPELINE_CAPACITY),
                    },
                )
            } else {
                Pipeline::new(
                    "LSM Proxy",
                    LsmWorker {
                        active: Vec::new(),
                        flushed: Vec::new(),
                        threshold: BATCH_SIZE,
                    },
                )
            };

            let mut generator = StreamGenerator::<i64>::new(pattern, 42);
            let start_total = Instant::now();
            let mut processed = 0;

            while processed < STREAM_SIZE {
                let batch = generator.next_batch(BATCH_SIZE);
                if batch.is_empty() {
                    break;
                }
                let start_ingest = Instant::now();
                if let Some(p) = profiler {
                    let (_, m, _) = p.profile(|| pipeline.ingest(batch).unwrap());
                    metrics_acc.0 += m.cpu_cycles;
                    metrics_acc.1 += m.instructions;
                    metrics_acc.2 += m.cache_misses;
                    metrics_acc.3 += m.cache_references;
                } else {
                    pipeline.ingest(batch).unwrap();
                }
                hist.record(start_ingest.elapsed().as_micros() as u64).ok();
                processed += BATCH_SIZE;
            }

            let start_cons = Instant::now();
            let name = pipeline.name.clone();
            let res = pipeline.finalize();
            let cons_dur = start_cons.elapsed();
            std::hint::black_box(res);

            let throughput = STREAM_SIZE as f64 / start_total.elapsed().as_secs_f64();
            let ipc = if metrics_acc.0 > 0 {
                metrics_acc.1 as f64 / metrics_acc.0 as f64
            } else {
                0.0
            };
            let cache_rate = if metrics_acc.3 > 0 {
                metrics_acc.2 as f64 / metrics_acc.3 as f64
            } else {
                0.0
            };

            println!(
                "  [Int64] {}: P99={:?}us, IPC={:.2}, Throughput={:.1}M",
                name,
                hist.value_at_quantile(0.99),
                ipc,
                throughput / 1e6
            );

            writeln!(
                csv,
                "Int64,{},{},{},{:.1},{:.3},{:.4},{:.2}",
                name,
                pattern_name,
                hist.value_at_quantile(0.99),
                throughput,
                ipc,
                cache_rate,
                cons_dur.as_secs_f64() * 1000.0
            )?;
        }
    }
    Ok(())
}

fn run_string_suite(
    csv: &mut File,
    profiler: &mut Option<Profiler>,
) -> Result<(), Box<dyn std::error::Error>> {
    let patterns = vec![
        StreamPattern::Uniform,
        StreamPattern::Zipfian,
        StreamPattern::Bursty,
    ];
    for &pattern in &patterns {
        let pattern_name = format!("{:?}", pattern);
        for alg_idx in 0..2 {
            let mut hist = Histogram::<u64>::new_with_bounds(1, 100_000_000, 3)?;
            let mut metrics_acc = (0, 0, 0, 0);

            let pipeline = if alg_idx == 0 {
                Pipeline::new(
                    "ARS Stream",
                    ArsWorker {
                        streamer: ARSStreamer::new_bounded(BATCH_SIZE, PIPELINE_CAPACITY),
                    },
                )
            } else {
                Pipeline::new(
                    "LSM Proxy",
                    LsmWorker {
                        active: Vec::new(),
                        flushed: Vec::new(),
                        threshold: BATCH_SIZE,
                    },
                )
            };

            let mut generator = StreamGenerator::<String>::new(pattern, 42);
            let start_total = Instant::now();
            let mut processed = 0;

            while processed < STREAM_SIZE {
                let batch = generator.next_batch(BATCH_SIZE);
                if batch.is_empty() {
                    break;
                }
                let start_ingest = Instant::now();
                if let Some(p) = profiler {
                    let (_, m, _) = p.profile(|| pipeline.ingest(batch).unwrap());
                    metrics_acc.0 += m.cpu_cycles;
                    metrics_acc.1 += m.instructions;
                    metrics_acc.2 += m.cache_misses;
                    metrics_acc.3 += m.cache_references;
                } else {
                    pipeline.ingest(batch).unwrap();
                }
                hist.record(start_ingest.elapsed().as_micros() as u64).ok();
                processed += BATCH_SIZE;
            }

            let start_cons = Instant::now();
            let name = pipeline.name.clone();
            let res = pipeline.finalize();
            let cons_dur = start_cons.elapsed();
            std::hint::black_box(res);

            let throughput = STREAM_SIZE as f64 / start_total.elapsed().as_secs_f64();
            let ipc = if metrics_acc.0 > 0 {
                metrics_acc.1 as f64 / metrics_acc.0 as f64
            } else {
                0.0
            };
            let cache_rate = if metrics_acc.3 > 0 {
                metrics_acc.2 as f64 / metrics_acc.3 as f64
            } else {
                0.0
            };

            println!(
                "  [String] {}: P99={:?}us, IPC={:.2}, Throughput={:.1}M",
                name,
                hist.value_at_quantile(0.99),
                ipc,
                throughput / 1e6
            );

            writeln!(
                csv,
                "String,{},{},{},{:.1},{:.3},{:.4},{:.2}",
                name,
                pattern_name,
                hist.value_at_quantile(0.99),
                throughput,
                ipc,
                cache_rate,
                cons_dur.as_secs_f64() * 1000.0
            )?;
        }
    }
    Ok(())
}
