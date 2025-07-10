# safety-tool

A demo to show how safety properties can be checked on unsafe Rust code.

## Install

Several projects are checked, while each project may pin own toolchain.

So to support them in the same tool, conditional compilation gated by `--features` is needed.

Projects and feature names:

| project           | `--features` (or `-F`) |
|-------------------|------------------------|
| [verify-rust-std] | `std`                  |
| [Rust for Linux]  | `rfl`                  |
| [asterinas]       | `asterinas`            |

[verify-rust-std]: https://github.com/Artisan-Lab/rapx-verify-rust-std
[Rust for Linux]: https://github.com/Artisan-Lab/tag-rust-for-linux
[asterinas]: https://github.com/Artisan-Lab/tag-asterinas

There is no default toolchain for now, so one must set up it first and then build or install it.

For example, to check Rust for Linux codebase, specify `rfl` like this:

```bash
./gen_rust_toolchain_toml.rs rfl
cargo build -Frfl
```
