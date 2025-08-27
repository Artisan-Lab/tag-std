use crate::{Result, Utf8Path, Utf8PathBuf};
use cargo_metadata::{Artifact, CrateType, Message};
use eyre::{Context, ContextCompat};
use safety_tool::utils::sysroot::{self, CARGO_SAFETY_TOOL, SAFETY_TOOL, SAFETY_TOOL_RFL};
use std::{
    fs,
    io::ErrorKind,
    process::{Command, Stdio},
    sync::LazyLock,
};

pub fn dev() -> Result<()> {
    // NOTE: this compiles safety-lib on host target
    run("safety-lib", CopyMode::Lib, "safety-lib")?;
    SAFETY_TOOL_SYSROOT.copy_safety_macro()?;
    // cp safety-tool's bins
    run(".", CopyMode::Bin, "safety-tool")?;
    Ok(())
}

fn run(dir: &str, mode: CopyMode, prefix: &str) -> Result<()> {
    // Pass extra `cargo build` arguments, like `-Fstd` to compile safety-tool.
    let cargo_build_args = std::env::var("CARGO_BUILD_ARGS").unwrap_or_default();

    // Ensure the rendered field of JSON messages contains
    // embedded ANSI color codes for respecting rustc’s default color scheme
    let mut command = Command::new("cargo")
        .args(["build", "--message-format=json-diagnostic-rendered-ansi"])
        .args(cargo_build_args.split_whitespace())
        .current_dir(dir)
        .stdout(Stdio::piped())
        .spawn()
        .context("Failed to run `cargo build`.")?;

    let sysroot = &*SAFETY_TOOL_SYSROOT;
    sysroot.create_metadata_json(prefix)?;

    let mut artifacts = Vec::with_capacity(32);
    let reader = std::io::BufReader::new(command.stdout.take().unwrap());
    for message in Message::parse_stream(reader) {
        if let Message::CompilerArtifact(artifact) = message.context("Faied to read message.")? {
            sysroot.copy_artifacts(&artifact, mode)?;
            artifacts.push(artifact);
        }
    }
    sysroot.create_artifacts_json(&artifacts, prefix)?;

    Ok(())
}

static SAFETY_TOOL_SYSROOT: LazyLock<SafetyToolSysroot> =
    LazyLock::new(|| SafetyToolSysroot::init().unwrap());

#[derive(Debug)]
struct SafetyToolSysroot {
    root: Utf8PathBuf,
    lib: Utf8PathBuf,
    bin: Utf8PathBuf,
}

impl SafetyToolSysroot {
    fn init() -> Result<Self> {
        let this = Self::new();
        info!(?this);
        let Self { root, lib, bin } = &this;

        // clean sysroot
        remove_dir(root)?;

        // crate sysroot dir and child dirs
        create_dir(root)?;
        create_dir(lib)?;
        create_dir(bin)?;

        Ok(this)
    }

    fn new() -> Self {
        let root = sysroot::root();
        let lib = sysroot::lib();
        let bin = sysroot::bin();
        SafetyToolSysroot { root, lib, bin }
    }

    fn copy_artifacts(&self, artifact: &Artifact, mode: CopyMode) -> Result<()> {
        for file in &artifact.filenames {
            let filename = file
                .file_name()
                .with_context(|| format!("Unable to know filename from `{file}`."))?;

            if matches!(mode, CopyMode::Bin | CopyMode::Both)
                && artifact.target.crate_types.contains(&CrateType::Bin)
            {
                let name = &*artifact.target.name;
                if name == CARGO_SAFETY_TOOL || name == SAFETY_TOOL || name == SAFETY_TOOL_RFL {
                    fs::copy(file, self.bin.join(filename))?;
                }
            }

            if matches!(mode, CopyMode::Lib | CopyMode::Both)
                && let Some(ext) = file.extension()
            {
                let crate_type = CrateType::from(ext);
                if matches!(
                    crate_type,
                    CrateType::Lib
                        | CrateType::RLib
                        | CrateType::DyLib
                        | CrateType::CDyLib
                        | CrateType::StaticLib
                ) || is_system_lib(ext)
                {
                    fs::copy(file, self.lib.join(filename))?;
                };
            }
        }

        Ok(())
    }

    fn create_artifacts_json(&self, artifacts: &[Artifact], prefix: &str) -> Result<()> {
        let filename = format!("{prefix}_artifacts.json");
        let file = fs::File::create(self.root.join(filename))?;
        serde_json::to_writer_pretty(file, artifacts)?;
        Ok(())
    }

    fn create_metadata_json(&self, prefix: &str) -> Result<()> {
        let filename = format!("{prefix}_cargo_metadata.json");
        let file = fs::File::create(self.root.join(filename))?;
        let output = Command::new("cargo").args(["metadata", "--format-version=1"]).output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        ensure!(
            output.status.success(),
            "Failed to run `cargo metadata --format-version=1`:\nstdout={stdout}\nstderr={stderr}",
            stderr = String::from_utf8_lossy(&output.stderr)
        );

        let json: serde_json::Value = serde_json::from_str(&stdout)?;
        serde_json::to_writer_pretty(file, &json)?;
        Ok(())
    }

    /// copy libsafety_macro*.so to libsafety_macro.so.
    /// If libsafety_macro.so, do nothing.
    fn copy_safety_macro(&self) -> Result<()> {
        const SO: &str = "libsafety_macro.so";
        for entry in fs::read_dir(&self.lib)? {
            let entry = entry?;
            let path = entry.path();
            #[allow(clippy::collapsible_if)]
            if let Some(file_name) = path.file_name() {
                if let Some(file_name) = file_name.to_str() {
                    if file_name == SO {
                        return Ok(());
                    }
                    if file_name.starts_with("libsafety_macro") && file_name.ends_with(".so") {
                        fs::copy(&path, self.lib.join(SO))?;
                        return Ok(());
                    }
                }
            }
        }
        Ok(())
    }
}

/// Create a folder, but ignore if it has already existed.
fn create_dir(path: &Utf8Path) -> Result<()> {
    if let Err(err) = fs::create_dir(path) {
        ensure!(err.kind() == ErrorKind::AlreadyExists, "Failed to create dir `{path}`: {err:?}")
    }
    Ok(())
}

/// Remove a folder and all contents, but ignore if it doesn't exist.
fn remove_dir(path: &Utf8Path) -> Result<()> {
    if let Err(err) = fs::remove_dir_all(path) {
        ensure!(err.kind() == ErrorKind::NotFound, "Failed to remove dir `{path}`: {err:?}")
    }
    Ok(())
}

fn is_system_lib(ext: &str) -> bool {
    let system_lib_ext = if cfg!(target_os = "linux") {
        "so"
    } else if cfg!(target_os = "macos") {
        "dylib"
    } else {
        unimplemented!("system lib extension");
    };
    ext == system_lib_ext
}

#[derive(Debug, Clone, Copy)]
enum CopyMode {
    Bin,
    Lib,
    #[allow(dead_code)]
    Both,
}
