#![feature(register_tool)]
#![register_tool(tag_std)]

use std::slice;

// #[tag_std(!Reachable())]
#[tag_std::unreachable]
pub unsafe fn test() {
    println!("unreachable!");
}

pub struct MyStruct {
    ptr: *mut u8,
    len: usize,
}
impl MyStruct {
    pub fn from(p: *mut u8, l: usize) -> MyStruct {
        MyStruct { ptr: p, len: l }
    }
    ///contract(!Null(self.ptr); Align(self.ptr, u8); Allocated(self.ptr, u8, self.len, *); Init(self.ptr, u8, self.len, *); ValidInt(self.len*sizeof(u8), [0,isize::MAX]); Alias(self.ptr, *);
    #[tag_std::contract(
        !Null(self.ptr) && Align(self.ptr, u8) &&
        Allocated(self.ptr, u8, self.len, *) && Init(self.ptr, u8, self.len, *) &&
        ValidInt(self.len*sizeof(u8), [0,isize::MAX]) && Alias(self.ptr, *)
    )]
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn get(&self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}
