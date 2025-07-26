use proc_macro2::{TokenStream, TokenTree};
use syn::{
    parse::{Parse, ParseStream, Parser},
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

#[derive(Debug)]
pub struct PropertiesAndReason {
    pub properties: Punctuated<Property, Token![,]>,
    pub desc: Option<Description>,
}

impl Parse for PropertiesAndReason {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(PropertiesAndReason {
            properties: input.step(|cursor| {
                let mut tokens = TokenStream::new();
                let mut rest = *cursor;
                while let Some((tt, next)) = rest.token_tree() {
                    if let TokenTree::Punct(punct) = &tt {
                        let ch = punct.as_char();
                        if ch == ':' || ch == ';' {
                            // reached at `: "reason"` or `;`
                            return Ok((Punctuated::parse_terminated.parse2(tokens)?, rest));
                        }
                    }
                    tokens.extend([tt]);
                    rest = next;
                }
                Ok((Punctuated::parse_terminated.parse2(tokens)?, rest))
            })?,
            desc: if input.peek(Token![:]) { Some(input.parse()?) } else { None },
        })
    }
}

#[derive(Debug)]
pub struct Property {
    /// `SP` or `type::SP`. Single `SP` means `precond::SP`.
    pub path: Path,
    /// Args in `SP(args)` such as `arg1, arg2`.
    pub args: Option<PropertyArgs>,
}

impl Parse for Property {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Property {
            path: input.parse()?,
            args: if input.peek(Paren) { Some(input.parse()?) } else { None },
        })
    }
}

#[derive(Debug)]
pub struct PropertyArgs {
    pub token: Paren,
    pub args: Punctuated<Expr, Token![,]>,
}

impl Parse for PropertyArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(PropertyArgs {
            token: parenthesized!(content in input),
            args: Punctuated::parse_terminated(&content)?,
        })
    }
}

#[derive(Debug)]
pub struct Description {
    pub token: Token![:],
    pub desc: LitStr,
}

impl Parse for Description {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Description { token: input.parse()?, desc: input.parse()? })
    }
}

#[test]
fn parse_safety_attr() {
    let _: SafetyAttr = parse_str("#[safety {}]").unwrap();

    let attr = "#[safety { SP }]";
    let attr: SafetyAttr = parse_str(attr).unwrap();
    let arg = attr.args.args.first().unwrap();
    let sp = arg.properties.first().unwrap();
    let ident = sp.path.get_ident().unwrap();
    assert_eq!(ident, "SP");
}

#[test]
fn parse_safety_args() {
    let parse_args = |s: &str| parse_str::<SafetyAttrArgs>(s);

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
    let _ = parse_args(r#" SP(a, b): "reason"; SP1, SP2: "reason"; SP3, SP4 "#).unwrap();
}
