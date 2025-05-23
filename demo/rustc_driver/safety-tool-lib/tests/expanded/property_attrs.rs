#![feature(prelude_import)]
#![feature(register_tool)]
#![register_tool(Safety)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use safety_tool_lib::safety;
/// reason
#[Safety::inner(kind = "precond", T, memo = "reason")]
pub fn api1() {}
/// reason
#[Safety::inner(kind = "hazard", T, memo = "reason")]
pub fn api2() {}
/// reason
#[Safety::inner(kind = "option", T, memo = "reason")]
pub fn api3() {}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
