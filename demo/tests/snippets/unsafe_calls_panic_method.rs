#![feature(stmt_expr_attributes)]
#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(dead_code)]

pub fn tag_block() {
    let s = Struct::new();
    unsafe {
        s.call();
    }
}

struct Struct {}

impl Struct {
    fn new() -> Self {
        Self {}
    }

    #[rapx::inner(property = Memo(Tag), kind = "memo")]
    unsafe fn call(&self) {}
}
