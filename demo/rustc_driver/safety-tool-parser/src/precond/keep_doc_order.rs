//! `#[safety]` attribute will generate safety doc (`#[doc]`), which will bring interactions
//! with `#[doc]` from user.
//!
//! Consider the following doc strings:
//!
//! ```ignore
//! /// Beginning
//! #[safety(memo = "safety doc")]
//! /// Tailling
//! ```
//!
//! It'd be pretty bad if the order is messed up.
//!
//! Thus this module should solve this problem by ensuring the doc attribute
//! emission is inserted to the tail. Because attribute macros are handled
//! from top to buttom, this hopefully keeps doc orders.

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse::Parser, *};

pub struct FnItem {
    pub fn_: ItemFn,
}

impl FnItem {
    pub fn new(fn_: ItemFn) -> Self {
        FnItem { fn_ }
    }

    pub fn insert_doc_string_to_the_back(&mut self, tokens: Vec<TokenStream>) {
        // Double check the given tokens are attributes.
        let tokens: TokenStream = tokens.into_iter().collect();
        let attrs = Attribute::parse_outer.parse2(tokens).unwrap();
        self.fn_.attrs.extend(attrs);
    }

    pub fn into_token_stream(self) -> TokenStream {
        self.fn_.to_token_stream()
    }
}
