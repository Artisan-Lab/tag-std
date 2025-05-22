use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use std::collections::BTreeSet;
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

type ListNamedArgs = Punctuated<NamedArgs, Token![,]>;

#[derive(Debug)]
pub struct SafetyAttrArgs {
    pub expr: Expr,
    pub comma: Option<Token![,]>,
    pub named: ListNamedArgs,
}

impl Parse for SafetyAttrArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(SafetyAttrArgs {
            expr: input.parse()?,
            comma: input.parse()?,
            named: Punctuated::parse_separated_nonempty(input)?,
        })
    }
}

impl SafetyAttrArgs {
    pub fn generate_doc_comments(&self) -> TokenStream {
        NamedArgsSet::new(&self.named).generate_doc_comments()
    }

    pub fn generate_safety_tool_attribute(&self, kind: &str) -> TokenStream {
        let Self { expr, named, .. } = self;
        quote! {
            #[Safety::inner(#expr, kind = #kind, #named)]
        }
    }
}

#[derive(Debug)]
pub struct NamedArgs {
    pub name: Ident,
    pub eq: Token![=],
    pub expr: Expr,
}

impl Parse for NamedArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(NamedArgs { name: input.parse()?, eq: input.parse()?, expr: input.parse()? })
    }
}

impl ToTokens for NamedArgs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.name.to_tokens(tokens);
        self.eq.to_tokens(tokens);
        self.expr.to_tokens(tokens);
    }
}

//  ******************** Attribute Analyzing ********************

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NamedArg {
    Kind(String),
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

#[derive(Debug)]
struct NamedArgsSet {
    set: BTreeSet<NamedArg>,
}

impl NamedArgsSet {
    fn new(named_args: &ListNamedArgs) -> Self {
        NamedArgsSet {
            set: named_args.iter().map(|arg| NamedArg::new(&arg.name, &arg.expr)).collect(),
        }
    }

    fn generate_doc_comments(&self) -> TokenStream {
        self.set.iter().flat_map(NamedArg::generate_doc_comments).collect()
    }
}
