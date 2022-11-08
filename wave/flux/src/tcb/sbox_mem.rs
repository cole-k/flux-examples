use crate::rvec::RVec;
use crate::types::{NativeIoVec, SboxPtr, VmCtx, WasmIoVec};

impl VmCtx {
    #[flux::sig(fn(&VmCtx[@cx], WasmIoVec) -> NativeIoVecOk[cx.base])]
    pub fn translate_iov(&self, iov: WasmIoVec) -> NativeIoVec {
        let swizzled_base = self.raw + iov.iov_base as usize;
        NativeIoVec {
            iov_base: swizzled_base,
            iov_len: iov.iov_len as usize,
        }
    }

    // FLUX-TODO: capacity
    #[flux::assume]
    #[flux::sig(fn(&VmCtx, &mut RVec<u8>[n], src: SboxPtr{src + n < LINEAR_MEM_SIZE}, n:u32{0 <= n}) -> ())]
    #[allow(unused_variables)]
    pub fn memcpy_from_sandbox(&self, dst: &mut RVec<u8>, src: SboxPtr, n: u32) {
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

    /// Function for memcpy from sandbox to host
    // #[with_ghost_var(trace: &mut Trace)]
    // #[external_calls(copy_nonoverlapping)]
    // // #[requires(src.len() >= (n as usize) )]
    // #[requires(self.fits_in_lin_mem(dst, n, trace))]
    // #[requires(ctx_safe(self))]
    // #[requires(trace_safe(trace, self))]
    // #[ensures(ctx_safe(self))]
    // #[ensures(trace_safe(trace, self))]
    // // #[ensures(old(raw_ptr(self.mem.as_slice())) == raw_ptr(self.mem.as_slice()))]
    // #[ensures(effects!(old(trace), trace, effect!(WriteMem, addr, count) if
    //     addr == raw_ptr(self.mem.as_slice()) + dst as usize &&
    //     count == n as usize
    // ))]
    // #[trusted]

    // FLUX-TODO: wierd shenanigans with @silly -- DELETING the spec triggers a precondition ERROR!
    //            which then goes away if you JUST add the @silly ...
    #[flux::assume]
    #[flux::sig(fn(&mut VmCtx[@silly], dst: SboxPtr{dst + n < LINEAR_MEM_SIZE}, &RVec<u8>{sz:n <= sz}, n:u32) -> ())]
    // NO bizarre error!
    //#[flux::sig(fn(&mut VmCtx[@silly], dst: SboxPtr, &RVec<u8>, n:u32) -> ())]
    #[allow(unused_variables)]
    pub fn memcpy_to_sandbox(&mut self, dst: SboxPtr, src: &RVec<u8>, n: u32) {
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
