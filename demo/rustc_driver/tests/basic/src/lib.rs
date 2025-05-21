#![feature(register_tool)]
#![register_tool(safety)]

#[safety::precond(!Reachable())]
pub unsafe fn test() {
    println!("unreachable!");
}

pub struct MyStruct {
    ptr: *mut u8,
    len: usize,
}
impl MyStruct {
    pub fn from(p: *mut u8, l: usize) -> MyStruct {
        MyStruct { ptr: p, len: l }
    }
    #[safety::precond::Init(self.ptr, u8, self.len)]
    #[safety::precond::InBound(self.ptr, u8, self.len)]
    #[safety::precond::ValidNum(self.len*sizeof(u8), [0,isize::MAX])]
    #[safety::hazard::Alias(self.ptr)]
    pub unsafe fn get(&self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}
