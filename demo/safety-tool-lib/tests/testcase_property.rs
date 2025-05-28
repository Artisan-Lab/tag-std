#![feature(register_tool)]
#![register_tool(Safety)]

use safety_tool_lib::safety;

#[safety::precond::Align(p, T)]
#[allow(unused_variables)]
pub unsafe fn align<T>(p: T) {}

#[safety::precond::Unwrap(p, T)]
#[allow(unused_variables)]
pub unsafe fn unwrap<T>(p: Option<T>) {}

#[safety::hazard::Alias(p)]
#[allow(unused_variables)]
pub unsafe fn alias<T>(p: T) {}

#[safety::option::Trait(T, Copy, "Alias")]
#[allow(unused_variables)]
pub unsafe fn foo_trait<T>(p: T) {}
