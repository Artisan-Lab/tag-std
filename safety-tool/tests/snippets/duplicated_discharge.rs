#![feature(stmt_expr_attributes)]
#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(dead_code)]

pub fn tag_block() {
    #[rapx::tag_block(Tag(a), Tag(b))]
    unsafe {
        call()
    }
}

#[rapx::inner(Tag(a), Tag(b))]
unsafe fn call() {}
