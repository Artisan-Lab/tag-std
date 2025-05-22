use proc_macro2::TokenStream;
use quote::quote;

/// memo = "reason" will generate `///reason`. which is a bit ugly.
/// So this function trim all whitespaces, and add single one for
/// each line.
pub fn memo(s: &str) -> Vec<TokenStream> {
    s.trim()
        .lines()
        .map(|line| {
            let doc = format!(" {}", line.trim());
            quote!(#[doc = #doc])
        })
        .collect()
}
