#![feature(prelude_import)]
#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]
#![feature(vec_into_raw_parts)]
#![feature(register_tool)]
#![register_tool(rapx)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use demo::MyStruct;
use safety_lib::safety;
fn main() {
    let (p, l, _c) = Vec::new().into_raw_parts();
    let a = MyStruct::from(p, l);
    {
        ::std::io::_print(
            format_args!(
                "{0:?}\n",
                unsafe {
                    #[rapx::inner(property = Alias(), kind = "hazard")]
                    #[rapx::inner(property = ValidNum(), kind = "precond")]
                    #[rapx::inner(property = InBound(), kind = "precond")]
                    #[rapx::inner(property = Init(), kind = "precond")]
                    #[rapx::inner(property = Unknown(UserPropertyGet), kind = "memo")]
                    a.get()
                },
            ),
        );
    };
}
