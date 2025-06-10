#![feature(prelude_import)]
#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(clippy::missing_safety_doc, clippy::mut_from_ref, internal_features)]
#![feature(core_intrinsics)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use safety_lib::safety;
/// Unreachable: Make sure the current program point should not be reachable during execution before calling this function.
#[rapx::inner(property = Unreachable(), kind = "precond")]
pub unsafe fn test() -> ! {
    unsafe { std::intrinsics::unreachable() }
}
pub struct MyStruct {
    ptr: *mut u8,
    len: usize,
}
impl MyStruct {
    /// UserProperty: auto doc placeholder.
    /// Customed user property.
    #[rapx::inner(
        property = Unknown(UserProperty),
        kind = "memo",
        memo = "Customed user property."
    )]
    pub fn from(p: *mut u8, l: usize) -> MyStruct {
        MyStruct { ptr: p, len: l }
    }
    /// Init: Make sure the memory range [self.ptr, self.ptr + sizeof(u8)*self.len] must be fully initialized for type T before calling this function.
    #[rapx::inner(property = Init(self.ptr, u8, self.len), kind = "precond")]
    /// InBound: Make sure the pointer self.ptr and its offset up to sizeof(u8)*self.len must point to a single allocated object before calling this function.
    #[rapx::inner(property = InBound(self.ptr, u8, self.len), kind = "precond")]
    /// ValidNum: Make sure the value of self.len * sizeof(u8) must lie within the valid [0, isize :: MAX] before calling this function.
    #[rapx::inner(
        property = ValidNum(self.len*sizeof(u8), [0, isize::MAX]),
        kind = "precond"
    )]
    /// Alias: Make sure self.ptr must not have other alias after calling this function.
    #[rapx::inner(property = Alias(self.ptr), kind = "hazard")]
    /// UserPropertyGet: auto doc placeholder.
    /// Customed user property.
    #[rapx::inner(
        property = Unknown(UserPropertyGet),
        kind = "memo",
        memo = "Customed user property."
    )]
    /// UserPropertyGet2: auto doc placeholder.
    #[rapx::inner(property = Unknown(UserPropertyGet2), kind = "memo")]
    pub unsafe fn get(&self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}
