use proc_macro2::TokenStream;
use quote::ToTokens;
use std::mem::take;
use syn::*;

/// Since `#[safety}` is applied on unsafe function item and any expression,
/// we have to fully parse to split attributes from them to insert our doc
/// string to the tail of all attributes.
pub fn split_attrs_and_rest(ts: TokenStream) -> Input {
    if let Ok(stmt) = parse2::<Stmt>(ts.clone()) {
        match stmt {
            Stmt::Local(mut local) => Input::new(take(&mut local.attrs), local),
            Stmt::Item(item) => parse_item(item),
            Stmt::Expr(expr, _) => parse_expr(expr),
            Stmt::Macro(mut macro_) => Input::new(take(&mut macro_.attrs), macro_),
        }
    } else {
        Input::new(Vec::new(), ts)
    }
}

fn parse_item(mut item: Item) -> Input {
    let attrs = match &mut item {
        Item::Fn(fun) => take(&mut fun.attrs),
        Item::Impl(imp) => take(&mut imp.attrs),
        Item::Trait(trait_) => take(&mut trait_.attrs),
        _ => Vec::new(),
    };
    Input::new(attrs, item).set_gen_doc()
}

fn parse_expr(mut expr: Expr) -> Input {
    let attrs = match &mut expr {
        Expr::Unsafe(unsafe_) => take(&mut unsafe_.attrs),
        Expr::Let(let_) => take(&mut let_.attrs),
        Expr::Assign(assign) => take(&mut assign.attrs),
        Expr::Block(block) => take(&mut block.attrs),
        Expr::Call(call) => take(&mut call.attrs),
        Expr::MethodCall(call) => take(&mut call.attrs),
        Expr::Match(match_) => take(&mut match_.attrs),
        Expr::If(if_) => take(&mut if_.attrs),
        Expr::While(while_) => take(&mut while_.attrs),
        Expr::Loop(loop_) => take(&mut loop_.attrs),
        Expr::ForLoop(for_) => take(&mut for_.attrs),
        Expr::Macro(macro_) => take(&mut macro_.attrs),
        Expr::Struct(struct_) => take(&mut struct_.attrs),
        Expr::Array(array) => take(&mut array.attrs),
        Expr::Closure(closure) => take(&mut closure.attrs),
        Expr::Path(path) => take(&mut path.attrs),
        Expr::Async(async_) => take(&mut async_.attrs),
        Expr::Await(await_) => take(&mut await_.attrs),
        Expr::Const(const_) => take(&mut const_.attrs),
        Expr::Break(break_) => take(&mut break_.attrs),
        Expr::Continue(continue_) => take(&mut continue_.attrs),
        Expr::Repeat(repeat) => take(&mut repeat.attrs),
        Expr::Return(return_) => take(&mut return_.attrs),
        Expr::Try(try_) => take(&mut try_.attrs),
        Expr::TryBlock(block) => take(&mut block.attrs),
        Expr::Yield(yield_) => take(&mut yield_.attrs),
        _ => Vec::new(),
    };
    Input::new(attrs, expr)
}

#[derive(Debug)]
pub struct Input {
    pub attrs: TokenStream,
    pub rest: TokenStream,
    /// Indicate to generate `#[doc]`. True only for items.
    pub gen_doc: bool,
}

impl Input {
    fn new(attrs: Vec<Attribute>, rest: impl ToTokens) -> Input {
        let attrs = attrs.into_iter().map(|attr| attr.into_token_stream()).collect();
        Input { attrs, rest: rest.into_token_stream(), gen_doc: false }
    }

    fn set_gen_doc(mut self) -> Self {
        self.gen_doc = true;
        self
    }
}

#[test]
fn split_attrs_on_item() {
    let ts = quote::quote! {
        #[a]
        unsafe fn f() {}
    };
    let input = dbg!(split_attrs_and_rest(ts));
    assert!(!input.attrs.is_empty());
}

#[test]
fn split_attrs_on_expr() {
    let ts = quote::quote! {
        #[a]
        unsafe { call() };
    };
    dbg!(split_attrs_and_rest(ts));
}
