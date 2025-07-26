use super::*;

#[test]
fn parse_safety_attr() {
    let parse_attr = |s: &str| parse_str::<SafetyAttr>(s);

    // empty SP
    let _: SafetyAttr = parse_attr("#[safety {}]").unwrap();

    // simplest SP
    let attr = parse_attr("#[safety { SP }]").unwrap();
    let sp = attr.args.property_reason().next().unwrap().0;
    assert_eq!(sp.tag.name_type(), ["SP", DEFAULT_TYPE]);

    // SP with path prefix and arguments
    let attr = parse_attr("#[safety { hazard.Alias(p, q) }]").unwrap();
    let sp = attr.args.property_reason().next().unwrap().0;
    assert_eq!(sp.tag.name_type(), ["Alias", "hazard"]);
    let args: Vec<_> = sp
        .args
        .iter()
        .map(|arg| {
            if let Expr::Path(path) = arg {
                path.path.get_ident().unwrap().to_string()
            } else {
                unreachable!()
            }
        })
        .collect();
    assert_eq!(args, ["p", "q"]);
}

fn parse_args(s: &str) -> syn::Result<SafetyAttrArgs> {
    parse_str::<SafetyAttrArgs>(s)
}

#[test]
fn parse_safety_args() {
    // vanilla SP
    _ = parse_args("SP").unwrap();
    _ = parse_args("SP1, SP2").unwrap();
    _ = parse_args("SP1; SP2").unwrap();

    // SP with reason
    _ = parse_args(r#" SP1: "reason" "#).unwrap();
    _ = parse_args(r#" SP1: "reason"; SP2: "reason" "#).unwrap();
    _ = parse_args(r#" SP1: "reason", SP2: "reason" "#).unwrap_err();

    // grouped SPs
    _ = parse_args(r#" SP1, SP2: "reason" "#).unwrap();
    _ = parse_args(r#" SP1, SP2: "reason"; SP3 "#).unwrap();
    _ = parse_args(r#" SP3; SP1, SP2: "reason" "#).unwrap();
    _ = parse_args(r#" SP3, SP4; SP1, SP2: "reason" "#).unwrap();
    _ = parse_args(r#" SP3; SP1, SP2: "reason"; SP4 "#).unwrap();

    // trailing punct
    _ = parse_args(r#" SP1, SP2: "reason"; SP3; "#).unwrap();
    _ = parse_args(r#" SP1, SP2: "reason"; SP3, "#).unwrap();

    // arguments in single SP
    _ = parse_args(r#" SP1(a) "#).unwrap();
    _ = parse_args(r#" SP1(a, b) "#).unwrap();
    _ = parse_args(r#" SP1(a, b, call()) "#).unwrap();

    // arguments with other SPs
    _ = parse_args(r#" SP1(a), SP2: "reason"; SP3 "#).unwrap();
    _ = parse_args(r#" SP(a, b): "reason"; SP1, SP2: "reason"; SP3, SP4 "#).unwrap();
}

#[test]
fn parse_safety_complex_args() {
    // SP path prefix
    _ = parse_args(r#" hazard.Alias(p, q) "#).unwrap();

    // complex expressions in arguments
    _ = parse_args(r#" hazard.Alias(A {a: self.a}, a::b(c![])) : "" "#).unwrap();
}
