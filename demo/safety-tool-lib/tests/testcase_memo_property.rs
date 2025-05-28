#![feature(register_tool)]
#![register_tool(Safety)]
#![allow(dead_code, non_snake_case)]
use safety_tool_lib::safety;

#[safety::Memo(Ident, memo = "This is a user defined property.")]
fn memo_property() {}
