use super::external_specs::vec::*;
use crate::tcb::sbox_mem::{raw_ptr, valid_linmem};
use crate::types::{VmCtx, LINEAR_MEM_SIZE};
use prusti_contracts::*;

#[cfg(feature = "verify")]
predicate! {
    pub fn ctx_safe(ctx: &VmCtx) -> bool {
        //let mem_ptr = raw_ptr(ctx.mem.as_slice());
        ctx.memlen == LINEAR_MEM_SIZE &&
        ctx.argc < 1024 &&
        ctx.envc < 1024 &&
        ctx.arg_buffer.len() < 1024 * 1024 &&
        ctx.env_buffer.len() < 1024 * 1024 &&
        // netlist_unmodified(&ctx.netlist) &&
        valid_linmem(raw_ptr(ctx.mem.as_slice())) //&&
        //mem_ptr <= mem_ptr + count
    }
}

use std::vec::Vec;

#[extern_spec]
impl<T> Vec<T> {
    #[ensures(result.len() == 0)]
    #[ensures(result.capacity() == 0)]
    fn new() -> Vec<T>;

    #[pure]
    fn len<A>(&self) -> usize;

    #[ensures(self.len() == old(self.len()) + 1)]
    #[ensures(self.capacity() >= old(self.capacity()))]
    fn push<A>(&mut self, value: T);

    #[ensures(self.len() == 0)]
    fn clear<A>(&mut self);

    #[pure]
    fn capacity<A>(&self) -> usize;

    #[ensures(self.capacity() >= old(self.len() + additional))]
    #[ensures(self.len() == old(self.len()))]
    fn reserve_exact<A>(&mut self, additional: usize);

    #[pure]
    fn as_slice<A>(&self) -> &[T];

    // #[pure]
    // fn as_slice(&self) -> &[T];

    // #[pure]
    // fn as_mut_slice(&mut self) -> &mut [T];

    // #[pure]
    // #[requires (index < MAX_SBOX_FDS )]
    // // #[requires(0 <= index && index < self.len())]
    // // #[ensures(*result == old(self.lookup(index)))]
    // pub fn get(&self, index: usize) -> &T {
    //     self.get(index).unwrap()
    // }
}
