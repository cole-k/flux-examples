extern crate prusti_contracts;
use prusti_contracts::*;

pub struct VecWrapperUSize {
    v: Vec<usize>,
}

/// Need this because Prusti doesn't seem to be able to handle polymorphic types
impl VecWrapperUSize {
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
    pub fn lookup(&self, index: usize) -> usize {
        self.v[index]
    }

    #[trusted]
    #[requires(index < self.len())]
    #[ensures(*result == old(self.lookup(index)))]
    #[after_expiry(
        self.len() == old(self.len()) &&
        self.lookup(index) == before_expiry(*result) &&
        forall(
            |i: usize| (i < self.len() && i != index) ==>
            self.lookup(i) == old(self.lookup(i))
        )
    )]
    pub fn index_mut(&mut self, index: usize) -> &mut usize {
        self.v.get_mut(index).unwrap()
    }
}

#[derive(PartialEq, Clone)]
pub struct VecWrapperI32 {
    v: Vec<i32>,
}

/// Need this because Prusti doesn't seem to be able to handle polymorphic types
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
    fn lookup(&self, index: usize) -> i32 {
        self.v[index]
    }

    /// A ghost function for specifying values stored in the vector.
    #[trusted]
    #[requires(index < self.len())]
    #[ensures(self.len() == old(self.len()))]
    /* && forall(
        |i: usize| (i < self.len() && i != index) ==>
        self.lookup(i) == old(self.lookup(i))
    ) && self.lookup(index) == value)]*/
    pub fn store(&mut self, index: usize, value: i32) {
        self.v[index] = value;
    }

    #[trusted]
    #[requires(index < self.len())]
    //#[ensures(*result == old(self.lookup(index)))]
    #[after_expiry(
        self.len() == old(self.len())// &&
        //self.lookup(index) == before_expiry(*result) &&
        /*forall(
            |i: usize| (i < self.len() && i != index) ==>
            self.lookup(i) == old(self.lookup(i))
        )*/
    )]
    pub fn index_mut(&mut self, index: usize) -> &mut i32 {
        self.v.get_mut(index).unwrap()
    }

    #[trusted]
    #[ensures(result.len() == self.len())]
    //#[ensures(forall(|i: usize| i < result.len() ==> result.lookup(i) == self.lookup(i)))]
    pub fn clone(&self) -> VecWrapperI32 {
        Self { v: self.v.clone() }
    }

    /*
    #[trusted]
    #[ensures(result == (self.len() == other.len() && forall(|i: usize| (i < self.len()) ==> (self.lookup(i) == other.lookup(i)))))]
    pub fn equal(&mut self, other: VecWrapperI32) -> bool {
        self.v == other.v
    }*/
}

predicate! {
    fn vec_wrappers_eq(v1: &VecWrapperI32, v2: &VecWrapperI32) -> bool {
        (v1.len() == v2.len())
        // && forall(|i: usize|
        //      i < v1.len() ==> v1.lookup(i) == v2.lookup(i)))
    }
}

pub struct VecWrapperVecI32 {
    v: Vec<VecWrapperI32>,
}

/// Need this because Prusti doesn't seem to be able to handle polymorphic types
impl VecWrapperVecI32 {
    #[trusted]
    #[ensures(result.len() == 0)]
    pub fn new() -> Self {
        Self { v: Vec::new() }
    }

    /// A ghost function for getting the length of the vector
    #[trusted]
    #[pure]
    pub fn len(&self) -> usize {
        self.v.len()
    }

    /*#[trusted]
    #[pure]
    pub fn get_clone(&self, index: usize) -> VecWrapperI32 {
        self.v[index].clone()
    }*/

    /// A ghost function for specifying values stored in the vector.
    /*
    #[trusted]
    #[requires(index < self.len())]
    fn lookup(&self, index: usize) -> VecWrapperI32 {
        self.v[index].clone()
    }*/

    /// A ghost function for specifying values stored in the vector.
    #[trusted]
    #[pure]
    #[requires(index < self.len())]
    pub fn get(&self, index: usize) -> &VecWrapperI32 {
        &self.v[index]
    }

    /// A ghost function for specifying values stored in the vector.
    #[trusted]
    #[ensures(self.len() == old(self.len()) + 1 &&
    vec_wrappers_eq(self.get(self.len() - 1), &value) &&
    forall(
        |i: usize| i < old(self.len()) ==>
        vec_wrappers_eq(self.get(i), old(self.get(i))))
    )]
    pub fn push(&mut self, value: VecWrapperI32) {
        self.v.push(value);
    }

    #[trusted]
    #[requires(index < self.len())]
    #[ensures(vec_wrappers_eq(old(self.get(index)), result))]
    #[after_expiry(
        self.len() == old(self.len()) &&
        vec_wrappers_eq(self.get(index), before_expiry(result)) &&
        forall(
            |i: usize| (i < self.len() && i != index) ==>
            vec_wrappers_eq(old(self.get(i)), self.get(i))
        )
    )]
    pub fn index_mut(&mut self, index: usize) -> &mut VecWrapperI32 {
        self.v.get_mut(index).unwrap()
    }
}

/////////////////////////////////////////////////////////////
#[trusted]
fn i32_max() -> i32 {
    i32::MAX
}

#[trusted]
fn i32_div(n:i32, d:usize) -> i32 {
    n / (d as i32)
}

#[trusted]
#[ensures(result.len() == n)]
pub fn from_elem_n_i32(elem: i32, n: usize) -> VecWrapperI32
{
    let mut vec = Vec::new();
    let mut i = 0;
    while i < n {
        vec.push(elem);
        i += 1;
    }
    VecWrapperI32 { v: vec }
}

#[trusted]
#[ensures(result.len() == n)]
pub fn from_elem_n_usize(elem: usize, n: usize) -> VecWrapperUSize
{
    let mut vec = Vec::new();
    let mut i = 0;
    while i < n {
        vec.push(elem);
        i += 1;
    }
    VecWrapperUSize { v: vec }
}

/////////////////////////////////////////////////////////////

/// distance between two points
#[requires(x.len() == y.len())]
fn dist(x:&VecWrapperI32, y:&VecWrapperI32) -> i32 {
    let mut res = 0;
    let mut i = 0;
    while i < x.len() {
        body_invariant!(x.len() == y.len());
        body_invariant!(i <= x.len());
        let di = x.lookup(i) - y.lookup(i);
        res += di*di;
        i += 1;
    }
    res
}

/// adding two points (updates the first)
#[requires(x.len() == y.len())]
fn add(x:&mut VecWrapperI32, y:&VecWrapperI32) -> i32 {
    let mut i = 0;
    let n = x.len();
    while i < n {
        body_invariant!(x.len() == n);
        body_invariant!(i < n);

        let xi = x.lookup(i);
        let yi = y.lookup(i);
        *x.index_mut(i) = xi + yi;
        i += 1;
    }
    0
}

/// normalizing a point (cluster) by size
#[ensures(x.len() == old(x.len()))]
fn normal(x:&mut VecWrapperI32, n: usize) -> i32 {
    let mut i = 0;
    while i < x.len() {
        body_invariant!(i <= x.len());
        body_invariant!(x.len() == old(x.len()));

        let xi = x.lookup(i);
        x.store(i, i32_div(xi,n));
        i += 1;
    }
    0
}

/// creating (empty) 0-center for each cluster
#[requires(k > 0)]
#[ensures(result.len() == k)]
#[ensures(forall(|i: usize| i < result.len() ==> result.get(i).len() == n))]
fn init_centers(n: usize, k: usize) -> VecWrapperVecI32 {
  let mut res = VecWrapperVecI32::new();
  let mut i = 0;
  // Prusti complains otherwise
  let zero = 0;
  while i < k {
      let center = from_elem_n_i32(zero, n);
      res.push(center);
      i += 1;
  }
  res
}

/// finding the nearest center to a point
#[requires(cs.len() > 0)]
#[requires(forall(|i: usize| i < cs.len() ==> cs.get(i).len() == p.len()))]
#[ensures(result < cs.len())]
fn nearest(p:&VecWrapperI32, cs: &VecWrapperVecI32) -> usize {
    // let n = p.len();
    let k = cs.len();
    let mut res = 0;
    let mut min = i32_max();
    let mut i = 0;
    while i < k {
        body_invariant!(forall(|i: usize| i < cs.len() ==> cs.get(i).len() == p.len()));
        //body_invariant!(i < cs.len());
        body_invariant!(res < cs.len());

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

// TODO: the `n` is not needed, except to prevent a silly parse error!
//#[lr::sig(fn(n: usize, cs: &mut k@RVec<RVec<i32>[n]>, weights: &RVec<usize>[k]) -> i32)]
fn normalize_centers(_n: usize, cs: &mut VecWrapperVecI32, weights: &VecWrapperUSize) -> i32 {
    let k = cs.len();
    let mut i = 0;
    while i < k {
        normal(cs.index_mut(i), weights.lookup(i));
        
        i += 1;
    }
    0
}

/// updating the centers
//#[lr::sig(fn(n:usize, cs: k@RVec<RVec<i32>[n]>{0 < k}, ps: &RVec<RVec<i32>[n]>) -> RVec<RVec<i32>[n]>[k])]
fn kmeans_step(n:usize, cs: VecWrapperVecI32, ps: &VecWrapperVecI32) -> VecWrapperVecI32 {
    let k = cs.len();

    let mut res_points = init_centers(n, k);

    let mut res_size = from_elem_n_usize(0, k);

    let mut i = 0;
    while i < ps.len() {
        let p = ps.get(i);
        let j = nearest(p, &cs);
        add(res_points.index_mut(j), &p);
        *res_size.index_mut(j) += 1;
        i += 1;
    }

    normalize_centers(n, &mut res_points, &res_size);

    res_points
}

/// kmeans: iterating the center-update-steps
//#[lr::sig(fn(n:usize, cs: k@RVec<RVec<i32>[n]>{0 < k}, ps: &RVec<RVec<i32>[n]>, iters: i32) -> RVec<RVec<i32>[n]>[k])]
pub fn kmeans(n:usize, cs: VecWrapperVecI32, ps: &VecWrapperVecI32, iters: i32) -> VecWrapperVecI32 {
    let mut i = 0;
    let mut res = cs;
    while i < iters {
        res = kmeans_step(n, res, ps);
        i += 1;
    }
    res
}

pub fn main() {
}
