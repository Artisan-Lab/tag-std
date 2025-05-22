use proc_macro::TokenStream;
use safety_tool_parser::{
    precond::{FnItem, SafetyAttrArgs},
    syn::*,
};

#[proc_macro_attribute]
pub fn precond(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate(attr, item)
}

#[proc_macro_attribute]
pub fn hazard(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate(attr, item)
}

#[proc_macro_attribute]
pub fn option(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate(attr, item)
}

fn generate(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);
    let attr = parse_macro_input!(attr as SafetyAttrArgs);

    let gen_code = attr.generate_code();

    let mut fn_item = FnItem::new(item);
    fn_item.insert_doc_string_to_the_back(gen_code);
    fn_item.into_token_stream().into()
}
