use proc_macro::TokenStream;
use safety_tool_parser::{
    precond::{FnItem, Kind, SafetyAttrArgs},
    syn::*,
};

#[proc_macro_attribute]
pub fn precond(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate(Kind::Precond, attr, item)
}

#[proc_macro_attribute]
pub fn hazard(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate(Kind::Hazard, attr, item)
}

#[proc_macro_attribute]
pub fn option(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate(Kind::Option, attr, item)
}

fn generate(kind: Kind, attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);
    let attr = parse_macro_input!(attr as SafetyAttrArgs);

    let safety_tool_attr = attr.generate_safety_tool_attribute(kind);
    let named_args_set = attr.into_named_args_set(Some(kind));
    let doc_comments = named_args_set.generate_doc_comments();

    let mut fn_item = FnItem::new(item);
    fn_item.insert_attributes_to_the_back(doc_comments);
    fn_item.insert_attributes_to_the_back(safety_tool_attr);
    fn_item.into_token_stream().into()
}
