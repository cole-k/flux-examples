#![allow(unused_attributes)]
#![feature(register_tool)]
#![register_tool(lr)]

#[path = "lib/rvec.rs"]
pub mod rvec;
use rvec::RVec;

// RJ: requires HOFs ...

fn fold<T>(a:&RVec<i32>, b: T, f:fn(&RVec<i32>, usize, T) -> T) -> T {
  let n = a.len();
  let mut i = 0;
  let mut res = b;
  while i < n {
    res = f(a, i, res);
    i  += 1;
  }
  res
}

fn step(arr:&RVec<i32>, i:usize, cur:i32) -> i32 {
  let val = *arr.get(i);
  if val > cur { val } else { cur }
}

pub fn arraymax(a:&RVec<i32>) -> i32 {
  fold(a, 0, step)
}

// Just to make sure the code above does what I think it does...
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test0() {
    let mut vec : RVec<i32> = RVec::new();
    vec.push(10);
    vec.push(20);
    vec.push(15);
    let m = arraymax(&vec);
    assert_eq!(m, 20)
  }
}
