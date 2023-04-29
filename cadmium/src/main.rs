#![allow(dead_code)]
#![allow(unused_variables)]
// use truck_meshalgo::prelude::*;
// use truck_modeling::*;

mod sketch;

fn main() {
    let a = sketch::Point::new(0.01, 4.0, "A");
    let b = sketch::Point::new(0.0, 0.0, "B");
    let c = sketch::Point::new(4.0, 0.01, "C");
    println!("{}", b);
}
