#![feature(prelude_import)]
#![allow(clippy::missing_safety_doc)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use safety_macro::safety;
pub unsafe fn vanilla1() {}
pub unsafe fn vanilla2() {}
pub unsafe fn vanilla3() {}
pub unsafe fn sp_with_reason1() {}
pub unsafe fn sp_with_reason2() {}
pub unsafe fn grouped_sp1() {}
pub unsafe fn grouped_sp2() {}
pub unsafe fn grouped_sp3() {}
pub unsafe fn grouped_sp4() {}
pub unsafe fn grouped_sp5() {}
pub unsafe fn trailing_punct1() {}
pub unsafe fn trailing_punct2() {}
pub unsafe fn single_sp_with_args1() {}
pub unsafe fn single_sp_with_args2() {}
pub unsafe fn single_sp_with_args3() {}
pub unsafe fn multiple_sp_with_args1() {}
pub unsafe fn multiple_sp_with_args2() {}
pub unsafe fn complex1() {}
pub unsafe fn complex2() {}
pub unsafe fn complex3() {}
pub unsafe fn complex4() {}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
