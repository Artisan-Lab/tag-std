use super::*;

#[test]
fn precond_align() {
    let attr = r#"#[safety::precond(Align(self.ptr, T), memo = "reason")]"#;
    let safety_attr: SafetyAttr = parse_str(attr).unwrap();
    dbg!(&safety_attr);
}
