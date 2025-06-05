#![feature(stmt_expr_attributes)]
#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(dead_code)]

#[rapx::inner(property = Memo(Tag), kind = "memo")]
unsafe fn call() {}

// Indirect call expressions are not supported yet.

pub fn assign() {
    let f = call;
    #[rapx::assign(property = Memo(Tag), kind = "memo")]
    unsafe {
        f()
    };
}
