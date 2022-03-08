# Examples

## Run liquid-rust

To run liquid-rust in one of the examples copy the [liquid-rust](liquid-rust) script to somewhere in your path
and edit the file to define the `LIQUID_RUST` variable to point to the root of the liquid-rust repository.
Then you should be able to run it on examples like so

```bash
$ liquid-rust --crate-type=rlib src/ex5_simple_rvec.rs
```

## Comparisons with Prusti

| Example Name  | Prusti          | LR              | Source               |
|:--------------|:----------------|:----------------|:---------------------|
| Fill          | [Pr][pr-fil]    |                 | toy                  |
| Min Index     | [Pr][pr-min]    | [LR][lr-min]    | toy                  |
| KMP           | [Pr][pr-kmp]    | [LR][lr-kmp]    | [LH][src-kmp]        |
| Union-Find    | [Pr][pr-uf]     |                 | [src][src-uf]        |
| Heap-Sort     | [Pr][pr-hps]    | [LR][lr-hps]    | [rosetta][src-hps]   |
| Knight's Tour | [Pr][pr-knt]    |                 | [rosetta][src-knt]   |
| Knuth Shuffle | [Pr][pr-shuf]   | [LR][lr-shuf]   | [rosetta][src-shuf]  |
| Copy          |                 | [LR][lr-bc]     | DML                  |
| BinSearch     |                 | [LR][lr-bc]     | DML                  |
| Dot-Product   |                 | [LR][lr-dot]    | DML                  |
| FFT           |                 | [LR][lr-fft]    | DML                  |
| Simplex       |                 | [LR][lr-splx]   | DML                  |
| Kmeans        |                 | [LR][lr-kmeans] | self                 |

[lr-bc]:     https://github.com/liquid-rust/examples/blob/main/src/bcopy.rs
[lr-bs]:     https://github.com/liquid-rust/examples/blob/main/src/bsearch.rs
[lr-dot]:    https://github.com/liquid-rust/examples/blob/main/src/dotprod.rs
[lr-kmeans]: https://github.com/liquid-rust/examples/blob/main/src/kmeans.rs
[lr-fft]:    https://github.com/liquid-rust/examples/blob/main/src/fft.rs
[lr-splx]:   https://github.com/liquid-rust/examples/blob/main/src/simplex.rs
[pr-fil]: https://github.com/liquid-rust/examples/blob/main/prusti/ex1_fill/ex1_fill.rs
[pr-min]: https://github.com/liquid-rust/examples/blob/main/prusti/ex2_min_index_loop/ex2_min_index_loop.rs
[lr-min]: https://github.com/liquid-rust/liquid-rust/blob/atgeller/new_tests/liquid-rust-tests/tests/pos/ex2_min_index_loop.rs
[pr-kmp]: https://github.com/liquid-rust/examples/blob/main/prusti/ex4_kmp/ex4_kmp.rs
[lr-kmp]: https://github.com/liquid-rust/liquid-rust/blob/main/liquid-rust-tests/tests/pos/kmp.rs
[pr-uf]:  https://github.com/liquid-rust/examples/blob/main/prusti/union_find/partition_vec.rs
[src-uf]: https://github.com/DDOtten/partitions
[src-kmp]: https://github.com/ucsd-progsys/liquidhaskell/blob/develop/tests/pos/kmpVec.hs
[pr-hps]: https://github.com/viperproject/prusti-dev/blob/master/prusti-tests/tests/verify/pass/rosetta/Heapsort.rs
[lr-hps]: https://github.com/liquid-rust/liquid-rust/blob/main/liquid-rust-tests/tests/pos/heapsort.rs
[src-hps]: https://github.com/viperproject/prusti-dev/tree/master/prusti-tests/tests/verify/pass/rosetta
[pr-knt]: https://github.com/viperproject/prusti-dev/blob/master/prusti-tests/tests/verify/pass/rosetta/Knights_tour.rs
[src-knt]: https://github.com/viperproject/prusti-dev/tree/master/prusti-tests/tests/verify/pass/rosetta
[pr-shuf]: https://github.com/viperproject/prusti-dev/blob/master/prusti-tests/tests/verify/pass/rosetta/Knuth_shuffle.rs
[lr-shuf]: https://github.com/liquid-rust/liquid-rust/blob/atgeller/new_tests/liquid-rust-tests/tests/pos/knuth_shuffle.rs
[src-shuf]: https://github.com/viperproject/prusti-dev/tree/master/prusti-tests/tests/verify/pass/rosetta


## Dsolve Examples

- [~] src/arraymax.rs (requires HOF/`Fn` support)


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
