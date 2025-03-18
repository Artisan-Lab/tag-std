#![allow(unused_imports)]

use proc_macro::TokenStream; 
use quote::quote;
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

