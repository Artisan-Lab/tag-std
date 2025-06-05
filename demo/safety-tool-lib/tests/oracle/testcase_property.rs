#![feature(prelude_import)]
#![feature(register_tool)]
#![register_tool(rapx)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use safety_tool_lib::safety;
#[allow(unused_variables)]
/// Align: Make sure pointer `p` must be properly aligned for type `T` before calling this function.
#[rapx::inner(property = Align(p, T), kind = "precond")]
pub unsafe fn align<T>(p: T) {}
#[allow(unused_variables)]
/// Unwrap: Make sure the value p must be Some(T) before calling this function.
#[rapx::inner(property = Unwrap(p, T), kind = "precond")]
pub unsafe fn unwrap<T>(p: Option<T>) {}
#[allow(unused_variables)]
/// Alias: Make sure p must not have other alias after calling this function.
#[rapx::inner(property = Alias(p), kind = "hazard")]
pub unsafe fn alias<T>(p: T) {}
#[allow(unused_variables)]
/// Trait: To be noticed that, if type T implements trait Copy, the property "Alias" is mitigated.
#[rapx::inner(property = Trait(T, Copy, "Alias"), kind = "option")]
pub unsafe fn foo_trait<T>(p: T) {}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
