#![feature(stmt_expr_attributes)]
#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(dead_code)]

pub fn tag_expr() {
    unsafe {
        #[rapx::tag_expr(Tag)]
        call()
    };
}

pub fn tag_block() {
    #[rapx::tag_block(Tag)]
    unsafe {
        call();
    }
}

#[rapx::inner(Tag)]
unsafe fn call() {}

#[rapx::tag_unsafe_fn(Tag)]
pub unsafe fn tag_unsafe_fn() {
    call();
}
