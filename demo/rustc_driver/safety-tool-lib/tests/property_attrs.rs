#![feature(register_tool)]
#![register_tool(Safety)]

use safety_tool_lib::safety;

#[safety::precond::Align(T, memo = "reason")]
pub fn api() {}

#[safety::precond::UnReachable(T, memo = "reason")]
pub fn api2() {}
