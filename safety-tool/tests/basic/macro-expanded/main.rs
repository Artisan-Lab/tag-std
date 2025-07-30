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
use safety_macro::safety;
fn main() {
    let (p, l, _c) = Vec::new().into_raw_parts();
    let a = MyStruct::from(p, l);
    {
        ::std::io::_print(
            format_args!(
                "{0:?}\n",
                unsafe {
                    #[rapx::inner(
                        Init:"This is from a valid Vec object.";InBound:"This is from a valid Vec object.";ValidNum:"self.len is valid.";Alias:"p is no longer used.";RustdocLinkToItem
                    )] /**# Safety

*/
                    ///This is from a valid Vec object.
                    #[doc = "* the memory range [,  + sizeof()*] must be fully initialized for type T\n\n"]
                    /**# Safety

*/
                    ///This is from a valid Vec object.
                    #[doc = "* the pointer  and its offset up to sizeof()* must point to a single allocated object\n\n"]
                    /**# Safety

*/
                    ///self.len is valid.
                    #[doc = "* the value of  must lie within the valid \n\n"]
                    /**# Safety

*/
                    ///p is no longer used.
                    #[doc = "*  must not have other alias\n\n"] /**# Safety

*/
                    #[doc = "* [``]\n\n"] a.get()
                },
            ),
        );
    };
}
