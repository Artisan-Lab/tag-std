#![feature(register_tool)]
#![register_tool(safe)]

#[allow(clippy::mut_from_ref)]

use std::slice;

#[safe::require(!Reachable())]
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
    #[safe::require(Init(self.ptr, u8, self.len))]
    #[safe::require(InBound(self.ptr, u8, self.len))]
    #[safe::require(ValidNum(self.len*sizeof(u8), [0,isize::MAX]))] 
    #[safe::hazard(Alias(self.ptr))]
    pub unsafe fn get(&self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}
