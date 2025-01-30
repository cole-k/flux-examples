# Change

Added a signature to `with_capacity_in`

# Error

```
error[E0999]: refinement type error
   --> src/vec_deque.rs:249:9
    |
249 |         Self::with_capacity_in(capacity, Global)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ a precondition cannot be proved
    |
note: this is the condition that cannot be proved
   --> src/vec_deque.rs:279:37
    |
279 |     #[flux::sig(fn (capacity: usize{capacity < MAXIMUM_ZST_CAPACITY && capacity ...    |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

Fix: add a signature to `with_capacity`.
