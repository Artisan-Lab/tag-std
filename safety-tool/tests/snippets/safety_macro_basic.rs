extern crate safety_macro;
use safety_macro::safety;

#[safety { Align(a, b, c) }]
fn f() {}

#[safety { Ident }]
fn memo() {}
