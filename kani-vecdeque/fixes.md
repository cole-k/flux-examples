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

## `bec485f`

### `wrap_index`

```
error[E0999]: refinement type error
   --> src/vec_deque.rs:130:9
    |
130 |         wrap_index(idx.wrapping_add(addend), self.cap())
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ a postcondition cannot be proved
    |
note: this is the condition that cannot be proved
   --> src/vec_deque.rs:128:86
    |
128 |     #[flux::sig(fn (self: &VecDeque<T,A>[@s], idx: usize, addend: usize) -> usize{v: v < s.cap})]
    |                                                                                      ^^^^^^^^^
    = note: constraint that could not be proven: `wrap_index(idx.wrapping_add(addend), self.cap()) < s.cap`
note: try adding a refinement to the function `vec_deque::wrap_index`
   --> src/vec_deque.rs:582:4
    |
582 | fn wrap_index(index: usize, size: Size) -> usize {
    |    ^^^^^^^^^^
note: `wrap_index(idx.wrapping_add(addend), self.cap())` defined here
   --> src/vec_deque.rs:130:9
    |
130 |         wrap_index(idx.wrapping_add(addend), self.cap())
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

* Needs to have an output < `s.cap`, `wrap_index` only sees `self.cap()`,
  so we will revisit this when we set `self.cap()`'s output to be `self.cap`.

### `with_capacity`

```
error[E0999]: refinement type error
   --> src/vec_deque.rs:252:9
    |
252 |         Self::with_capacity_in(capacity, Global)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ a precondition cannot be proved
    |
note: this is the condition that cannot be proved
   --> src/vec_deque.rs:283:46
    |
283 |     #[flux::sig(fn (capacity: {usize[@cap] | cap < MAXIMUM_ZST_CAPACITY}, alloc: A) -> VecDeque<T, A>)]
    |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: constraint that could not be proven: `capacity < vec_deque::MAXIMUM_ZST_CAPACITY`
note: try adding a refinement to `capacity`, defined here
   --> src/vec_deque.rs:251:26
    |
251 |     pub fn with_capacity(capacity: usize) -> VecDeque<T> {
    |                          ^^^^^^^^
```

* `capacity < MAXIMUM_ZST_CAPACITY`

### `cap`

```
error[E0999]: refinement type error
   --> src/vec_deque.rs:401:17
    |
401 |                 self.handle_capacity_increase(old_cap);
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ a precondition cannot be proved
    |
note: this is the condition that cannot be proved
   --> src/vec_deque.rs:174:75
    |
174 |     #[flux::sig(fn (self: &strg VecDeque<T,A>[@s], old_capacity: usize{v: s.tail <= v}) ensures self: VecDeque<T, A>)]
    |                                                                           ^^^^^^^^^^^
    = note: constraint that could not be proven: `s.tail ≤ old_cap`
note: try adding a refinement to the function `vec_deque::VecDeque::<T, A>::cap`
   --> src/vec_deque.rs:102:8
    |
102 |     fn cap(&self) -> usize {
    |        ^^^
note: `old_cap` defined here
   --> src/vec_deque.rs:390:23
    |
390 |         let old_cap = self.cap();
    |                       ^^^^^^^^^^
```

* `cap`'s output needs to be `> self.tail`.

## `4a65ff9`

### `len`

```
error[E0999]: arithmetic operation may overflow
   --> src/vec_deque.rs:121:9
    |
121 |         self.cap() - self.len() == 1
    |         ^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: constraint that could not be proven: `self.cap() - self.len() ≥ 0`
note: try adding a refinement to the function `vec_deque::VecDeque::<T, A>::len`
   --> src/vec_deque.rs:420:12
    |
420 |     pub fn len(&self) -> usize {
    |            ^^^
note: `self.len()` defined here
   --> src/vec_deque.rs:121:22
    |
121 |         self.cap() - self.len() == 1
    |                      ^^^^^^^^^^
note: try adding a refinement to the function `vec_deque::VecDeque::<T, A>::cap`
   --> src/vec_deque.rs:101:8
    |
101 |     fn cap(&self) -> usize {
    |        ^^^
note: `self.cap()` defined here
   --> src/vec_deque.rs:121:9
    |
121 |         self.cap() - self.len() == 1
    |         ^^^^^^^^^^
```

* `self.len() <= self.cap()` (actual refinement should be `<`)

### `wrap_index`

```
error[E0999]: refinement type error
   --> src/vec_deque.rs:129:9
    |
129 |         wrap_index(idx.wrapping_add(addend), self.cap())
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ a postcondition cannot be proved
    |
note: this is the condition that cannot be proved
   --> src/vec_deque.rs:127:86
    |
127 |     #[flux::sig(fn (self: &VecDeque<T,A>[@s], idx: usize, addend: usize) -> usize{v: v < s.cap})]
    |                                                                                      ^^^^^^^^^
    = note: constraint that could not be proven: `wrap_index(idx.wrapping_add(addend), self.cap()) < s.cap`
note: try adding a refinement to the function `vec_deque::wrap_index`
   --> src/vec_deque.rs:582:4
    |
582 | fn wrap_index(index: usize, size: Size) -> usize {
    |    ^^^^^^^^^^
note: `wrap_index(idx.wrapping_add(addend), self.cap())` defined here
   --> src/vec_deque.rs:129:9
    |
129 |         wrap_index(idx.wrapping_add(addend), self.cap())
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```
* wrap_index output needs to be less than its input `size`

### `copy_nonoverlapping`

```
error[E0999]: refinement type error
   --> src/vec_deque.rs:145:9
    |
145 |         assert(dst + len <= self.cap());
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ a precondition cannot be proved
    |
note: this is the condition that cannot be proved
   --> src/vec_deque.rs:611:21
    |
611 | #[flux::sig(fn(bool[true]))]
    |                     ^^^^
    = note: constraint that could not be proven: `(dst + len ≤ self.cap()) = true`
note: try adding a refinement to the function `vec_deque::VecDeque::<T, A>::cap`
   --> src/vec_deque.rs:101:8
    |
101 |     fn cap(&self) -> usize {
    |        ^^^
note: `self.cap()` defined here
   --> src/vec_deque.rs:145:29
    |
145 |         assert(dst + len <= self.cap());
    |                             ^^^^^^^^^^
note: try adding a refinement to `len`, defined here
   --> src/vec_deque.rs:143:66
    |
143 |     unsafe fn copy_nonoverlapping(&self, dst: usize, src: usize, len: usize) {
    |                                                                  ^^^
note: try adding a refinement to `dst`, defined here
   --> src/vec_deque.rs:143:42
    |
143 |     unsafe fn copy_nonoverlapping(&self, dst: usize, src: usize, len: usize) {
    |                                          ^^^
```

* `dst + len <= self.cap`
* (analagous fix for `src`)

## `8ed06de`

### `wrap_index`

```
error[E0999]: refinement type error
   --> src/vec_deque.rs:587:5
    |
587 |     assert(is_power_of_two(size));
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ a precondition cannot be proved
    |
note: this is the condition that cannot be proved
   --> src/vec_deque.rs:613:21
    |
613 | #[flux::sig(fn(bool[true]))]
    |                     ^^^^
    = note: constraint that could not be proven: `pow2(size) = true`

error[E0999]: arithmetic operation may overflow
   --> src/vec_deque.rs:588:13
    |
588 |     index & (size - 1)
    |             ^^^^^^^^^^
    |
    = note: constraint that could not be proven: `size - 1 ≥ 0`

```
* Both errors combine to be `size: Size`
* I'm also adding the `#[flux::trusted]` to the function because we can't prove the postcondition without it.

### `new_capacity`

```
error[E0999]: type invariant may not hold (when place is folded)
   --> src/vec_deque.rs:401:17
    |
401 |                 self.handle_capacity_increase(old_cap);
    |                 ^^^^
    |
    = note: constraint that could not be proven: `pow2(self.len() + 1 + new_cap - self.len() + 1)`
note: try adding a refinement to the function `vec_deque::new_capacity`
   --> src/vec_deque.rs:625:4
    |
625 | fn new_capacity(_old_cap: usize, used_cap: usize, additional: usize) -> usize {
    |    ^^^^^^^^^^^^
note: `new_cap` defined here
   --> src/vec_deque.rs:392:23
    |
392 |         let new_cap = new_capacity(old_cap, used_cap, additional);
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: try adding a refinement to the function `vec_deque::VecDeque::<T, A>::len`
   --> src/vec_deque.rs:421:12
    |
421 |     pub fn len(&self) -> usize {
    |            ^^^
note: `self.len()` defined here
   --> src/vec_deque.rs:391:24
    |
391 |         let used_cap = self.len() + 1;
    |                        ^^^^^^^^^^
```

* output must be a power of 2

### `count`

```
error[E0999]: refinement type error
   --> src/vec_deque.rs:422:9
    |
422 |         count(self.tail, self.head, self.cap())
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ a postcondition cannot be proved
    |
note: this is the condition that cannot be proved
   --> src/vec_deque.rs:420:56
    |
420 |     #[flux::sig(fn (&VecDeque<T,A>[@self]) -> usize{v: v <= self.cap})]
    |                                                        ^^^^^^^^^^^^^
    = note: constraint that could not be proven: `count(self.tail, self.head, self.cap()) ≤ self.cap`
note: try adding a refinement to the function `vec_deque::count`
   --> src/vec_deque.rs:594:4
    |
594 | fn count(tail: usize, head: usize, size: Size) -> usize {
    |    ^^^^^
note: `count(self.tail, self.head, self.cap())` defined here
   --> src/vec_deque.rs:422:9
    |
422 |         count(self.tail, self.head, self.cap())
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```
* Output must be <= the third argument (`size`)

### `handle_capacity_increase`

```
error[E0999]: refinement type error
   --> src/vec_deque.rs:198:17
    |
198 |                 self.copy_nonoverlapping(old_capacity, 0, head); // FLUX-PANIC: self.head -> head
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ a precondition cannot be proved
    |
note: this is the condition that cannot be proved
   --> src/vec_deque.rs:142:61
    |
142 |     #[flux::sig(fn (self: &VecDeque<T,A>[@s], dst: usize{v: v + len <= s.cap}, src: usize{v: v + len <= s.cap}, len: usize))]
    |                                                             ^^^^^^^^^^^^^^^^
    = note: constraint that could not be proven: `old_capacity + s.head ≤ s.cap`
note: try adding a refinement to `old_capacity`, defined here
   --> src/vec_deque.rs:174:51
    |
174 |     unsafe fn handle_capacity_increase(&mut self, old_capacity: usize) {
    |                                                   ^^^^^^^^^^^^

error[E0999]: refinement type error
   --> src/vec_deque.rs:207:17
    |
207 |                 self.copy_nonoverlapping(new_tail, tail, old_capacity - tail); // FLUX-PANIC: self.tail -> tail
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ a precondition cannot be proved
    |
note: this is the condition that cannot be proved
   --> src/vec_deque.rs:142:94
    |
142 |     #[flux::sig(fn (self: &VecDeque<T,A>[@s], dst: usize{v: v + len <= s.cap}, src: usize{v: v + len <= s.cap}, len: usize))]
    |                                                                                              ^^^^^^^^^^^^^^^^
    = note: constraint that could not be proven: `s.tail + old_capacity - s.tail ≤ s.cap`
note: try adding a refinement to `old_capacity`, defined here
   --> src/vec_deque.rs:174:51
    |
174 |     unsafe fn handle_capacity_increase(&mut self, old_capacity: usize) {
    |                                                   ^^^^^^^^^^^^
```
* `old_capacity` must be `<= s.cap - s.head` (the second constraint is striclty weaker)

## `7d66cee`

### `count`

```
error[E0999]: refinement type error
   --> src/vec_deque.rs:597:5
    |
597 |     wrap_index(head.wrapping_sub(tail), size)
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ a precondition cannot be proved
    |
note: this is the condition that cannot be proved
   --> src/vec_deque.rs:30:36
    |
30  | #[flux::alias(type Size = usize{v: pow2(v) && 1<=v })]
    |                                    ^^^^^^^
    = note: constraint that could not be proven: `pow2(size)`

error[E0999]: refinement type error
   --> src/vec_deque.rs:597:5
    |
597 |     wrap_index(head.wrapping_sub(tail), size)
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ a precondition cannot be proved
    |
note: this is the condition that cannot be proved
   --> src/vec_deque.rs:30:47
    |
30  | #[flux::alias(type Size = usize{v: pow2(v) && 1<=v })]
    |                                               ^^^^
    = note: constraint that could not be proven: `1 ≤ size`
```
* `size` argument must be a `Size`.

### `new_capacity`

```
error[E0999]: refinement type error
   --> src/vec_deque.rs:401:17
    |
401 |                 self.handle_capacity_increase(old_cap);
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ a precondition cannot be proved
    |
note: this is the condition that cannot be proved
   --> src/vec_deque.rs:173:90
    |
173 |     #[flux::sig(fn (self: &strg VecDeque<T,A>[@s], old_capacity: usize{v: s.tail <= v && v <= s.cap - s.head}) ensures self: VecDeque<T, A>)]
    |                                                                                          ^^^^^^^^^^^^^^^^^^^
    = note: constraint that could not be proven: `old_cap ≤ self.len() + 1 + new_cap - self.len() + 1 - s.head`
note: try adding a refinement to the function `vec_deque::new_capacity`
   --> src/vec_deque.rs:626:4
    |
626 | fn new_capacity(_old_cap: usize, used_cap: usize, additional: usize) -> usize {
    |    ^^^^^^^^^^^^
note: `new_cap` defined here
   --> src/vec_deque.rs:392:23
    |
392 |         let new_cap = new_capacity(old_cap, used_cap, additional);
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: try adding a refinement to the function `vec_deque::VecDeque::<T, A>::len`
   --> src/vec_deque.rs:421:12
    |
421 |     pub fn len(&self) -> usize {
    |            ^^^
note: `self.len()` defined here
   --> src/vec_deque.rs:391:24
    |
391 |         let used_cap = self.len() + 1;
    |                        ^^^^^^^^^^
note: try adding a refinement to the function `vec_deque::VecDeque::<T, A>::cap`
   --> src/vec_deque.rs:101:8
    |
101 |     fn cap(&self) -> usize {
    |        ^^^
note: `old_cap` defined here
   --> src/vec_deque.rs:390:23
    |
390 |         let old_cap = self.cap();
    |                       ^^^^^^^^^^
```
* In essence this needs the output of `new_capacity` to be `>= old_cap + s.head`,
  but we don't have access to `s.head`. We know that `old_cap >= s.head`, so 
  a reasonable thing to od would be to use it instead.
  
  This gives us the human annotation `new_capacity >= 2 * old_cap`.
  
## `2b8d86b`

### `handle_capacity_increase`

```
error[E0999]: type invariant may not hold (when place is folded)
   --> src/vec_deque.rs:214:28
    |
214 |         assert(self.head < self.cap());
    |                            ^^^^
    |
    = note: constraint that could not be proven: `a1 < s.cap`

error[E0999]: type invariant may not hold (when place is folded)
   --> src/vec_deque.rs:214:28
    |
214 |         assert(self.head < self.cap());
    |                            ^^^^
    |
    = note: constraint that could not be proven: `a2 < s.cap`
```
* I think what is happening here is that in the two branches where there isn't
  a noop, we can't prove that `self.head < self.cap` (although it should be
  `self.tail < self.cap` for one of the cases). It seems like we don't
  automatically find the definition of `a1` or `a2`. I suspect that what we
  need to do is figure out if this is a mechanization issue or an actually
  hard problem.
  
## `0f54d57`

### `handle_capacity_increase`

```
error[E0999]: type invariant may not hold (when place is folded)
   --> src/vec_deque.rs:201:32
    |
201 |             assert(self.head < self.cap());
    |                                ^^^^
    |
    = note: constraint that could not be proven: `s.head + old_capacity < s.cap`
note: try adding a refinement to `old_capacity`, defined here
   --> src/vec_deque.rs:174:51
    |
174 |     unsafe fn handle_capacity_increase(&mut self, old_capacity: usize) {
    |                                                   ^^^^^^^^^^^^

```

* I moved the assertion so that the error message from above is clearer.
  Right now `old_capacity` is annotated such that `old_capacity <= s.cap - s.head`
  (remember: the naive fix we got from before was to prevent an overflow; but we
  require a stricter inequality for the general safety property to hold).
  Fix this naively by replacing `v <= s.cap - s.head` with `v < s.cap - s.head`.
  
