use super::Rust;

#[test]
fn plain1() {
    let src = "#[Allocated] fn f() {}";
    let mut rust = Rust::new();

    rust.update_node_tree(src.to_owned());
    let attrs = dbg!(rust.find_attrs());
    assert_eq!(attrs.len(), 1);
}

#[test]
fn method1() {
    let src = "impl S { #[Allocated] fn f() {} }";
    let mut rust = Rust::new();

    rust.update_node_tree(src.to_owned());
    let attrs = dbg!(rust.find_attrs());
    assert_eq!(attrs.len(), 1);
}

#[test]
fn method2() {
    let src = "impl S { #[Allocated] fn f() { #[Alias] fn g() {} } }";
    let mut rust = Rust::new();

    rust.update_node_tree(src.to_owned());
    let attrs = dbg!(rust.find_attrs());
    assert_eq!(attrs.len(), 2);
}

#[test]
fn mod2() {
    let src = "#[Allocated] fn f() {} mod a { #[Alias] fn g() {} fn a() {} }";
    let mut rust = Rust::new();

    rust.update_node_tree(src.to_owned());
    let attrs = dbg!(rust.find_attrs());
    assert_eq!(attrs.len(), 2);
}

#[test]
fn expr1() {
    let src = "mod a { #[Alias] fn g() {} fn a() { #[Alias] g() } }";
    let mut rust = Rust::new();

    rust.update_node_tree(src.to_owned());
    let attrs = dbg!(rust.find_attrs());
    assert_eq!(attrs.len(), 2);
}

#[test]
fn expr2() {
    let src = " mod a { #[Alias] fn g() {} fn a() {#[Alias] g()} }";
    let mut rust = Rust::new();

    rust.update_node_tree(src.to_owned());
    let attrs = dbg!(rust.find_attrs());
    assert_eq!(attrs.len(), 2);
}
