use proc_macro2::{Literal, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use syn::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Property {
    pub kind: Kind,
    pub name: PropertyName,
    pub expr: Expr,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    Precond,
    Hazard,
    Option,
}

impl ToTokens for Kind {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let kind = match self {
            Kind::Precond => "precond",
            Kind::Hazard => "hazard",
            Kind::Option => "option",
        };
        tokens.append(Literal::string(kind));
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PropertyName {
    Align,
    Size,
    NoPadding,
    NotNull,
    Allocated,
    InBound,
    NotOverlap,
    ValidNum,
    ValidString,
    ValidCStr,
    Init,
    Unwrap,
    Typed,
    Ownninig,
    Alias,
    Alive,
    Pinned,
    NotVolatile,
    Opened,
    Trait,
    UnReachable,
    // A placeholder for invalid or future-proof property
    Unknown,
}

impl PropertyName {
    pub fn new(s: &str) -> Self {
        match s {
            "Align" => Self::Align,
            "Size" => Self::Size,
            "NoPadding" => Self::NoPadding,
            "NotNull" => Self::NotNull,
            "Allocated" => Self::Allocated,
            "InBound" => Self::InBound,
            "NotOverlap" => Self::NotOverlap,
            "ValidNum" => Self::ValidNum,
            "ValidString" => Self::ValidString,
            "ValidCStr" => Self::ValidCStr,
            "Init" => Self::Init,
            "Unwrap" => Self::Unwrap,
            "Typed" => Self::Typed,
            "Ownninig" => Self::Ownninig,
            "Alias" => Self::Alias,
            "Alive" => Self::Alive,
            "Pinned" => Self::Pinned,
            "NotVolatile" => Self::NotVolatile,
            "Opened" => Self::Opened,
            "Trait" => Self::Trait,
            "UnReachable" => Self::UnReachable,
            _ => Self::Unknown,
        }
    }

    pub fn try_from_expr_ident(expr: &Expr) -> Option<Self> {
        let ident_str = super::expr_ident_opt(expr)?.to_string();
        Some(PropertyName::new(&ident_str))
    }

    pub fn from_expr_ident(expr: &Expr) -> Self {
        let ident_str = super::expr_ident(expr).to_string();
        PropertyName::new(&ident_str)
    }

    pub fn to_str(self) -> &'static str {
        match self {
            Self::Align => "Align",
            Self::Size => "Size",
            Self::NoPadding => "NoPadding",
            Self::NotNull => "NotNull",
            Self::Allocated => "Allocated",
            Self::InBound => "InBound",
            Self::NotOverlap => "NotOverlap",
            Self::ValidNum => "ValidNum",
            Self::ValidString => "ValidString",
            Self::ValidCStr => "ValidCStr",
            Self::Init => "Init",
            Self::Unwrap => "Unwrap",
            Self::Typed => "Typed",
            Self::Ownninig => "Ownninig",
            Self::Alias => "Alias",
            Self::Alive => "Alive",
            Self::Pinned => "Pinned",
            Self::NotVolatile => "NotVolatile",
            Self::Opened => "Opened",
            Self::Trait => "Trait",
            Self::UnReachable => "UnReachable",
            Self::Unknown => "Unknown",
        }
    }
}
