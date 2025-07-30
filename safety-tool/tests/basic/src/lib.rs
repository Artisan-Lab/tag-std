#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(clippy::missing_safety_doc, clippy::mut_from_ref, internal_features)]
#![feature(core_intrinsics)]

use safety_macro::safety;

#[safety{ Unreachable }]
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

    /// correct link: [`test`]
    #[safety {
        Init(self.ptr, u8, self.len),
        InBound(self.ptr, u8, self.len),
        ValidNum(self.len*sizeof(u8), [0,isize::MAX]),
        Alias(self.ptr),
        RustdocLinkToItem("test")
    }]
    pub unsafe fn get(&self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}
