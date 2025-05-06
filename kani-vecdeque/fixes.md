# Fixes made

## `7509d20`

Note: line numbers may be changed slightly

### To `cap`

```
error[E0999]: refinement type error
   --> src/vec_deque.rs:126:9
    |
126 |         wrap_index(idx.wrapping_add(addend), self.cap())
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ a precondition cannot be proved
    |
note: this is the condition that cannot be proved
   --> src/vec_deque.rs:30:36
    |
30  | #[flux::alias(type Size = usize{v: pow2(v) && 1<=v })]
    |                                    ^^^^^^^
    = note: constraint that could not be proven: `pow2(self.cap())`
note: try adding a refinement to the function `vec_deque::VecDeque::<T, A>::cap`
   --> src/vec_deque.rs:99:8
    |
99  |     fn cap(&self) -> usize {
    |        ^^^
note: `self.cap()` defined here
   --> src/vec_deque.rs:126:46
    |
126 |         wrap_index(idx.wrapping_add(addend), self.cap())
    |                                              ^^^^^^^^^^
```

* Add `pow2(v)` to `cap`'s output
* (analagous error for `1<=v`) Add `1<=v` to `cap`'s output

```
error[E0999]: type invariant may not hold (when place is folded)
   --> src/vec_deque.rs:566:23
    |
566 |         let new_cap = self.cap();
    |                       ^^^^
    |
    = note: constraint that could not be proven: `s.tail < old_cap + old_cap`
note: try adding a refinement to the function `vec_deque::VecDeque::<T, A>::cap`
   --> src/vec_deque.rs:101:8
    |
101 |     fn cap(&self) -> usize {
    |        ^^^
note: `old_cap` defined here
   --> src/vec_deque.rs:563:23
    |
563 |         let old_cap = self.cap();
    |                       ^^^^^^^^^^

error[E0999]: type invariant may not hold (when place is folded)
   --> src/vec_deque.rs:566:23
    |
566 |         let new_cap = self.cap();
    |                       ^^^^
    |
    = note: constraint that could not be proven: `s.head < old_cap + old_cap`
note: try adding a refinement to the function `vec_deque::VecDeque::<T, A>::cap`
   --> src/vec_deque.rs:101:8
    |
101 |     fn cap(&self) -> usize {
    |        ^^^
note: `old_cap` defined here
   --> src/vec_deque.rs:563:23
    |
563 |         let old_cap = self.cap();
    |                       ^^^^^^^^^^
```
* output > `1/2 * self.tail`
* output > `1/2 * self.head`

Not implemented because I'm not sure how to make it work.

### To `handle_capacity_increase`

```
error[E0999]: arithmetic operation may overflow
   --> src/vec_deque.rs:187:31
    |
187 |         } else if self.head < old_capacity - self.tail {
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: constraint that could not be proven: `old_capacity - a3.tail ≥ 0`
note: try adding a refinement to `old_capacity`, defined here
   --> src/vec_deque.rs:167:51
    |
167 |     unsafe fn handle_capacity_increase(&mut self, old_capacity: usize) {
    |                                                   ^^^^^^^^^^^^
```

* Make `old_capacity >= self.tail`

```
error[E0999]: arithmetic operation may overflow
   --> src/vec_deque.rs:197:28
    |
197 |             let new_tail = new_capacity - (old_capacity - self.tail);
    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: constraint that could not be proven: `new_capacity - old_capacity - a3.tail ≥ 0`
note: try adding a refinement to the function `vec_deque::VecDeque::<T, A>::cap`
   --> src/vec_deque.rs:99:8
    |
99  |     fn cap(&self) -> usize {
    |        ^^^
note: `new_capacity` defined here
   --> src/vec_deque.rs:168:28
    |
168 |         let new_capacity = self.cap();
    |                            ^^^^^^^^^^
note: try adding a refinement to `old_capacity`, defined here
   --> src/vec_deque.rs:167:51
    |
167 |     unsafe fn handle_capacity_increase(&mut self, old_capacity: usize) {
    |                                                   ^^^^^^^^^^^^
```

* If we suppose that `old_capacity > self.tail` is given, then we get the constraint:

```
$k_cap+(self.head, self.tail, self.cap, new_capacity) /\ (old_capacity >= self.tail) => new_capacity - (old_capacity - self.tail) >= 0
```

which admits the trivial solution

```
new_capacity - (old_capacity - self.tail) >= 0 \/ !(old_capacity >= self.tail)
```

by DeMorgans

```
!(new_capacity - (old_capacity - self.tail) >= 0) /\ old_capacity >= self.tail)
```

Introduce an exists

```
exists old_capacity. !(new_capacity - (old_capacity - self.tail) >= 0) /\ old_capacity >= self.tail)
exists old_capacity. !(new_capacity >= old_capacity - self.tail) /\ old_capacity >= self.tail
exists old_capacity. new_capacity < old_capacity - self.tail /\ old_capacity >= self.tail
```

Z3 can't seem to handle this, unfortuantely, because `old_capacity` has no upper bound. Well it isn't a Z3 issue, it's a logical formula issue. Without an upper bound for `old_capacity`, we can't eliminate the variable.

One possible solution here is to deduce the weaker (stronger?) clause `new_capacity >= old_capacity - self.tail`, which is valid to put as a refinement on `old_capacity`. Note that the actual refinement is `new_capacity >= 2 * old_capacity`.

### To `with_capacity_in`

```
error[E0999]: refinement type error
   --> src/vec_deque.rs:285:9
    |
285 |         assert(capacity < MAXIMUM_ZST_CAPACITY);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ a precondition cannot be proved
    |
note: this is the condition that cannot be proved
   --> src/vec_deque.rs:609:21
    |
609 | #[flux::sig(fn(bool[true]))]
    |                     ^^^^
    = note: constraint that could not be proven: `(capacity < 9223372036854775808) = true`
note: try adding a refinement to `capacity`, defined here
   --> src/vec_deque.rs:282:25
    |
282 |     fn with_capacity_in(capacity: usize, alloc: A) -> VecDeque<T, A> {
    |                         ^^^^^^^^
```

* Add `capacity < MAXIMUM_ZST_CAPACITY` to sig

### To `real_capacity`

```
error[E0999]: refinement type error
   --> src/vec_deque.rs:289:9
    |
289 | /         VecDeque {
290 | |             tail: 0,
291 | |             head: 0,
292 | |             buf: RawVec::with_capacity_in(cap, alloc),
293 | |         }
    | |_________^ a precondition cannot be proved
    |
note: this is the condition that cannot be proved
   --> src/vec_deque.rs:77:41
    |
77  |     #[flux::field({ RawVec<T, A>[cap] | pow2(cap) && 1 <= cap } )]
    |                                         ^^^^^^^^^
    = note: constraint that could not be proven: `pow2(cap)`
note: try adding a refinement to the function `vec_deque::real_capacity`
   --> src/vec_deque.rs:614:4
    |
614 | fn real_capacity(capacity: usize) -> usize {
    |    ^^^^^^^^^^^^^
note: `cap` defined here
   --> src/vec_deque.rs:287:19
    |
287 |         let cap = real_capacity(capacity);
    |                   ^^^^^^^^^^^^^^^^^^^^^^^

error[E0999]: refinement type error
   --> src/vec_deque.rs:289:9
    |
289 | /         VecDeque {
290 | |             tail: 0,
291 | |             head: 0,
292 | |             buf: RawVec::with_capacity_in(cap, alloc),
293 | |         }
    | |_________^ a precondition cannot be proved
    |
note: this is the condition that cannot be proved
   --> src/vec_deque.rs:77:54
    |
77  |     #[flux::field({ RawVec<T, A>[cap] | pow2(cap) && 1 <= cap } )]
    |                                                      ^^^^^^^^
    = note: constraint that could not be proven: `1 ≤ cap`
note: try adding a refinement to the function `vec_deque::real_capacity`
   --> src/vec_deque.rs:614:4
    |
614 | fn real_capacity(capacity: usize) -> usize {
    |    ^^^^^^^^^^^^^
note: `cap` defined here
   --> src/vec_deque.rs:287:19
    |
287 |         let cap = real_capacity(capacity);
    |                   ^^^^^^^^^^^^^^^^^^^^^^^
```

* Add `pow2(v)` and `1 <= v` to the output of `real_capacity` (i.e. `size(v)`)

### `wrap_sub` and `wrap_add`

```
error[E0999]: type invariant may not hold (when place is folded)
   --> src/vec_deque.rs:526:13
    |
526 |             self.buffer_write(tail, value);
    |             ^^^^
    |
    = note: constraint that could not be proven: `self.wrap_sub(tail, 1) < a2.cap`
note: try adding a refinement to the function `vec_deque::VecDeque::<T, A>::wrap_sub`
   --> src/vec_deque.rs:136:8
    |
136 |     fn wrap_sub(&self, idx: usize, subtrahend: usize) -> usize {
    |        ^^^^^^^^
note: `self.wrap_sub(tail, 1)` defined here
   --> src/vec_deque.rs:523:21
    |
523 |         self.tail = self.wrap_sub(tail, 1);
    |                     ^^^^^^^^^^^^^^^^^^^^^^
```
* Output should be `<= self.cap`
