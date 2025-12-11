#![feature(stmt_expr_attributes)]
#![feature(register_tool)]
#![register_tool(rapx)]
use std::ptr::NonNull;

pub unsafe fn init(base_register_vaddr: NonNull<u8>) {
    // let _f = || unsafe { FaultEventRegisters::new(base_register_vaddr) };
    unsafe { FaultEventRegisters::new(base_register_vaddr) }
}

struct FaultEventRegisters {}

impl FaultEventRegisters {
    #[rapx::requires(
        ValidBaseAddr(base_register_vaddr, hardware = "IOMMU"),
        OwnedResource(base_register_vaddr, owner = FaultEventRegisters)
    )]
    pub unsafe fn new(base_register_vaddr: NonNull<u8>) {
        _ = base_register_vaddr;
    }
}
