#![feature(vec_into_raw_parts)]
#[allow(unused_variables)]
use crate::contract::extract_contract;
use contract;
use std::slice;

///safe::require(!Reachable())
#[extract_contract]
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
    ///safe::require(Init(self.ptr, u8, self.len))
    ///safe::require(InBound(self.ptr, u8, self.len))
    ///safe::require(ValidNum(self.len*sizeof(u8), [0,isize::MAX])) 
    ///safe::hazard(Alias(self.ptr))
    #[extract_contract]
    pub unsafe fn get(&self) -> &mut [u8] {
        slice::from_raw_parts_mut(self.ptr, self.len)
    }
}

fn main() {
    let (p, l, _c) = Vec::new().into_raw_parts();
    let a = MyStruct::from(p, l);
    println!("{:?}", unsafe { 
        MyStruct::get_contract();
        a.get() 
    });
}
