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

#[rapx::inner(
    any ( SP1, SP2 ),
    any { SP3, SP4 }
)]
unsafe fn call() {}
