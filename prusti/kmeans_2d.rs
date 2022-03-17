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
    fn get(&self, index: usize) -> i32 {
        self.v[index]
    }
}

/// Need this because Prusti doesn't seem to be able to handle polymorphic types
pub struct RMatI32 {
    inner: Vec<Vec<i32>>,
}

impl RMatI32 {
    #[trusted]
    fn from_elem_n(n:usize, elem: i32) -> Vec<i32>
    {
        let mut res = Vec::new();
        for _i in 0..n {
            res.push(elem);
        }
        res
    }

    //#[lr::assume]
    //#[lr::sig(fn(rows: usize, cols: usize, elem: T) -> RMatI32[rows, cols])]
    #[trusted]
    #[ensures(result.cols() == cols && result.rows() == rows)]
    pub fn new(rows: usize, cols: usize, elem: i32) -> RMatI32
    {
        let mut inner = Vec::<Vec<i32>>::new();
        for _i in 0..rows {
            let mut r = Vec::<i32>::new();
            for _i in 0..cols {
                r.push(elem);
            }
            inner.push(r);
        }
        Self { inner }
    }

    #[trusted]
    #[pure]
    pub fn rows(&self) -> usize {
        self.inner.len()
    }

    #[trusted]
    #[pure]
    pub fn cols(&self) -> usize {
        if self.inner.len() > 0 {
            self.inner[0].len()
        } else {
            0
        }
    }

    #[trusted]
    #[pure]
    #[requires(i < self.rows() && j < self.cols())]
    pub fn get(&self, i: usize, j: usize) -> i32 {
        self.inner[i][j]
    }

    #[trusted]
    #[requires(i < self.rows() && j < self.cols())]
    #[ensures(
        self.cols() == old(self.cols()) &&
        self.rows() == old(self.rows())
    )]
    pub fn set(&mut self, i: usize, j: usize, value: i32) {
        self.inner[i][j] = value;
    }

    #[trusted]
    #[requires(i < self.rows() && j < self.cols())]
    #[ensures(
        self.cols() == old(self.cols()) &&
        self.rows() == old(self.rows())
    )]
    #[after_expiry(
        self.cols() == old(self.cols()) &&
        self.rows() == old(self.rows())
    )]
    pub fn get_mut(&mut self, i: usize, j: usize) -> &mut i32 {
        &mut self.inner[i][j]
    }

    #[trusted]
    #[ensures(result.len() == self.cols())]
    pub fn get_row(&self, i: usize) -> VecWrapperI32 {
        VecWrapperI32 { v: self.inner[i].clone() }
    }

    #[trusted]
    //#[ensures(result.rows() == old(self.rows()))]
    //#[ensures(result.cols() == old(self.cols()))]
    pub fn clone(&self) -> RMatI32 {
        let mut new = Vec::<Vec<i32>>::new();
        for inner in &self.inner {
            new.push(inner.clone());
        }
        Self { inner: new }
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
fn dist(x:VecWrapperI32, y:&VecWrapperI32) -> i32 {
    let mut res = 0;
    let mut i = 0;
    while i < x.len() {
        body_invariant!(x.len() == y.len());
        body_invariant!(i <= x.len());
        let di = x.get(i) - y.get(i);
        res += di*di;
        i += 1;
    }
    res
}

/// adding two points (updates the first)
#[requires(x.cols() == y.len())]
#[requires(row < x.rows())]
#[ensures(x.rows() == old(x.rows()))]
#[ensures(x.cols() == old(x.cols()))]
fn add_to_row(x: &mut RMatI32, row: usize, y:&VecWrapperI32) {
    let mut i = 0;

    while i < x.cols() {
        body_invariant!(i <= x.cols());
        body_invariant!(x.rows() == old(x.rows()));
        body_invariant!(x.cols() == old(x.cols()));

        let xi = x.get(row, i);
        let yi = y.get(i);
        x.set(row, i, xi + yi);
        i += 1;
    }
}

/// normalizing a point (cluster) by size
#[requires(row < x.rows())]
#[ensures(x.rows() == old(x.rows()))]
#[ensures(x.cols() == old(x.cols()))]
fn normalize_row(x: &mut RMatI32, row: usize, n: usize) {
    let mut i = 0;

    while i < x.cols() {
        body_invariant!(i <= x.cols());
        body_invariant!(x.rows() == old(x.rows()));
        body_invariant!(x.cols() == old(x.cols()));

        let xi = x.get(row, i);
        x.set(row, i, i32_div(xi,n));
        i += 1;
    }
}

/// creating (empty) 0-center for each cluster
#[requires(k > 0)]
#[ensures(result.rows() == k)]
#[ensures(result.cols() == n)]
fn init_centers(n: usize, k: usize) -> RMatI32 {
    RMatI32::new(k, n, 0)
}

/// finding the nearest center to a point
#[requires(cs.rows() > 0)]
#[requires(cs.cols() == p.len())]
#[ensures(result < cs.rows())]
fn nearest(p:&VecWrapperI32, cs: &RMatI32) -> usize {
    // let n = p.len();
    let k = cs.rows();
    let mut res = 0;
    let mut min = i32_max();
    let mut i = 0;
    while i < k {
        body_invariant!(res < cs.rows());

        let ci = cs.get_row(i);
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
#[requires(cs.rows() == weights.len())]
#[ensures(cs.rows() == old(cs.rows()))]
#[ensures(cs.cols() == old(cs.cols()))]
fn normalize_centers(cs: &mut RMatI32, weights: &VecWrapperUSize) {
    let k = cs.rows();
    let mut i = 0;
    while i < k {
        body_invariant!(i <= cs.rows());
        body_invariant!(cs.rows() == old(cs.rows()));
        body_invariant!(cs.cols() == old(cs.cols()));
        normalize_row(cs, i, weights.lookup(i));
        
        i += 1;
    }
}

#[trusted]
#[requires(cs.rows() == ps.rows())]
#[requires(ps.cols() == n)]
#[requires(cs.cols() == n)]
#[requires(cs.rows() > 0)]
#[ensures(result.rows() == cs.rows())]
#[ensures(result.cols() == n)]
fn foo(n:usize, cs: RMatI32, ps: &RMatI32) -> RMatI32 {
    unimplemented!();
}

/// updating the centers
//#[lr::sig(fn(n:usize, cs: k@RVec<RVec<i32>[n]>{0 < k}, ps: &RVec<RVec<i32>[n]>) -> RVec<RVec<i32>[n]>[k])]
#[requires(cs.rows() == ps.rows())]
#[requires(ps.cols() == n)]
#[requires(cs.cols() == n)]
#[requires(cs.rows() > 0)]
#[ensures(result.rows() == old(cs.rows()))]
#[ensures(result.cols() == n)]
fn kmeans_step(n:usize, cs: RMatI32, ps: &RMatI32) -> RMatI32 {
    let k = cs.rows();

    let mut res_points = init_centers(n, k);

    let mut res_size = from_elem_n_usize(0, k);

    let mut i = 0;
    while i < ps.rows() {
        body_invariant!(res_points.cols() == cs.cols());
        body_invariant!(res_points.rows() == cs.rows());
        body_invariant!(res_size.len() == cs.rows());

        let p = ps.get_row(i);
        let j = nearest(&p, &cs);
        add_to_row(&mut res_points, j, &p);
        *res_size.index_mut(j) += 1;
        i += 1;
    }

    normalize_centers(&mut res_points, &res_size);

    res_points
}

/// kmeans: iterating the center-update-steps
//#[lr::sig(fn(n:usize, cs: k@RVec<RVec<i32>[n]>{0 < k}, ps: &RVec<RVec<i32>[n]>, iters: i32) -> RVec<RVec<i32>[n]>[k])]
#[requires(cs.rows() == ps.rows())]
#[requires(ps.cols() == n)]
#[requires(cs.cols() == n)]
#[requires(cs.rows() > 0)]
#[ensures(result.rows() == old(cs.rows()))]
#[ensures(result.cols() == n)]
pub fn kmeans(n:usize, cs: RMatI32, ps: &RMatI32, iters: i32) -> RMatI32 {
    kmeans_inner(0, n, cs, ps, iters)
}

#[requires(cs.rows() == ps.rows())]
#[requires(ps.cols() == n)]
#[requires(cs.cols() == n)]
#[requires(cs.rows() > 0)]
#[ensures(result.rows() == old(cs.rows()))]
#[ensures(result.cols() == n)]
pub fn kmeans_inner(i: i32, n: usize, cs: RMatI32, ps: &RMatI32, iters: i32) -> RMatI32 {
    if i < iters {
        //let clone = clone(ps);
        let res = kmeans_step(n, cs, ps);
        kmeans_inner(i+1, n, res, ps, iters)
    } else {
        cs
    }
}

pub fn main() {
}