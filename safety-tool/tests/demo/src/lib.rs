#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(clippy::missing_safety_doc, clippy::mut_from_ref, internal_features)]
#![feature(core_intrinsics)]

use safety_macro as safety;

/// correct link: [`crate::test`]
#[safety::requires {
    Init(self.ptr, u8, self.len),
    InBound(self.ptr, u8, self.len),
    ValidNum(self.len*sizeof(u8), [0,isize::MAX]),
    Alias(self.ptr),
    RustdocLinkToItem("crate::test"),
    any { Deref(self.ptr, u8, 1), Alive(self.ptr, _) }
}]
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

    #[safety::requires {
        NonNull(self.ptr),
        ValidPtr(self.ptr, u8, self.len),
        Init(self.ptr, u8, self.len),
        Alive(self.ptr, _),
        Alias(self.ptr),
        Align(self.ptr, u8),
        ValidNum(self.len*sizeof(u8), [0,isize::MAX]),
    }]
    pub unsafe fn get(&self) -> &mut [u8] {
        // SAFETY: safety requirements are delegated to the caller.
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}
