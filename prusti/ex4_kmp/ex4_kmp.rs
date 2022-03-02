extern crate prusti_contracts;
use prusti_contracts::*;

pub struct StrWrapper {
    v: Vec<char>,
}

/// Need this because Prusti doesn't seem to be able to handle polymorphic types
impl StrWrapper {
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
    pub fn lookup(&self, index: usize) -> char {
        self.v[index]
    }
}

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

    /// A ghost function for specifying values stored in the vector.
    #[trusted]
    #[requires(index < self.len())]
    #[ensures(self.len() == old(self.len()) && forall(
        |i: usize| (i < self.len() && i != index) ==>
        self.lookup(i) == old(self.lookup(i))
    ) && self.lookup(index) == value)]
    pub fn store(&mut self, index: usize, value: usize) {
        self.v[index] = value;
    }
}

pub enum TrustedOption {
    Some(usize),
    None,
}

impl TrustedOption {
    #[pure]
    pub fn is_none(&self) -> bool {
        match self {
            TrustedOption::Some(_) => false,
            TrustedOption::None => true,
        }
    }

    #[pure]
    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    #[pure]
    #[requires(self.is_some())]
    pub fn peek(&self) -> usize {
        match self {
            TrustedOption::Some(val) => *val,
            TrustedOption::None => unreachable!(),
        }
    }
}

/// Prusti cannot reason about vector initialization, so this wrapper is necessary to ensure that Prusti knows that the vecwrapper has the correct size and is filled with zeros
#[trusted]
#[ensures(result.len() == len)]
#[ensures(forall(|x: usize| x < result.len() ==> result.lookup(x) == 0))]
pub fn init_vec_wrapper_usize(len: usize) -> VecWrapperUSize {
    VecWrapperUSize { v: vec![0; len] }
}

// rust port of https://github.com/ucsd-progsys/liquidhaskell/blob/develop/tests/pos/kmpVec.hs
#[requires(p.len() > 0)]
#[ensures(result.len() == p.len())]
#[ensures(forall(|x: usize| x < result.len() ==> result.lookup(x) <= p.len()))]
fn kmp_table(p: &StrWrapper) -> VecWrapperUSize {
    let m = p.len();
    let mut t = init_vec_wrapper_usize(m);
    let mut i = 1;
    let mut j = 0;
    while i < m {
        body_invariant!(forall(|x: usize| x < t.len() ==> t.lookup(x) <= i));
        body_invariant!(j <= i);
        body_invariant!(i < p.len());
        body_invariant!(m == t.len());
        body_invariant!(t.len() == p.len());

        if p.lookup(i) == p.lookup(j) {
            t.store(i, j + 1);
            i = i + 1;
            j = j + 1;
        } else if j == 0 {
            let zero = 0;
            t.store(i, zero);
            i = i + 1;
        } else {
            //check_assert(&p,&t,i,j);
            j = t.lookup(j - 1);
        }
    }
    t
}

#[requires((pattern.len() > 0) && (target.len() > 0) && (target.len() >= pattern.len()))]
#[ensures(result.is_some() ==> result.peek() < target.len())]
fn kmp_search(pattern: StrWrapper, target: StrWrapper) -> TrustedOption {
    let mut t_i: usize = 0;
    let mut p_i: usize = 0;
    let target_len: usize = target.len();
    let pattern_len = pattern.len();
    let mut result_idx = 0;

    let t = kmp_table(&pattern);

    while (t_i <= target_len - 1) && (p_i <= pattern_len - 1) {
        body_invariant!(t_i < target_len);
        body_invariant!(p_i < pattern_len);
        body_invariant!(t.len() == pattern.len());
        body_invariant!(p_i < target_len);
        body_invariant!(forall(|x: usize| x < t.len() ==> t.lookup(x) <= target.len()));
        body_invariant!(result_idx <= t_i);

        if target.lookup(t_i) == pattern.lookup(p_i) {
            if result_idx == 0 {
                result_idx = t_i;
            }
            t_i = t_i + 1;
            p_i = p_i + 1;
            if p_i >= pattern_len {
                return TrustedOption::Some(result_idx);
            }
        } else {
            if p_i == 0 {
                p_i = 0;
            } else {
                p_i = t.lookup(p_i - 1);
            }
            t_i = t_i + 1;
            result_idx = 0;
        }
    }
    TrustedOption::None
}

/*pub fn search(pat: &str, str: &str) -> i32 {
    //let res = kmp_search(pat, str);
    //println!("kmp_search: pat = {}, str = {}, res = {:?}", pat, str, res);
    //res
}*/

pub fn main() {}
