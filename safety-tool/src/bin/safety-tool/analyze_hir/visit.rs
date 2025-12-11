use crate::analyze_hir::{
    db::{Property, TagState, ToolAttrs, tool_attr_on_hir},
    diagnostics::EmitDiagnostics,
    stat,
};
use rustc_hir::{
    def::{DefKind, Res},
    def_id::DefId,
    intravisit::*,
    *,
};
use rustc_middle::ty::{TyCtxt, TypeckResults};
use safety_parser::{safety::SafetyAttr, syn};

#[derive(Debug, Clone, Copy)]
pub struct Call {
    /// function use id
    pub hir_id: HirId,
    /// function def id
    pub def_id: DefId,
}

impl Call {
    pub fn check_tool_attrs(
        &self,
        caller: HirId,
        tool_attrs: &mut ToolAttrs,
        diagnostics: &mut EmitDiagnostics,
    ) {
        let tcx = diagnostics.tcx();
        let Some(tag_state) = tool_attrs.get_tags(self.def_id, tcx) else {
            // No tool attrs to be checked.
            return;
        };

        let mut check = |hir_id: HirId| {
            debug!(?hir_id, ?caller);

            let properties = Property::new_with_hir_id(hir_id, tcx);

            let is_empty = properties.is_empty();
            if !is_empty {
                for tag in &properties {
                    if let Err(err) = tag_state.discharge(tag) {
                        diagnostics.push_duplicate_discharge(hir_id, &err);
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
            if !empty || parent == caller {
                break;
            }
        }

        // make sure Safety tags are all discharged
        check_tag_state(tag_state, self.hir_id, diagnostics);
    }

    pub fn stat<'tcx>(
        self,
        caller: HirId,
        tcx: TyCtxt<'tcx>,
        tool_attrs: &mut ToolAttrs,
    ) -> Option<CollectCalleeTags<'tcx>> {
        CollectCalleeTags::new(self, caller, tcx, tool_attrs)
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
    let title = undischarged.title();
    if !title.is_empty() {
        diagnostics.push_missing_discharge(hir_id, &title, &undischarged.info());
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
        match &ex.kind {
            ExprKind::Path(qpath) => {
                let qpath_res = self.tyck.qpath_res(qpath, hir_id);
                // maybe use [DefKind::is_fn_like](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_hir/def/enum.DefKind.html#method.is_fn_like)
                if let Res::Def(DefKind::Fn | DefKind::AssocFn, def_id) = qpath_res {
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
        self.calls.iter().filter(|call| stat::is_unsafe(call.def_id, self.tcx)).collect()
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

// Collect tags on a callee, by bubbling up HIR nodes to find the nearest safety attributes.
//
// NOTE: the tags on the caller's signature aren't directly counted as the callee's tags.
pub struct CollectCalleeTags<'tcx> {
    tcx: TyCtxt<'tcx>,
    tags: Vec<stat::Tag>,
    callee: Call,
    #[allow(dead_code)]
    caller: HirId,
}

impl<'tcx> CollectCalleeTags<'tcx> {
    fn new(
        callee: Call,
        caller: HirId,
        tcx: TyCtxt<'tcx>,
        _tool_attrs: &mut ToolAttrs,
    ) -> Option<Self> {
        // let Some(_tag_state) = tool_attrs.get_tags(callee.def_id, tcx) else {
        //     // No tool attrs to be checked on the callee.
        //     return None;
        // };

        let mut found_nearest_tags = false;
        let mut tags = Vec::new();
        // FIXME: the validity of attributes are not checked. Tags that do not target
        // any calls should be warned.
        for parent in parent_hirs(tcx, callee.hir_id) {
            for attr_str in tool_attr_on_hir(parent, tcx) {
                match syn::parse_str::<SafetyAttr>(&attr_str) {
                    Ok(attr) => {
                        let seg = &attr.attr.path().segments;
                        if seg.first().map(|i| i.ident != crate::REGISTER_TOOL).unwrap_or(true) {
                            // Skip non rapx attributes.
                            continue;
                        }
                        if let Some(path) = seg.last()
                            && path.ident == "checked"
                        {
                            // FIXME: we should only push valid tags
                            // for the callee through tag_state
                            stat::push_tag(attr.args.args, &mut tags);
                            found_nearest_tags = true;
                        }
                    }
                    Err(err) => eprintln!("{attr_str} is not parsed as SafetyAttr: {err}"),
                }
            }
            // Treat nearest parent tags as the call's tags.
            // This can be problematic if we allow partial discharging,
            // in which case we should continue bubbling up.
            // We don't include the caller tags here: callee must be
            // discharged inside the function body.
            if found_nearest_tags && parent == caller {
                break;
            }
        }
        Some(CollectCalleeTags { tcx, tags, callee, caller })
    }

    pub fn into_stat_func(self) -> stat::Func {
        stat::new_callee(self.callee.hir_id, self.callee.def_id, self.tcx, self.tags)
    }
}
