#![allow(warnings)]
pub use ars_native::{ARSValue, COMPARISONS, MOVES};
use std::fmt::Debug;
use std::sync::atomic::Ordering;

pub mod hardware_profiler;
pub use hardware_profiler::{HardwareMetrics, Profiler};

pub mod gpu_support {
    use pollster::block_on;
    use wgpu_algorithms::{Context, Sorter};

    pub struct GpuSorter {
        pub sorter: Sorter,
    }

    impl GpuSorter {
        pub fn new() -> Option<Self> {
            block_on(async {
                let ctx = Context::init().await?;
                let sorter = Sorter::new(&ctx);
                Some(Self { sorter })
            })
        }

        pub fn sort_u32(&mut self, data: &mut [u32]) {
            let sorted = block_on(self.sorter.sort(data));
            data.copy_from_slice(&sorted);
        }
    }
}

// --- SCIENTIFIC INSTRUMENTATION CORE ---

pub fn reset_metrics() {
    COMPARISONS.store(0, Ordering::SeqCst);
    MOVES.store(0, Ordering::SeqCst);
}

pub fn get_metrics() -> (u64, u64) {
    (
        COMPARISONS.load(Ordering::SeqCst),
        MOVES.load(Ordering::SeqCst),
    )
}

#[derive(Debug)]
pub struct Tracked<T: PartialOrd + Clone + Debug + Send + Sync> {
    pub inner: T,
    pub original_index: usize,
}

impl<T: PartialOrd + Clone + Debug + Send + Sync> Tracked<T> {
    pub fn new(inner: T, original_index: usize) -> Self {
        Self {
            inner,
            original_index,
        }
    }
}

impl<T: PartialOrd + Clone + Debug + Send + Sync> Clone for Tracked<T> {
    fn clone(&self) -> Self {
        MOVES.fetch_add(1, Ordering::Relaxed);
        Self {
            inner: self.inner.clone(),
            original_index: self.original_index,
        }
    }
}

impl<T: PartialOrd + Clone + Debug + Send + Sync> PartialEq for Tracked<T> {
    fn eq(&self, other: &Self) -> bool {
        COMPARISONS.fetch_add(1, Ordering::Relaxed);
        self.inner.partial_cmp(&other.inner) == Some(std::cmp::Ordering::Equal)
    }
}

impl<T: PartialOrd + Clone + Debug + Send + Sync> Eq for Tracked<T> {}

impl<T: PartialOrd + Clone + Debug + Send + Sync> PartialOrd for Tracked<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        COMPARISONS.fetch_add(1, Ordering::Relaxed);
        self.inner.partial_cmp(&other.inner)
    }
}

impl<T: PartialOrd + Clone + Debug + Send + Sync> Ord for Tracked<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        COMPARISONS.fetch_add(1, Ordering::Relaxed);
        self.inner
            .partial_cmp(&other.inner)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl<T: ARSValue + PartialOrd + Clone + Debug + Send + Sync> ARSValue for Tracked<T> {
    fn to_spatial_u64(&self) -> u64 {
        self.inner.to_spatial_u64()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CustomRecord {
    pub id: i64,
    pub score: i64,
    pub metadata: u64,
}

impl PartialOrd for CustomRecord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.score.cmp(&other.score))
    }
}

impl Ord for CustomRecord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl ARSValue for CustomRecord {
    fn to_spatial_u64(&self) -> u64 {
        self.score.to_spatial_u64()
    }
}

// --- ALGORITHMS PHYLOGENY ---

pub mod algorithms {
    use super::*;
    use rayon::prelude::*;

    // 1. Timsort (Rust standard stable sort)
    pub fn timsort<T: PartialOrd>(data: &mut [T]) {
        data.sort_by(|a, b| a.partial_cmp(b).expect("Sort failure"));
    }

    // 2. PDQsort (Rust standard unstable sort)
    pub fn pdqsort<T: PartialOrd>(data: &mut [T]) {
        data.sort_unstable_by(|a, b| a.partial_cmp(b).expect("Sort failure"));
    }

    // 3. Mergesort (proxy using std stable sort)
    pub fn mergesort<T: PartialOrd>(data: &mut [T]) {
        data.sort_by(|a, b| a.partial_cmp(b).expect("Sort failure"));
    }

    // 4. Heapsort (implemented using BinaryHeap)
    pub fn heapsort<T: PartialOrd + Clone>(data: &mut [T]) {
        // Since BinaryHeap requires Ord, we use a wrapper for PartialOrd
        #[derive(Clone, PartialEq, PartialOrd)]
        struct Wrapper<'a, T>(&'a T);
        impl<'a, T: PartialOrd> Eq for Wrapper<'a, T> {}
        #[allow(clippy::derive_ord_xor_partial_ord)]
        impl<'a, T: PartialOrd> Ord for Wrapper<'a, T> {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.0
                    .partial_cmp(other.0)
                    .unwrap_or(std::cmp::Ordering::Equal)
            }
        }

        let mut heap = std::collections::BinaryHeap::new();
        for item in data.iter() {
            heap.push(Wrapper(item));
        }
        let mut temp = Vec::with_capacity(data.len());
        while let Some(Wrapper(val)) = heap.pop() {
            temp.push(val.clone());
        }
        temp.reverse();
        for (i, val) in temp.into_iter().enumerate() {
            data[i] = val;
        }
    }

    // 5. Introsort (using introsort crate)
    pub fn introsort<T: Ord>(data: &mut [T]) {
        introsort::sort(data);
    }

    // 6. Spreadsort / Radix Hybrid (using rdst for stability across distributions)
    pub fn spreadsort_i64(data: &mut [i64]) {
        use rdst::RadixSort;
        data.radix_sort_unstable();
    }

    pub fn spreadsort_f64(data: &mut [f64]) {
        use rdst::RadixSort;
        data.radix_sort_unstable();
    }

    pub fn spreadsort_string(data: &mut [String]) {
        data.sort_unstable();
    }

    // 7. IPS4o (Using Rayon's parallel unstable sort as a proxy for parallel scalar sort)
    pub fn ips4o_proxy<T: PartialOrd + Send>(data: &mut [T]) {
        data.par_sort_unstable_by(|a, b| a.partial_cmp(b).expect("Sort failure"));
    }

    // 8. Fluxsort (Using glidesort as a modern stable adaptive sort)
    pub fn fluxsort_proxy<T: Ord + Send>(data: &mut [T]) {
        glidesort::sort(data);
    }

    // 9. Radix Sort Baseline (radsort)
    pub fn radsort_lsd<T: radsort::Key>(data: &mut [T]) {
        radsort::sort(data);
    }

    // 10. MSD Radix Sort (rdst - serial and parallel)
    pub fn rdst_serial<T: rdst::RadixKey + Send + Sync + Copy>(data: &mut [T]) {
        use rdst::RadixSort;
        data.radix_sort_builder().with_parallel(false).sort();
    }

    pub fn rdst_parallel<T: rdst::RadixKey + Send + Sync + Copy>(data: &mut [T]) {
        use rdst::RadixSort;
        data.radix_sort_unstable();
    }

    // 11. Parallel Radix (voracious_radix_sort multi-threaded)
    pub fn voracious_mt_sort_i64(data: &mut [i64]) {
        use voracious_radix_sort::RadixSort;
        data.voracious_mt_sort(rayon::current_num_threads());
    }

    pub fn voracious_mt_sort_f64(data: &mut [f64]) {
        use voracious_radix_sort::RadixSort;
        data.voracious_mt_sort(rayon::current_num_threads());
    }

    // 12. String-specific: afsort (American Flag Sort)
    pub fn afsort_string(data: &mut [String]) {
        use afsort::AFSortable;
        data.af_sort_unstable();
    }

    // 13. String-specific: universal_radix_sort
    pub fn universal_radix_sort_string(data: &mut [String]) {
        use universal_radix_sort::{RadixDataType, RadixSort, SortDirection};
        let sorter = RadixSort::<String>::new(RadixDataType::String, SortDirection::Ascending);
        sorter.sort(data).expect("universal_radix_sort failed");
    }

    // Target ARS Algorithm
    pub fn ars_gen5_optimized_apex<T: ARSValue + PartialOrd + Send + Clone>(data: &mut [T]) {
        ars_optimized_apex::ARSOptimizedApex::sort(data);
    }

    pub fn ars_gen5_optimized_apex_stable<T: ARSValue + PartialOrd + Send + Clone>(data: &mut [T]) {
        ars_optimized_apex::ARSOptimizedApex::sort_stable(data);
    }

    pub fn ars_gen6_aero<T: ARSValue + PartialOrd + Send + Sync + Clone>(data: &mut [T]) {
        ars_gen_6_aero::ARSAero::sort(data);
    }

    pub fn ars_gen6_aero_stable<T: ARSValue + PartialOrd + Send + Sync + Clone>(data: &mut [T]) {
        ars_gen_6_aero::ARSAero::sort_stable(data);
    }
}
