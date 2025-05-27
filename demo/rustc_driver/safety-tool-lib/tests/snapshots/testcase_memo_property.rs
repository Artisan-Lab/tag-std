#![feature(prelude_import)]
#![feature(register_tool)]
#![register_tool(Safety)]
#![allow(dead_code, non_snake_case)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use safety_tool_lib::safety;
/// Ident: auto doc placeholder.
/// This is a user defined property.
#[Safety::inner(
    property = Unknown(Ident),
    kind = "memo",
    memo = "This is a user defined property."
)]
fn memo_property() {}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
