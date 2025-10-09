#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(clippy::missing_safety_doc)]
use safety_macro::requires;

#[requires { SP }]
pub unsafe fn vanilla1() {}
#[requires { SP1, SP2 }]
pub unsafe fn vanilla2() {}
#[requires { SP1; SP2 }]
pub unsafe fn vanilla3() {}

#[requires { SP1: "reason" }]
pub unsafe fn sp_with_reason1() {}
#[requires { SP1: "reason"; SP2: "reason" }]
pub unsafe fn sp_with_reason2() {}

#[requires { SP1, SP2: "reason" }]
pub unsafe fn grouped_sp1() {}
#[requires { SP1, SP2: "reason"; SP3 }]
pub unsafe fn grouped_sp2() {}
#[requires { SP3; SP1, SP2: "reason" }]
pub unsafe fn grouped_sp3() {}
#[requires { SP3, SP4; SP1, SP2: "reason" }]
pub unsafe fn grouped_sp4() {}
#[requires { SP3; SP1, SP2: "reason"; SP4 }]
pub unsafe fn grouped_sp5() {}

#[requires { SP1, SP2: "reason"; SP3; }]
pub unsafe fn trailing_punct1() {}
#[requires { SP1, SP2: "reason"; SP3, }]
pub unsafe fn trailing_punct2() {}

#[requires { SP1(a) }]
pub unsafe fn single_sp_with_args1() {}
#[requires { SP1(a, b) }]
pub unsafe fn single_sp_with_args2() {}
#[requires { SP1(a, b, call()) }]
pub unsafe fn single_sp_with_args3() {}

#[requires { SP1(a), SP2: "reason"; SP3 }]
pub unsafe fn multiple_sp_with_args1() {}
#[requires { SP(a, b): "reason"; SP1, SP2: "reason"; SP3, SP4 }]
pub unsafe fn multiple_sp_with_args2() {}

#[requires { hazard.Alias(p, q) }]
pub unsafe fn complex1() {}
#[requires { hazard.Alias(A {a: self.a}, a::b(c![])) }]
pub unsafe fn complex2() {}
#[requires { hazard.Alias(A {a: self.a}, a::b(c![])): "" }]
pub unsafe fn complex3() {}
#[requires { hazard.Alias(A {a: self.a}, a::b(c![])): ""; SP }]
pub unsafe fn complex4() {}
