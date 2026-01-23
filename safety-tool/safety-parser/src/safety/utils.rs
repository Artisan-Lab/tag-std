use super::PropertiesAndReason;
use crate::configuration::env::need_check;
use indexmap::IndexMap;
use proc_macro2::Span;
use serde::{Deserializer, Serializer, ser::SerializeSeq};
use syn::{
    AttrStyle, Attribute, Expr, ExprLit, File, Ident, Lit, MetaNameValue, Path, PathSegment,
    punctuated::Punctuated,
};

pub fn expr_to_string(expr: &Expr) -> String {
    if let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = expr {
        s.value()
    } else {
        // Workaroud: we can use `quote!` to format expr as string, but the result is weird
        // such as `self.field` being `self . field`
        let (pound_token, bracket_token, inner, eq_token) = Default::default();
        let attr = Attribute {
            pound_token,
            bracket_token,
            style: AttrStyle::Inner(inner),
            meta: MetaNameValue {
                path: Path {
                    leading_colon: None,
                    segments: Punctuated::from_iter([PathSegment::from(Ident::new(
                        "arg",
                        Span::call_site(),
                    ))]),
                },
                eq_token,
                value: expr.clone(),
            }
            .into(),
        };
        let file = File { shebang: None, attrs: vec![attr], items: Vec::new() };
        let s = prettyplease::unparse(&file);
        let s = s.strip_prefix("#![arg = ").unwrap();
        let s = s.strip_suffix("]").unwrap();
        s.to_owned()
    }
}

/// Serialize Expr as string in JSON.
pub fn serialize_expr_to_str<S: Serializer>(
    v_expr: &[Expr],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut seq = serializer.serialize_seq(None)?;
    for expr in v_expr {
        let string = expr_to_string(expr);
        seq.serialize_element(&string)?;
    }
    seq.end()
}

/// Deserialize string back to Expr from JSON.
pub fn deserialize_str_to_expr<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Box<[Expr]>, D::Error> {
    let v = <Vec<String> as serde::Deserialize>::deserialize(deserializer)?;
    Ok(v.iter()
        .map(|string| {
            syn::parse_str(string)
                .unwrap_or_else(|err| panic!("Failed to parse Expression from `{string}`:\n{err}"))
        })
        .collect())
}

/// Each expr must be in the form of `SP(expr)`. Return `(SP string, &Tag)`.
pub fn parse_args_in_any_tag(args: &[Expr]) -> Vec<PropertiesAndReason> {
    let need_check = need_check();
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

#[test]
#[should_panic]
fn failed_to_find_value_c_from_path_c() {
    #[derive(serde::Serialize)]
    struct Val {
        a: u8,
        b: &'static str,
    }
    let s = "{a}, {b}, {c}";
    let mut template = tinytemplate::TinyTemplate::new();
    template.add_template("", s).unwrap();
    println!("rendered: {}", template.render("", &Val { a: 123, b: "hi" }).unwrap());
}
