#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]
#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(unused)]

use std::mem::size_of;
use safety_macro as safety;

#[safety::requires(Align(p,T))]
pub unsafe fn foo<T>(p: *const T) {
    assert!((p as usize) % size_of::<T>() == 0);
}

fn bar() {
    let p = 1 as *const u8;
    //#[safety::checked {Align: "p is aligned"}]
    unsafe{
        foo(p);
    }
}

fn main() {
    bar();
}
