#![feature(rustc_private)]
#![cfg_attr(feature = "asterinas", feature(integer_sign_cast))]
#![cfg_attr(not(feature = "std"), feature(let_chains))]

extern crate itertools;
extern crate rustc_ast;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_hir_pretty;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_span;

// Conditional compilation of feature names. Used in [`crossfig::switch`].
// NOTE: before compilation (i.e. calling `cargo build` or something)
// `./gen_rust_toolchain_toml.rs $proj` should be run first
// where $proj is one of std, rfl, or asterinas.
crossfig::alias! {
    // verify-rust-std
    std: { #[cfg(feature = "std")] },
    // Rust for Linux
    rfl: { #[cfg(feature = "rfl")] },
    // Asterinas OS
    asterinas: { #[cfg(feature = "asterinas")] }
}

crossfig::switch! {
    std => {
        #[macro_use]
        extern crate rustc_public;
        use rustc_public::rustc_internal::internal;
    }
    _ => {
        #[macro_use]
        extern crate rustc_smir;
        extern crate stable_mir;
        use stable_mir as rustc_public;
        use rustc_smir::rustc_internal::{self, internal};
    }
}

use eyre::Result;
use rustc_data_structures::fx::FxHashSet;
use rustc_middle::ty::TyCtxt;
use rustc_public::{
    CompilerError, CrateDef, ItemKind,
    mir::{
        MirVisitor,
        mono::{Instance, InstanceKind},
        visit::Location,
    },
    ty::Ty,
};
use std::ops::ControlFlow;

#[macro_use]
extern crate tracing;

mod analyze_hir;

fn main() {
    safety_tool::logger::init();

    let rustc_args: Vec<_> = std::env::args().collect();

    crossfig::switch! {
        std => { let rustc_args = &rustc_args; }
        _ => { }
    };

    let res = run_with_tcx!(rustc_args, |tcx| {
        analyze_hir::analyze_hir(tcx);
        analyze(tcx);
        compilation_status()
    });

    if let Err(CompilerError::Failed) = res {
        std::process::abort();
    }
}

fn compilation_status() -> ControlFlow<()> {
    // When STOP_COMPILATION is set to non-0, stop compiling.
    if std::env::var("STOP_COMPILATION").map(|s| s != "0").unwrap_or(false) {
        ControlFlow::<(), ()>::Break(())
    } else {
        ControlFlow::<(), ()>::Continue(())
    }
}

fn analyze(tcx: TyCtxt) {
    let mut reachability = Reachability::default();
    let local_items = rustc_public::all_local_items();
    let functions = local_items.iter().filter(|item| matches!(item.kind(), ItemKind::Fn));

    for fun in functions {
        let instance = match Instance::try_from(*fun) {
            Ok(instance) => instance,
            Err(_) => {
                // eprintln!("Failed to get the instance for {fun:?}:\n{err:?}");
                continue;
            }
        };
        reachability.add_instance(instance);
    }

    let local_crate = rustc_public::local_crate();
    println!(
        "********* {name:?} {typ:?} has reached {len} instances *********",
        name = local_crate.name,
        typ = tcx.crate_types(),
        len = reachability.instances.len()
    );
    reachability.print_tag_std_attrs(tcx);
}

#[derive(Debug, Default)]
struct Reachability {
    /// Collect monomorphized instances.
    instances: FxHashSet<Instance>,
}

impl Reachability {
    fn add_instance(&mut self, instance: Instance) {
        if self.instances.insert(instance)
            && instance.has_body()
            && matches!(instance.kind, InstanceKind::Item)
        {
            // recurse if this is the first time of insertion
            if let Some(body) = instance.body() {
                self.visit_body(&body);
            }
        }
    }

    fn print_tag_std_attrs(&self, tcx: TyCtxt) {
        for instance in &self.instances {
            // Resort to internal API for all attrs, rather than tool attrs.
            print_tag_std_attrs_through_internal_apis(tcx, instance);
        }
    }
}

const REGISTER_TOOL: &str = "rapx";

fn is_tool_attr(attr: &rustc_hir::Attribute) -> bool {
    crossfig::switch! {
        asterinas => {
            if let rustc_hir::AttrKind::Normal(tool_attr) = &attr.kind
                && tool_attr.path.segments[0].as_str() == REGISTER_TOOL
            {
                return true;
            }
            false
        }
        _  => {
            if let rustc_hir::Attribute::Unparsed(tool_attr) = attr
                && tool_attr.path.segments[0].as_str() == REGISTER_TOOL
            {
                return true;
            }
            false
        }
    }
}

fn print_tag_std_attrs_through_internal_apis(tcx: TyCtxt<'_>, instance: &Instance) {
    let def_id = internal(tcx, instance.def.def_id());

    crossfig::switch! {
        crate::std => { let attrs = tcx.get_all_attrs(def_id).iter(); }
        crate::rfl => { let attrs = tcx.get_all_attrs(def_id); }
        crate::asterinas => { let attrs = tcx.get_attrs_unchecked(def_id).iter(); }
    }

    let tool_attrs = attrs.filter(|&attr| is_tool_attr(attr));
    for attr in tool_attrs {
        println!(
            "{fn_name:?} ({span:?})\n => {attr:?}\n",
            fn_name = instance.name(),
            span = instance.def.span().diagnostic(),
            attr = rustc_hir_pretty::attribute_to_string(&tcx, attr)
        );
    }
}

impl MirVisitor for Reachability {
    fn visit_ty(&mut self, ty: &Ty, _: Location) {
        if let Some((fn_def, args)) = ty.kind().fn_def() {
            // Add an instance.
            let instance = Instance::resolve(fn_def, args).unwrap();
            self.add_instance(instance);
        }
    }
}
