use proc_macro::TokenStream;
use safety_tool_parser::{
    proc_macro2::{Ident, TokenStream as TokenStream2},
    property_attr::{
        FnItem, SafetyAttrArgs,
        property::{Kind, PropertyName},
    },
    quote::quote_spanned,
    syn::{parse::Parser, punctuated::Punctuated, *},
};

fn generate(
    kind: Kind,
    property: PropertyName,
    attr: TokenStream,
    item: TokenStream,
) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);
    let attr = parse_macro_input!(attr as SafetyAttrArgs);

    let named_args_set = attr.into_named_args_set(kind, property);
    let doc_comments = named_args_set.generate_doc_comments();
    let safety_tool_attr = named_args_set.generate_safety_tool_attribute();

    let mut fn_item = FnItem::new(item);
    fn_item.insert_attributes_to_the_back(doc_comments);
    fn_item.insert_attributes_to_the_back(safety_tool_attr);
    fn_item.into_token_stream().into()
}

macro_rules! kind_property {
    ($f:ident: $kind:ident, $property:ident) => {
        #[proc_macro_attribute]
        #[allow(non_snake_case)]
        pub fn $f(attr: TokenStream, item: TokenStream) -> TokenStream {
            generate(Kind::$kind, PropertyName::$property, attr, item)
        }
    };

    (
      $(
        $f:ident: $kind:ident, $property:ident
      );+ $(;)?
    ) => {
        $( kind_property!($f: $kind, $property); )+
    };
}

kind_property! {
    Precond_Align: Precond, Align;
    Precond_Size: Precond, Size;
    Precond_NoPadding: Precond, NoPadding;
    Precond_NotNull: Precond, NotNull;
    Precond_Allocated: Precond, Allocated;
    Precond_InBound: Precond, InBound;
    Precond_NotOverlap: Precond, NotOverlap;
    Precond_ValidNum: Precond, ValidNum;
    Precond_ValidString: Precond, ValidString;
    Precond_ValidCStr: Precond, ValidCStr;
    Precond_Init: Precond, Init;
    Precond_Unwrap: Precond, Unwrap;
    Precond_Typed: Precond, Typed;
    Precond_Owning: Precond, Owning;
    Precond_Alias: Precond, Alias;
    Precond_Alive: Precond, Alive;
    Precond_Pinned: Precond, Pinned;
    Precond_NotVolatile: Precond, NotVolatile;
    Precond_Opened: Precond, Opened;
    Precond_Trait: Precond, Trait;
    Precond_Unreachable: Precond, Unreachable;

    Hazard_ValidString: Hazard, ValidString;
    Hazard_Init: Hazard, Init;
    Hazard_Alias: Hazard, Alias;
    Hazard_Pinned: Hazard, Pinned;

    Option_Size: Option, Size;
    Option_Init: Option, Init;
    Option_Trait: Option, Trait;
}

/// Pub use a attribute by stripping the prefix.
#[proc_macro]
pub fn pub_use(tokens: TokenStream) -> TokenStream {
    Punctuated::<Ident, token::Comma>::parse_terminated
        .parse(tokens.clone())
        .unwrap_or_else(|err| panic!("{tokens:?} is not a comma separated idents: {err}"))
        .iter()
        .map(|ident| {
            let name = ident.to_string();
            let pos = name.find('_').unwrap_or_else(|| panic!("{name} doesn't contain `_`"));
            let property_name = &name[pos + 1..];
            let span = ident.span();
            let property_ident = Ident::new(property_name, span);
            quote_spanned! { span =>
                pub use ::safety_tool_macro::#ident as #property_ident;
            }
        })
        .collect::<TokenStream2>()
        .into()
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Memo(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate(Kind::Memo, PropertyName::Unknown, attr, item)
}
