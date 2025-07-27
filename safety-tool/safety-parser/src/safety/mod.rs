use crate::{
    Str,
    configuration::{TagType, config_exists, get_tag},
};
use indexmap::IndexMap;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Paren,
    *,
};

mod utils;

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

        assert!(attrs.len() == 1, "Given input must be a single #[safety] attribute.");
        let attr = attrs.remove(0);
        drop(attrs);

        // We don't check attribute name. Normally, it's #[safety { ... }],
        // but it can be #[path::to::safety {}], or #[reexported {}], or #[rapx::inner {}].

        let args = attr.parse_args()?;
        Ok(SafetyAttr { attr, args })
    }
}

/// Parse a full attribute such as `#[rapx::inner { ... }]` to get properties.
pub fn parse_attr_and_get_properties(attr: &str) -> Box<[PropertiesAndReason]> {
    let attr: SafetyAttr = parse_str(attr)
        .unwrap_or_else(|e| panic!("Failed to parse {attr:?} as a safety attribute:\n{e}"));
    attr.args.args.into_iter().collect()
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
    pub tags: Box<[Property]>,
    pub desc: Option<Str>,
}

impl Parse for PropertiesAndReason {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut tags = Vec::<Property>::new();
        let mut desc = None;

        while !input.cursor().eof() {
            let tag: TagNameType = input.parse()?;
            if config_exists() {
                tag.check_type();
            }
            let sp = if input.peek(Paren) {
                let content;
                parenthesized!(content in input);
                let args = Punctuated::<Expr, Token![,]>::parse_terminated(&content)?;
                let args = args.into_iter().collect();
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
        Ok(PropertiesAndReason { tags: tags.into(), desc })
    }
}

impl PropertiesAndReason {
    /// Generate
    ///
    /// ```text
    /// /// Grouped desc
    /// /// * SP1: desc
    /// /// * SP2: desc
    /// ```
    pub fn gen_doc(&self) -> TokenStream {
        let mut ts = TokenStream::default();
        if let Some(desc) = self.desc.as_deref() {
            ts.extend(quote! { #[doc = #desc] });
        }
        for tag in &self.tags {
            let name = tag.tag.name();
            let tokens = if let Some(desc) = tag.gen_doc() {
                quote! { #[doc = concat!("* ", #name, ": ", #desc)] }
            } else {
                quote! { #[doc = concat!("* ", #name)] }
            };
            ts.extend(tokens);
        }
        ts
    }
}

#[derive(Debug)]
pub struct Property {
    /// `SP` or `type::SP`. Single `SP` means `precond::SP`.
    pub tag: TagNameType,
    /// Args in `SP(args)` such as `arg1, arg2`.
    pub args: Box<[Expr]>,
}

impl Property {
    /// Generate `#[doc]` for this property from its desc string interpolation.
    /// None means SP is not defined with desc, thus nothing to generate.
    pub fn gen_doc(&self) -> Option<String> {
        let defined_tag = get_tag(self.tag.name());
        // NOTE: this tolerates missing args, but position matters.
        let args_len = self.args.len().min(defined_tag.args.len());

        // map defined arg names to user inputs
        let defined_args = defined_tag.args[..args_len].iter().map(|s| &**s);
        let input_args = self.args[..args_len].iter().map(utils::expr_to_string);
        let mut map_defined_arg_input_arg: IndexMap<_, _> = defined_args.zip(input_args).collect();
        // if input arg is missing, defined arg will be an empty string
        for defined_arg in &defined_tag.args {
            if !map_defined_arg_input_arg.contains_key(&**defined_arg) {
                map_defined_arg_input_arg.insert(defined_arg, String::new());
            }
        }

        defined_tag.desc.as_deref().map(|desc| utils::template(desc, &map_defined_arg_input_arg))
    }
}

/// Typed SP: `type.SP`
#[derive(Debug)]
pub struct TagNameType {
    /// Default tag type is the one in single defined_types.
    //
    /// Deserialization will fill the default tag type as Precond.
    typ: Option<TagType>,
    /// Single ident string.
    name: Str,
}

impl Parse for TagNameType {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;
        let first = ident.to_string();
        Ok(if input.peek(Token![.]) {
            let _: Token![.] = input.parse()?;
            let second: Ident = input.parse()?;
            let name = second.to_string().into();
            TagNameType { name, typ: Some(TagType::new(&first)) }
        } else {
            TagNameType { name: first.into(), typ: None }
        })
    }
}

impl TagNameType {
    pub fn name(&self) -> &str {
        &self.name
    }

    // FIXME: no pinned default tag, because we want default tag to be
    // the one in single defined_types. Deserialization will fill the
    // default tag type as Precond.
    pub fn typ(&self) -> Option<TagType> {
        self.typ
    }

    pub fn name_type(&self) -> (&str, Option<TagType>) {
        (&self.name, self.typ)
    }

    /// Check if the tag in macro is wrongly specified.
    pub fn check_type(&self) {
        let (name, typ) = self.name_type();
        let defined_types = &get_tag(name).types;
        if let Some(typ) = typ {
            assert!(
                defined_types.contains(&typ),
                "For tag {name:?}, defined_types is {defined_types:?}, \
                 while user's {typ:?} doesn't exist."
            );
        } else {
            assert_eq!(
                defined_types.len(),
                1,
                "For tag {name:?} without explicit type, \
                 the default type is the single defined type.\n\
                 But defined_types has multiple types: {defined_types:?}.\n\
                 So choose a type to be `type.{name}`."
            );
        }
    }
}
