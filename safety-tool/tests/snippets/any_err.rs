#![feature(stmt_expr_attributes)]
#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(dead_code)]

pub fn tag_expr() {
    unsafe { call() };
}

#[rapx::inner(any(SP1, SP2))]
unsafe fn call() {}
