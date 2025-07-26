use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Paren,
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
        assert!(attrs.len() == 1, "#[safety] must be single");
        let attr = attrs.remove(0);
        drop(attrs);

        let ident = attr.path().get_ident().unwrap();
        assert_eq!(ident, "safety", "should pass a #[safety] macro here");
        let args = attr.parse_args()?;
        Ok(SafetyAttr { attr, args })
    }
}

#[derive(Debug)]
pub struct SafetyAttrArgs {
    pub args: Punctuated<PropertiesAndReason, Token![;]>,
}
impl Parse for SafetyAttrArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(SafetyAttrArgs { args: Punctuated::parse_terminated(input)? })
    }
}

impl SafetyAttrArgs {
    pub fn property_reason(&self) -> impl Iterator<Item = (&Property, Option<&LitStr>)> {
        self.args.iter().flat_map(|arg| arg.tags.iter().map(|prop| (prop, arg.desc.as_ref())))
    }
}

#[derive(Debug)]
pub struct PropertiesAndReason {
    pub tags: Vec<Property>,
    pub desc: Option<LitStr>,
}

impl Parse for PropertiesAndReason {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut tags = Vec::<Property>::new();
        let mut desc = None;

        while !input.cursor().eof() {
            let tag: TagNameType = input.parse()?;
            let sp = if input.peek(Paren) {
                let content;
                parenthesized!(content in input);
                let args = Punctuated::parse_terminated(&content)?;
                Property { tag, args }
            } else {
                Property { tag, args: Default::default() }
            };
            tags.push(sp);

            if input.peek(Token![,]) {
                // consume `,` in multiple tags
                let _: Token![,] = input.parse()?;
            }
            if input.peek(Token![:]) {
                let _: Token![:] = input.parse()?;
                // `:` isn't in args, thus parse desc
                desc = Some(input.parse()?);
                break;
            }
            if input.peek(Token![;]) {
                // new grouped SPs
                break;
            }
        }
        Ok(PropertiesAndReason { tags, desc })
    }
}

#[derive(Debug)]
pub struct Property {
    /// `SP` or `type::SP`. Single `SP` means `precond::SP`.
    pub tag: TagNameType,
    /// Args in `SP(args)` such as `arg1, arg2`.
    pub args: Punctuated<Expr, Token![,]>,
}

impl Parse for Property {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Property {
            tag: input.parse()?,
            args: {
                if input.peek(Paren) {
                    let content;
                    _ = parenthesized!(content in input);
                    Punctuated::parse_terminated(&content)?
                } else {
                    Punctuated::default()
                }
            },
        })
    }
}

#[derive(Debug)]
pub enum TagNameType {
    /// Single ident SP, default to `precond` type.
    SP(Ident),
    /// Typed SP: `type.SP`
    TypeSP { typ: Ident, dot: Token![.], sp: Ident },
}

impl Parse for TagNameType {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;
        Ok(if input.peek(Token![.]) {
            TagNameType::TypeSP { typ: ident, dot: input.parse()?, sp: input.parse()? }
        } else {
            TagNameType::SP(ident)
        })
    }
}

impl TagNameType {
    pub fn name(&self) -> String {
        match self {
            TagNameType::SP(sp) => sp,
            TagNameType::TypeSP { sp, .. } => sp,
        }
        .to_string()
    }

    pub const DEFAULT_TYPE: &str = "precond";

    pub fn typ(&self) -> String {
        match self {
            TagNameType::SP(_) => Self::DEFAULT_TYPE.to_string(),
            TagNameType::TypeSP { typ, .. } => typ.to_string(),
        }
    }

    pub fn name_type(&self) -> [String; 2] {
        match self {
            TagNameType::SP(sp) => [sp.to_string(), Self::DEFAULT_TYPE.to_string()],
            TagNameType::TypeSP { typ, sp, .. } => [sp.to_string(), typ.to_string()],
        }
    }
}

#[test]
fn parse_safety_attr() {
    let parse_attr = |s: &str| parse_str::<SafetyAttr>(s);

    // empty SP
    let _: SafetyAttr = parse_attr("#[safety {}]").unwrap();

    // simplest SP
    let attr = parse_attr("#[safety { SP }]").unwrap();
    let sp = attr.args.property_reason().next().unwrap().0;
    assert_eq!(sp.tag.name(), "SP");
    assert_eq!(sp.tag.name_type(), [TagNameType::DEFAULT_TYPE, "SP"]);

    // SP with path prefix and arguments
    let attr = parse_attr("#[safety { hazard.Alias(p, q) }]").unwrap();
    let sp = attr.args.property_reason().next().unwrap().0;
    assert_eq!(sp.tag.name_type(), ["hazard", "Alias"]);
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

#[cfg(test)]
fn parse_args(s: &str) -> syn::Result<SafetyAttrArgs> {
    parse_str::<SafetyAttrArgs>(s)
}

#[test]
fn parse_safety_complex_args() {
    // SP path prefix
    _ = parse_args(r#" hazard.Alias(p, q) "#).unwrap();

    // complex expressions in arguments
    _ = parse_args(r#" hazard.Alias(A {a: self.a}, a::b(c![])) : "" "#).unwrap();
}
