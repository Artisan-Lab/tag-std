#![feature(register_tool)]
#![register_tool(rapx)]

#[rapx::inner(property = Memo(Tag), kind = "memo")]
unsafe fn call() {}

#[rapx::tag_unsafe_fn(property = Memo(Tag), kind = "memo")]
#[rapx::tag_unsafe_fn(property = Align(), kind = "precond")]
pub unsafe fn tag_unsafe_fn() {
    call();
}
