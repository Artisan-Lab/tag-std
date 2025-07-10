#!/usr/bin/env -S RUSTC_BOOTSTRAP=1 cargo -Zscript
fn main() {
    // The first argument as a project name.
    // e.g. ./gen_rust_toolchain_toml.rs rfl
    let proj = std::env::args().nth(1).expect("Choose a project to generate rust-toolchain.toml");
    Toolchain::new(&proj).write_rust_toolchain_toml();
}

#[derive(Clone, Copy, Debug)]
enum Toolchain {
    Std,
    RustForLinux,
    Asterinas,
    Default,
}

impl Toolchain {
    /// Specified by `std`, `rfl`, `asterinas`, or nothing.
    fn new(proj: &str) -> Self {
        match &*proj.to_lowercase() {
            "std" | "verify-rust-std" => Toolchain::Std,
            "rfl" | "rust-for-linux" => Toolchain::RustForLinux,
            "asterinas" => Toolchain::Asterinas,
            _ => Toolchain::Default,
        }
    }

    /// Each project may pin a toolchain, so safety-tool must match it.
    /// Must update the value once toolchain is updated from submodules.
    fn channel(self) -> &'static str {
        match self {
            Toolchain::Std => "nightly-2025-06-02",
            Toolchain::RustForLinux => "1.87",
            Toolchain::Asterinas => todo!("Haven't supported Asterinas yet."),
            Toolchain::Default => todo!("Haven't decided which toolchain as default to support."),
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
        use std::io::Write;

        const FILE: &str = "rust-toolchain.toml";
        let s = self.rust_toolchain_str();

        let mut file = std::fs::File::create(FILE).unwrap();
        file.write_all(s.as_bytes()).unwrap();
    }
}
