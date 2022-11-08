use crate::tcb::misc::*;
use prusti_contracts::*;

pub const LINEAR_MEM_SIZE: usize = 4294965096; //4GB

pub const TWO_POWER_20: usize = 1024 * 1024;

pub const PATH_MAX: usize = 4096;

pub type RuntimeResult<T> = Result<T, RuntimeError>;

pub type SboxPtr = u32;
pub type HostPtr = usize;

pub enum RuntimeError {
    Success = 0,
    Efault,
    Eoverflow,
    Eloop,
    Enotcapable,
    Enametoolong,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct WasmIoVec {
    pub iov_base: u32,
    pub iov_len: u32,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NativeIoVec {
    pub iov_base: usize,
    pub iov_len: usize,
}

// Wrapper around Vec<NativeIoVecs> used to make proof cleaner
pub struct WasmIoVecs {
    pub iovs: Vec<WasmIoVec>,
}

impl WasmIoVecs {
    #[pure]
    pub fn len(&self) -> usize {
        self.iovs.len()
    }

    #[ensures(result.len() == 0)]
    pub fn new() -> Self {
        Self { iovs: Vec::new() }
    }

    // Have to dispatch to trusted function because Prusti won't let me
    // inspect a vector inside a proof
    #[pure]
    #[requires(index < self.len())]
    pub fn lookup(&self, index: usize) -> WasmIoVec {
        wasm_iovs_checked_lookup(self, index)
    }
}

pub struct NativeIoVecs {
    pub iovs: Vec<NativeIoVec>,
}

impl NativeIoVecs {
    #[pure]
    pub fn len(&self) -> usize {
        self.iovs.len()
    }

    #[ensures(result.len() == 0)]
    pub fn new() -> Self {
        Self { iovs: Vec::new() }
    }

    // Have to dispatch to trusted function because Prusti won't let me
    // inspect a vector inside a proof
    #[pure]
    #[requires(index < self.len())]
    pub fn lookup(&self, index: usize) -> NativeIoVec {
        iovs_checked_lookup(self, index)
    }
}

#[macro_export]
macro_rules! unwrap_result {
    ($p:ident) => {
        let $p = match $p {
            Ok(oc) => oc,
            Err(e) => {
                return Err(e);
            }
        };
    };
}

pub struct VmCtx {
    pub raw: usize, // TODO: valid_linmem (UIF)
    pub mem: Vec<u8>,
    pub memlen: usize,
    // FLUX pub fdmap: FdMap,
    // FLUX pub homedir: String,
    // FLUX pub homedir_host_fd: HostFd,
    // #[flux::field(RVec<u8>{v: v < TWO_POWER_20 })] TODO: flux issue #156
    pub arg_buffer: Vec<u8>,
    // #[flux::field(RVec<u8>{v: v < TWO_POWER_20 })] TODO: flux issue #156
    pub env_buffer: Vec<u8>,
    pub envc: usize,
    pub argc: usize,
    // FLUX pub netlist: Netlist,           TODO: UIF
}
/* src/tcb/verifier/spec.rs
pub fn ctx_safe(ctx: &VmCtx) -> bool {
    ctx.memlen == LINEAR_MEM_SIZE &&
    ctx.argc < 1024 &&
    ctx.envc < 1024 &&
    ctx.arg_buffer.len() < 1024 * 1024 &&
    ctx.env_buffer.len() < 1024 * 1024 &&
    netlist_unmodified(&ctx.netlist) &&
    valid_linmem(raw_ptr(ctx.mem.as_slice()))
}
*/

// impl VmCtx {
//     #[flux::sig(fn(&VmCtx[@base], &WasmIoVec) -> NativeIoVecOk[base])]
//     pub fn translate_iov(&self, iov: &WasmIoVec) -> NativeIoVec {
//         let swizzled_base = self.raw + as_usize(iov.iov_base);
//         NativeIoVec {
//             iov_base: swizzled_base,
//             iov_len: as_usize(iov.iov_len),
//         }
//     }
// }

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct HostFd(usize);

impl HostFd {
    pub(crate) fn to_raw(&self) -> usize {
        self.0
    }

    #[allow(dead_code)]
    pub(crate) fn from_raw(w: usize) -> HostFd {
        HostFd(w)
    }
}
