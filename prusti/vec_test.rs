extern crate prusti_contracts;
use prusti_contracts::*;

pub struct VecWrapper<T> {
    v: Vec<T>,
}

impl<T: PartialEq> VecWrapper<T> {
    #[trusted]
    #[pure]
    fn len(&self) -> usize {
        self.v.len()
    }

    #[trusted]
    #[pure]
    #[requires(i < self.len())]
    fn get(&self, i: usize) -> &T {
        &self.v[i]
    }

    #[trusted]
    #[requires(index < self.len())]
    #[ensures(result == old(self.get(index)))]
    #[after_expiry(
        self.len() == old(self.len()) &&
        self.get(index) == before_expiry(result) &&
        forall(
            |i: usize| (i < self.len() && i != index) ==>
            self.get(i) == old(self.get(i))
        )
    )]
    pub fn index_mut(&mut self, index: usize) -> &mut T {
        self.v.get_mut(index).unwrap()
    }
}

impl PartialEq for VecWrapper<i32> {
    #[pure]
    fn eq(&self, other: &VecWrapper<i32>) -> bool {
        self.len() == other.len()
    }
}

#[requires(m1.len() == m2.len())]
#[requires(forall(|i: usize| i < m1.len() ==> m1.get(i).len() == m2.get(i).len()))]
#[requires(forall(|i: usize, j: usize| i < m1.len() && j < m1.len() ==> m1.get(i).len() == m1.get(j).len()))]
pub fn mat_add(m1: &mut VecWrapper<VecWrapper<i32>>, m2: &VecWrapper<VecWrapper<i32>>) {
    let mut i = 0;
    let mut j = 0;

    while i < m1.len() {
        let inner_vec1 = m1.index_mut(i);
        let inner_vec2 = m2.get(i);

        while j < inner_vec1.len() {
            let x1 = inner_vec1.get(0);
            let x2 = inner_vec2.get(0);
            *inner_vec1.index_mut(i) = x1+x2;

            j = j + 1;
        }

        i = i + 1;
    }
}

pub fn main() {}