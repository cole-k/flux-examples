#![feature(register_tool)]
#![register_tool(lr)]

#[path = "lib/rvec.rs"]
mod rvec;
use rvec::RVec;

#[lr::sig(fn(vec: &mut n@RVec<i32>) -> i32; vec:RVec<i32>[n])]
pub fn heap_sort(vec: &mut RVec<i32>) -> i32 {
    let len = vec.len();

    if len <= 0 {
         return 0;
    }

    let mut start = len / 2;
    while start > 0 {
        start -= 1;
        shift_down(vec, start, len - 1);
    }

    let mut end = len;
    while end > 1 {
        end -= 1;
        let start = 0;
        vec.swap(start, end);
        shift_down(vec, start, end - 1);
    }
    0
}

#[lr::sig(fn(vec: &mut len@RVec<i32>, s:usize{0 <= s && s < len}, e:usize{0 <= e && e < len}) -> i32; vec: RVec<i32>[len])]
pub fn shift_down(vec: &mut RVec<i32>, start: usize, end: usize) -> i32 {
    let mut root = start;
    loop {
        let mut child = root * 2 + 1;
        if child > end {
            break;
        } else {
            if child + 1 <= end {
                if *vec.get(child) < *vec.get(child + 1) {
                    child += 1;
                }
            }
            if *vec.get(root) < *vec.get(child) {
                vec.swap(root, child);
                root = child;
            } else {
                break;
            }
        }
    }
    0
}
