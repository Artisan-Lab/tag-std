#![feature(prelude_import)]
#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(clippy::missing_safety_doc)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use safety_macro::safety;
#[rapx::proof(SP)]
pub unsafe fn vanilla1() {}
#[rapx::proof(SP1, SP2)]
pub unsafe fn vanilla2() {}
#[rapx::proof(SP1;SP2)]
pub unsafe fn vanilla3() {}
#[rapx::proof(SP1:"reason")]
///reason
pub unsafe fn sp_with_reason1() {}
#[rapx::proof(SP1:"reason";SP2:"reason")]
///reason
///reason
pub unsafe fn sp_with_reason2() {}
#[rapx::proof(SP1, SP2:"reason")]
///reason
pub unsafe fn grouped_sp1() {}
#[rapx::proof(SP1, SP2:"reason";SP3)]
///reason
pub unsafe fn grouped_sp2() {}
#[rapx::proof(SP3;SP1, SP2:"reason")]
///reason
pub unsafe fn grouped_sp3() {}
#[rapx::proof(SP3, SP4;SP1, SP2:"reason")]
///reason
pub unsafe fn grouped_sp4() {}
#[rapx::proof(SP3;SP1, SP2:"reason";SP4)]
///reason
pub unsafe fn grouped_sp5() {}
#[rapx::proof(SP1, SP2:"reason";SP3;)]
///reason
pub unsafe fn trailing_punct1() {}
#[rapx::proof(SP1, SP2:"reason";SP3)]
///reason
pub unsafe fn trailing_punct2() {}
#[rapx::proof(SP1(a))]
pub unsafe fn single_sp_with_args1() {}
#[rapx::proof(SP1(a, b))]
pub unsafe fn single_sp_with_args2() {}
#[rapx::proof(SP1(a, b, call()))]
pub unsafe fn single_sp_with_args3() {}
#[rapx::proof(SP1(a), SP2:"reason";SP3)]
///reason
pub unsafe fn multiple_sp_with_args1() {}
#[rapx::proof(SP(a, b):"reason";SP1, SP2:"reason";SP3, SP4)]
///reason
///reason
pub unsafe fn multiple_sp_with_args2() {}
#[rapx::proof(hazard.Alias(p, q))]
#[doc = "* `p` must not have other alias\n\n"]
pub unsafe fn complex1() {}
#[rapx::proof(hazard.Alias(A{a:self.a}, a::b(c![])))]
#[doc = "* `A { a : self.a }` must not have other alias\n\n"]
pub unsafe fn complex2() {}
#[rapx::proof(hazard.Alias(A{a:self.a}, a::b(c![])):"")]
///
#[doc = "* `A { a : self.a }` must not have other alias\n\n"]
pub unsafe fn complex3() {}
#[rapx::proof(hazard.Alias(A{a:self.a}, a::b(c![])):"";SP)]
///
#[doc = "* `A { a : self.a }` must not have other alias\n\n"]
pub unsafe fn complex4() {}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
