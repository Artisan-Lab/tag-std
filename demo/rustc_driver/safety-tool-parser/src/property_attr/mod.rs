use indexmap::IndexSet;
use proc_macro2::TokenStream;
use property::{Kind, Property, PropertyName};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    *,
};

mod utils;

mod keep_doc_order;
pub use keep_doc_order::FnItem;

pub mod property;

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

impl SafetyAttrArgs {
    pub fn into_named_args_set(self, kind: Kind, property: PropertyName) -> NamedArgsSet {
        NamedArgsSet::new_kind_and_property(self, kind, property)
    }
}

/// Single arguement component in a safety attribute.
///
/// Currently, these forms are supported:
/// * `#[Property(args)]` from a kind -> user-faced syntax
/// * `Safety::inner(property = Property, kind = kind, memo = ".")` -> only for internal use
//
// where `kind = {precond, hazard, option}`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NamedArg {
    /// A safety property with kind, name, and expression.
    Property(Box<Property>),
    /// A kind among precond, hazard, and option.
    Kind(String),
    /// An optional user description.
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
            return NamedArg::Kind(kind.value());
        }
        panic!("{ident:?} is not a supported ident.\nCurrently supported named arguments: memo.")
    }

    /// Like generate rustdoc attributes to display doc comment in rustdoc HTML.
    fn generate_doc_comments(&self) -> TokenStream {
        match self {
            NamedArg::Property(property) => property.generate_doc_comments(),
            _ => TokenStream::new(),
        }
    }
}

#[derive(Debug)]
pub struct NamedArgsSet {
    pub set: IndexSet<NamedArg>,
}

impl NamedArgsSet {
    // `#[kind::Property(..., memo = "...")]`
    //
    // * `kind = {precond, hazard, option}`
    // * memo is optional
    // * Property: The first positional arguement is the whole Property.
    fn new_kind_and_property(args: SafetyAttrArgs, kind: Kind, pname: PropertyName) -> Self {
        let exprs = args.exprs;
        let mut set = IndexSet::with_capacity(exprs.len());

        let mut non_named_exprs = Vec::new();

        // parse all named arguments such as memo
        parse_named_args(exprs, &mut set, &mut non_named_exprs);

        // positional arguments are collected into a tuple expr
        let property = Property::new(kind, pname, non_named_exprs, &set);
        let first = set.insert(NamedArg::Property(Box::new(property)));
        assert!(first, "{kind:?} {pname:?} exists.");

        set.sort();
        NamedArgsSet { set }
    }

    pub fn generate_doc_comments(&self) -> TokenStream {
        self.set.iter().flat_map(NamedArg::generate_doc_comments).collect()
    }

    pub fn generate_safety_tool_attribute(&self) -> TokenStream {
        let mut args = Punctuated::<TokenStream, Token![,]>::new();
        for arg in &self.set {
            match arg {
                NamedArg::Property(property) => {
                    let call = property.property_tokens();
                    let kind = property.kind;
                    args.extend([quote!(property = #call), quote!(kind = #kind)]);
                }
                NamedArg::Memo(memo) => args.extend([quote!(memo = #memo)]),
                _ => (),
            }
        }
        quote! {
            #[Safety::inner(#args)]
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
                // ident = expr
                let ident = &expr_ident(&assign.left);
                let first = set.insert(NamedArg::new(ident, &assign.right));
                assert!(first, "{ident} exists.");
            }
            _ => non_named_exprs.push(arg),
        }
    }
}

/// Parse expr as single ident.
///
/// Panic if expr is not Path or a path with multiple segments.
fn expr_ident(expr: &Expr) -> Ident {
    let Expr::Path(path) = expr else { panic!("{expr:?} is not path expr.") };
    path.path.get_ident().unwrap().clone()
}

/// Parse expr as single ident.
///
/// Panic if expr is not Path or a path with multiple segments.
fn expr_ident_opt(expr: &Expr) -> Option<Ident> {
    let Expr::Path(path) = expr else { return None };
    path.path.get_ident().cloned()
}
