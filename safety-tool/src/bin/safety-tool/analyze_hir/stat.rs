use camino::Utf8PathBuf;
use rustc_hir::def_id::CrateNum;
use rustc_middle::ty::TyCtxt;
use rustc_session::config::CrateType as RawCrateType;
use safety_tool::stat::*;

pub fn new(tcx: TyCtxt) -> Stat {
    Stat {
        krate: new_crate(tcx),
        specs: safety_parser::configuration::CACHE.clone(),
        funcs: Vec::new(),
        metrics: Metrics { coverage_rate: 0 },
    }
}

fn new_crate(tcx: TyCtxt) -> Krate {
    let local_crate = CrateNum::ZERO;

    let name = tcx.crate_name(local_crate).to_string();
    let path = {
        let path = || Utf8PathBuf::from(tcx.sess.io.input.source_name().prefer_local().to_string());
        path().canonicalize_utf8().unwrap_or_else(|_| path())
    };
    let typ = crate_type(tcx.crate_types());
    let version = std::env::var("CARGO_PKG_VERSION").unwrap_or_default();

    Krate { name, path, typ, version }
}

fn crate_type(v: &[RawCrateType]) -> CrateType {
    if v.contains(&RawCrateType::Executable) {
        CrateType::Bin
    } else {
        assert!(!v.is_empty(), "There is no crate type available.");
        CrateType::Lib
    }
}
