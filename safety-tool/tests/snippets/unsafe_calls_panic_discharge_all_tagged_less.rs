#![feature(register_tool)]
#![register_tool(rapx)]

#[rapx::inner(Tag)]
#[rapx::tag_unsafe_fn(Align)]
unsafe fn call() {}

#[rapx::tag_unsafe_fn(Tag)]
pub unsafe fn tag_unsafe_fn() {
    call();
}
