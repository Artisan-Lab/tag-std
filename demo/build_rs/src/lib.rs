#[allow(unused_variables)]
use std::slice;

///contract(!Reachable())
pub unsafe fn test(){
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
    pub unsafe fn get(&self) -> &mut [u8] {
        slice::from_raw_parts_mut(self.ptr, self.len)
    }
}
