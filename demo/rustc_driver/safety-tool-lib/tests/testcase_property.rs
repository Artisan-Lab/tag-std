#![feature(register_tool)]
#![register_tool(Safety)]

use safety_tool_lib::safety;

// #[Property(args)] syntax

#[safety::precond::Align(p, T)]
pub fn align() {}

#[safety::precond::Unwrap(p, T)]
pub fn unwrap() {}

#[safety::hazard::Alias(p1)]
pub fn alias() {}

#[safety::option::Unreachable()]
pub fn unreachable() {}
