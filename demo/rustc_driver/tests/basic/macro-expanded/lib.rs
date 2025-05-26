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
/// Unreachable: auto doc placeholder.
#[Safety::inner(property = Unreachable(), kind = "precond")]
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
    #[Safety::inner(
        property = Unknown(UserProperty),
        kind = "memo",
        memo = "Customed user property."
    )]
    pub fn from(p: *mut u8, l: usize) -> MyStruct {
        MyStruct { ptr: p, len: l }
    }
    /// Init: auto doc placeholder.
    #[Safety::inner(property = Init(self.ptr u8 self.len), kind = "precond")]
    /// InBound: auto doc placeholder.
    #[Safety::inner(property = InBound(self.ptr u8 self.len), kind = "precond")]
    /// ValidNum: auto doc placeholder.
    #[Safety::inner(
        property = ValidNum(self.len*sizeof(u8)[0, isize::MAX]),
        kind = "precond"
    )]
    /// Alias: auto doc placeholder.
    #[Safety::inner(property = Alias(self.ptr), kind = "hazard")]
    pub unsafe fn get(&self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}
