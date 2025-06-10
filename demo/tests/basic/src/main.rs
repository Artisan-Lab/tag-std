#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]
#![feature(vec_into_raw_parts)]
#![feature(register_tool)]
#![register_tool(rapx)]

use demo::MyStruct;
use safety_lib::safety;

fn main() {
    let (p, l, _c) = Vec::new().into_raw_parts();
    let a = MyStruct::from(p, l);
    println!("{:?}", unsafe {
        #[safety::discharges(Memo(UserPropertyGet))]
        // The following discharges are checked if
        // `DISCHARGES_ALL_PROPERTIES=1` is set.
        #[safety::discharges(Precond_Init)]
        #[safety::discharges(Precond_InBound)]
        #[safety::discharges(Precond_ValidNum)]
        #[safety::discharges(Hazard_Alias)]
        a.get()
    });
}
