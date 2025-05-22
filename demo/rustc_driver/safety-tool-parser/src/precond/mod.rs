use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    *,
};

#[derive(Debug)]
pub struct SafetyAttr {
    pub attr: Attribute,
    pub args: SafetyAttrArgs,
}

impl Parse for SafetyAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut attrs = Attribute::parse_outer(input)?;
        let attr = attrs.remove(0);
        let args = attr.parse_args()?;
        Ok(SafetyAttr { attr, args })
    }
}

#[derive(Debug)]
pub struct SafetyAttrArgs {
    pub expr: Expr,
    pub comma: Option<Token![,]>,
    pub named: Punctuated<NamedArgs, Token![,]>,
}

impl Parse for SafetyAttrArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(SafetyAttrArgs {
            expr: input.parse()?,
            comma: input.parse()?,
            named: Punctuated::parse_separated_nonempty(input)?,
        })
    }
}

#[derive(Debug)]
pub struct NamedArgs {
    pub name: Ident,
    pub eq: Token![=],
    pub expr: Expr,
}

impl Parse for NamedArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(NamedArgs { name: input.parse()?, eq: input.parse()?, expr: input.parse()? })
    }
}

#[test]
fn precond_align() {
    let attr = r#"#[safety::precond(Align(self.ptr, T), memo = "reason")]"#;
    let safety_attr: SafetyAttr = parse_str(attr).unwrap();
    dbg!(&safety_attr);
}
