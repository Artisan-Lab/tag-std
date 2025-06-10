#![feature(register_tool)]
#![register_tool(rapx)]

#[rapx::inner(property = Memo(Tag), kind = "memo")]
#[rapx::tag_unsafe_fn(property = Align(), kind = "precond")]
unsafe fn call() {}

#[rapx::tag_unsafe_fn(property = Memo(Tag), kind = "memo")]
pub unsafe fn tag_unsafe_fn() {
    call();
}
