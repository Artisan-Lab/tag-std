// #![feature(register_tool)]
// #![register_tool(safety)]
#![allow(clippy::missing_safety_doc, clippy::mut_from_ref, internal_features)]
#![feature(core_intrinsics)]

use safety_tool_macro::safety;

#[safety(
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
    #[safety(
        Init(self.ptr, u8, self.len),
        memo = "The ptr must be initialized first!"
    )]
    // #[safety::precond::InBound(
    //     self.ptr, u8, self.len,
    //     memo = "The ptr must be within the length."
    // )]
    // #[safety::precond::ValidNum(
    //     self.len*sizeof(u8), [0,isize::MAX],
    //     memo = "Slice length can't exceed isize::MAX due to allocation limit."
    // )]
    // #[safety::hazard::Alias(
    //     self.ptr,
    //     memo = "Make sure don't alias the ptr."
    // )]
    pub unsafe fn get(&self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}
