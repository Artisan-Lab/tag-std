#![feature(prelude_import)]
#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(clippy::missing_safety_doc, clippy::mut_from_ref, internal_features)]
#![feature(core_intrinsics)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use safety_macro::safety;
#[rapx::inner(Unreachable)]
///* Unreachable: the current program point should not be reachable during execution
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
    #[rapx::inner(
        Init(self.ptr, u8, self.len),
        InBound(self.ptr, u8, self.len),
        ValidNum(self.len*sizeof(u8), [0, isize::MAX]),
        Alias(self.ptr)
    )]
    ///* Init: the memory range [self.ptr, self.ptr + sizeof(u8)*self.len] must be fully initialized for type T
    ///* InBound: the pointer self.ptr and its offset up to sizeof(u8)*self.len must point to a single allocated object
    ///* ValidNum: the value of self.len * sizeof(u8) must lie within the valid [0, isize :: MAX]
    ///* Alias: self.ptr must not have other alias
    pub unsafe fn get(&self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}
