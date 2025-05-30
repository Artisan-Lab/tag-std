#![feature(proc_macro_hygiene)]
#![feature(vec_into_raw_parts)]
#![feature(stmt_expr_attributes)]

use demo::MyStruct;
use safety_tool_lib::safety;

fn main() {
    let (p, l, _c) = Vec::new().into_raw_parts();
    let a = MyStruct::from(p, l);
    println!(
        "{:?}",
        #[safety::discharges(Init(self.ptr, u8, self.len), memo = "a.ptr originates from a local vector, so it is not null")]
        #[safety::discharges(InBound(self.ptr, u8, self.len), memo = "samilar as Init")]
        #[safety::discharges(ValidNum(self.len*sizeof(u8), [0,isize::MAX]), memo = "why?" )]
        #[safety::discharges(Alias(self.ptr), memo = "The aliases of a.ptr are not used after this statement.")]
        unsafe {
            a.get()
        }
    );
}
