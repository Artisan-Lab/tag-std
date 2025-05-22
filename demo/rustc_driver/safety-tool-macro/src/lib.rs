use proc_macro::TokenStream;
use safety_tool_parser::{precond::SafetyAttrArgs, syn::*};

#[proc_macro_attribute]
pub fn safety(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as SafetyAttrArgs);
    dbg!(&attr);
    item
}
