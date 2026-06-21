#![allow(warnings)]
use perf_event::events::Hardware;
use perf_event::{Builder, Counter};

pub struct HardwareMetrics {
    pub cache_misses: u64,
    pub cache_references: u64,
    pub branch_misses: u64,
    pub instructions: u64,
    pub cpu_cycles: u64,
}

pub struct Profiler {
    cache: Counter,
    cache_refs: Counter,
    branch: Counter,
    inst: Counter,
    cycles: Counter,
}

impl Profiler {
    pub fn new() -> std::io::Result<Self> {
        let mut b1 = Builder::new();
        b1 = b1.kind(Hardware::CACHE_MISSES);
        b1.inherit(true);
        let cache = b1.build()?;

        let mut b2 = Builder::new();
        b2 = b2.kind(Hardware::CACHE_REFERENCES);
        b2.inherit(true);
        let cache_refs = b2.build()?;

        let mut b3 = Builder::new();
        b3 = b3.kind(Hardware::BRANCH_MISSES);
        b3.inherit(true);
        let branch = b3.build()?;

        let mut b4 = Builder::new();
        b4 = b4.kind(Hardware::INSTRUCTIONS);
        b4.inherit(true);
        let inst = b4.build()?;

        let mut b5 = Builder::new();
        b5 = b5.kind(Hardware::CPU_CYCLES);
        b5.inherit(true);
        let cycles = b5.build()?;

        Ok(Profiler {
            cache,
            cache_refs,
            branch,
            inst,
            cycles,
        })
    }

    pub fn profile<F, R>(&mut self, f: F) -> (R, HardwareMetrics, std::time::Duration)
    where
        F: FnOnce() -> R,
    {
        self.cache.enable().unwrap();
        self.cache_refs.enable().unwrap();
        self.branch.enable().unwrap();
        self.inst.enable().unwrap();
        self.cycles.enable().unwrap();

        let start = std::time::Instant::now();
        let result = f();
        let duration = start.elapsed();

        self.cache.disable().unwrap();
        self.cache_refs.disable().unwrap();
        self.branch.disable().unwrap();
        self.inst.disable().unwrap();
        self.cycles.disable().unwrap();

        let metrics = HardwareMetrics {
            cache_misses: self.cache.read().unwrap(),
            cache_references: self.cache_refs.read().unwrap(),
            branch_misses: self.branch.read().unwrap(),
            instructions: self.inst.read().unwrap(),
            cpu_cycles: self.cycles.read().unwrap(),
        };

        // Reset for next run
        self.cache.reset().unwrap();
        self.cache_refs.reset().unwrap();
        self.branch.reset().unwrap();
        self.inst.reset().unwrap();
        self.cycles.reset().unwrap();

        (result, metrics, duration)
    }
}
