use ahash::AHasher;
use ars_native::ARSValue;
use hashbrown::HashMap;
use pyo3::prelude::*;
use pyo3::types::PyList;
use rayon::prelude::*;
use std::hash::BuildHasherDefault;

type FastHasher = BuildHasherDefault<AHasher>;

#[derive(Clone, Debug)]
pub struct Run<T> {
    pub min: T,
    pub max: T,
    pub buffer: Vec<T>,
}
impl<T: PartialOrd + Copy> Run<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        Run {
            min: val,
            max: val,
            buffer: vec![val],
        }
    }
    #[inline]
    pub fn add(&mut self, val: T) {
        if val < self.min {
            self.min = val;
        } else if val > self.max {
            self.max = val;
        }
        self.buffer.push(val);
    }
    #[inline]
    pub fn merge(&mut self, mut other: Run<T>) {
        if other.min < self.min {
            self.min = other.min;
        }
        if other.max > self.max {
            self.max = other.max;
        }
        self.buffer.append(&mut other.buffer);
    }
}

// ============================================================
// ARSHash (Spatial) - KEPT
// ============================================================
#[pyclass]
pub struct ARSHash {
    grid: HashMap<i64, usize, FastHasher>,
    runs: Vec<Run<i64>>,
    cell_shift: u32,
}

#[pymethods]
impl ARSHash {
    #[new]
    pub fn new() -> Self {
        ARSHash {
            grid: HashMap::with_hasher(FastHasher::default()),
            runs: Vec::with_capacity(128),
            cell_shift: 10,
        }
    }

    pub fn sort(&mut self, data: Vec<i64>) -> Vec<i64> {
        if data.is_empty() {
            return vec![];
        }
        for val in data {
            self.process_value(val);
        }
        self.get_output()
    }

    pub fn sort_parallel(&mut self, data: Vec<i64>) -> Vec<i64> {
        let n = data.len();
        if n < 10000 {
            return self.sort(data);
        }
        let results: Vec<Vec<Run<i64>>> = data
            .par_chunks(5000)
            .map(|chunk| {
                let mut sorter = ARSHash::new();
                for &val in chunk {
                    sorter.process_value(val);
                }
                sorter.runs
            })
            .collect();

        let mut all = Vec::new();
        for mut rs in results {
            all.append(&mut rs);
        }
        all.sort_unstable_by_key(|r| r.min);
        let mut merged: Vec<Run<i64>> = Vec::new();
        for r in all {
            if let Some(last) = merged.last_mut() {
                if r.min <= last.max + 1 {
                    last.merge(r);
                    continue;
                }
            }
            merged.push(r);
        }
        self.runs = merged;
        self.get_output()
    }

    pub fn process_value(&mut self, value: i64) {
        let cell = value >> self.cell_shift;
        if let Some(&hint) = self.grid.get(&cell) {
            if hint < self.runs.len() {
                if value >= self.runs[hint].min - 1 && value <= self.runs[hint].max + 1 {
                    self.runs[hint].add(value);
                    self.robust_merge_at(hint, 1);
                    return;
                }
            }
        }
        let res = self.runs.binary_search_by(|p| {
            if p.max < value - 1 {
                std::cmp::Ordering::Less
            } else if p.min > value + 1 {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        match res {
            Ok(idx) => {
                self.runs[idx].add(value);
                self.robust_merge_at(idx, 1);
            }
            Err(idx) => {
                self.runs.insert(idx, Run::new(value));
                self.grid.insert(cell, idx);
            }
        }
    }

    pub fn get_output(&mut self) -> Vec<i64> {
        let mut result = Vec::new();
        let mut runs = std::mem::take(&mut self.runs);
        runs.sort_unstable_by_key(|r| r.min);
        for mut run in runs {
            run.buffer.sort_unstable();
            result.extend(run.buffer);
        }
        result
    }
    fn robust_merge_at(&mut self, idx: usize, tol: i64) {
        let mut c_idx = idx;
        while c_idx + 1 < self.runs.len() && self.runs[c_idx].max + tol >= self.runs[c_idx + 1].min
        {
            let next = self.runs.remove(c_idx + 1);
            self.runs[c_idx].merge(next);
        }
        while c_idx > 0 && self.runs[c_idx - 1].max + tol >= self.runs[c_idx].min {
            let cur = self.runs.remove(c_idx);
            self.runs[c_idx - 1].merge(cur);
            c_idx -= 1;
        }
    }
}

// ============================================================
// ARSBucket (Spatial) - KEPT
// ============================================================
#[pyclass]
pub struct ARSBucket {
    pub tolerance: f64,
    grid: HashMap<i64, usize, FastHasher>,
    runs: Vec<Run<f64>>,
    scale: f64,
}
#[pymethods]
impl ARSBucket {
    #[new]
    #[pyo3(signature = (tolerance = 1e-6))]
    pub fn new(tolerance: f64) -> Self {
        ARSBucket {
            tolerance,
            grid: HashMap::with_hasher(FastHasher::default()),
            runs: Vec::with_capacity(128),
            scale: 1.0 / tolerance.max(1e-9),
        }
    }
    pub fn sort(&mut self, data: Vec<f64>) -> Vec<f64> {
        for val in data {
            self.process_value(val);
        }
        self.get_output()
    }
    pub fn sort_parallel(&mut self, data: Vec<f64>) -> Vec<f64> {
        let n = data.len();
        if n < 10000 {
            return self.sort(data);
        }
        let tol = self.tolerance;
        let results: Vec<Vec<Run<f64>>> = data
            .par_chunks(5000)
            .map(|chunk| {
                let mut s = ARSBucket::new(tol);
                for &val in chunk {
                    s.process_value(val);
                }
                s.runs
            })
            .collect();
        let mut all = Vec::new();
        for mut rs in results {
            all.append(&mut rs);
        }
        all.sort_unstable_by(|a, b| a.min.partial_cmp(&b.min).unwrap());
        let mut merged: Vec<Run<f64>> = Vec::new();
        for r in all {
            if let Some(last) = merged.last_mut() {
                if r.min <= last.max + tol {
                    last.merge(r);
                    continue;
                }
            }
            merged.push(r);
        }
        self.runs = merged;
        self.get_output()
    }
    pub fn process_value(&mut self, value: f64) {
        let tol = self.tolerance;
        let cell = (value * self.scale) as i64;
        if let Some(&hint) = self.grid.get(&cell) {
            if hint < self.runs.len() {
                if value >= self.runs[hint].min - tol && value <= self.runs[hint].max + tol {
                    self.runs[hint].add(value);
                    self.robust_merge_at(hint, tol);
                    return;
                }
            }
        }
        let res = self.runs.binary_search_by(|p| {
            if p.max < value - tol {
                std::cmp::Ordering::Less
            } else if p.min > value + tol {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        match res {
            Ok(idx) => {
                self.runs[idx].add(value);
                self.robust_merge_at(idx, tol);
            }
            Err(idx) => {
                self.runs.insert(idx, Run::new(value));
                self.grid.insert(cell, idx);
            }
        }
    }
    pub fn get_output(&mut self) -> Vec<f64> {
        let mut result = Vec::new();
        let mut runs = std::mem::take(&mut self.runs);
        runs.sort_unstable_by(|a, b| a.min.partial_cmp(&b.min).unwrap());
        for mut run in runs {
            run.buffer
                .sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            result.extend(run.buffer);
        }
        result
    }
    fn robust_merge_at(&mut self, idx: usize, tol: f64) {
        let mut c_idx = idx;
        while c_idx + 1 < self.runs.len() && self.runs[c_idx].max + tol >= self.runs[c_idx + 1].min
        {
            let next = self.runs.remove(c_idx + 1);
            self.runs[c_idx].merge(next);
        }
        while c_idx > 0 && self.runs[c_idx - 1].max + tol >= self.runs[c_idx].min {
            let cur = self.runs.remove(c_idx);
            self.runs[c_idx - 1].merge(cur);
            c_idx -= 1;
        }
    }
}

// ============================================================
// ARSGeneric (Evolution)
// ============================================================

pub struct RunGeneric<T> {
    pub min: u64,
    pub max: u64,
    pub buffer: Vec<T>,
}
impl<T: ARSValue + PartialOrd> RunGeneric<T> {
    pub fn new(val: T) -> Self {
        let s = val.to_spatial_u64();
        RunGeneric {
            min: s,
            max: s,
            buffer: vec![val],
        }
    }
    pub fn add(&mut self, val: T) {
        let s = val.to_spatial_u64();
        if s < self.min {
            self.min = s;
        }
        if s > self.max {
            self.max = s;
        }
        self.buffer.push(val);
    }
    pub fn merge(&mut self, mut other: RunGeneric<T>) {
        if other.min < self.min {
            self.min = other.min;
        }
        if other.max > self.max {
            self.max = other.max;
        }
        self.buffer.append(&mut other.buffer);
    }
}

pub struct ARSGeneric<T> {
    grid: HashMap<u64, usize, FastHasher>,
    runs: Vec<RunGeneric<T>>,
    tol: u64,
}

impl<T: ARSValue + PartialOrd + Send + Sync + Clone> ARSGeneric<T> {
    pub fn new(tol: u64) -> Self {
        ARSGeneric {
            grid: HashMap::with_hasher(FastHasher::default()),
            runs: Vec::with_capacity(128),
            tol,
        }
    }
    pub fn sort_parallel(&mut self, data: Vec<T>) -> Vec<T> {
        let n = data.len();
        if n < 10000 {
            return self.sort(data);
        }
        let results: Vec<Vec<RunGeneric<T>>> = data
            .par_chunks(5000)
            .map(|chunk| {
                let mut s = ARSGeneric::new(self.tol);
                for val in chunk {
                    s.process_value(val.clone());
                }
                s.runs
            })
            .collect();
        let mut all = Vec::new();
        for rs in results {
            all.extend(rs);
        }
        all.sort_unstable_by_key(|r| r.min);
        let mut merged: Vec<RunGeneric<T>> = Vec::new();
        for r in all {
            if let Some(last) = merged.last_mut() {
                if r.min <= last.max.saturating_add(self.tol) {
                    last.merge(r);
                    continue;
                }
            }
            merged.push(r);
        }
        self.runs = merged;
        self.get_output()
    }
    pub fn sort(&mut self, data: Vec<T>) -> Vec<T> {
        for val in data {
            self.process_value(val);
        }
        self.get_output()
    }
    fn process_value(&mut self, val: T) {
        let s = val.to_spatial_u64();
        let cell = s >> 10; // Generic spatial grid shift
        if let Some(&hint) = self.grid.get(&cell) {
            if hint < self.runs.len() {
                if s >= self.runs[hint].min.saturating_sub(self.tol)
                    && s <= self.runs[hint].max.saturating_add(self.tol)
                {
                    self.runs[hint].add(val);
                    self.robust_merge_at(hint);
                    return;
                }
            }
        }
        let res = self.runs.binary_search_by(|p| {
            if p.max < s.saturating_sub(self.tol) {
                std::cmp::Ordering::Less
            } else if p.min > s.saturating_add(self.tol) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        match res {
            Ok(idx) => {
                self.runs[idx].add(val);
                self.robust_merge_at(idx);
            }
            Err(idx) => {
                self.runs.insert(idx, RunGeneric::new(val));
                self.grid.insert(cell, idx);
            }
        }
    }
    fn robust_merge_at(&mut self, idx: usize) {
        let mut ci = idx;
        while ci + 1 < self.runs.len()
            && self.runs[ci].max.saturating_add(self.tol) >= self.runs[ci + 1].min
        {
            let next = self.runs.remove(ci + 1);
            self.runs[ci].merge(next);
        }
        while ci > 0 && self.runs[ci - 1].max.saturating_add(self.tol) >= self.runs[ci].min {
            let cur = self.runs.remove(ci);
            self.runs[ci - 1].merge(cur);
            ci -= 1;
        }
    }
    pub fn get_output(&mut self) -> Vec<T> {
        let mut result = Vec::new();
        let mut runs = std::mem::take(&mut self.runs);
        runs.sort_unstable_by_key(|r| r.min);
        for mut run in runs {
            run.buffer
                .sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            result.extend(run.buffer);
        }
        result
    }
}

// ============================================================
// ARSAdapt (Spatial) - KEPT
// ============================================================
#[pyclass]
pub struct ARSAdapt {
    runs: Vec<Vec<Py<PyAny>>>,
}
#[pymethods]
impl ARSAdapt {
    #[new]
    pub fn new() -> Self {
        ARSAdapt { runs: Vec::new() }
    }
    pub fn sort(&mut self, py: Python<'_>, data: Bound<'_, PyList>) -> PyResult<Vec<Py<PyAny>>> {
        if let Ok(ints) = data.extract::<Vec<i64>>() {
            let mut s = ARSHash::new();
            let res = s.sort_parallel(ints);
            return Ok(res
                .into_iter()
                .map(|v| v.into_py(py).into_bound(py).unbind())
                .collect());
        }
        if let Ok(floats) = data.extract::<Vec<f64>>() {
            let mut s = ARSBucket::new(1e-6);
            let res = s.sort_parallel(floats);
            return Ok(res
                .into_iter()
                .map(|v| v.into_py(py).into_bound(py).unbind())
                .collect());
        }
        for val in data.iter() {
            let mut found = false;
            for run in self.runs.iter_mut() {
                if val.ge(run[0].bind(py))? && val.le(run.last().unwrap().bind(py))? {
                    let pos = run
                        .binary_search_by(|p| p.bind(py).compare(&val).unwrap())
                        .unwrap_or_else(|e| e);
                    run.insert(pos, val.clone().unbind());
                    found = true;
                    break;
                }
            }
            if !found {
                self.runs.push(vec![val.clone().unbind()]);
                self.runs
                    .sort_by(|a, b| a[0].bind(py).compare(b[0].bind(py)).unwrap());
            }
        }
        let mut res = Vec::new();
        for r in std::mem::take(&mut self.runs) {
            res.extend(r);
        }
        Ok(res)
    }
    pub fn sort_parallel(
        &mut self,
        py: Python<'_>,
        data: Bound<'_, PyList>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        self.sort(py, data)
    }
}

#[pymodule]
fn ars_spatial_grid(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ARSHash>()?;
    m.add_class::<ARSBucket>()?;
    m.add_class::<ARSAdapt>()?;
    Ok(())
}
