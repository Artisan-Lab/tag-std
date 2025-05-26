#![feature(prelude_import)]
#![feature(register_tool)]
#![register_tool(Safety)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use safety_tool_lib::safety;
#[Safety::inner(property = Align(T), kind = "precond")]
pub fn api1() {}
#[Safety::inner(property = Alias(T), kind = "hazard")]
pub fn api2() {}
#[Safety::inner(property = Unreachable(), kind = "option")]
pub fn api3() {}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
