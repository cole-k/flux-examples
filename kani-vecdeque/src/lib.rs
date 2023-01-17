#![feature(slice_range)]
#![feature(extend_one)]
#![feature(try_reserve_kind)]
#![feature(allocator_api)]
#![feature(dropck_eyepatch)]
#![feature(rustc_attrs)]
#![feature(core_intrinsics)]
#![feature(ptr_internals)]
#![feature(rustc_allow_const_fn_unstable)]
#![feature(register_tool)]
#![register_tool(flux)]
#![allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]
#![allow(unused_comparisons)]
#![feature(custom_inner_attributes)]
#![flux::defs {
    fn pow2(x:int) -> bool;
    fn size(n:int) -> bool { pow2(n) && 1 <= n }
  }]
pub mod cve;
pub mod raw_vec;
