#![allow(unused_attributes)]
#![feature(register_tool)]
#![register_tool(lr)]

#[path = "lib/rvec.rs"]
pub mod rvec;
use rvec::RVec;

// Points

pub fn dist(x: &RVec<f32>, y:&RVec<f32>) -> f32 {
    let mut res = 0.0;
    let mut i = 0;
    while i < x.len() {
        let di = x.get(i) - y.get(i);
        res += di*di;
        i += 1;
    }
    res
}

fn add(x:&mut RVec<f32>, y:&RVec<f32>) -> i32 {
    let mut i = 0;
    while i < x.len() {
        *x.get_mut(i) += *y.get(i);
        i += 1;
    }
    0
}

fn normal(x:&mut RVec<f32>, n: f32) -> i32 {
    let mut i = 0;
    while i < x.len() {
        let xi = *x.get(i);
        *x.get_mut(i) = xi / n;
        i += 1;
    }
    0
}

pub fn center(n: usize, xs: RVec<&RVec<f32>>) -> RVec<f32> {
    let mut res = RVec::from_elem_n(0.0, n);
    let mut i = 0;
    while i < xs.len() {
        add(&mut res, xs.get(i));
        i += 1;
    }
    normal(&mut res, xs.len() as f32);
    res
}

type Point = RVec<f32>;
type Centers = RVec<Point>;

pub fn kmeans_step(cs: RVec<Point>, ps: &RVec<&Point>) -> RVec<Point> {
    cs
}