#![feature(prelude_import)]
#![feature(register_tool)]
#![register_tool(Safety)]
#![allow(dead_code, non_snake_case)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use safety_tool_lib::safety;
/// Unwrap: Make sure the value x must be Some(u32) before calling this function.
/// This is a single line.
#[Safety::inner(
    property = Unwrap(x u32),
    kind = "precond",
    memo = "This is a single line."
)]
fn single_line() {}
/// Unwrap: Make sure the value x must be Some(String) before calling this function.
/// Line 1.
/// Line 2.
#[Safety::inner(
    property = Unwrap(x String),
    kind = "precond",
    memo = "Line 1.\nLine 2."
)]
fn multi_lines() {}
/// Unwrap: Make sure the value x must be Some(Box) before calling this function.
/// Line 1.
/// Line 2.
#[Safety::inner(property = Unwrap(x Box), kind = "precond", memo = "\nLine 1.\nLine 2.")]
fn multi_lines2() {}
/// Line 1.
/// Unwrap: Make sure the value x must be Some(T) before calling this function.
/// Line 2.
/// Line 3.
#[Safety::inner(
    property = Unwrap(x T),
    kind = "precond",
    memo = "\n    Line 2.\n    Line 3."
)]
fn multi_lines3() {}
/// Line 1.
/// Unwrap: Make sure the value x must be Some(T) before calling this function.
/// Line 2.
/// Line 3.
#[Safety::inner(
    property = Unwrap(x T),
    kind = "precond",
    memo = "\n    Line 2.\n    Line 3."
)]
/// Unwrap: Make sure the value x must be Some(T) before calling this function.
/// Line 4.
#[Safety::inner(property = Unwrap(x T), kind = "precond", memo = "Line 4.")]
fn multi_lines4() {}
/// Line 3.
/// Unwrap: Make sure the value x must be Some(T) before calling this function.
/// Line 1.
/// Line 2.
#[Safety::inner(
    property = Unwrap(x T),
    kind = "precond",
    memo = "\n    Line 1.\n    Line 2."
)]
fn multi_lines3_DONT_DO_THIS() {}
/// Line 1.
/// Line 4.
/// Unwrap: Make sure the value x must be Some(T) before calling this function.
/// Line 2.
/// Line 3.
#[Safety::inner(property = Unwrap(x T), kind = "precond", memo = "Line 2.\nLine 3.")]
fn multi_lines4_DONT_DO_THIS() {}
/// Line 1.
/// Line 5.
/// Unwrap: Make sure the value x must be Some(T) before calling this function.
/// Line 2.
/// Line 3.
#[Safety::inner(property = Unwrap(x T), kind = "precond", memo = "Line 2.\nLine 3.")]
/// Unwrap: Make sure the value x must be Some(T) before calling this function.
/// Line 4.
#[Safety::inner(property = Unwrap(x T), kind = "precond", memo = "Line 4.")]
fn multi_lines5_DONT_DO_THIS() {}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
