#![feature(register_tool)]
#![register_tool(Safety)]

use safety_tool_lib::safety;

#[safety::precond::Align(T, memo = "reason")]
pub fn api1() {}

#[safety::hazard::Alias(T, memo = "reason")]
pub fn api2() {}

#[safety::option::Unreachable()]
pub fn api3() {}
