#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]
#![feature(vec_into_raw_parts)]
#![feature(register_tool)]
#![register_tool(rapx)]

use demo::MyStruct;
use safety_macro::safety;

fn main() {
    let (p, l, _c) = Vec::new().into_raw_parts();
    let a = MyStruct::from(p, l);
    println!("{:?}", unsafe {
        #[safety {
            Init: "This is from a valid Vec object.";
            InBound: "This is from a valid Vec object.";
            ValidNum: "self.len is valid.";
            Alias: "p is no longer used.";
            RustdocLinkToItem
        }]
        a.get()
    });
}
