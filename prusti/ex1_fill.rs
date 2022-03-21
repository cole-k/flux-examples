extern crate prusti_contracts;

use prusti_contracts::*;
#[path = "lib/vecwrapper.rs"]
pub mod vecwrapper;
use vecwrapper::VecWrapper;

fn toss() -> bool {
    true
}

fn test(val: i32) {
    // create a vector
    let mut vec = VecWrapper::<i32>::new();
    let mut k = val;

    // fill it with values >= val
    while toss() && k < i32::MAX - 1 {
        body_invariant!(vec.len() >= 0);
        vec.push(k);
        k = k + 1;
    }

    // assert values >= val using iter
    let mut i = 0;
    while i < vec.len() {
        body_invariant!(vec.len() >= 0);
        assert!(i < vec.len());
        i += 1;
    }

    // assert values >= val using plain pop
    while !vec.is_empty() {
        body_invariant!(vec.len() > 0);
        let v = vec.pop();
        assert!(true);
    }
}

fn main() {
    test(0);
}
