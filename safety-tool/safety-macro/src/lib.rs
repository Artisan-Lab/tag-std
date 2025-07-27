use proc_macro::TokenStream;
use safety_parser::{
    configuration::config_exists, proc_macro2::TokenStream as TokenStream2, quote::quote,
    safety::SafetyAttrArgs as AttrArgs, syn,
};

/// Tag SPs on an unsafe function item, or discharge SPs on an expression.
///
/// # Syntax Example
///
/// ```
/// #![feature(proc_macro_hygiene)]
/// #![feature(register_tool)]
/// #![register_tool(rapx)]
/// # use safety_macro::safety;
///
/// // Tag SPs:
/// #[safety { SP1 }] unsafe fn foo() {}
/// #[safety { SP1, SP2 }] unsafe fn bar() {}
///
/// // Discharge SPs:
/// #[safety { SP1 }] unsafe { foo() };
/// #[safety { SP1: "reason" }] unsafe { foo() };
/// #[safety { SP1, SP2: "shared reason" }] unsafe { bar() };
/// #[safety { SP1: "reason1"; SP2: "reason2" }] unsafe { bar() };
/// ```
#[proc_macro_attribute]
pub fn safety(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ts = TokenStream::new();

    // add registered tool attr
    let tool_attr = {
        let attr = TokenStream2::from(attr.clone());
        TokenStream::from(quote! { #[rapx::inner(#attr)] })
    };
    ts.extend(tool_attr);

    let attr_args: AttrArgs = syn::parse(attr).unwrap();

    if config_exists() {
        for tag in &attr_args.args {
            ts.extend(TokenStream::from(tag.gen_doc()));
        }
    }

    // add item or expression back
    ts.extend(item);
    ts
}
