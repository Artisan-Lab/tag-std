extern crate safety_lib;
use safety_lib::safety;

#[safety::precond::Align(a, b, c)]
fn f() {}

#[safety::Memo(ident)]
fn memo() {}
