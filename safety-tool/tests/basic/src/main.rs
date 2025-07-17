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
        #[safety::discharges(Precond_Init, memo = "This is from a valid Vec object.")]
        #[safety::discharges(Precond_InBound, memo = "This is from a valid Vec object.")]
        #[safety::discharges(Precond_ValidNum, memo = "self.len is valid.")]
        #[safety::discharges(Hazard_Alias, memo = "p is no longer used.")]
        a.get()
    });
}
