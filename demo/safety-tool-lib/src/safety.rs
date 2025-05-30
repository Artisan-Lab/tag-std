pub use safety_tool_macro::Memo;
pub use safety_tool_macro::discharges;

use safety_tool_macro::pub_use;
pub mod precond {
    super::pub_use! {
        Precond_Align,
        Precond_Size,
        Precond_NoPadding,
        Precond_NotNull,
        Precond_Allocated,
        Precond_InBound,
        Precond_NotOverlap,
        Precond_ValidNum,
        Precond_ValidString,
        Precond_ValidCStr,
        Precond_Init,
        Precond_Unwrap,
        Precond_Typed,
        Precond_Owning,
        Precond_Alias,
        Precond_Alive,
        Precond_Pinned,
        Precond_NotVolatile,
        Precond_Opened,
        Precond_Trait,
        Precond_Unreachable,
    }
}

pub mod hazard {
    super::pub_use! {
        Hazard_ValidString,
        Hazard_Init,
        Hazard_Alias,
        Hazard_Pinned,
    }
}

pub mod option {
    super::pub_use! {
        Option_Size,
        Option_Init,
        Option_Trait,
    }
}
