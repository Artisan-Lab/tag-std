use super::PropertiesAndReason;
use crate::configuration::env::config_exists;
use indexmap::IndexMap;
use syn::{Expr, ExprLit, Lit};

pub fn expr_to_string(expr: &Expr) -> String {
    if let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = expr {
        s.value()
    } else {
        let tokens = quote::quote! { #expr };
        tokens.to_string()
    }
}

/// Each expr must be in the form of `SP(expr)`. Return `(SP string, &Tag)`.
pub fn parse_args_in_any_tag(args: &[Expr]) -> Vec<PropertiesAndReason> {
    let need_check = config_exists();
    let mut v_sp = Vec::with_capacity(args.len());
    for expr in args {
        let prop: PropertiesAndReason = syn::parse_quote!(#expr);
        if need_check {
            prop.tags.iter().for_each(|t| t.tag.check_type());
        }
        v_sp.push(prop);
    }
    v_sp
}

pub fn template(desc: &str, map: &IndexMap<&str, String>) -> String {
    let mut template = tinytemplate::TinyTemplate::new();
    template.add_template("", desc).unwrap();
    let mut doc = template.render("", map).unwrap();
    doc.push('\n'); // add extra newline
    doc.push('\n'); // add extra newline
    doc
}

#[test]
fn string_interpolation() {
    #[derive(serde::Serialize)]
    struct Val {
        a: u8,
        b: &'static str,
    }
    let s = "{a}, {b}";
    let mut template = tinytemplate::TinyTemplate::new();
    template.add_template("", s).unwrap();
    println!("rendered: {}", template.render("", &Val { a: 123, b: "hi" }).unwrap());
}
