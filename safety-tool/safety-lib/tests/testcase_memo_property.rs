#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(dead_code, non_snake_case)]
use safety_lib::safety;

#[safety::Memo(Ident, memo = "This is a user defined property.")]
fn memo_property() {}
