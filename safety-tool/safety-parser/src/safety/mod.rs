use crate::{
    Str,
    configuration::{TagType, get_tag},
};
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
    pub fn property_reason(&self) -> impl Iterator<Item = (&Property, Option<&str>)> {
        self.args.iter().flat_map(|arg| arg.tags.iter().map(|prop| (prop, arg.desc.as_deref())))
    }
}

#[derive(Debug)]
pub struct PropertiesAndReason {
    pub tags: Vec<Property>,
    pub desc: Option<Str>,
}

impl Parse for PropertiesAndReason {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut tags = Vec::<Property>::new();
        let mut desc = None;

        while !input.cursor().eof() {
            let tag: TagNameType = input.parse()?;
            tag.check_type();
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
                let s: LitStr = input.parse()?;
                desc = Some(s.value().into());
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
    TypeSP { typ: TagType, sp: Str },
}

impl Parse for TagNameType {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;
        let first = ident.to_string();
        Ok(if input.peek(Token![.]) {
            let _: Token![.] = input.parse()?;
            let second: Ident = input.parse()?;
            let sp = second.to_string().into();
            TagNameType::TypeSP { typ: TagType::new(&first), sp }
        } else {
            TagNameType::SP(first.into())
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

    pub fn typ(&self) -> TagType {
        match self {
            TagNameType::SP(_) => TagType::default(),
            TagNameType::TypeSP { typ, .. } => *typ,
        }
    }

    pub fn name_type(&self) -> (&str, TagType) {
        match self {
            TagNameType::SP(sp) => (sp, TagType::default()),
            TagNameType::TypeSP { typ, sp } => (sp, *typ),
        }
    }

    /// Check if the tag in macro is wrongly specified.
    pub fn check_type(&self) {
        let (name, typ) = self.name_type();
        let defined_types = &get_tag(name).types;
        assert!(
            defined_types.contains(&typ),
            "For tag {name:?}, defined_types is {defined_types:?}, while user's {typ:?} doesn't exist."
        );
    }
}
