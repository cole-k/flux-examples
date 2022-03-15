extern crate prusti_contracts;
use prusti_contracts::*;

pub struct VecWrapperI32 {
  v: Vec<i32>,
}

impl VecWrapperI32 {
  /// A ghost function for getting the length of the vector
  #[trusted]
  #[pure]
  pub fn len(&self) -> usize {
      self.v.len()
  }

  /// A ghost function for specifying values stored in the vector.
  #[trusted]
  #[pure]
  #[requires(index < self.len())]
  pub fn lookup(&self, index: usize) -> i32 {
      self.v[index]
  }
}

//#[lr::sig(fn(v1: &n@RVec<i32>, v2:RVec<i32>[n]) -> i32)]
#[requires(v1.len() == v2.len())]
pub fn dotprod(v1: &VecWrapperI32, v2:VecWrapperI32) -> i32 {
  let n = v1.len();
  let mut sum = 0;
  let mut i = 0;
  while i < n {
    body_invariant!(i < v1.len());
    let x1 = v1.lookup(i);
    let x2 = v2.lookup(i);
    sum += x1 * x2;
    i += 1;
  }
  sum
}