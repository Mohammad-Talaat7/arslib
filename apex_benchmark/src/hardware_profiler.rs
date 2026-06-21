use perf_event::events::Hardware;
use perf_event::{Builder, Counter, Group};

pub struct HardwareMetrics {
    pub cache_misses: u64,
    pub branch_misses: u64,
    pub instructions: u64,
}

pub struct Profiler {
    group: Group,
    cache: Counter,
    branch: Counter,
    inst: Counter,
}

impl Profiler {
    pub fn new() -> std::io::Result<Self> {
        let mut group = Group::new()?;
        let cache = Builder::new()
            .kind(Hardware::CACHE_MISSES)
            .group(&mut group)
            .build()?;
        let branch = Builder::new()
            .kind(Hardware::BRANCH_MISSES)
            .group(&mut group)
            .build()?;
        let inst = Builder::new()
            .kind(Hardware::INSTRUCTIONS)
            .group(&mut group)
            .build()?;

        Ok(Profiler {
            group,
            cache,
            branch,
            inst,
        })
    }

    pub fn profile<F, R>(&mut self, f: F) -> (R, HardwareMetrics, std::time::Duration)
    where
        F: FnOnce() -> R,
    {
        self.group.enable().unwrap();
        let start = std::time::Instant::now();
        let result = f();
        let duration = start.elapsed();
        self.group.disable().unwrap();

        let counts = self.group.read().unwrap();

        let metrics = HardwareMetrics {
            cache_misses: counts[&self.cache],
            branch_misses: counts[&self.branch],
            instructions: counts[&self.inst],
        };

        (result, metrics, duration)
    }
}
