#![feature(register_tool)]
#![register_tool(Safety)]

use safety_tool_lib::safety;

// #[Property(args)] syntax

#[safety::precond::Align(p, T)]
pub fn api1() {}

#[safety::hazard::Alias(p1)]
pub fn api2() {}

#[safety::option::Unreachable()]
pub fn api3() {}
