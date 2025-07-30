use indexmap::IndexMap;

pub fn expr_to_string(expr: &syn::Expr) -> String {
    if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) = expr {
        s.value()
    } else {
        let tokens = quote::quote! { #expr };
        tokens.to_string()
    }
}

pub fn template(desc: &str, map: &IndexMap<&str, String>) -> String {
    let mut template = tinytemplate::TinyTemplate::new();
    template.add_template("", desc).unwrap();
    let mut doc = template.render("", map).unwrap();
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
