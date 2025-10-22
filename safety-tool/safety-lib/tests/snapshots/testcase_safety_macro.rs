#![feature(prelude_import)]
#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(clippy::missing_safety_doc)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use safety_macro::requires;
#[rapx::requires(SP)]
pub unsafe fn vanilla1() {}
#[rapx::requires(SP1, SP2)]
pub unsafe fn vanilla2() {}
#[rapx::requires(SP1;SP2)]
pub unsafe fn vanilla3() {}
#[rapx::requires(SP1:"reason")]
pub unsafe fn sp_with_reason1() {}
#[rapx::requires(SP1:"reason";SP2:"reason")]
pub unsafe fn sp_with_reason2() {}
#[rapx::requires(SP1, SP2:"reason")]
pub unsafe fn grouped_sp1() {}
#[rapx::requires(SP1, SP2:"reason";SP3)]
pub unsafe fn grouped_sp2() {}
#[rapx::requires(SP3;SP1, SP2:"reason")]
pub unsafe fn grouped_sp3() {}
#[rapx::requires(SP3, SP4;SP1, SP2:"reason")]
pub unsafe fn grouped_sp4() {}
#[rapx::requires(SP3;SP1, SP2:"reason";SP4)]
pub unsafe fn grouped_sp5() {}
#[rapx::requires(SP1, SP2:"reason";SP3;)]
pub unsafe fn trailing_punct1() {}
#[rapx::requires(SP1, SP2:"reason";SP3)]
pub unsafe fn trailing_punct2() {}
#[rapx::requires(SP1(a))]
pub unsafe fn single_sp_with_args1() {}
#[rapx::requires(SP1(a, b))]
pub unsafe fn single_sp_with_args2() {}
#[rapx::requires(SP1(a, b, call()))]
pub unsafe fn single_sp_with_args3() {}
#[rapx::requires(SP1(a), SP2:"reason";SP3)]
pub unsafe fn multiple_sp_with_args1() {}
#[rapx::requires(SP(a, b):"reason";SP1, SP2:"reason";SP3, SP4)]
pub unsafe fn multiple_sp_with_args2() {}
#[rapx::requires(hazard.Alias(p, q))]
pub unsafe fn complex1() {}
#[rapx::requires(hazard.Alias(A{a:self.a}, a::b(c![])))]
pub unsafe fn complex2() {}
#[rapx::requires(hazard.Alias(A{a:self.a}, a::b(c![])):"")]
pub unsafe fn complex3() {}
#[rapx::requires(hazard.Alias(A{a:self.a}, a::b(c![])):"";SP)]
pub unsafe fn complex4() {}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
