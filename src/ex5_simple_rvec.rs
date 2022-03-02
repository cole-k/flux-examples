#![allow(unused_attributes)]
#![feature(register_tool)]
#![register_tool(lr)]

#[path = "lib/rvec.rs"]
pub mod rvec;
use rvec::RVec;

#[lr::ty(fn<len: int{len >= 2}>(l: RVec<i32>@len; ref<l>, bool) -> i32@0; l: RVec<i32>@len)]
pub fn test1(vec: &mut RVec<i32>, b: bool) -> i32 {
    let r;
    if b {
        r = vec.get_mut(0);
    } else {
        r = vec.get_mut(1);
    }
    *r = 12;
    0
}
