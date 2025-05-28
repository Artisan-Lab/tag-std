#![feature(prelude_import)]
#![feature(register_tool)]
#![register_tool(Safety)]
#![allow(dead_code, non_snake_case)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use safety_tool_lib::safety;
/// x: auto doc placeholder.
/// This is a single line.
#[Safety::inner(property = Unknown(x), kind = "memo", memo = "This is a single line.")]
fn single_line() {}
/// x: auto doc placeholder.
/// Line 1.
/// Line 2.
#[Safety::inner(property = Unknown(x), kind = "memo", memo = "Line 1.\nLine 2.")]
fn multi_lines() {}
/// x: auto doc placeholder.
/// Line 1.
/// Line 2.
#[Safety::inner(property = Unknown(x), kind = "memo", memo = "\nLine 1.\nLine 2.")]
fn multi_lines2() {}
/// Line 1.
/// x: auto doc placeholder.
/// Line 2.
/// Line 3.
#[Safety::inner(
    property = Unknown(x),
    kind = "memo",
    memo = "\n    Line 2.\n    Line 3."
)]
fn multi_lines3() {}
/// Line 1.
/// x: auto doc placeholder.
/// Line 2.
/// Line 3.
#[Safety::inner(
    property = Unknown(x),
    kind = "memo",
    memo = "\n    Line 2.\n    Line 3."
)]
/// x: auto doc placeholder.
/// Line 4.
#[Safety::inner(property = Unknown(x), kind = "memo", memo = "Line 4.")]
fn multi_lines4() {}
/// Line 3.
/// x: auto doc placeholder.
/// Line 1.
/// Line 2.
#[Safety::inner(
    property = Unknown(x),
    kind = "memo",
    memo = "\n    Line 1.\n    Line 2."
)]
fn multi_lines3_DONT_DO_THIS() {}
/// Line 1.
/// Line 4.
/// x: auto doc placeholder.
/// Line 2.
/// Line 3.
#[Safety::inner(property = Unknown(x), kind = "memo", memo = "Line 2.\nLine 3.")]
fn multi_lines4_DONT_DO_THIS() {}
/// Line 1.
/// Line 5.
/// x: auto doc placeholder.
/// Line 2.
/// Line 3.
#[Safety::inner(property = Unknown(x), kind = "memo", memo = "Line 2.\nLine 3.")]
/// x: auto doc placeholder.
/// Line 4.
#[Safety::inner(property = Unknown(x), kind = "memo", memo = "Line 4.")]
fn multi_lines5_DONT_DO_THIS() {}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
