# Change
Added the signature
```
#[flux::sig(fn (capacity: usize{capacity < MAXIMUM_ZST_CAPACITY && capacity > 1}, alloc: A) -> VecDeque<T, A>{v: v.head == 0 && v.tail == 0 && capacity <= v.cap})]
```
on L275

# Next errors

1. Propagate precondition
2. Ensure that flux can satisfy VecDeque conditions.

```
error[E0999]: refinement type error
   --> src/vec_deque.rs:245:9
    |
245 |         Self::with_capacity_in(capacity, Global)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ a precondition cannot be proved
    |
note: this is the condition that cannot be proved
   --> src/vec_deque.rs:275:37
    |
275 |     #[flux::sig(fn (capacity: usize{capacity < MAXIMUM_ZST_CAPACITY && capacity > 1}, alloc: A) -> VecDeque<T, A>{v: v.head == 0 && v.tail == 0 && capaci...
    |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0999]: refinement type error
   --> src/vec_deque.rs:245:9
    |
245 |         Self::with_capacity_in(capacity, Global)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ a precondition cannot be proved
    |
note: this is the condition that cannot be proved
   --> src/vec_deque.rs:275:72
    |
275 |     #[flux::sig(fn (capacity: usize{capacity < MAXIMUM_ZST_CAPACITY && capacity > 1}, alloc: A) -> VecDeque<T, A>{v: v.head == 0 && v.tail == 0 && capaci...
    |                                                                        ^^^^^^^^^^^^

error[E0999]: refinement type error
   --> src/vec_deque.rs:283:9
    |
283 | /         VecDeque {
284 | |             tail: 0,
285 | |             head: 0,
286 | |             buf: RawVec::with_capacity_in(cap, alloc),
287 | |         }
    | |_________^ a precondition cannot be proved
    |
note: this is the condition that cannot be proved
   --> src/vec_deque.rs:74:35
    |
74  |     #[flux::field({ usize[tail] | tail < cap })]
    |                                   ^^^^^^^^^^

error[E0999]: refinement type error
   --> src/vec_deque.rs:283:9
    |
283 | /         VecDeque {
284 | |             tail: 0,
285 | |             head: 0,
286 | |             buf: RawVec::with_capacity_in(cap, alloc),
287 | |         }
    | |_________^ a precondition cannot be proved
    |
note: this is the condition that cannot be proved
   --> src/vec_deque.rs:76:35
    |
76  |     #[flux::field({ usize[head] | head < cap })]
    |                                   ^^^^^^^^^^

error[E0999]: refinement type error
   --> src/vec_deque.rs:283:9
    |
283 | /         VecDeque {
284 | |             tail: 0,
285 | |             head: 0,
286 | |             buf: RawVec::with_capacity_in(cap, alloc),
287 | |         }
    | |_________^ a precondition cannot be proved
    |
note: this is the condition that cannot be proved
   --> src/vec_deque.rs:78:41
    |
78  |     #[flux::field({ RawVec<T, A>[cap] | pow2(cap) && 1 <= cap } )]
    |                                         ^^^^^^^^^

error[E0999]: refinement type error
   --> src/vec_deque.rs:283:9
    |
283 | /         VecDeque {
284 | |             tail: 0,
285 | |             head: 0,
286 | |             buf: RawVec::with_capacity_in(cap, alloc),
287 | |         }
    | |_________^ a precondition cannot be proved
    |
note: this is the condition that cannot be proved
   --> src/vec_deque.rs:78:54
    |
78  |     #[flux::field({ RawVec<T, A>[cap] | pow2(cap) && 1 <= cap } )]
    |                                                      ^^^^^^^^

error[E0999]: refinement type error
   --> src/vec_deque.rs:288:5
    |
288 |     }
    |     ^ a postcondition cannot be proved
    |
note: this is the condition that cannot be proved
   --> src/vec_deque.rs:275:148
    |
275 | ... 1}, alloc: A) -> VecDeque<T, A>{v: v.head == 0 && v.tail == 0 && capacity <= v.cap})]
    |                                                                      ^^^^^^^^^^^^^^^^^
```
