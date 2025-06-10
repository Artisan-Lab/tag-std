#![feature(stmt_expr_attributes)]
#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(dead_code)]

pub fn tag_expr() {
    unsafe {
        #[rapx::tag_expr(property = Memo(Tag), kind = "memo")]
        call()
    };
}

pub fn tag_block() {
    #[rapx::tag_block(property = Memo(Tag), kind = "memo")]
    unsafe {
        call();
    }
}

#[rapx::inner(property = Memo(Tag), kind = "memo")]
unsafe fn call() {}

#[rapx::tag_unsafe_fn(property = Memo(Tag), kind = "memo")]
pub unsafe fn tag_unsafe_fn() {
    call();
}
