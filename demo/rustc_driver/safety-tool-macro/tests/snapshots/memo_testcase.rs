#![feature(prelude_import)]
#![allow(dead_code)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use safety_tool_macro::safety;
/// This is a single line.
fn single_line() {}
/// Line 1.
/// Line 2.
fn multi_lines() {}
/// Line 1.
/// Line 2.
fn multi_lines2() {}
/// Line 1.
/// Line 2.
/// Line 3.
fn multi_lines3() {}
/// Line 2.
/// Line 3.
/// Line 1.
/// Line 4.
fn multi_lines4() {}
/// Line 4.
/// Line 2.
/// Line 3.
/// Line 1.
/// Line 5.
fn multi_lines5() {}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
