use indexmap::IndexSet;
use proc_macro2::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    *,
};

mod utils;

mod keep_doc_order;
pub use keep_doc_order::FnItem;

#[cfg(test)]
mod tests;

//  ******************** Attribute Parsing ********************

#[derive(Debug)]
pub struct SafetyAttr {
    pub attr: Attribute,
    pub args: SafetyAttrArgs,
}

impl Parse for SafetyAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut attrs = Attribute::parse_outer(input)?;
        let attr = attrs.remove(0);
        let args = attr.parse_args()?;
        Ok(SafetyAttr { attr, args })
    }
}

type ListExprs = Punctuated<Expr, Token![,]>;

#[derive(Debug)]
pub struct SafetyAttrArgs {
    pub exprs: ListExprs,
}

impl Parse for SafetyAttrArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(SafetyAttrArgs { exprs: Punctuated::parse_terminated(input)? })
    }
}

// impl SafetyAttrArgs {
//     pub fn generate_doc_comments(&self) -> TokenStream {
//         NamedArgsSet::new(&self.named).generate_doc_comments()
//     }
//
//     pub fn generate_safety_tool_attribute(&self, kind: &str) -> TokenStream {
//         let Self { exprs, .. } = self;
//         quote! {
//             #[Safety::inner(kind = #kind, #exprs)]
//         }
//     }
// }

//  ******************** Attribute Analyzing ********************

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum NamedArg {
    Kind(String),
    Property(Property),
    Memo(String),
}

impl NamedArg {
    fn new(ident: &Ident, expr: &Expr) -> Self {
        if ident == "memo"
            && let Expr::Lit(lit) = expr
            && let Lit::Str(memo) = &lit.lit
        {
            return NamedArg::Memo(memo.value());
        }

        if ident == "kind"
            && let Expr::Lit(lit) = expr
            && let Lit::Str(kind) = &lit.lit
        {
            return NamedArg::Memo(kind.value());
        }

        panic!("{ident:?} is not a supported ident.\nCurrently supported named arguments: memo.")
    }

    /// Like generate rustdoc attributes to display doc comment in rustdoc HTML.
    fn generate_doc_comments(&self) -> TokenStream {
        match self {
            NamedArg::Memo(memo) => utils::memo(memo),
            _ => TokenStream::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Property {
    kind: Kind,
    expr: Expr,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Kind {
    Precond,
    Hazard,
    Option,
}

#[derive(Debug)]
struct NamedArgsSet {
    set: IndexSet<NamedArg>,
}

impl NamedArgsSet {
    fn new(args: SafetyAttrArgs, kind: Option<Kind>) -> Self {
        let exprs = args.exprs;
        let mut set = IndexSet::with_capacity(exprs.len());

        let mut non_named_exprs = Vec::new();

        // parse all named arguments
        parse_named_args(exprs, &mut set, &mut non_named_exprs);

        // parse positional arguments
        parse_positional_args(kind, &mut set, non_named_exprs);

        NamedArgsSet { set }
    }
}

fn parse_positional_args(
    kind: Option<Kind>,
    set: &mut IndexSet<NamedArg>,
    non_named_exprs: Vec<Expr>,
) {
    for (idx, expr) in non_named_exprs.into_iter().enumerate() {
        match idx {
            0 => {
                let kind = if let Some(kind) = kind {
                    kind
                } else if let Some(kind) = set.iter().find_map(|arg| {
                    if let NamedArg::Kind(kind) = arg { Some(kind.as_str()) } else { None }
                }) {
                    match kind {
                        "precond" => Kind::Precond,
                        "hazard" => Kind::Hazard,
                        "option" => Kind::Option,
                        _ => unreachable!(
                            "{kind} is invalid: should be one of precond, hazard, and option."
                        ),
                    }
                } else if let Some(property) = set.iter().find_map(|arg| {
                    if let NamedArg::Property(property) = arg { Some(property) } else { None }
                }) {
                    panic!("Only single property allowed. There is one: {property:?}.")
                } else {
                    unreachable!("No kind available.")
                };
                set.insert(NamedArg::Property(Property { kind, expr }));
            }
            1 => {
                if let Some(memo) = set.iter().find_map(|arg| {
                    if let NamedArg::Memo(memo) = arg { Some(memo.as_str()) } else { None }
                }) {
                    panic!("Only single memo allowed. There is one: {memo:?}.")
                } else if let Expr::Lit(lit) = &expr
                    && let Lit::Str(memo) = &lit.lit
                {
                    set.insert(NamedArg::Memo(memo.value()));
                } else {
                    panic!("{expr:?} is not string literal as a memo.")
                }
            }
            _ => (),
        }
    }
}

fn parse_named_args(
    exprs: Punctuated<Expr, token::Comma>,
    set: &mut IndexSet<NamedArg>,
    non_named_exprs: &mut Vec<Expr>,
) {
    for arg in exprs {
        match &arg {
            Expr::Assign(assign) => {
                let Expr::Path(path) = &*assign.left else {
                    panic!("{arg:?} is not normal assign expr.")
                };
                // ident = expr
                let ident = path.path.get_ident().unwrap();
                let first = set.insert(NamedArg::new(ident, &assign.right));
                assert!(!first, "{ident} exists.");
            }
            _ => non_named_exprs.push(arg),
        }
    }
}

// impl NamedArgsSet {
//     fn new(named_args: &ListNamedArgs) -> Self {
//         NamedArgsSet {
//             set: named_args.iter().map(|arg| NamedArg::new(&arg.name, &arg.expr)).collect(),
//         }
//     }
//
//     fn generate_doc_comments(&self) -> TokenStream {
//         self.set.iter().flat_map(NamedArg::generate_doc_comments).collect()
//     }
// }
