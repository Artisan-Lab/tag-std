#![feature(vec_into_raw_parts)]
#[allow(unused_variables)]

use demo::MyStruct;

fn main() {
    let (p, l, _c) = Vec::new().into_raw_parts();
    let a = MyStruct::from(p, l);
    println!("{:?}", unsafe { 
        a.get() 
    });
}
