#![allow(unused_imports)]

use proc_macro::TokenStream; 
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, ItemFn, Meta, Lit, Expr, punctuated::Punctuated, token::Comma};

#[proc_macro_attribute]
pub fn contract(attr: TokenStream, item: TokenStream) -> TokenStream {
    let function = parse_macro_input!(item as ItemFn);
    let name = &function.sig.ident;
    let block = &function.block;
    let signature = &function.sig;
    let contract_content: String = attr.to_string(); 
    let output = quote! {
          #signature {
              println!("Contract: {}", #contract_content);
              println!("Function {} executed.", stringify!(#name));
              #block  
          }
    };
    output.into()
}

#[proc_macro_attribute]
pub fn extract_contract(attr: TokenStream, item: TokenStream) -> TokenStream {
    let function = parse_macro_input!(item as ItemFn);
    let docs: Vec<String> = function.attrs.iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc") {
                Some(attr.to_token_stream().to_string())
            } else {
                None
            }
    }).collect();
    let name = &function.sig.ident;
    let new_fn = format_ident!("{}_contract", name);
    let docs_output = if docs.is_empty() {
            quote! { println!("No RustDoc found for `{}`.", stringify!(#name)); }
        } else {
            quote! { println!("RustDoc for `{}`: {:?}", stringify!(#name), [#(#docs),*]); }
        };
    let output = quote! {
        #function
        pub fn #new_fn() {
            #docs_output
        }
    };
    output.into()
}

