#![feature(stmt_expr_attributes)]
#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(dead_code)]

#[rapx::inner(Tag)]
unsafe fn call() {}

// Indirect call expressions are not supported yet.

pub fn assign_fn_ptr() {
    let f: unsafe fn() = call;
    unsafe {
        #[rapx::assign_fn_ptr(Tag)]
        f()
    };
}
