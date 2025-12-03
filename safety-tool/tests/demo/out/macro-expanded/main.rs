#![feature(prelude_import)]
#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]
#![feature(vec_into_raw_parts)]
#![feature(register_tool)]
#![register_tool(rapx)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use demo::MyStruct;
use safety_macro as safety;
fn main() {
    let (p, l, _c) = Vec::new().into_raw_parts();
    (
        match p {
            tmp => {
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3} = {4:#?}\n",
                            "src/main.rs",
                            12u32,
                            5u32,
                            "p",
                            &&tmp as &dyn ::std::fmt::Debug,
                        ),
                    );
                };
                tmp
            }
        },
        match l {
            tmp => {
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3} = {4:#?}\n",
                            "src/main.rs",
                            12u32,
                            5u32,
                            "l",
                            &&tmp as &dyn ::std::fmt::Debug,
                        ),
                    );
                };
                tmp
            }
        },
    );
    let a = MyStruct::from(p, l);
    #[rapx::checked(
        NonNull(
            p
        ):"Vec::new generate a dangling pointer, but it's not null";ValidPtr(
            p,
            u8,
            l
        ):"l is zero in this case, and zero size access is always valid";Init(
            p,
            u8,
            l
        ):"no element yet, so no need to initialize anything";Alive(
            p,
            l
        ):"there is no real data, so this is met";Alias(
            p
        ):"p is no longer used other than in MyStruct";Align(
            p,
            u8
        ):"Vec makes an aligned pointer";ValidNum(
            l,
            [0,
            isize::MAX]
        ):"l is zero here, thus in range"
    )]
    let val = unsafe { a.get() };
    {
        ::std::io::_print(format_args!("{0:?}\n", val));
    };
}
