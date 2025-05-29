# Usage of Safety Properties in a Rust Project
## Unsafe API Annotation
Each unsafe API is associated with one or more safety properties, each of which is represented as an attribute prefixed with the `safety` keyword. 
For example, the following three attributes declare three safety properties:
```rust
#[safety::precond::Align(p, T)]
#[safety::option::Trait(T, Copy)]
#[safety::hazard::Alias(p, 0)]
```

Here, the middle keywords `precond`, `option`, and `hazard` correspond to the three types of safety properties defined in [primitive-sp](../primitive_sp.md). Users can also customize new properties or provide text descriptions about the property via `Memo`. 
```rust
#[safety::Memo(UserProperty, memo = "Customed user property.")]
```

## Callsite Annotation
To facilitate reviewing the usage of unsafe APIs, developers can annotate how each safety property is addressed as follows: 
```rust
#[safety::discharges(Align(p, T), memo = "...")]
#[safety::discharges(Alias(p, 0), "..."]
```
We use the keyword `discharges` to indicate that the associated safety property has been satisfied, 
with supporting justification provided via the `memo` keyword.

## RustDoc Generation 
The safety attribute can be automatically expanded into text descriptions through a procedural macro library named [safety-tool-lib](demo/safety-tool-lib). Take the following code as an example.
```rust
use safety_tool_lib::safety;
#[safety::precond::Align(p, T)]
```

The generated doc is shown below.
```rust
use safety_tool_lib::safety;
/// Align: Make sure pointer `p` must be properly aligned for type `T` before calling this function.
```



