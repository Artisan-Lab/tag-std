#![feature(rustc_private)]

extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
#[macro_use]
extern crate rustc_smir;
extern crate stable_mir;

use rustc_data_structures::fx::FxHashSet;
use stable_mir::{
    CrateDef, CrateItem, ItemKind,
    mir::{MirVisitor, mono::Instance, visit::Location},
    ty::Ty,
};

fn main() {
    let rustc_args: Vec<_> = std::env::args().collect();
    _ = run!(&rustc_args, || {
        analyze();

        // Don't emit real artifacts.
        ControlFlow::<(), ()>::Break(())
    });
}

const REGISTER_TOOL_ATTR: &str = "#[tag_std";

fn analyze() {
    let mut reachability = Reachability::new();
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

        reachability.add_instance(instance);
    }

    dbg!(reachability.instances.len(), reachability.cross_crates);
    reachability.print_tag_std_attrs();
}

#[derive(Debug, Default)]
struct Reachability {
    /// Collect monomorphized instances.
    instances: FxHashSet<Instance>,
    /// Enable this to collect instances from dependencies.
    #[allow(dead_code)]
    cross_crates: bool,
}

impl Reachability {
    fn new() -> Self {
        Self {
            cross_crates: std::env::var("TAG_STD_CROSS_CRATES")
                .map(|var| var == "true")
                .unwrap_or(false),
            ..Default::default()
        }
    }

    fn add_instance(&mut self, instance: Instance) {
        if self.instances.insert(instance) {
            // recurse if this is the first time of insertion
            if let Some(body) = instance.body() {
                println!("(body) {:?}", instance.name());
                self.visit_body(&body);
            }
        }
    }

    fn print_tag_std_attrs(&self) {
        // let mut file = std::fs::File::create("tag-std.txt").unwrap();
        for instance in &self.instances {
            // Only user defined instances can be converted.
            match CrateItem::try_from(*instance) {
                Ok(item) => {
                    let tool_attrs = item
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
                Err(err) => {
                    println!("(error) {} not converted to be an item: {err:?}", instance.name())
                }
            }
        }
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
