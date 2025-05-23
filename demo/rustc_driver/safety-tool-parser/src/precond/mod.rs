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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Kind {
    Precond,
    Hazard,
    Option,
}

#[derive(Debug)]
struct NamedArgsSet {
    set: IndexSet<NamedArg>,
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
