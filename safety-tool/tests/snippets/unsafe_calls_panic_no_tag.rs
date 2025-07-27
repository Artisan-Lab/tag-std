#![feature(stmt_expr_attributes)]
#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(dead_code)]

#[rapx::inner(Tag)]
unsafe fn call() {}

// No tag cases should really panic.

mod submod {
    unsafe fn submod_no_tag() {
        super::call();
    }
}
