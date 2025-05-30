# Fixes made mostly using naive weak kvar instantiations

Flux commit `40c7e40e4d8a57dd391d2a6a1403a0332f629b9b`

I'll note each suggestion and whether I accepted it.
I'll also note the manual changes I made.
I'll deduplicate repeated suggestions.

I'll pass over errors in the order they appear.

# `7509d20 `

## ⨯ `is_power_of_two(x) = true` always

**Human annotation**

I didn't strictly accept this but I did uncomment it because it's not clear
that we have a good way of discovering the signature of this function.

## ✓ `pow2(self.cap())`

Accepted.

## ✓ `self.cap() >= 1`

Accepted with the above as `size(self.cap())`.

## ✓ `handle_capacity_increase(&mut self, old_capacity: usize)` require `old_capacity >= self.tail`

Accepted. Note that the required refinement is `>` not `>=`.

## ✓ `with_capacity_in(capacity: usize)` require `capacity > MAX_ZST_CAPACITY`

Accepted.

## Various additions for `with_capacity_in` 

- ✓ `real_capacity` output `> 0`
- ⨯ `RawVec::with_capacity_in` output `> 0`

Accepted. This is then duplicated for `tail`, which is also 0.

- ✓ `real_capacity` output is `pow2`
- ⨯ `RawVec::with_capacity_in` output is `pow2`

Accepted.

- ✓ `real_capacity` output is `>= 1`
- ⨯ `RawVec::with_capacity_in` output is `>= 1`

Accepted with the above as `size`.

## ✓ `new_capacity(old_cap: usize, used_cap: usize, additional: usize)` output `> used_cap`

Accepted.

## Changes to `new_capacity` for `reserve`

- ✓ `new_capacity(old_cap: usize, used_cap: usize, additional: usize)` output `pow2`
- ⨯ `RawVec::reserve_exact(self, len, additional)` output `pow2`

Accepted (this is actually manifest as `pow2(used_cap + output - used_cap)` but
I simplified it).

## ✓ `s.cap()` output s.t. `s.tail < s.cap() + s.cap()`

Kind of janky but it's correct.

## ✓ `s.cap()` output s.t. `s.head < s.cap() + s.cap()`

Kind of janky but it's correct.

## Changes because of `grow()`

- ⨯ `s.cap()` output s.t. `pow2(s.cap() + s.cap())`
- ⨯ `RawVec::reserve_exact(self, len, additional)` output s.t. `pow2(additional + additional)`
- ✓ `lem_power_two(v)` output s.t. `pow2(v + v)`

Kind of cool: all of these are technically correct, but the right place to put
it is in `lem_power_two()`.

- ✓ `s.cap()` output s.t. `s.cap() + s.cap() >= 1`
- ⨯ `RawVec::reserve_exact(self, len, additional)` output s.t. `additional + additional >= 1`
- ⨯ `lem_power_two(v)` output s.t. `v + v >= 1`

Technically true, but subsumed by previous refinements.

# `d3f08d7`

## ✓ `with_capacity(cap)` s.t. `cap < MAXIMUM_ZST_CAPACITY`

Accepted.

## ✓ `self.cap() >= self.tail`

Technically correct.

# `146294f `

## Manual fix: refine `cap` so that the output is the actual capacity

I don't think we can discover this easily, especially since it has to be trusted.

# `0f55b07`

## Fix the manual fix: `cap` returns the index

Oops.

## ✓ `self.len() <= self.cap()`

Accepted: the actual is technically `<` and not `<=`.

## ✓ `copy_nonoverlapping(&self, dst: usize, src: usize, len: usize)` s.t. `dst + len <= self.cap`

Accepted.

## ✓ `copy_nonoverlapping(&self, dst: usize, src: usize, len: usize)` s.t. `src + len <= self.cap`

Accepted.

# `c6279f2 `

## ✓ `handle_capacity_increase(&self, old_capacity: usize)` s.t. `old_capacity <= self.cap`

Accepted.

## ✓ `handle_capacity_increase(&self, old_capacity: usize)` s.t. `old_capacity + self.head <= self.cap`

Accepted; overrides the previous.

# NOTE: update to flux to fix expr hashing

Prev: exprs had metadata that caused hashes to be different.

# `bcb9847 `

## ✓ `count(tail, head, cap)` s.t. output `< cap`

Accepted.

# `1ed14db `

## ✓ `wrap_index(index, size)` s.t. output `< size`
