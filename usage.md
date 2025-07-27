# Usage of Safety Properties in a Rust Project

## Import safety-lib

Generally, we want to have `#[safety]` namespace available in each module, so rename safety-lib
crate to safety as dependency in Cargo.toml:

```toml
safety = { version = "0.3.0", package = "safety-macro" }
```

## Safety Property Definition

The basic form is

```toml
[tag.Aligned]
args = [ "p", "T" ]
desc = "pointer `{p}` must be properly aligned for type `{T}`"
expr = "p % alignment(T) = 0"
url = "https://doc.rust-lang.org/nightly/std/ptr/index.html#alignment"
```

* Fields can be omitted to have default behavior, like types will default to `[Precond]`, args will
default to `[]`
* `desc` supports dynamic string by interpolating variables from arg names: 
  * e.g. for `desc = a {var} c`, and `args = ["var"]`, if user input is `SP(b)`, then 
    `#[doc = "a b c"]` will be emitted through proc-macro and rendered in rustdoc
  * `sp-core.toml` and `sp-rust-for-linux.toml` under `safety-tool/safety-tool` are examples to show
  how SPs should be defined.
* `SP_FILE=/path/to/single/toml` or `SP_DIR=/path/to/toml/foler` is recognized to enable code
  relying on tag definitions, such as tag checking and rustdoc rendering for desc
  * If both env var are given, only `SP_FILE` is used.
  * All toml files under `SP_DIR` will be merged into a SP map: SP must be only defined once,
    meaning duplicated SP names will panic.

## Unsafe API Annotation

Each unsafe API is associated with one or more safety properties, each of which is represented as an
attribute prefixed with the `safety` keyword. For example, the following three attributes declare
three safety properties:

```rust
use safety::safety;

#[safety { Align }] // lightweight tag 
#[safety { Align(p, T) }] // or verfication tag
pub unsafe fn foo<T>(p: T) { ... }
```

## Callsite Annotation

To facilitate reviewing the usage of unsafe APIs, developers can annotate how each safety property
is addressed as follows: 

```rust
#[safety { Align, CustomProperty: "reason is optional" }]
unsafe { call() }
```

## RustDoc Generation 

The safety attribute can be automatically expanded into text descriptions once configuration is set.


```rust
// SP_FILE=path/to/sp-core.toml

use safety::safety;
#[safety { Aligned(ptr, T) }]
pub unsafe fn foo<T>(ptr: T) { ... }
```

The generated doc is:

```rust
/// Aligned: pointer `ptr` must be properly aligned for type `T`.
```

![](https://github.com/user-attachments/assets/48ec3740-5a49-4afd-b17d-64bfc8b7e8e3)

