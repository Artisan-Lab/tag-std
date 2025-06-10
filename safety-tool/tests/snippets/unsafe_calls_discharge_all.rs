#![feature(stmt_expr_attributes)]
#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(dead_code)]

pub fn tag_expr() {
    unsafe {
        #[rapx::tag_expr(property = Memo(Tag), kind = "memo")]
        #[rapx::tag_expr(property = Align(), kind = "precond")]
        call()
    };
}

pub fn tag_block() {
    #[rapx::tag_block(property = Memo(Tag), kind = "memo")]
    #[rapx::tag_block(property = Align(), kind = "precond")]
    unsafe {
        call();
    }
}

#[rapx::inner(property = Memo(Tag), kind = "memo")]
#[rapx::inner(property = Align(), kind = "precond")]
unsafe fn call() {}

#[rapx::tag_unsafe_fn(property = Memo(Tag), kind = "memo")]
#[rapx::tag_unsafe_fn(property = Align(), kind = "precond")]
pub unsafe fn tag_unsafe_fn() {
    call();
}
