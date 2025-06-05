#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]
#![feature(vec_into_raw_parts)]
#![feature(register_tool)]
#![register_tool(rapx)]

use demo::MyStruct;
use safety_tool_lib::safety;

fn main() {
    let (p, l, _c) = Vec::new().into_raw_parts();
    let a = MyStruct::from(p, l);
    println!("{:?}", unsafe {
        #[safety::discharges(Memo(UserPropertyGet))]
        a.get()
    });
}
