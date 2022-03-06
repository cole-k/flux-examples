#![allow(unused_attributes)]
#![feature(register_tool)]
#![register_tool(lr)]

#[path = "lib/rvec.rs"]
pub mod rvec;
use rvec::RVec;

type Point = RVec<f32>;

/// distance between two points
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

/// adding two points (updates the first)
fn add(x:&mut RVec<f32>, y:&RVec<f32>) -> i32 {
    let mut i = 0;
    while i < x.len() {
        *x.get_mut(i) += *y.get(i);
        i += 1;
    }
    0
}

/// normalizing a point (cluster) by size
fn normal(x:&mut RVec<f32>, n: usize) -> i32 {
    let mut i = 0;
    while i < x.len() {
        let xi = *x.get(i);
        *x.get_mut(i) = xi / (n as f32);
        i += 1;
    }
    0
}

/// finding the nearest center to a point
fn nearest(cs: &RVec<Point>, p: &Point) -> usize {
    let mut res = 0;
    let mut min = f32::MAX;
    let mut i = 0;
    while i < cs.len() {
        let ci = cs.get(i);
        let di = dist(ci, p);
        if di < min {
            res = i;
            min = di;
        }
        i += 1;
    }
    res
}

/// creating (empty) 0-center for each cluster
fn init_centers(n: usize, k: usize) -> RVec<Point> {
  let mut res = RVec::new();
  let mut i = 0;
  while i < k {
      res.push(RVec::from_elem_n(0.0, n));
      i += 1;
  }
  res
}

/// updating the centers
fn kmeans_step(n:usize, cs: RVec<Point>, ps: &RVec<&Point>) -> RVec<Point> {
    let k = cs.len();

    let mut res_points = init_centers(n, k);
    let mut res_size = RVec::from_elem_n(0, k);

    let mut i = 0;
    while i < ps.len() {
        let p = *ps.get(i);
        let j= nearest(&cs, p);
        add(res_points.get_mut(j), p);
        *res_size.get_mut(j) += 1;
        i += 1;
    }

    let mut i = 0;
    while i < k {
        normal(res_points.get_mut(i), *res_size.get(i));
        i += 1;
    }
    res_points
}

/// kmeans: iterating the center-update-steps
pub fn kmeans(n:usize, cs: RVec<Point>, ps: &RVec<&Point>, iters: i32) -> RVec<Point> {
    let mut i = 0;
    let mut res = cs;
    while i < iters {
        res = kmeans_step(n, res, ps);
        i += 1;
    }
    res
}