# Usage of Safety Properties in a Rust Project

## Import safety-lib

Generally, we want to have `#[safety]` namespace available in each module, so rename safety-lib crate to safety as dependency in Cargo.toml:

```toml
safety = { version = "0.2.1", package = "safety-lib" }
```

safety-lib is no_std, only as a wrapper around safe-macro the proc-macro crate, to provide path-based attibute access like `#[safety::precond::Prop]`
instead of raw `#[safety_lib::Precond_Prop]`. This is because proc-macro only allows root module access to macros.

## Unsafe API Annotation

Each unsafe API is associated with one or more safety properties, each of which is represented as an attribute prefixed with the `safety` keyword. 
For example, the following three attributes declare three safety properties:
```rust
// p is aligned: p % alignment(T) = 0
#[safety::precond::Align(p, T)]

// It's sound for `T: Copy`, but unsound if T is not Copy for some reason. 
// Example: https://doc.rust-lang.org/std/ptr/fn.read.html#ownership-of-the-returned-value
#[safety::option::Trait(T, Copy)]

// pointer p and returned pointer are aliased (i.e. they point to the same memory place)
#[safety::hazard::Alias(p, ret)]
```

Here, the middle keywords `precond`, `option`, and `hazard` correspond to the three types of safety properties defined in [primitive-sp](../primitive_sp.md). Users can also customize new properties or provide text descriptions about the property via `Memo`. 
```rust
#[safety::Memo(UserProperty, memo = "Customed user property.")]
```

## Callsite Annotation
To facilitate reviewing the usage of unsafe APIs, developers can annotate how each safety property is addressed as follows: 
```rust
#[safety::discharges(Align(p, T)]
#[safety::discharges(Memo(CustomProperty)]
#[safety::discharges(Memo(CustomProperty, memo = "reason is optional")]
```
We use the keyword `discharges` to indicate that the associated safety property has been satisfied.
with supporting justification provided via the `memo` keyword.

![](https://github.com/user-attachments/assets/06a3c34e-0687-4ad1-b822-e39bbf2d13f6)

## RustDoc Generation 
The safety attribute can be automatically expanded into text descriptions through a procedural macro library named [safety-lib](safety-tool/safety-lib). Take the following code as an example.
```rust
use safety_tool_lib::safety;
#[safety::precond::Align(p, T)]
```

The generated doc is shown below.
```rust
use safety_tool_lib::safety;
/// Align: Make sure pointer `p` must be properly aligned for type `T` before calling this function.
```

![](https://github.com/user-attachments/assets/17dfa26b-fa8f-4e96-9832-29722c5ded81)

