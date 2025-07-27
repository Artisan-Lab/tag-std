#![feature(stmt_expr_attributes)]
#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(dead_code)]

pub fn tag_expr() {
    let s = Struct::new();
    unsafe {
        #[rapx::tag_expr(Tag)]
        s.call()
    };
}

pub fn tag_block() {
    let s = Struct::new();
    #[rapx::tag_block(Tag)]
    unsafe {
        s.call();
    }
}

struct Struct {}

impl Struct {
    fn new() -> Self {
        Self {}
    }

    #[rapx::inner(Tag)]
    unsafe fn call(&self) {}
}

#[rapx::tag_unsafe_fn(Tag)]
pub unsafe fn tag_unsafe_fn() {
    let s = Struct::new();
    s.call();
}
