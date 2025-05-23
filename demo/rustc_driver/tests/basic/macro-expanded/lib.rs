#![feature(prelude_import)]
#![feature(register_tool)]
#![register_tool(Safety)]
#![allow(clippy::missing_safety_doc, clippy::mut_from_ref, internal_features)]
#![feature(core_intrinsics)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use safety_tool_lib::safety;
/// It's undefined behavior to reach code marked with this intrinsic function.
#[Safety::inner(
    kind = "precond",
    UnReachable,
    memo = "It's undefined behavior to reach code marked with this intrinsic function."
)]
pub unsafe fn test() -> ! {
    unsafe { std::intrinsics::unreachable() }
}
pub struct MyStruct {
    ptr: *mut u8,
    len: usize,
}
impl MyStruct {
    pub fn from(p: *mut u8, l: usize) -> MyStruct {
        MyStruct { ptr: p, len: l }
    }
    /// The ptr must be initialized first!
    #[Safety::inner(
        kind = "precond",
        Init(self.ptr, u8, self.len),
        memo = "The ptr must be initialized first!"
    )]
    /// The ptr must be within the length.
    #[Safety::inner(
        kind = "precond",
        InBound(self.ptr, u8, self.len),
        memo = "The ptr must be within the length."
    )]
    /// Slice length can't exceed isize::MAX due to allocation limit.
    #[Safety::inner(
        kind = "precond",
        ValidNum(self.len*sizeof(u8), [0, isize::MAX]),
        memo = "Slice length can't exceed isize::MAX due to allocation limit."
    )]
    /// Make sure don't alias the ptr.
    #[Safety::inner(
        kind = "hazard",
        Alias(self.ptr),
        memo = "Make sure don't alias the ptr."
    )]
    pub unsafe fn get(&self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}
