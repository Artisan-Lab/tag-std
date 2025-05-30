#![feature(proc_macro_hygiene)]
#![feature(vec_into_raw_parts)]
#![allow(unused_variables)]

use demo::MyStruct;
use safety_tool_lib::safety;

fn main() {
    let (p, l, _c) = Vec::new().into_raw_parts();
    #[safety::discharge(Memo(UserProperty))]
    let a = MyStruct::from(p, l);
    println!("{:?}", unsafe { a.get() });
}
