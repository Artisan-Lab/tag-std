use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, Ident};

/// memo = "reason" will generate `///reason`. which is a bit ugly.
/// So this function trim all whitespaces, and add single one for
/// each line.
pub fn memo(s: &str) -> TokenStream {
    s.trim()
        .lines()
        .map(|line| {
            let doc = format!(" {}", line.trim());
            quote!(#[doc = #doc])
        })
        .collect()
}

pub fn find<T, U>(v: &[T], f: impl FnMut(&T) -> Option<U>, or_else: impl FnOnce() -> U) -> U {
    v.iter().find_map(f).unwrap_or_else(or_else)
}

pub fn find_some<T, U>(v: &[T], f: impl FnMut(&T) -> Option<U>) -> Option<U> {
    v.iter().find_map(f)
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
pub fn expr_ident_opt(expr: &Expr) -> Option<Ident> {
    let Expr::Path(path) = expr else { return None };
    path.path.get_ident().cloned()
}

pub fn expr_to_string(expr: &Expr) -> String {
    let tokens = quote! { #expr };
    tokens.to_string()
}
