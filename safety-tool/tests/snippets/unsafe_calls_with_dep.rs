#![feature(stmt_expr_attributes)]
#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(dead_code)]

extern crate unsafe_calls;

#[rapx::tag_unsafe_fn(property = Memo(Tag), kind = "memo")]
fn use_tag_unsafe_fn() {
    unsafe { unsafe_calls::tag_unsafe_fn() }
}
