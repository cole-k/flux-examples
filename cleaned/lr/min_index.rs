#![feature(register_tool)]
#![register_tool(flux)]

#[path = "lib/rvec.rs"]
mod rvec;
use rvec::RVec;

#[flux::sig(fn({RVec<i32>[@l] : l > 0 }) -> usize{x: x < l})]
pub fn min_index(vec: RVec<i32>) -> usize {
    let sz = vec.len();
    let mut res: usize = 0;
    let mut i: usize = 0;

    while i < sz {
        res = if *vec.get(i) < *vec.get(res) { i } else { res };

        i = i + 1;
    }
    res
}
