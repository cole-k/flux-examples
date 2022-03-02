extern crate prusti_contracts;

use prusti_contracts::*;

struct VecWrapperI32 {
    v: Vec<i32>,
}

impl VecWrapperI32 {
    #[trusted]
    #[ensures(result.len() == 0)]
    fn new() -> Self {
        Self { v: Vec::new() }
    }

    #[trusted]
    #[pure]
    fn len(&self) -> usize {
        self.v.len()
    }

    #[trusted]
    #[ensures(self.len() == old(self.len()) + 1)]
    #[ensures(forall(|i: usize| i < self.len() - 1 ==> self.lookup(i) == old(self.lookup(i))))]
    #[ensures(self.lookup(self.len() - 1) == x)]
    fn push(&mut self, x: i32) {
        self.v.push(x)
    }

    #[trusted]
    #[pure]
    #[requires(i < self.len())]
    fn lookup(&self, i: usize) -> i32 {
        self.v[i]
    }

    #[trusted]
    #[requires(i < self.len())]
    #[ensures(*result == self.lookup(i))]
    fn get(&self, i: usize) -> &i32 {
        &self.v[i]
    }

    #[trusted]
    #[ensures(result == (self.len() == 0))]
    fn is_empty(&self) -> bool {
        self.v.is_empty()
    }

    #[trusted]
    #[requires(self.len() > 0)]
    #[ensures(self.len() == old(self.len()) - 1)]
    #[ensures(forall(|i: usize| i < self.len() ==> self.lookup(i) == old(self.lookup(i))))]
    #[ensures(result == old(self.lookup(self.len() - 1)))]
    fn pop(&mut self) -> i32 {
        self.v.pop().unwrap()
    }
}

fn toss() -> bool {
    true
}

fn test(val: i32) {
    // create a vector
    let mut vec = VecWrapperI32::new();
    let mut k = val;

    // fill it with values >= val
    while toss() && k < i32::MAX - 1 {
        body_invariant!(val <= k && k < i32::MAX - 1);
        body_invariant!(forall(|i: usize| i < vec.len() ==> val <= vec.lookup(i)));
        vec.push(k);
        k = k + 1
    }

    // assert values >= val using iter
    let mut i = 0;
    while i < vec.len() {
        body_invariant!(i < vec.len());
        assert!(val <= *vec.get(i));
        i += 1;
    }

    // assert values >= val using plain pop
    while !vec.is_empty() {
        body_invariant!(vec.len() > 0);
        body_invariant!(forall(|i: usize| i < vec.len() ==> val <= vec.lookup(i)));
        let v = vec.pop();
        assert!(val <= v);
    }
}

fn main() {
    test(0);
}
