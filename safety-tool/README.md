# safety-tool

This workspace is intentionally trimmed to keep only one capability:

- define safety tags for unsafe APIs in Rust std

The following tag-definition crates are retained:

- `safety-parser`: parse tag specs and safety attributes
- `safety-macro`: `#[requires]` and `#[checked]` attribute macros
- `safety-lib`: re-export helper crate for using tag macros

All extra capabilities (cross-project adapters, compile-time checking pipeline,
statistics emission, and LSP client/server integration) are removed from the
default build path.

## Build

```bash
cargo build
```

## Example

```rust
#![feature(register_tool)]
#![register_tool(rapx)]

use safety_lib::{checked, requires};

#[requires { ValidPtr(p) }]
unsafe fn read_raw(p: *const u8) -> u8 {
	unsafe { *p }
}

fn demo(p: *const u8) -> u8 {
	#[checked { ValidPtr(p): "pointer is validated by caller" }]
	unsafe { read_raw(p) }
}
```
