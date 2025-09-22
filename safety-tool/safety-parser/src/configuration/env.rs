use std::{
    env::{self, var},
    fs,
    path::Path,
    sync::LazyLock,
};

/// Single toml config file path.
pub const ENV_SP_FILE: &str = "SP_FILE";
/// Folder where all toml files are searched.
pub const ENV_SP_DIR: &str = "SP_DIR";
/// SP file to crate being compiled.
pub const LOCAL_SP_FILE: &str = "safety-tags.toml";
/// SP folder to crate being compiled.
pub const LOCAL_SP_DIR: &str = "safety-tags";

struct Env {
    config_exists: bool,
}

static ENV: LazyLock<Env> = LazyLock::new(|| Env {
    config_exists: crate_sp_paths().is_some()
        || var(ENV_SP_FILE).is_ok()
        || var(ENV_SP_DIR).is_ok(),
});

/// If ENV_SP_DIR or ENV_SP_DIR is provided, check tag and emit `#[doc]` for each tag.
/// If neither is provided, do nothing.
pub fn config_exists() -> bool {
    ENV.config_exists
}

fn list_toml_files(dir: &str) -> Vec<String> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir).unwrap_or_else(|e| panic!("Failed to read {dir} folder:\n{e}")) {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().map(|ext| ext == "toml").unwrap_or(false) {
            files.push(path.into_os_string().into_string().unwrap());
        }
    }
    files
}

/// Search in the crate being compiled, i.e. `CARGO_MANIFEST_DIR/safety-tags.toml`
/// or `CARGO_MANIFEST_DIR/safety-tags/`.
pub fn crate_sp_paths() -> Option<Vec<String>> {
    if let Ok(dir) = env::var("CARGO_MANIFEST_DIR") {
        let dir = Path::new(&*dir);
        let sp_file = dir.join(LOCAL_SP_FILE);
        let sp_dir = dir.join(LOCAL_SP_DIR);
        if sp_file.exists() {
            return Some(vec![sp_file.to_str()?.to_owned()]);
        } else if sp_dir.exists() {
            return Some(list_toml_files(sp_dir.to_str()?));
        }
    }
    None
}

/// Paths to toml config.
///
/// First, search `safety-tags.toml` or `safety-tags` folder
/// in the crate being compiled:
/// * `CARGO_MANIFEST_DIR/safety-tags.toml`
/// * `CARGO_MANIFEST_DIR/safety-tags/`
/// * if both exist, only respect safety-tags.toml
///
/// If no toml found, pass one of these env vars:
/// * if `SP_FILE` is specified, use that toml path
/// * if `SP_DIR` is specified, use that path to find toml files
/// * if both are given, only respect `SP_FILE`
pub fn toml_file_paths() -> Vec<String> {
    if let Some(paths) = crate_sp_paths() {
        paths
    } else if let Ok(file) = env::var(ENV_SP_FILE) {
        vec![file]
    } else if let Ok(dir) = env::var(ENV_SP_DIR) {
        list_toml_files(&dir)
    } else {
        panic!("Environment variable `SP_FILE` or `SP_DIR` should be specified.");
    }
}
