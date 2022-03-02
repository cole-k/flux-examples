# Examples

## Run liquid-rust
To run liquid-rust in one of the examples copy the [liquid-rust](liquid-rust) script to somewhere in your path
and edit the file to define the `LIQUID_RUST` variable to point to the root of the liquid-rust repository.
Then you should be able to run it on examples like so

```bash
$ liquid-rust --crate-type=rlib src/ex5_simple_rvec.rs
```

## Comparisons with Prusti
| Example Name | Versions | Source |
| ------------ | -------- | ------ |
| Fill         | [Prusti](https://github.com/liquid-rust/examples/blob/main/prusti/ex1_fill/ex1_fill.rs) | Made up? |
| Min Index (Loop-based) | [Prusti](https://github.com/liquid-rust/examples/blob/main/prusti/ex2_min_index_loop/ex2_min_index_loop.rs), [Liquid Rust](https://github.com/liquid-rust/liquid-rust/blob/atgeller/new_tests/liquid-rust-tests/tests/pos/ex2_min_index_loop.rs) | Made up? |
| KMP | [Prusti](https://github.com/liquid-rust/examples/blob/main/prusti/ex4_kmp/ex4_kmp.rs), [Liquid Rust](https://github.com/liquid-rust/liquid-rust/blob/main/liquid-rust-tests/tests/pos/kmp.rs) | Made up? |
| Union-Find | [Prusti](https://github.com/liquid-rust/examples/blob/main/prusti/union_find/partition_vec.rs) | https://github.com/DDOtten/partitions |
| Heapsort | [Prusti](https://github.com/viperproject/prusti-dev/blob/master/prusti-tests/tests/verify/pass/rosetta/Heapsort.rs), [Liquid Rust](https://github.com/liquid-rust/liquid-rust/blob/main/liquid-rust-tests/tests/pos/heapsort.rs) | [Prusti Tests](https://github.com/viperproject/prusti-dev/tree/master/prusti-tests/tests/verify/pass/rosetta) |
| Knight's Tour | [Prusti](https://github.com/viperproject/prusti-dev/blob/master/prusti-tests/tests/verify/pass/rosetta/Knights_tour.rs) | [Prusti Tests](https://github.com/viperproject/prusti-dev/tree/master/prusti-tests/tests/verify/pass/rosetta) |
| Knuth Shuffle | [Prusti](https://github.com/viperproject/prusti-dev/blob/master/prusti-tests/tests/verify/pass/rosetta/Knuth_shuffle.rs), [Liquid Rust](https://github.com/liquid-rust/liquid-rust/blob/atgeller/new_tests/liquid-rust-tests/tests/pos/knuth_shuffle.rs) | [Prusti Tests](https://github.com/viperproject/prusti-dev/tree/master/prusti-tests/tests/verify/pass/rosetta) |

## Dsolve Examples

- [ ] *RJ* https://github.com/ucsd-progsys/dsolve/blob/master/tests/PLDI2008/arraymax.ml
- [ ] *RJ* https://github.com/ucsd-progsys/dsolve/blob/master/tests/PLDI2008/bcopy.ml
- [ ] *RJ* https://github.com/ucsd-progsys/dsolve/blob/master/tests/PLDI2008/bsearch.ml
- [ ] *RJ* https://github.com/ucsd-progsys/dsolve/blob/master/tests/PLDI2008/dotprod.ml
- [ ] *AG* https://github.com/ucsd-progsys/dsolve/blob/master/tests/PLDI2008/dotprod2.ml
- [ ] *AG* https://github.com/ucsd-progsys/dsolve/blob/master/tests/PLDI2008/fft.ml
- [ ] *AG* https://github.com/ucsd-progsys/dsolve/tree/master/tests/PLDI2008/simplex.ml

## Crates.io Examples
TODO: Filling in possibilities for now, will narrow down later

### Core
| Example Name | Crate | Version Number | Verified | Notes |
| ------------ | ----- | -------------- | -------- | ----- |
| [write](https://github.com/rust-lang/rust/blob/181e91567c9f347e055b33b1d7e9894f769aafe3/library/core/src/fmt/mod.rs#L1154) | core/fmt | N/A | NO | Unsafe fn |
| [run](https://github.com/rust-lang/rust/blob/181e91567c9f347e055b33b1d7e9894f769aafe3/library/core/src/fmt/mod.rs#L1198) | core/fmt | N/A | NO | Unsafe fn |
| [getcount](https://github.com/rust-lang/rust/blob/181e91567c9f347e055b33b1d7e9894f769aafe3/library/core/src/fmt/mod.rs#L1219) | core/fmt | N/A | NO | Unsafe fn |
| [next](https://github.com/rust-lang/rust/blob/ce0f7baf5651606c706b7014b5abdaa930cf2600/library/core/src/array/iter.rs#L241) | core/array | N/A | NO | Invariant may be tricky |
| [fold](https://github.com/rust-lang/rust/blob/ce0f7baf5651606c706b7014b5abdaa930cf2600/library/core/src/array/iter.rs#L264) | core/array | N/A | NO | See above |
| [advance_by](https://github.com/rust-lang/rust/blob/ce0f7baf5651606c706b7014b5abdaa930cf2600/library/core/src/array/iter.rs#L285) | core/array | N/A | NO | See above |
| [next_back](https://github.com/rust-lang/rust/blob/ce0f7baf5651606c706b7014b5abdaa930cf2600/library/core/src/array/iter.rs#L309) | core/array | N/A | NO | See above |
| [advance_back_by](https://github.com/rust-lang/rust/blob/ce0f7baf5651606c706b7014b5abdaa930cf2600/library/core/src/array/iter.rs#L326) | core/array | N/A | NO | See above

### Std
| Example Name | Crate | Version Number | Verified | Notes |
| ------------ | ----- | -------------- | -------- | ----- |
| [as_bytes](https://github.com/rust-lang/rust/blob/8f117a77d0880ed59afcc1a19c72ec5c1e44b97c/library/std/src/ffi/c_str.rs#L623) | N/A | NO | Rust CStrings invariantly have len >= 1 (null terminator) |
| [drop](https://github.com/rust-lang/rust/blob/8f117a77d0880ed59afcc1a19c72ec5c1e44b97c/library/std/src/ffi/c_str.rs#L778) | std/ffi/cstring | N/A | NO | See above |
| [to_bytes](https://github.com/rust-lang/rust/blob/8f117a77d0880ed59afcc1a19c72ec5c1e44b97c/library/std/src/ffi/c_str.rs#L1347) | std/ffi/cstring | N/A | NO | See above, may be very similar to as_bytes |

