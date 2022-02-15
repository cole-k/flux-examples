extern crate prusti_contracts;
use prusti_contracts::*;

pub struct VecWrapperI32 {
    v: Vec<i32>
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
    pub fn lookup(&self, index: usize) -> i32 {
        self.v[index]
    }
}

// Note: reduce helper function inlined because support for closures not yet in Prusti
#[requires(vec.len() > 0)]
#[ensures(vec.len() == old(vec.len()))]
#[ensures(result < vec.len())]
// Removed as these are for full correctness, not just array bounds
//#[ensures(forall(|i: usize| i < vec.len() ==> old(vec.lookup(i)) == vec.lookup(i)))]
//#[ensures(forall(|i: usize|
//    (i < vec.len() ==> (vec.lookup(result) <= vec.lookup(i)))))]
fn min_index(vec:VecWrapperI32) -> usize {
    let mut res = 0;
    let sz = vec.len();
    let mut i = 0;

    while i < sz {
        body_invariant!(i < sz);
        body_invariant!(res < sz);

        // Removed as these are for full correctness, not just array bounds
        //body_invariant!(forall(|y: usize| y < vec.len() ==> old(vec.lookup(y)) == vec.lookup(y)));
        //body_invariant!(forall(|x: usize|
        //    (x < i ==> (vec.lookup(res) <= vec.lookup(x)))));

        res = if vec.lookup(i) < vec.lookup(res) {
            i
        } else {
            res
        };

        i = i + 1;
    }
    res
}

pub fn main() {
    // Have to wrap in VecWrapperI32 now
    //let v0:Vec<i32> = vec![11,30,2,12,41,10,15,32,1,99];
    //println!("ex_2_min_index_loop = {:?}", min_index(v0));
}