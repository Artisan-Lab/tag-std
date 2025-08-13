#![feature(stmt_expr_attributes)]
#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(dead_code)]

pub fn tag_expr() {
    unsafe {
        #[rapx::tag_expr(SP1)]
        call()
    };
}

pub fn tag_block() {
    #[rapx::tag_block(SP2)]
    unsafe {
        call();
    }
}

#[rapx::inner(any(SP1, SP2))]
unsafe fn call() {}

#[rapx::tag_unsafe_fn(SP1, SP2)] // not recommended
pub unsafe fn tag_unsafe_fn() {
    call();
}
