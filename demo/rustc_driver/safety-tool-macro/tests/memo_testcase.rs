#![allow(dead_code)]
use safety_tool_macro::safety;

#[safety(Property, memo = "This is a single line.")]
fn single_line() {}

#[safety(Property, memo = "Line 1.\nLine 2.")]
fn multi_lines() {}

#[safety(
    Property,
    memo = "
Line 1.
Line 2."
)]
fn multi_lines2() {}

#[safety(
    Property,
    memo = "
    Line 1.
    Line 2."
)]
#[doc = " Line 3."]
fn multi_lines3() {}
