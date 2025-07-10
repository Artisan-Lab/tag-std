fn main() {
    println!("{}", Toolchain::new().rust_toolchain_str());
}

#[derive(Clone, Copy, Debug)]
enum Toolchain {
    Std,
    RustForLinux,
    Asterinas,
    Default,
}

impl Toolchain {
    /// Specified by `-Fstd`, `-Frfl`, `-Fasterinas`, or nothing.
    fn new() -> Self {
        let is_enable = |var: &str| std::env::var(var).is_ok();
        if is_enable("CARGO_FEATURE_STD") {
            Toolchain::Std
        } else if is_enable("CARGO_FEATURE_RFL") {
            Toolchain::RustForLinux
        } else if is_enable("CARGO_FEATURE_ASTERINAS") {
            Toolchain::Asterinas
        } else {
            Toolchain::Default
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
            r#"
[toolchain]
channel = "{channel}"
components = ["llvm-tools", "rustc-dev", "rust-src", "rust-analyzer", "rustfmt", "clippy"]
"#
        )
    }
}
