#![allow(warnings)]

use ars_native::ARSValue;
use pyo3::prelude::*;
use pyo3::types::PyList;
use rayon::prelude::*;
use std::collections::BTreeMap;

const THRESHOLD: usize = 1024;

#[derive(Clone, Debug)]
pub enum Run {
    Leaf {
        min: i64,
        max: i64,
        buffer: Vec<i64>,
    },
    Branch {
        min: i64,
        max: i64,
        sub_runs: BTreeMap<i64, Run>,
    },
}

impl Run {
    pub fn new(val: i64) -> Self {
        Run::Leaf {
            min: val,
            max: val,
            buffer: vec![val],
        }
    }
    pub fn min(&self) -> i64 {
        match self {
            Run::Leaf { min, .. } => *min,
            Run::Branch { min, .. } => *min,
        }
    }
    pub fn max(&self) -> i64 {
        match self {
            Run::Leaf { max, .. } => *max,
            Run::Branch { max, .. } => *max,
        }
    }
    pub fn add(&mut self, val: i64) {
        match self {
            Run::Leaf { min, max, buffer } => {
                if val < *min {
                    *min = val;
                }
                if val > *max {
                    *max = val;
                }
                buffer.push(val);
            }
            Run::Branch { min, max, .. } => {
                if val < *min {
                    *min = val;
                }
                if val > *max {
                    *max = val;
                }
            }
        }
    }
    pub fn collect(&mut self, result: &mut Vec<i64>) {
        match self {
            Run::Leaf { buffer, .. } => {
                buffer.sort_unstable();
                result.extend_from_slice(buffer);
            }
            Run::Branch { sub_runs, .. } => {
                for (_, run) in sub_runs.iter_mut() {
                    run.collect(result);
                }
            }
        }
    }
    pub fn merge(&mut self, other: Run) {
        match (self, other) {
            (
                Run::Leaf {
                    buffer: b1,
                    min: min1,
                    max: max1,
                },
                Run::Leaf {
                    buffer: b2,
                    min: min2,
                    max: max2,
                },
            ) => {
                b1.extend(b2);
                if min2 < *min1 {
                    *min1 = min2;
                }
                if max2 > *max1 {
                    *max1 = max2;
                }
            }
            _ => {}
        }
    }
}

// --- PREVIOUS SPECIALIZED IMPLEMENTATIONS (KEPT) ---

#[pyclass]
pub struct ARSHash {
    runs: Vec<Run>,
}
#[pymethods]
impl ARSHash {
    #[new]
    pub fn new() -> Self {
        ARSHash {
            runs: Vec::with_capacity(128),
        }
    }
    pub fn sort(&mut self, data: Vec<i64>) -> Vec<i64> {
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
        let results: Vec<Vec<Run>> = data
            .par_chunks(5000)
            .map(|chunk| {
                let mut s = ARSHash::new();
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
        all.sort_unstable_by_key(|r| r.min());
        let mut merged: Vec<Run> = Vec::new();
        for r in all {
            if let Some(last) = merged.last_mut() {
                if r.min() <= last.max() + 1 {
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
        let res = self.runs.binary_search_by(|p| {
            if p.max() < value - 1 {
                std::cmp::Ordering::Less
            } else if p.min() > value + 1 {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        match res {
            Ok(idx) => {
                self.runs[idx].add(value);
                self.robust_merge_at(idx);
            }
            Err(idx) => {
                self.runs.insert(idx, Run::new(value));
            }
        }
    }
    pub fn get_output(&mut self) -> Vec<i64> {
        let mut result = Vec::new();
        for mut r in std::mem::take(&mut self.runs) {
            r.collect(&mut result);
        }
        result
    }
    fn robust_merge_at(&mut self, idx: usize) {
        let mut ci = idx;
        while ci + 1 < self.runs.len() && self.runs[ci].max() + 1 >= self.runs[ci + 1].min() {
            let next = self.runs.remove(ci + 1);
            self.runs[ci].merge(next);
        }
        while ci > 0 && self.runs[ci - 1].max() + 1 >= self.runs[ci].min() {
            let cur = self.runs.remove(ci);
            self.runs[ci - 1].merge(cur);
            ci -= 1;
        }
    }
}

#[pyclass]
pub struct ARSBucket {
    runs: Vec<RunF>,
    tolerance: f64,
}
#[derive(Clone, Debug)]
pub struct RunF {
    pub min: f64,
    pub max: f64,
    pub buffer: Vec<f64>,
}
impl RunF {
    pub fn new(val: f64) -> Self {
        RunF {
            min: val,
            max: val,
            buffer: vec![val],
        }
    }
    pub fn add(&mut self, val: f64) {
        if val < self.min {
            self.min = val;
        }
        if val > self.max {
            self.max = val;
        }
        self.buffer.push(val);
    }
    pub fn merge(&mut self, mut other: RunF) {
        if other.min < self.min {
            self.min = other.min;
        }
        if other.max > self.max {
            self.max = other.max;
        }
        self.buffer.append(&mut other.buffer);
    }
}
#[pymethods]
impl ARSBucket {
    #[new]
    #[pyo3(signature = (tolerance = 1e-6))]
    pub fn new(tolerance: f64) -> Self {
        ARSBucket {
            runs: Vec::new(),
            tolerance,
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
        let results: Vec<Vec<RunF>> = data
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
        let mut merged: Vec<RunF> = Vec::new();
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
    fn process_value(&mut self, val: f64) {
        let tol = self.tolerance;
        let res = self.runs.binary_search_by(|p| {
            if p.max < val - tol {
                std::cmp::Ordering::Less
            } else if p.min > val + tol {
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
                self.runs.insert(idx, RunF::new(val));
            }
        }
    }
    pub fn get_output(&mut self) -> Vec<f64> {
        let mut result = Vec::new();
        for mut r in std::mem::take(&mut self.runs) {
            r.buffer.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            result.extend(r.buffer);
        }
        result
    }
    fn robust_merge_at(&mut self, idx: usize) {
        let tol = self.tolerance;
        let mut ci = idx;
        while ci + 1 < self.runs.len() && self.runs[ci].max + tol >= self.runs[ci + 1].min {
            let next = self.runs.remove(ci + 1);
            self.runs[ci].merge(next);
        }
        while ci > 0 && self.runs[ci - 1].max + tol >= self.runs[ci].min {
            let cur = self.runs.remove(ci);
            self.runs[ci - 1].merge(cur);
            ci -= 1;
        }
    }
}

// --- NEW GENERIC EVOLUTION CORE ---

#[derive(Clone, Debug)]
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
    pub runs: Vec<RunGeneric<T>>,
    pub tol: u64,
}
impl<T: ARSValue + PartialOrd + Send + Sync + Clone> ARSGeneric<T> {
    pub fn new(tol: u64) -> Self {
        ARSGeneric {
            runs: Vec::with_capacity(128),
            tol,
        }
    }
    pub fn sort(&mut self, data: Vec<T>) -> Vec<T> {
        for val in data {
            self.process_value(val);
        }
        self.get_output()
    }
    pub fn sort_parallel(&mut self, data: Vec<T>) -> Vec<T> {
        let n = data.len();
        if n < 10000 {
            return self.sort(data);
        }
        let tol = self.tol;
        let results: Vec<Vec<RunGeneric<T>>> = data
            .par_chunks(5000)
            .map(|chunk| {
                let mut s = ARSGeneric::new(tol);
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
    fn process_value(&mut self, val: T) {
        let s = val.to_spatial_u64();
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
            }
        }
    }
    fn robust_merge_at(&mut self, idx: usize) {
        let mut ci = idx;
        while ci + 1 < self.runs.len() && self.runs[ci].max + self.tol >= self.runs[ci + 1].min {
            let next = self.runs.remove(ci + 1);
            self.runs[ci].merge(next);
        }
        while ci > 0 && self.runs[ci - 1].max + self.tol >= self.runs[ci].min {
            let cur = self.runs.remove(ci);
            self.runs[ci - 1].merge(cur);
            ci -= 1;
        }
    }
    pub fn get_output(&mut self) -> Vec<T> {
        let mut result = Vec::new();
        let mut runs = std::mem::take(&mut self.runs);
        runs.sort_unstable_by_key(|r| r.min);
        for mut r in runs {
            r.buffer.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            result.extend(r.buffer);
        }
        result
    }
}

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
fn ars_recursive(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ARSHash>()?;
    m.add_class::<ARSBucket>()?;
    m.add_class::<ARSAdapt>()?;
    Ok(())
}
