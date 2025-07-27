#![feature(stmt_expr_attributes)]
#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(dead_code)]

pub fn tag_expr() {
    unsafe {
        #[rapx::tag_expr(Tag)]
        #[rapx::tag_expr(Align)]
        call()
    };
}

pub fn tag_block() {
    #[rapx::tag_block(Tag)]
    #[rapx::tag_block(Align)]
    unsafe {
        call();
    }
}

#[rapx::inner(Tag)]
#[rapx::inner(Align)]
unsafe fn call() {}

#[rapx::tag_unsafe_fn(Tag)]
#[rapx::tag_unsafe_fn(Align)]
pub unsafe fn tag_unsafe_fn() {
    call();
}
