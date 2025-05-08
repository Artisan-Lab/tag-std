#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
#[macro_use]
extern crate rustc_smir;
extern crate stable_mir;

use rustc_middle::ty::TyCtxt;
use stable_mir::{CrateDef, ItemKind, mir::mono::Instance};

fn main() {
    let rustc_args: Vec<_> = std::env::args().collect();
    _ = run_with_tcx!(&rustc_args, |tcx| {
        analyze(tcx);

        // Don't emit real artifacts.
        ControlFlow::<(), ()>::Break(())
    });
}

const REGISTER_TOOL_ATTR: &str = "#[tag_std";

fn analyze(tcx: TyCtxt) {
    let local_items = stable_mir::all_local_items();
    let functions = local_items.iter().filter(|item| matches!(item.kind(), ItemKind::Fn));

    for fun in functions {
        let instance = match Instance::try_from(*fun) {
            Ok(instance) => instance,
            Err(err) => {
                eprintln!("Failed to get the instance for {fun:?}:\n{err:?}");
                continue;
            }
        };
        let tool_attrs = fun
            .all_tool_attrs()
            .into_iter()
            .filter(|attr| attr.as_str().starts_with(REGISTER_TOOL_ATTR));
        for tag_attr in tool_attrs {
            println!(
                "[{fn_name:?}] {attrs:?}\n",
                fn_name = instance.name(),
                attrs = tag_attr.as_str()
            );
        }
    }
}
