#![feature(prelude_import)]
#![feature(register_tool)]
#![register_tool(Safety)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use safety_tool_lib::safety;
/// Align: Make sure pointer `p` must be properly aligned for type `T` before calling this function.
#[Safety::inner(property = Align(p T), kind = "precond")]
pub fn api1() {}
/// Alias: Make sure p1 must not have other alias after calling this function.
#[Safety::inner(property = Alias(p1), kind = "hazard")]
pub fn api2() {}
/// Unreachable: To be noticed that, the current program point should not be reachable during execution.
#[Safety::inner(property = Unreachable(), kind = "option")]
pub fn api3() {}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
