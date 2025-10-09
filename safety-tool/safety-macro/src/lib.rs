use proc_macro::TokenStream;
use safety_parser::{
    configuration::env::config_exists, proc_macro2::TokenStream as TokenStream2, quote::quote,
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
/// # use safety_macro::requires;
///
/// // Tag SPs:
/// #[requires { SP1 }] unsafe fn foo() {}
/// #[requires { SP1, SP2 }] unsafe fn bar() {}
/// ```
#[proc_macro_attribute]
pub fn requires(attr: TokenStream, item: TokenStream) -> TokenStream {
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

/// Discharge SPs.
///
/// NOTE: there is no check on whether the annotated is an expression or not.
///
/// # Syntax Example
///
/// ```
/// #![feature(stmt_expr_attributes)]
/// #![feature(proc_macro_hygiene)]
/// #![feature(register_tool)]
/// #![register_tool(rapx)]
/// # use safety_macro::{checked, requires};
///
/// // Tag SPs:
/// #[requires { SP1 }] unsafe fn foo() {}
/// #[requires { SP1, SP2 }] unsafe fn bar() {}
///
/// // Discharge SPs:
/// #[checked { SP1 }] unsafe { foo() };
/// #[checked { SP1: "reason" }] unsafe { foo() };
/// #[checked { SP1, SP2: "shared reason" }] unsafe { bar() };
/// #[checked { SP1: "reason1"; SP2: "reason2" }] unsafe { bar() };
/// ```
#[proc_macro_attribute]
pub fn checked(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ts = TokenStream::new();

    // Prepend the attribute above all attributes on the expression.
    let tool_attr: TokenStream = {
        // attr is all the arguments in #[check(args)]
        let attr = TokenStream2::from(attr.clone());
        quote! { #[rapx::checked(#attr)] }.into()
    };
    ts.extend(tool_attr);

    ts.extend(item);
    ts
}
