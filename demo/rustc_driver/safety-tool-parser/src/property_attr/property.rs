use core::cmp::Ordering;

use proc_macro2::{Literal, Span, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use syn::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Property {
    pub kind: Kind,
    pub name: PropertyName,
    /// Should be a fn call expr, containing the name.
    pub expr: Expr,
    /// User-provided desciption.
    pub memo: Option<String>,
}

impl PartialOrd for Property {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Property {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind.cmp(&other.kind) {
            Ordering::Equal => {}
            ord => return ord,
        }
        match self.name.cmp(&other.name) {
            Ordering::Equal => {}
            ord => return ord,
        }
        // Unable compare expr.
        Ordering::Equal
    }
}

impl Property {
    pub fn from_components(kind: Kind, name: PropertyName, expr: Vec<Expr>) -> Self {
        Property {
            kind,
            name,
            expr: Expr::Call(ExprCall {
                attrs: Vec::new(),
                func: Box::new(Expr::Path(ExprPath {
                    attrs: Vec::new(),
                    qself: None,
                    path: Ident::new(name.to_str(), Span::call_site()).into(),
                })),
                paren_token: Default::default(),
                args: expr.into_iter().collect(),
            }),
            // TODO: extract memo from asign expr?
            memo: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    Precond,
    Hazard,
    Option,
    Memo,
}

impl ToTokens for Kind {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let kind = match self {
            Kind::Precond => "precond",
            Kind::Hazard => "hazard",
            Kind::Option => "option",
            Kind::Memo => "memo",
        };
        tokens.append(Literal::string(kind));
    }
}

impl Kind {
    pub fn new(kind: &str) -> Self {
        match kind {
            "precond" => Kind::Precond,
            "hazard" => Kind::Hazard,
            "option" => Kind::Option,
            "memo" => Kind::Option,
            _ => unreachable!(
                "{kind} is invalid: should be one of \
                 precond, hazard, option, and memo."
            ),
        }
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
    Owning,
    Alias,
    Alive,
    Pinned,
    NotVolatile,
    Opened,
    Trait,
    Unreachable,
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
            "Owning" => Self::Owning,
            "Alias" => Self::Alias,
            "Alive" => Self::Alive,
            "Pinned" => Self::Pinned,
            "NotVolatile" => Self::NotVolatile,
            "Opened" => Self::Opened,
            "Trait" => Self::Trait,
            "Unreachable" => Self::Unreachable,
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
            Self::Owning => "Owning",
            Self::Alias => "Alias",
            Self::Alive => "Alive",
            Self::Pinned => "Pinned",
            Self::NotVolatile => "NotVolatile",
            Self::Opened => "Opened",
            Self::Trait => "Trait",
            Self::Unreachable => "Unreachable",
            Self::Unknown => "Unknown",
        }
    }
}
