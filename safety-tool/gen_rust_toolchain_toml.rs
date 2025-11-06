#!/usr/bin/env -S RUSTC_BOOTSTRAP=1 cargo -Zscript

---cargo
[package]
edition = "2024"
[dependencies]
serde_json = "1"
---
use std::io::{Seek, Write};

fn main() {
    // The first argument as a project name.
    // e.g. ./gen_rust_toolchain_toml.rs rfl
    let first_arg =
        std::env::args().nth(1).expect("Choose a project to generate rust-toolchain.toml");
    let proj = Project::new(&first_arg);
    proj.write_rust_toolchain_toml();
    proj.set_rust_analyzer_cargo_features();
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Project {
    Std,
    RustForLinux,
    Asterinas,
    Default,
}

impl Project {
    /// Specified by `std`, `rfl`, `asterinas`, or nothing.
    fn new(proj: &str) -> Self {
        match &*proj.to_lowercase() {
            "std" | "verify-rust-std" => Project::Std,
            "rfl" | "rust-for-linux" => Project::RustForLinux,
            "asterinas" => Project::Asterinas,
            _ => Project::Default,
        }
    }

    /// Each project may pin a toolchain, so safety-tool must match it.
    /// Must update the value once toolchain is updated from submodules.
    fn channel(self) -> &'static str {
        match self {
            Project::Std => "nightly-2025-09-09",
            Project::RustForLinux => "1.87",
            Project::Asterinas => "nightly-2025-02-01",
            Project::Default => todo!("Haven't decided which toolchain as default to support."),
        }
    }

    /// Content in rust-toolchain.toml file.
    fn rust_toolchain_str(self) -> String {
        let channel = self.channel();
        format!(
            r#"[toolchain]
channel = "{channel}"
components = ["llvm-tools", "rustc-dev", "rust-src", "rust-analyzer", "rustfmt", "clippy"]
"#
        )
    }

    fn write_rust_toolchain_toml(self) {
        const FILE: &str = "rust-toolchain.toml";
        let s = self.rust_toolchain_str();

        let mut file = std::fs::File::create(FILE).unwrap();
        file.write_all(s.as_bytes()).unwrap();
    }

    /// Returns a name in [features] of Cargo.toml.
    fn as_feature_name(self) -> &'static str {
        match self {
            Project::Std => "std",
            Project::RustForLinux => "rfl",
            Project::Asterinas => "asterinas",
            Project::Default => todo!(),
        }
    }

    fn from_feature_name(s: &str) -> Option<Self> {
        Some(match s {
            "std" => Project::Std,
            "rfl" => Project::RustForLinux,
            "asterinas" => Project::Asterinas,
            _ => return None,
        })
    }

    fn set_rust_analyzer_cargo_features(self) {
        const FILE: &str = "./.vscode/settings.json";
        const KEY: &str = "rust-analyzer.cargo.features";

        let ra_settins = {
            let mut opts = std::fs::OpenOptions::new();
            opts.read(true).write(true);
            opts.open(FILE).unwrap()
        };
        let mut r#override = false;

        let mut json: serde_json::Value = serde_json::from_reader(&ra_settins).unwrap();
        let features = json[KEY].as_array_mut().unwrap();
        for feat in features {
            // Replace old project feature by current one.
            let name = feat.as_str().unwrap();
            if let Some(old) = Self::from_feature_name(name) {
                if old != self {
                    let new = self.as_feature_name();
                    println!("[.vscode/settings.json] replace {KEY}: {name} => {new}");
                    *feat = new.into();
                    r#override = true;
                    break;
                }
            }
        }

        if r#override {
            // clear content and write new json
            (&ra_settins).seek(std::io::SeekFrom::Start(0)).unwrap();
            ra_settins.set_len(0).unwrap();
            serde_json::to_writer_pretty(ra_settins, &json).unwrap();
        }
    }
}
