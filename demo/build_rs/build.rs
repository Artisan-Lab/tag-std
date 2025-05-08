use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::env;
use syn::{parse_file, File as SynFile, Attribute};
use syn::__private::ToTokens;

fn extract_doc_lines(attrs: &[Attribute]) -> Vec<String> {
    attrs.iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc") {
                let tokens = attr.into_token_stream().to_string(); // -> = "string"
                let doc_str = tokens
                    .trim_start_matches('=')
                    .trim()
                    .trim_matches('"');
                Some(doc_str.to_string())
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    let src = fs::read_to_string("src/lib.rs").expect("Failed to read src/lib.rs");
    let ast: SynFile = parse_file(&src).expect("Failed to parse file");

    let mut out = String::new();
    out.push_str("pub fn doc_for(name: &str) -> Option<&'static str> {\n");
    out.push_str("    match name {\n");

    for item in ast.items {
        match item {
            syn::Item::Fn(func) => {
                if func.sig.unsafety.is_some() {
                    let fn_name = func.sig.ident.to_string();
                    println!("cargo:warning=unsafe free fn: {}", fn_name);
                    let doc = extract_doc_lines(&func.attrs).join("\n").replace('"', "\\\"");
                    out.push_str(&format!("        \"{}\" => Some(\"{}\"),\n", fn_name, doc));
                }
            }
            syn::Item::Impl(item_impl) => {
                let type_name = if let syn::Type::Path(type_path) = *item_impl.self_ty.clone() {
                    type_path.path.segments.last().map(|s| s.ident.to_string())
                } else {
                    None
                };
                for impl_item in item_impl.items {
                    if let syn::ImplItem::Fn(method) = impl_item {
                       if method.sig.unsafety.is_some() {
                            let method_name = method.sig.ident.to_string();
                            if let Some(ref ty) = type_name {
                                let full_name = format!("{}::{}", ty, method_name);
                                println!("cargo:warning=unsafe method: {}", full_name);
                                let doc = extract_doc_lines(&method.attrs).join("\n").replace('"', "\\\"");
                                out.push_str(&format!("        \"{}\" => Some(\"{}\"),\n", full_name, doc));
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    out.push_str("        _ => None,\n    }\n}\n");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("doc_map.rs");
    let mut f = File::create(dest_path).unwrap();
    f.write_all(out.as_bytes()).unwrap();
}

