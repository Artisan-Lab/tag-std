#![feature(prelude_import)]
#![feature(proc_macro_hygiene)]
#![feature(register_tool)]
#![register_tool(rapx)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use safety_lib::safety;
/// # Safety
/// Align: Make sure pointer `p` must be properly aligned for type `T` before calling this function.
#[rapx::inner(property = Align(p, T), kind = "precond")]
pub unsafe fn align<T>(_: T) {}
pub fn discharges_align() {
    #[rapx::inner(property = Align(), kind = "precond")] unsafe { align(()) };
}
/// # Safety
/// Prop: auto doc placeholder.
#[rapx::inner(property = Unknown(Prop), kind = "memo")]
pub unsafe fn memo() {}
pub fn discharges_memo() {
    #[rapx::inner(property = Unknown(Prop), kind = "memo")] unsafe { memo() };
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
