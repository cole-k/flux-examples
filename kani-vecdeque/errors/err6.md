# Change

Added signatures to count and wrap_index.

# Error

```
error[E0999]: refinement type error
   --> src/vec_deque.rs:418:9
    |
418 |         count(self.tail, self.head, self.cap())
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ a postcondition cannot be proved
    |
note: this is the condition that cannot be proved
   --> src/vec_deque.rs:416:56
    |
416 |     #[flux::sig(fn (&VecDeque<T,A>[@self]) -> usize{v: v < self.cap})]
    |                                                        ^^^^^^^^^^^^
```

Needed to change from `<= size` to `< size`.
