# Change

Added refinements to wrap_index and count.

# Error

```
error[E0999]: refinement type error
   --> src/vec_deque.rs:589:5
    |
589 |     wrap_index(head.wrapping_sub(tail), size)
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ a precondition cannot be proved
    |
note: this is the condition that cannot be proved
   --> src/vec_deque.rs:575:32
    |
575 | #[flux::sig(fn(index: usize{v: v <= size}, size: Size) -> usize{v: v <= size})]
    |                                ^^^^^^^^^
```

Need to refine the signature on count.
