#![allow(unused_attributes)]
#![feature(register_tool)]
#![register_tool(lr)]

#[path = "../lib/rmat.rs"]
pub mod rmat;
use rmat::RMat;

#[lr::sig(fn (arr2: &RMat<f32>, m:usize{0 < m}, n: usize{ 0 < n}) -> bool)]
pub fn is_neg(arr2: &RMat<f32>, _m:usize, n: usize) -> bool {
  let mut j = 1;
  while j < n - 1 {
    if *arr2.get(0, j) < 0.0 {
      return true
    }
    j += 1;
  }
  false
}
