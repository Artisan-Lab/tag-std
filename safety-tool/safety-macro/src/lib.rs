use proc_macro::TokenStream;
use safety_parser::{
    configuration::config_exists, proc_macro2::TokenStream as TokenStream2, quote::quote,
    safety::SafetyAttrArgs as AttrArgs, split_attrs::split_attrs_and_rest, syn,
};

/// Tag SPs on an unsafe function item, or discharge SPs on an expression.
///
/// # Syntax Example
///
/// ```
/// #![feature(stmt_expr_attributes)]
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
    let mut ts = TokenStream2::new();

    // add registered tool attr
    let tool_attr = {
        let attr = TokenStream2::from(attr.clone());
        quote! { #[rapx::proof(#attr)] }
    };
    ts.extend(tool_attr);

    let input = split_attrs_and_rest(item.into());
    if !input.gen_doc {
        // no need to generate docs on expressions
        ts.extend(input.attrs);
        ts.extend(input.rest);
        return ts.into();
    }

    // push doc attrs first
    ts.extend(input.attrs);

    let attr_args: AttrArgs = syn::parse(attr).unwrap();
    // push generated doc if available
    if config_exists() {
        for tag in &attr_args.args {
            ts.extend(tag.gen_doc());
        }
    }

    // push rest tokens
    ts.extend(input.rest);
    ts.into()
}
