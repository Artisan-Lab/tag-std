use proc_macro2::TokenStream;
use property::{Kind, Property, PropertyName};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Parser},
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
    pub fn into_named_args_set(self, kind: Kind, property: PropertyName) -> NamedArgs {
        NamedArgs::new_kind_and_property(self, kind, property)
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

        if ident == "property" {
            let property = Property::from_call(expr);
            return NamedArg::Property(Box::new(property));
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

pub fn parse_inner_attr_from_str(s: &str) -> Option<Property> {
    let mut attrs = Attribute::parse_outer.parse_str(s).unwrap();
    assert!(attrs.len() < 2, "{s:?} shouldn't be parsed into multiple attributes.");
    let attr = attrs.pop()?;

    let args: SafetyAttrArgs = attr.parse_args().unwrap();
    let exprs = args.exprs;
    let mut named = Vec::with_capacity(exprs.len());
    let mut non_named_exprs = Vec::new();

    // parse all named arguments such as memo, but discard all positional args.
    parse_named_args(exprs, &mut named, &mut non_named_exprs);

    let mut property = named
        .iter()
        .find_map(|arg| {
            if let NamedArg::Property(property) = arg { Some(property.clone()) } else { None }
        })
        .unwrap_or_else(|| panic!("No property in {named:?}"));
    property.kind = named
        .iter()
        .find_map(|arg| if let NamedArg::Kind(kind) = arg { Some(Kind::new(kind)) } else { None })
        .unwrap_or_else(|| panic!("No kind in {named:?}"));
    property.memo = named
        .iter()
        .find_map(|arg| if let NamedArg::Memo(memo) = arg { Some(memo.clone()) } else { None });

    Some(*property)
}

pub fn parse_inner_attr_from_tokenstream(ts: TokenStream) -> Property {
    let v_expr = Punctuated::<Expr, Token![,]>::parse_separated_nonempty.parse2(ts).unwrap();
    let Expr::Call(call) = v_expr[0].clone() else {
        panic!("The first expr is not a call expr in {v_expr:?} ");
    };

    let mut named = Vec::with_capacity(call.args.len());

    let mut non_named_exprs = Vec::new();

    // parse all named arguments such as memo
    parse_named_args(call.args, &mut named, &mut non_named_exprs);

    let name = expr_ident(&call.func).to_string();
    if name != "Memo" {
        panic!("Only support `Memo` property, but got {name:?}");
    }
    let name = PropertyName::new(&name);
    Property::new(Kind::Memo, name, non_named_exprs, &named)
}

#[derive(Debug)]
pub struct NamedArgs {
    // NOTE: this field hasn't deduplicated values yet, and search
    // arg by finding the first occurence.
    pub named: Vec<NamedArg>,
}

impl NamedArgs {
    // `#[kind::Property(..., memo = "...")]`
    //
    // * `kind = {precond, hazard, option}`
    // * memo is optional
    // * Property: The first positional arguement is the whole Property.
    fn new_kind_and_property(args: SafetyAttrArgs, kind: Kind, pname: PropertyName) -> Self {
        let exprs = args.exprs;
        let mut named = Vec::with_capacity(exprs.len());

        let mut non_named_exprs = Vec::new();

        // parse all named arguments such as memo
        parse_named_args(exprs, &mut named, &mut non_named_exprs);

        // positional arguments are collected into a tuple expr
        let property = Property::new(kind, pname, non_named_exprs, &named);
        named.push(NamedArg::Property(Box::new(property)));

        named.sort();
        NamedArgs { named }
    }

    pub fn generate_doc_comments(&self) -> TokenStream {
        self.named.iter().flat_map(NamedArg::generate_doc_comments).collect()
    }

    pub fn generate_safety_tool_attribute(&self) -> TokenStream {
        let mut args = Punctuated::<TokenStream, Token![,]>::new();
        for arg in &self.named {
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
            #[rapx::inner(#args)]
        }
    }
}

fn parse_named_args(
    exprs: Punctuated<Expr, token::Comma>,
    set: &mut Vec<NamedArg>,
    non_named_exprs: &mut Vec<Expr>,
) {
    for arg in exprs {
        match &arg {
            Expr::Assign(assign) => {
                // ident = expr
                let ident = &expr_ident(&assign.left);
                set.push(NamedArg::new(ident, &assign.right));
            }
            _ => non_named_exprs.push(arg),
        }
    }
}

/// Parse expr as single ident.
///
/// Panic if expr is not Path or a path with multiple segments.
pub fn expr_ident(expr: &Expr) -> Ident {
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

fn expr_to_string(expr: &Expr) -> String {
    let tokens = quote! { #expr };
    tokens.to_string()
}
