use crate::is_tool_attr;
use rustc_hir::{BodyId, FnSig, HirId, ImplItemKind, ItemKind, Node, def_id::LocalDefId};
use rustc_middle::ty::TyCtxt;
use rustc_span::{Ident, source_map::get_source_map};

mod db;
mod visit;

pub mod diagnostic;

pub fn analyze_hir(tcx: TyCtxt) {
    let exit_and_emit = diagnostic::ExitAndEmit::new();
    let mut v_hir_fn = Vec::with_capacity(64);

    let def_items = tcx.hir_crate_items(()).definitions();
    for local_def_id in def_items {
        let node = tcx.hir_node_by_def_id(local_def_id);

        // fn item or associated fn item
        let hir_fn = match node {
            Node::Item(item) if matches!(item.kind, ItemKind::Fn { .. }) => {
                crossfig::switch! {
                    crate::asterinas => {
                        let name = item.ident;
                        let (sig, _generics, body) = item.expect_fn();
                    }
                    _ => { let (name, sig, _generics, body) = item.expect_fn(); }
                }

                let sig = *sig;
                HirFn { local: local_def_id, hir_id: item.hir_id(), name, sig, body }
            }
            Node::ImplItem(item) if matches!(item.kind, ImplItemKind::Fn(..)) => {
                let (sig, body) = item.expect_fn();
                let hir_id = item.hir_id();
                HirFn { local: local_def_id, hir_id, name: item.ident, sig: *sig, body }
            }
            _ => continue,
        };

        v_hir_fn.push(hir_fn);
    }

    let mut tool_attrs =
        db::get_all_tool_attrs(v_hir_fn.iter().filter_map(|f| f.to_data(tcx))).unwrap();
    let src_map = get_source_map().unwrap();

    let mut diagnostics = Vec::new();
    for hir_fn in &v_hir_fn {
        let body_id = hir_fn.body;

        crossfig::switch! {
            crate::asterinas => {
                let body_hir_id = body_id.hir_id;
                let body = tcx.hir_owner_nodes(body_hir_id.owner).bodies[&body_hir_id.local_id].value;
            }
            _ => { let body = tcx.hir_body(body_id).value; }
        }

        let tyck = tcx.typeck_body(body_id);
        let calls = visit::get_calls(tcx, body, tyck);
        let unsafe_calls = calls.get_unsafe_calls();
        if !unsafe_calls.is_empty() {
            debug!(?unsafe_calls);
            for call in &unsafe_calls {
                call.check_tool_attrs(
                    hir_fn.hir_id,
                    tcx,
                    &src_map,
                    &mut tool_attrs,
                    &mut diagnostics,
                );
            }
        }
    }

    if !diagnostics.is_empty() {
        if exit_and_emit.should_emit() {
            for diagnostic in &diagnostics {
                eprintln!("{}\n", diagnostic.render)
            }
            diagnostic::total(&diagnostics);
        }
        if exit_and_emit.should_abort() {
            std::process::abort()
        }
    }
}

#[allow(dead_code)]
struct HirFn<'hir> {
    local: LocalDefId,
    hir_id: HirId,
    name: Ident,
    sig: FnSig<'hir>,
    body: BodyId,
}

impl HirFn<'_> {
    fn has_tool_attrs(&self, tcx: TyCtxt) -> bool {
        let hir_id = self.hir_id;
        crossfig::switch! {
            crate::asterinas => { tcx.hir_attrs(hir_id.owner).get(hir_id.local_id).iter().any(is_tool_attr) }
            _ => { tcx.hir_attrs(hir_id).iter().any(is_tool_attr) }
        }
    }

    fn to_data(&self, tcx: TyCtxt) -> Option<db::Data> {
        self.has_tool_attrs(tcx).then(|| db::Data::new(self, tcx))
    }
}
