use crate::{Str, configuration::DEFAULT_TYPE};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Paren,
    *,
};

#[cfg(test)]
mod tests;

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

#[derive(Debug)]
pub enum TagNameType {
    /// Single ident SP, default to `precond` type.
    SP(Str),
    /// Typed SP: `type.SP`
    TypeSP { typ: Str, sp: Str },
}

impl Parse for TagNameType {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;
        let first = ident.to_string().into();
        Ok(if input.peek(Token![.]) {
            let _: Token![.] = input.parse()?;
            let second: Ident = input.parse()?;
            let sp = second.to_string().into();
            TagNameType::TypeSP { typ: first, sp }
        } else {
            TagNameType::SP(first)
        })
    }
}

impl TagNameType {
    pub fn name(&self) -> &str {
        match self {
            TagNameType::SP(sp) => sp,
            TagNameType::TypeSP { sp, .. } => sp,
        }
    }

    pub fn typ(&self) -> &str {
        match self {
            TagNameType::SP(_) => DEFAULT_TYPE,
            TagNameType::TypeSP { typ, .. } => typ,
        }
    }

    pub fn name_type(&self) -> [&str; 2] {
        match self {
            TagNameType::SP(sp) => [sp, DEFAULT_TYPE],
            TagNameType::TypeSP { typ, sp } => [sp, typ],
        }
    }
}
