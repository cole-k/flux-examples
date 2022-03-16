extern crate prusti_contracts;

use prusti_contracts::*;

pub struct VecWrapper<T> {
    v: Vec<T>,
}

impl<T: Copy + PartialEq> VecWrapper<T> {
  #[trusted]
  #[ensures(result.len() == n)]
  pub fn from_elem_n(elem: T, n: usize) -> VecWrapper<T>
  {
      let mut vec = Vec::new();
      let mut i = 0;
      while i < n {
          vec.push(elem);
          i += 1;
      }
      VecWrapper { v: vec }
  }

    #[trusted]
    #[pure]
    fn len(&self) -> usize {
        self.v.len()
    }

    #[trusted]
    #[pure]
    #[requires(index < self.len())]
    fn lookup(&self, index: usize) -> T {
        self.v[index]
    }

    #[trusted]
    #[requires(index < self.len())]
    #[ensures(self.len() == old(self.len()))]
    fn set(&mut self, index: usize, value: T) {
        self.v[index] = value;
    }
}

//#[lr::sig(fn(src: & n@RVec<i32>, dst: &mut RVec<i32>[n]) -> i32; dst: RVec<i32>[n])]
#[requires(src.len() == dst.len())]
#[ensures(dst.len() == old(dst.len()))]
fn bcopy_aux(src: &VecWrapper<i32>, dst: &mut VecWrapper<i32>) {
    let mut i = 0;
    let n = src.len();
    while i < n {
        body_invariant!(dst.len() == old(dst.len()));
        // let r = dst.get_mut(i);
        // *r = *src.get(i);
        dst.set(i, src.lookup(i));
        i += 1;
    }
}

//#[lr::sig(fn(src: & n@RVec<i32>) -> RVec<i32>[n])]
#[ensures(result.len() == src.len())]
pub fn bcopy(src: &VecWrapper<i32>) -> VecWrapper<i32> {
    let sz = src.len();
    let mut dst = VecWrapper::<i32>::from_elem_n(0, sz);
    bcopy_aux(src, &mut dst);
    dst
}