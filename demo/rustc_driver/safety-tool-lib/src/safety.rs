pub use safety_tool_macro::{hazard, option, precond};

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
        Precond_Ownninig,
        Precond_Alias,
        Precond_Alive,
        Precond_Pinned,
        Precond_NotVolatile,
        Precond_Opened,
        Precond_Trait,
        Precond_UnReachable,
    }
}

pub mod hazard {
    super::pub_use! {
        Hazard_Align,
        Hazard_Size,
        Hazard_NoPadding,
        Hazard_NotNull,
        Hazard_Allocated,
        Hazard_InBound,
        Hazard_NotOverlap,
        Hazard_ValidNum,
        Hazard_ValidString,
        Hazard_ValidCStr,
        Hazard_Init,
        Hazard_Unwrap,
        Hazard_Typed,
        Hazard_Ownninig,
        Hazard_Alias,
        Hazard_Alive,
        Hazard_Pinned,
        Hazard_NotVolatile,
        Hazard_Opened,
        Hazard_Trait,
        Hazard_UnReachable,
    }
}

pub mod option {
    super::pub_use! {
        Option_Align,
        Option_Size,
        Option_NoPadding,
        Option_NotNull,
        Option_Allocated,
        Option_InBound,
        Option_NotOverlap,
        Option_ValidNum,
        Option_ValidString,
        Option_ValidCStr,
        Option_Init,
        Option_Unwrap,
        Option_Typed,
        Option_Ownninig,
        Option_Alias,
        Option_Alive,
        Option_Pinned,
        Option_NotVolatile,
        Option_Opened,
        Option_Trait,
        Option_UnReachable,
    }
}
