#![feature(proc_macro_hygiene)]
#![feature(register_tool)]
#![register_tool(rapx)]

use safety_tool_lib::safety;

/// # Safety
#[safety::precond::Align(p, T)]
pub unsafe fn align<T>(_: T) {}

pub fn discharges_align() {
    #[safety::discharges(Precond_Align)]
    unsafe {
        align(())
    };
}

/// # Safety
#[safety::Memo(Prop)]
pub unsafe fn memo() {}

pub fn discharges_memo() {
    #[safety::discharges(Memo(Prop))]
    unsafe {
        memo()
    };
}
