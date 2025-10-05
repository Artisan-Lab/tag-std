#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]
#![feature(vec_into_raw_parts)]
#![feature(register_tool)]
#![register_tool(rapx)]

use demo::MyStruct;
use safety_macro::safety;

fn main() {
    let (p, l, _c) = Vec::new().into_raw_parts();
    dbg!(p, l);
    let a = MyStruct::from(p, l);
    #[safety {
        NonNull(p): "Vec::new generate a dangling pointer, but it's not null";
        ValidPtr(p, u8, l): "l is zero in this case, and zero size access is always valid";
        Init(p, u8, l): "no element yet, so no need to initialize anything";
        Alive(p, l): "there is no real data, so this is met";
        Alias(p): "p is no longer used other than in MyStruct";
        Align(p, u8): "Vec makes an aligned pointer";
        ValidNum(l, [0, isize::MAX]): "l is zero here, thus in range"
    }]
    let val = unsafe { a.get() };
    println!("{val:?}");
}
