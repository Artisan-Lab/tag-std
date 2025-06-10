#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(unused_variables)]

use safety_lib::safety;

/// # Safety
#[safety::precond::Align(p, T)]
pub unsafe fn align<T>(p: T) {}

/// # Safety
#[safety::precond::Unwrap(p, T)]
pub unsafe fn unwrap<T>(p: Option<T>) {}

/// # Safety
#[safety::hazard::Alias(p)]
pub unsafe fn alias<T>(p: T) {}

/// # Safety
#[safety::option::Trait(T, Copy, "Alias")]
pub unsafe fn foo_trait<T>(p: T) {}
