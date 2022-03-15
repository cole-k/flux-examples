extern crate prusti_contracts;

use prusti_contracts::*;

pub struct VecWrapperI32 {
    v: Vec<i32>,
}

impl VecWrapperI32 {
    #[trusted]
    #[pure]
    fn len(&self) -> usize {
        self.v.len()
    }

    #[trusted]
    #[pure]
    #[requires(i < self.len())]
    fn lookup(&self, i: usize) -> i32 {
        self.v[i]
    }
}

//#[lr::sig(fn(k: i32, items: &VecWrapperI32) -> usize)]
pub fn binary_search(k: i32, items: &VecWrapperI32) -> usize {
  let size = items.len();
  if size <= 0 {
    return size;
  }

  let mut low: usize = 0;
  let mut high: usize = size - 1;

  while low <= high {
      // SAFE   let middle = (high + low) / 2;
      // let middle = high + ((high - low) / 2);
      let middle = low + ((high - low) / 2);
      let current = items.lookup(middle);
      if current == k {
        return middle;
      }
      if current > k {
        if middle == 0 {
          return size;
        }
        high = middle - 1
      }
      if current < k {
        low = middle + 1
      }
  }
  size
}

pub fn main() {

}
