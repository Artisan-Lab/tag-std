#![allow(clippy::missing_safety_doc)]
use safety_macro::safety;

#[safety { SP }]
pub unsafe fn vanilla1() {}
#[safety { SP1, SP2 }]
pub unsafe fn vanilla2() {}
#[safety { SP1; SP2 }]
pub unsafe fn vanilla3() {}

#[safety { SP1: "reason" }]
pub unsafe fn sp_with_reason1() {}
#[safety { SP1: "reason"; SP2: "reason" }]
pub unsafe fn sp_with_reason2() {}

#[safety { SP1, SP2: "reason" }]
pub unsafe fn grouped_sp1() {}
#[safety { SP1, SP2: "reason"; SP3 }]
pub unsafe fn grouped_sp2() {}
#[safety { SP3; SP1, SP2: "reason" }]
pub unsafe fn grouped_sp3() {}
#[safety { SP3, SP4; SP1, SP2: "reason" }]
pub unsafe fn grouped_sp4() {}
#[safety { SP3; SP1, SP2: "reason"; SP4 }]
pub unsafe fn grouped_sp5() {}

#[safety { SP1, SP2: "reason"; SP3; }]
pub unsafe fn trailing_punct1() {}
#[safety { SP1, SP2: "reason"; SP3, }]
pub unsafe fn trailing_punct2() {}

#[safety { SP1(a) }]
pub unsafe fn single_sp_with_args1() {}
#[safety { SP1(a, b) }]
pub unsafe fn single_sp_with_args2() {}
#[safety { SP1(a, b, call()) }]
pub unsafe fn single_sp_with_args3() {}

#[safety { SP1(a), SP2: "reason"; SP3 }]
pub unsafe fn multiple_sp_with_args1() {}
#[safety { SP(a, b): "reason"; SP1, SP2: "reason"; SP3, SP4 }]
pub unsafe fn multiple_sp_with_args2() {}

#[safety { hazard.Alias(p, q) }]
pub unsafe fn complex1() {}
#[safety { hazard.Alias(A {a: self.a}, a::b(c![])) }]
pub unsafe fn complex2() {}
#[safety { hazard.Alias(A {a: self.a}, a::b(c![])): "" }]
pub unsafe fn complex3() {}
#[safety { hazard.Alias(A {a: self.a}, a::b(c![])): ""; SP }]
pub unsafe fn complex4() {}
