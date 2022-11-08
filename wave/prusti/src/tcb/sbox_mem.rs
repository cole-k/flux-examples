#[cfg(feature = "verify")]
use crate::tcb::verifier::*;
use crate::types::*;
use prusti_contracts::*;

impl VmCtx {
    #[requires(ctx_safe(self))]
    #[ensures(ctx_safe(self))]
    #[ensures(
        result.iov_base == raw_ptr(self.mem.as_slice()) + (iov.iov_base as usize) &&
        result.iov_len == (iov.iov_len as usize)
    )]
    #[trusted]
    pub fn translate_iov(&self, iov: WasmIoVec) -> NativeIoVec {
        let swizzled_base = self.raw + iov.iov_base as usize;
        NativeIoVec {
            iov_base: swizzled_base,
            iov_len: iov.iov_len as usize,
        }
    }

    // FLUX-TODO: capacity
    #[requires(dst.capacity() >= (n as usize) )]
    #[requires(self.fits_in_lin_mem(src, n))]
    #[requires(ctx_safe(self))]
    #[ensures(ctx_safe(self))]
    #[ensures(dst.len() == (n as usize) )]
    #[trusted]
    #[allow(unused_variables)]
    pub fn memcpy_from_sandbox(&self, dst: &mut Vec<u8>, src: SboxPtr, n: u32) {
        // unsafe {
        //     copy_nonoverlapping(
        //         // TODO-FLUX: convert RVec to raw poi
        //         self.mem.as_ptr().offset(src as isize),
        //         dst.as_mut_ptr(),
        //         n as usize,
        //     );
        //     dst.set_len(n as usize); // TODO: wrong, need to make sure copy_nonoverlapping actually copied it
        // };
        // do_effect!(effect!(ReadMem, src, n));
    }

    #[requires(self.fits_in_lin_mem(dst, n))]
    #[requires(ctx_safe(self))]
    #[ensures(ctx_safe(self))]
    #[trusted]
    #[allow(unused_variables)]
    pub fn memcpy_to_sandbox(&mut self, dst: SboxPtr, src: &Vec<u8>, n: u32) {
        // TODO
        // unsafe {
        //     copy_nonoverlapping(
        //         src.as_ptr(),
        //         self.mem.as_mut_ptr().offset(dst as isize),
        //         n as usize,
        //     )
        // };
    }
}

impl WasmIoVecs {
    #[trusted]
    #[ensures(self.len() == old(self.len()) + 1)]
    #[ensures(self.lookup(old(self.len())) == old(value))]
    #[ensures(forall(|i: usize| (i < old(self.len())) ==>
                    self.lookup(i) == old(self.lookup(i))))]
    pub fn push(&mut self, value: WasmIoVec) {
        self.iovs.push(value);
    }
}

impl NativeIoVecs {
    #[trusted]
    #[ensures(self.len() == old(self.len()) + 1)]
    #[ensures(self.lookup(old(self.len())) == old(value))]
    #[ensures(forall(|i: usize| (i < old(self.len())) ==>
                    self.lookup(i) == old(self.lookup(i))))]
    pub fn push(&mut self, value: NativeIoVec) {
        self.iovs.push(value);
    }
}

#[pure]
#[trusted]
#[ensures(result >= 0)]
pub fn raw_ptr(memptr: &[u8]) -> HostPtr {
    unimplemented!()
}

// bodyless viper function
#[pure]
#[trusted]
pub fn valid_linmem(memptr: usize) -> bool {
    unimplemented!()
}
