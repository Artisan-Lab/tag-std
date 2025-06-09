use proc_macro2::TokenStream;
use quote::quote;

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
