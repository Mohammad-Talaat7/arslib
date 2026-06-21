#![allow(warnings)]
use ars_master_benchmark::Tracked;
use ars_native::{ARSApex, ARSValue};

fn main() {
    let data: Vec<Tracked<i64>> = vec![Tracked::new(8), Tracked::new(3), Tracked::new(5)];
    for x in &data {
        println!("{}", x.to_spatial_u64());
    }
}
