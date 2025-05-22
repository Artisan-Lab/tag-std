use proc_macro::TokenStream;
use safety_tool_parser::{precond::SafetyAttrArgs, syn::*};

#[proc_macro_attribute]
pub fn safety(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as SafetyAttrArgs);
    let gen_code = attr.generate_code();
    gen_code.into_iter().map(TokenStream::from).chain([item]).collect()
}
