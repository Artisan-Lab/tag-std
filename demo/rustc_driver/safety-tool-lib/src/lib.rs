pub use safety_tool_parser;

pub mod safety {
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
}
