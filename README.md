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
| Copy          | [Pr][pr-bc]     | [LR][lr-bc]     | DML                  |
| BinSearch     | [Pr][pr-bs]     | [LR][lr-bc]     | DML                  |
| Dot-Product   | [Pr][pr-dot]    | [LR][lr-dot]    | DML                  |
| FFT           | [Pr][pr-fft]    | [LR][lr-fft]    | DML                  |
| Simplex       | [Pr][pr-splx]   | [LR][lr-splx]   | DML                  |
| Kmeans        | [Pr*][pr-kmns]  | [LR][lr-kmeans] | self                 |

[lr-bc]:     https://github.com/liquid-rust/examples/blob/main/lr/src/bcopy.rs
[pr-bc]:     https://github.com/liquid-rust/examples/blob/main/prusti/bcopy.rs
[lr-bs]:     https://github.com/liquid-rust/examples/blob/main/lr/src/bsearch.rs
[pr-bs]:     https://github.com/liquid-rust/examples/blob/main/prusti/bsearch.rs
[lr-dot]:    https://github.com/liquid-rust/examples/blob/main/lr/src/dotprod.rs
[pr-dot]:    https://github.com/liquid-rust/examples/blob/main/prusti/dotprod.rs
[lr-kmeans]: https://github.com/liquid-rust/examples/blob/main/lr/src/kmeans.rs
[pr-kmns]:   https://github.com/liquid-rust/examples/blob/main/prusti/kmeans_2d.rs
[lr-fft]:    https://github.com/liquid-rust/examples/blob/main/lr/src/fft.rs
[pr-fft]:    https://github.com/liquid-rust/examples/blob/main/prusti/fft.rs
[lr-splx]:   https://github.com/liquid-rust/examples/blob/main/lr/src/simplex.rs
[pr-splx]:   https://github.com/liquid-rust/examples/blob/main/prusti/simplex.rs
[pr-fil]: https://github.com/liquid-rust/examples/blob/main/prusti/ex1_fill.rs
[pr-min]: https://github.com/liquid-rust/examples/blob/main/prusti/ex2_min_index_loop.rs
[lr-min]: https://github.com/liquid-rust/liquid-rust/blob/atgeller/new_tests/liquid-rust-tests/tests/pos/ex2_min_index_loop.rs
[pr-kmp]: https://github.com/liquid-rust/examples/blob/main/prusti/ex4_kmp.rs
[lr-kmp]: https://github.com/liquid-rust/liquid-rust/blob/main/liquid-rust-tests/tests/pos/kmp.rs
[pr-uf]:  https://github.com/liquid-rust/examples/blob/main/prusti/partition_vec.rs
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




# Notes on Comparison with Prusti (WIP)
## Hard to reason about nested data structures
While it is possible to work around this issue, you are constrained in how you specify nested data structures.
Example: `simplex.rs` -- need to make 2D matrix wrapper, whereas in liquid rust you don't need to make one

There's also an ergonomics issue since the nested structure may not implement copy, so you have to pass around references.

### Especially mutable nested data structures
A big part of the issue is from https://github.com/viperproject/prusti-dev/issues/389, which seems to pop up in a lot of places when working with mutable nested data structures (e.g., in `kmeans.rs`).
Additionally, there's the following error in Prusti currently: https://github.com/viperproject/prusti-dev/issues/903.

With mutation, you want to use `==` to reason about the data in the outer structure.
However, the default implementation of `==` for, e.g., vectors isn't pure so can't be used in the specification.
Example: see `vec_test.rs`.


## Loop invariants often necessary, sometimes redundant
### Examples
`ex2_min_index_loop.rs` -- need `body_invariant!(i < sz);` despite the loop condition being `i < sz`.
`ex4_kmp.rs` -- need `body_invariant!(t.len() == pattern.len());` despite neither vector being mutable and their lengths not being changed