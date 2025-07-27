#![feature(register_tool)]
#![register_tool(rapx)]

#[rapx::inner(Tag)]
unsafe fn call() {}

// FIXME: distinguish discharge and definition tags.
// cc https://github.com/os-checker/tag-std/issues/17
#[rapx::tag_unsafe_fn(Tag)]
#[rapx::tag_unsafe_fn(Align)]
pub unsafe fn tag_unsafe_fn() {
    call();
}
