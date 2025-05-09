#![feature(vec_into_raw_parts)]
#[allow(unused_variables)]
use crate::contract::extract_contract;
use contract;
use std::slice;

struct MyStruct {
    ptr: *mut u8,
    len: usize,
}

impl MyStruct {
    fn from(p: *mut u8, l: usize) -> MyStruct {
        MyStruct { ptr: p, len: l }
    }
    ///contract(!Null(self.ptr); Align(self.ptr, u8); Allocated(self.ptr, u8, self.len, *); Init(self.ptr, u8, self.len, *); ValidInt(self.len*sizeof(u8), [0,isize::MAX]); Alias(self.ptr, *);
    #[extract_contract]
    /*
    #[cfg_attr(feature = "contract",
        contract(!Null(self.ptr);
            Align(self.ptr, u8);
            Allocated(self.ptr, u8, self.len, *);
            Init(self.ptr, u8, self.len, *);
            ValidInt(self.len*sizeof(u8), [0,isize::MAX]);
            Alias(self.ptr, *);
        )
    )]*/
    unsafe fn get(&self) -> &mut [u8] {
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
