#![feature(stmt_expr_attributes)]
#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(dead_code)]

#[rapx::inner(Tag)]
unsafe fn call() {}

// Indirect call expressions are not supported yet.

pub fn assign() {
    let f = call;
    #[rapx::assign(Tag)]
    unsafe {
        f()
    };
}
