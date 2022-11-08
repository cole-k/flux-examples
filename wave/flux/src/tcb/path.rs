// use crate::types::*;
// use owned_components::{readlinkat, OwnedComponent, OwnedComponents};
// use std::ffi::OsStr;
// use std::ffi::OsString;
// use std::os::unix::ffi::OsStrExt;
// use std::os::unix::ffi::OsStringExt;
use std::path::PathBuf;
// use std::str;

use owned_components::{readlinkat, OwnedComponent, OwnedComponents};

use crate::{rvec::RVec, types::HostFd};

#[flux::constant]
const DEPTH_ERR: i32 = i32::MIN; // as_isize(i32::MIN); // as isize;

// FLUX-TODO:SLICE
#[allow(dead_code)]
#[flux::opaque]
#[flux::refined_by(depth:int, is_relative:bool, non_symlink:bool, non_symlink_prefixes:bool)]
pub struct HostPath {
    inner: [u8; crate::types::PATH_MAX],
}

#[flux::opaque]
#[flux::refined_by(size:int, ns_prefix:int, depth:int, is_relative:bool)]
pub struct FOwnedComponents {
    inner: OwnedComponents,
}

#[flux::alias(type HostPathOc(oc) = HostPath{ v: v.depth == oc.depth && v.is_relative == oc.is_relative
                                              && v.non_symlink == (oc.size == oc.ns_prefix)
                                              && v.non_symlink_prefixes == (oc.size - 1 <= oc.ns_prefix) })]
pub type _HostPathOc = HostPath;

#[flux::alias(type NoSymLinks = FOwnedComponents{v: v.size == v.ns_prefix})]
pub type NoSymLinks_ = FOwnedComponents;

#[flux::alias(type LastSymLink(b) = FOwnedComponents{v: v.size - 1 <= v.ns_prefix && (b => v.size == v.ns_prefix)})]
pub type LastSymLink_ = FOwnedComponents;

impl FOwnedComponents {
    #[flux::assume]
    #[flux::sig(fn (&FOwnedComponents[@self]) -> usize[self.size])]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[flux::assume]
    #[flux::sig(fn (&FOwnedComponents[@self], idx:usize{0 <= idx && idx < self.size}) -> OwnedComponent)]
    pub fn lookup(&self, idx: usize) -> OwnedComponent {
        self.inner.lookup(idx)
    }

    #[flux::assume]
    #[flux::sig(fn () -> FOwnedComponents[0, 0, 0, false])]
    pub fn new() -> FOwnedComponents {
        FOwnedComponents {
            inner: OwnedComponents::new(),
        }
    }

    #[flux::assume]
    #[flux::sig(fn (self: &strg FOwnedComponents[@oc], OwnedComponent) -> ()
                ensures self: FOwnedComponents{v: v.size == oc.size + 1 && v.ns_prefix == oc.ns_prefix} )]
    pub fn push(&mut self, value: OwnedComponent) {
        self.inner.push(value);
    }

    #[flux::assume]
    #[flux::sig(fn (oc:FOwnedComponents) -> Option<HostPathOc[oc]>)]
    pub fn unparse(self) -> Option<HostPath> {
        let inner = self.inner.unparse()?;
        Some(HostPath { inner })
    }
}

// FLUX-TODO: unsupported generic in Component
#[flux::assume]
#[flux::sig(fn(&PathBuf) -> RVec<OwnedComponent>)]
pub fn get_components(path: &PathBuf) -> RVec<OwnedComponent> {
    let mut components = RVec::new();
    for c in path.components() {
        components.push(OwnedComponent::from_borrowed(&c));
    }
    components
}

// #[derive(Clone, Copy, PartialEq, Eq)]
pub struct NetEndpoint {
    pub protocol: WasiProto,
    pub addr: u32,
    pub port: u32,
}

// #[derive(Clone, Copy, PartialEq, Eq)]

pub enum WasiProto {
    Unknown,
    Tcp,
    Udp,
}

// If the first component is not the rootdir or a prefix (like Windows C://) its relative
#[flux::assume]
#[flux::sig(fn (&FOwnedComponents[@oc]) -> bool[oc.is_relative])]
pub fn is_relative(c: &FOwnedComponents) -> bool {
    let start = c.inner.lookup(0);
    !(matches!(start, OwnedComponent::RootDir))
}

// use really big negative number instead of option because the verifier does not like returning options from pure code
// apparently I can make it pure or I can make it untrusted but I cannot do both
// #[pure]
// #[trusted]

#[flux::assume]
#[flux::sig(fn (&FOwnedComponents[@oc]) -> isize[oc.depth])]
pub fn min_depth(components: &FOwnedComponents) -> isize {
    let mut curr_depth = 0;
    let mut idx = 0;
    while idx < components.len() {
        // body_invariant!(curr_depth >= 0);
        match components.inner.lookup(idx) {
            OwnedComponent::RootDir => {
                return DEPTH_ERR as isize;
            } // hacky, but fine for now
            OwnedComponent::CurDir => {}
            OwnedComponent::ParentDir => {
                curr_depth -= 1;
            }
            OwnedComponent::Normal(_) => {
                curr_depth += 1;
            }
        };
        // if curr_depth ever dips below 0, it is illegal
        // this prevents paths like ../other_sandbox_home
        if curr_depth < 0 {
            return curr_depth;
        }
        idx += 1;
    }
    curr_depth
}

// RJ: Strange spec -- shouldn't we talk about the OUTPUT OwnedComponents instead of the input?
// #[trusted]
// #[ensures(result.is_none() ==> old(!is_symlink(out_path)) )]
#[flux::assume]
#[flux::sig(fn (HostFd, &FOwnedComponents[@oc]) -> Option<{FOwnedComponents: oc.ns_prefix == oc.size}>)]
fn read_linkat_h(dirfd: HostFd, out_path: &FOwnedComponents) -> Option<FOwnedComponents> {
    let inner = readlinkat(dirfd.to_raw(), &out_path.inner.as_pathbuf())
        .ok()
        .map(|p| OwnedComponents::parse(p))?;
    Some(FOwnedComponents { inner })
}

// its an empty path, its not a symlink
// #[trusted]
// #[ensures(result.len() == 0)]
// #[ensures(!is_symlink(&result))]
// #[ensures(forall(|i: usize| (i < result.len()) ==> !is_symlink(result.prefix(i)) ))] // we should be able to solve this by knowing that length = 0

#[flux::assume]
#[flux::sig(fn () -> FOwnedComponents[0, 0, 0, false])]
pub fn fresh_components() -> FOwnedComponents {
    FOwnedComponents {
        inner: OwnedComponents::new(),
    }
}

// Looks at a single component of a path:
// if it is a symlink, return the linkpath.
// else, we just append the value to out_path
// #[trusted]

// #[requires(!is_symlink(out_path) )]
// #[requires(forall(|i: usize| (i < out_path.len()) ==> !is_symlink(out_path.prefix(i)) ))]
// #[ensures(!is_symlink(out_path))]
// #[ensures(forall(|i: usize| (i < out_path.len()) ==> !is_symlink(out_path.prefix(i)) ))]
#[flux::assume]
#[flux::sig(fn (HostFd, &mut NoSymLinks, OwnedComponent, &mut isize) -> Option<FOwnedComponents>)]
pub fn maybe_expand_component(
    dirfd: HostFd,
    out_path: &mut FOwnedComponents,
    comp: OwnedComponent,
    num_symlinks: &mut isize,
) -> Option<FOwnedComponents> {
    out_path.inner.push(comp);
    if let Some(linkpath) = read_linkat_h(dirfd, out_path) {
        out_path.inner.pop(); // pop the component we just added, since it is a symlink
        *num_symlinks += 1;
        return Some(linkpath);
    }
    return None;
}

/*
#[cfg(feature = "verify")]
predicate! {
    pub fn has_no_symlink_prefixes(v: &OwnedComponents) -> bool {
        forall(|i: usize| (i < v.len() - 1) ==> !is_symlink(v.prefix(i)))
    }
}

#[cfg(feature = "verify")]
predicate! {
    pub fn path_safe(v: &HostPath, should_follow: bool) -> bool {
        arr_is_relative(&v) &&
        (arr_depth(&v) >= 0) &&
        (should_follow ==> !arr_is_symlink(&v)) &&
        arr_has_no_symlink_prefixes(&v)
    }
}
*/
#[flux::alias(type HostPathSafe(b) = HostPath{v: v.depth >= 0 && v.is_relative && (b => v.non_symlink) && v.non_symlink_prefixes})]
pub type _HostPathSafe = HostPath;
