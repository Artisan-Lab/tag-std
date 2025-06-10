use super::{
    NamedArg,
    utils::{expr_ident, expr_ident_opt, expr_to_string, find_some},
};
use core::cmp::Ordering;
use proc_macro2::{Literal, Span, TokenStream};
use quote::{ToTokens, TokenStreamExt, quote};
use syn::{punctuated::Punctuated, *};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Property {
    pub kind: Kind,
    pub name: PropertyName,
    /// Should be a vec of args, not containing the name.
    pub expr: Vec<Expr>,
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
    pub fn new(kind: Kind, name: PropertyName, expr: Vec<Expr>, named_args: &[NamedArg]) -> Self {
        Property {
            kind,
            name,
            expr,
            // extract memo from named_args
            memo: find_some(named_args, |arg| {
                if let NamedArg::Memo(memo) = arg { Some(memo.clone()) } else { None }
            }),
        }
    }

    pub fn from_call(expr: &Expr) -> Self {
        let Expr::Call(call) = expr else { panic!("{expr:?} should be a call expr") };
        let name = expr_ident(&call.func).to_string();
        let name = PropertyName::new(&name);
        let args = call.args.iter().cloned().collect();
        // NOTE: kind = Memo is a temporary state
        Property { kind: Kind::Memo, name, expr: args, memo: None }
    }

    /// `PropertyName(arg1, arg2, ...)`
    pub fn property_tokens(&self) -> TokenStream {
        let name = Ident::new(&format!("{:?}", self.name), Span::call_site());
        let args: Punctuated<&Expr, Token![,]> = self.expr.iter().collect();
        quote! {
            #name (#args)
        }
    }

    pub fn generate_discharge_attr(&self) -> TokenStream {
        let mut args = Punctuated::<TokenStream, Token![,]>::new();
        let call = self.property_tokens();
        let kind = self.kind;
        args.extend([quote!(property = #call), quote!(kind = #kind)]);
        args.extend(self.memo.as_deref().map(|memo| quote!(memo = #memo)));
        quote! {
            #[rapx::inner(#args)]
        }
    }

    pub fn generate_doc_comments(&self) -> TokenStream {
        // auto doc from Property
        let auto = match self.kind {
            Kind::Memo => format!(" {}: auto doc placeholder.", expr_ident(&self.expr[0])),
            Kind::Precond => format!(
                " {:?}: Make sure {} before calling this function.",
                self.name,
                self.name.map_property_to_doc_comments(&self.expr)
            ),
            Kind::Hazard => format!(
                " {:?}: Make sure {} after calling this function.",
                self.name,
                self.name.map_property_to_doc_comments(&self.expr)
            ),
            Kind::Option => format!(
                " {:?}: To be noticed that, {}.",
                self.name,
                self.name.map_property_to_doc_comments(&self.expr)
            ),
        };
        let memo = self.memo.as_deref().map(super::utils::memo).unwrap_or_default();
        quote! {
            #[doc = #auto]
            #memo
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
            "memo" => Kind::Memo,
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
    NonNull,
    Allocated,
    InBound,
    NonOverlap,
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
    NonVolatile,
    Opened,
    Trait,
    Unreachable,
    ValidPtr,
    Deref,
    Ptr2Ref,
    Layout,
    // A placeholder for invalid or future-proof property
    Unknown,
}

impl PropertyName {
    pub fn new(s: &str) -> Self {
        match s {
            "Align" => Self::Align,
            "Size" => Self::Size,
            "NoPadding" => Self::NoPadding,
            "NonNull" => Self::NonNull,
            "Allocated" => Self::Allocated,
            "InBound" => Self::InBound,
            "NonOverlap" => Self::NonOverlap,
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
            "NonVolatile" => Self::NonVolatile,
            "Opened" => Self::Opened,
            "Trait" => Self::Trait,
            "Unreachable" => Self::Unreachable,
            "ValidPtr" => Self::ValidPtr,
            "Deref" => Self::Deref,
            "Ptr2Ref" => Self::Ptr2Ref,
            "Layout" => Self::Layout,
            _ => Self::Unknown,
        }
    }

    pub fn try_from_expr_ident(expr: &Expr) -> Option<Self> {
        let ident_str = expr_ident_opt(expr)?.to_string();
        Some(PropertyName::new(&ident_str))
    }

    pub fn from_expr_ident(expr: &Expr) -> Self {
        let ident_str = expr_ident(expr).to_string();
        PropertyName::new(&ident_str)
    }

    fn map_property_to_doc_comments(&self, expr: &[Expr]) -> String {
        let args: Vec<String> = expr.iter().map(expr_to_string).collect();
        if args.len() < self.args_len() {
            unreachable!("Arg length is invalid for {}", self.to_str())
        }
        match self {
            Self::Align => {
                format!("pointer `{}` must be properly aligned for type `{}`", args[0], args[1])
            }
            Self::Size => format!("the size of type {} should be {}", args[0], args[1]),
            Self::NoPadding => format!("type {} must have no padding bytes ", args[0]),
            Self::NonNull => format!("pointer {} must not be null", args[0]),
            Self::Allocated => format!(
                "the memory range [{}, {} + sizeof({})*{}) must be allocated by allocator {}",
                args[0], args[0], args[1], args[2], args[3]
            ),
            Self::InBound => format!(
                "the pointer {} and its offset up to sizeof({})*{} must point to a single allocated object",
                args[0], args[1], args[2]
            ),
            Self::NonOverlap => format!(
                "the memory ranges [{}, {} + sizeof({})*{}) and [{}, {} + sizeof({})*{}] must not overlap",
                args[0], args[0], args[2], args[3], args[1], args[1], args[2], args[3]
            ),
            Self::ValidNum => {
                format!("the value of {} must lie within the valid {}", args[0], args[1])
            }
            Self::ValidString => {
                format!("the memory range {} must contain valid UTF-8 bytes", args[0])
            }
            Self::ValidCStr => {
                format!(
                    "the memory range [{}, {} + {} + 1] must contain a valid C-style string",
                    args[0], args[0], args[1]
                )
            }
            Self::Init => {
                format!(
                    "the memory range [{}, {} + sizeof({})*{}] must be fully initialized for type T",
                    args[0], args[0], args[1], args[2]
                )
            }
            Self::Unwrap => format!("the value {} must be Some({})", args[0], args[1]),
            Self::Typed => {
                format!("the pointer {} must point to a value of {}", args[0], args[1])
            }
            Self::Owning => {
                format!("the pointer {} must hold exclusive ownership of its reference", args[0])
            }
            Self::Alias => {
                format!("{} must not have other alias", args[0])
            }
            Self::Alive => {
                format!("the reference of {} must outlive the lifetime {}", args[0], args[1])
            }
            Self::Pinned => {
                format!(
                    "pointer {} must remain at the same memory address for the duration of lifetime {}",
                    args[0], args[1]
                )
            }
            Self::NonVolatile => {
                format!(
                    "the memory access of [{}, {} + sizeof({})*{}] must be volatile",
                    args[0], args[0], args[1], args[2]
                )
            }
            Self::Opened => {
                format!("the file descriptor {} must be valid and open", args[0])
            }
            Self::Trait => {
                format!(
                    "if type {} implements trait {}, the property {} is mitigated",
                    args[0], args[1], args[2]
                )
            }
            Self::Unreachable => {
                "the current program point should not be reachable during execution".to_string()
            }
            Self::ValidPtr => {
                format!(
                    "pointer {} must be valid for reading and writing the sizeof({})*{} memory from it",
                    args[0], args[1], args[2]
                )
            }
            Self::Deref => {
                format!(
                    "pointer {} must be dereferencable in the sizeof({})*{} memory from it",
                    args[0], args[1], args[2]
                )
            }
            Self::Ptr2Ref => {
                format!("the reference conversion of the pointer {} must be valid", args[0])
            }
            Self::Layout => {
                format!("the memory pointed by {} must remain consistent with {}", args[0], args[1])
            }
            Self::Unknown => "unknown sp".to_string(),
        }
    }

    fn args_len(&self) -> usize {
        match self {
            Self::Align => 2,
            Self::Size => 2,
            Self::NoPadding => 1,
            Self::NonNull => 1,
            Self::Allocated => 3,
            Self::InBound => 3,
            Self::NonOverlap => 4,
            Self::ValidNum => 2,
            Self::ValidString => 1,
            Self::ValidCStr => 2,
            Self::Init => 3,
            Self::Unwrap => 2,
            Self::Typed => 2,
            Self::Owning => 1,
            Self::Alias => 1,
            Self::Alive => 2,
            Self::Pinned => 2,
            Self::NonVolatile => 3,
            Self::Opened => 1,
            Self::Trait => 3,
            Self::Unreachable => 0,
            Self::ValidPtr => 3,
            Self::Deref => 3,
            Self::Ptr2Ref => 2,
            Self::Layout => 2,
            Self::Unknown => 0, // Is it right?
        }
    }

    pub fn to_str(self) -> &'static str {
        match self {
            Self::Align => "Align",
            Self::Size => "Size",
            Self::NoPadding => "NoPadding",
            Self::NonNull => "NonNull",
            Self::Allocated => "Allocated",
            Self::InBound => "InBound",
            Self::NonOverlap => "NonOverlap",
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
            Self::NonVolatile => "NonVolatile",
            Self::Opened => "Opened",
            Self::Trait => "Trait",
            Self::Unreachable => "Unreachable",
            Self::ValidPtr => "ValidPtr",
            Self::Deref => "Deref",
            Self::Ptr2Ref => "Ptr2Ref",
            Self::Layout => "Layout",
            Self::Unknown => "Unknown",
        }
    }
}
