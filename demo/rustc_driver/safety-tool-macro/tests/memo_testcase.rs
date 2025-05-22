#![allow(dead_code, non_snake_case)]
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

#[doc = " Line 1."]
#[safety(
    Property,
    memo = "
    Line 2.
    Line 3."
)]
fn multi_lines3() {}

#[doc = " Line 1."]
#[safety(
    Property,
    memo = "
    Line 2.
    Line 3."
)]
#[safety(Property, memo = "Line 4.")]
fn multi_lines4() {}

//  WARNING: dont't put `#[doc]` (i.e. `///`) after `#[safety(memo)]`
// because the doc order will be messed up.

#[safety(
    Property,
    memo = "
    Line 1.
    Line 2."
)]
#[doc = " Line 3."] // don't do this.
fn multi_lines3_DONT_DO_THIS() {}

/// Line 1.
#[safety(Property, memo = "Line 2.\nLine 3.")]
#[doc = " Line 4."] // don't do this.
fn multi_lines4_DONT_DO_THIS() {}

/// Line 1.
#[safety(Property, memo = "Line 2.\nLine 3.")]
#[safety(Property, memo = "Line 4.")]
#[doc = " Line 5."] // don't do this.
fn multi_lines5_DONT_DO_THIS() {}
