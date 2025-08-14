use crate::analyze_hir::db::TagState;

use super::{
    db::{Property, ToolAttrs},
    diagnostics::EmitDiagnostics,
};
use rustc_hir::{
    def::{DefKind, Res},
    def_id::DefId,
    intravisit::*,
    *,
};
use rustc_middle::ty::{TyCtxt, TypeckResults};

#[derive(Debug)]
pub struct Call {
    /// function use id
    pub hir_id: HirId,
    /// function def id
    pub def_id: DefId,
}

impl Call {
    pub fn check_tool_attrs(
        &self,
        fn_hir_id: HirId,
        tool_attrs: &mut ToolAttrs,
        diagnostics: &mut EmitDiagnostics,
    ) {
        let tcx = diagnostics.tcx();
        let Some(tag_state) = tool_attrs.get_tags(self.def_id, tcx) else {
            // No tool attrs to be checked.
            return;
        };

        let mut check = |hir_id: HirId| {
            debug!(?hir_id, ?fn_hir_id);

            let properties = Property::new_with_hir_id(hir_id, tcx);

            let is_empty = properties.is_empty();
            if !is_empty {
                for tag in &properties {
                    if let Err(err) = tag_state.discharge(tag) {
                        diagnostics.push(hir_id, &err);
                    }
                }
                // only checks if Safety tags exist
                check_tag_state(tag_state, hir_id, diagnostics);
            }
            is_empty
        };
        check(self.hir_id);

        for parent in parent_hirs(tcx, self.hir_id) {
            let empty = check(parent);
            // Stop at first tool attrs or the function item.
            // For a function inside a nested module, hir_parent_id_iter
            // will pop up to the crate root, thus it's necessary to
            // stop when reaching the fn item.
            if !empty || parent == fn_hir_id {
                break;
            }
        }

        // make sure Safety tags are all discharged
        check_tag_state(tag_state, self.hir_id, diagnostics);
    }
}

fn parent_hirs(tcx: TyCtxt, hir_id: HirId) -> impl Iterator<Item = HirId> {
    crossfig::switch! {
        crate::asterinas => { tcx.hir().parent_id_iter(hir_id) }
        _ => { tcx.hir_parent_id_iter(hir_id) }
    }
}

fn check_tag_state(tag_state: &mut TagState, hir_id: HirId, diagnostics: &mut EmitDiagnostics) {
    let undischarged = tag_state.undischarged();
    let len = undischarged.len();
    if len != 0 {
        let undischarged_str = undischarged.join("\n");
        let newline = if len == 1 { " " } else { "\n" };
        let plural = if undischarged_str.matches(',').count() == 0 { "Tag is" } else { "Tags are" };
        let title = format!("{plural} not discharged:{newline}{undischarged_str}");
        diagnostics.push(hir_id, &title);
    }
}

pub struct Calls<'tcx> {
    tcx: TyCtxt<'tcx>,
    tyck: &'tcx TypeckResults<'tcx>,
    calls: Vec<Call>,
}

crossfig::switch! {
    crate::asterinas => {
        impl<'tcx> Visitor<'tcx> for Calls<'tcx> {
            type NestedFilter = rustc_middle::hir::nested_filter::OnlyBodies;
            type Result = ();

            fn nested_visit_map(&mut self) -> Self::Map {
                self.tcx.hir()
            }

            fn visit_expr(&mut self, ex: &'tcx Expr<'tcx>) -> Self::Result {
                self.inner_visit_expr(ex)
            }
        }
    }
    _ => {
        impl<'tcx> Visitor<'tcx> for Calls<'tcx> {
            type MaybeTyCtxt = TyCtxt<'tcx>;
            type NestedFilter = rustc_middle::hir::nested_filter::OnlyBodies;
            type Result = ();

            fn maybe_tcx(&mut self) -> Self::MaybeTyCtxt {
                self.tcx
            }

            fn visit_expr(&mut self, ex: &'tcx Expr<'tcx>) -> Self::Result {
                self.inner_visit_expr(ex)
            }
        }
    }
}

impl<'tcx> Calls<'tcx> {
    fn inner_visit_expr(&mut self, ex: &'tcx Expr<'tcx>) {
        let hir_id = ex.hir_id;
        match ex.kind {
            ExprKind::Path(QPath::Resolved(_opt_ty, path)) => {
                if let Res::Def(DefKind::Fn, def_id) = path.res {
                    self.calls.push(Call { hir_id, def_id });
                }
            }
            // https://doc.rust-lang.org/nightly/nightly-rustc/rustc_hir/hir/enum.ExprKind.html#variant.MethodCall
            //
            // res is Err, and  must resolve to get def_id, thus need to call
            // https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/struct.TypeckResults.html#method.type_dependent_def_id
            ExprKind::MethodCall(..) => {
                if let Some(def_id) = self.tyck.type_dependent_def_id(hir_id) {
                    self.calls.push(Call { hir_id, def_id });
                } else {
                    eprintln!("Unable to resolve DefId from {:?}", ex.kind);
                }
            }
            _ => (),
        }
        walk_expr(self, ex)
    }

    pub fn get_unsafe_calls(&self) -> Vec<&Call> {
        self.calls
            .iter()
            .filter(|call| self.tcx.fn_sig(call.def_id).skip_binder().safety().is_unsafe())
            .collect()
    }
}

pub fn get_calls<'tcx>(
    tcx: TyCtxt<'tcx>,
    expr: &'tcx Expr<'tcx>,
    tyck: &'tcx TypeckResults<'tcx>,
) -> Calls<'tcx> {
    let mut calls = Calls { tcx, tyck, calls: Vec::new() };
    walk_expr(&mut calls, expr);
    calls
}
