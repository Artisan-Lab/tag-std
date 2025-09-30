## Origin of Safety Properties
This article explains the origin of safety properties, _i.e.,_ why there are so many safety properties that are not directly related to memory safety.
In brief, there exists a core set of safety properties. 
These properties give rise to other, derived properties depending on program semantics, which aligns with our
[tracing-based verification methodology](https://hxuhack.github.io/writting/unsafe-tracing).

### 1. Core Safety Properties
Rust is primarily concerned with memory safety.
Accordingly, the unsafe intrinsic features of Rust, such as raw pointer dereference, 
turning a raw pointer into a reference, access to static mutable variables, focus on memory access safety.
These features give rise to safety properties specifically related to [`pointer validity`](https://github.com/Artisan-Lab/tag-std/blob/main/primitive-sp.md#32-pointer-validity), [`layout`](https://github.com/Artisan-Lab/tag-std/blob/main/primitive-sp.md#31-layout), and [alias](https://github.com/Artisan-Lab/tag-std/blob/main/primitive-sp.md#34-alias). 
Other safety properties of the standard library defined in [primitive-sp](https://github.com/Artisan-Lab/tag-std/blob/main/primitive-sp.md) may also be considered core safety properties. 
For example, `ValidNum` is unrelated to memory access, but it can serve as a core safety property when used to prevent integer overflow (_e.g.,_ in [usize::unchecked_add](https://doc.rust-lang.org/std/primitive.usize.html#method.unchecked_add)).
Additionally, `ValidNum` can be used bound to memory access (_e.g.,_ in [slice::from_raw_parts()](https://doc.rust-lang.org/nightly/std/slice/fn.from_raw_parts.html)), which is considered a derived safety property. 
Therefore, a safety property can be both a core safety property and a derived safety property. Furthermore, when developing operating systems, new core safety properties can be introduced, particularly those related to hardware access.

Based on the core safety properties, additional safety properties can be derived.

### 2. Transformation of Safety Properties
Now, we explain how additional safety properties can be derived.
We use raw pointer dereference as an example.
When a raw pointer dereference is used as a left value, it requires the safety properties of `Dereferenceable` . 
The following program introduces a new property, `ValidNum`, derived from `Dereferenceable`, because `x <= 0` ensures no undefined behavior. 
```rust
#[safety::requires(ValidNum)]
unsafe fn foo(p: *mut i32, x: i32) {
    if x > 0 {
        #[safety::transform(Deref => ValidNum)]
        unsafe { *p = x }
    }
}
```
In this way, we can compose programs to transform a safety property into any other property.

### 3. Viability of the Tag-based Representation 

Theoretically, the space of possible safety properties is unbounded, and their complexity can be arbitrarily high due to the Turing-complete nature of such programs.
A critical question arises: can such properties be represented using safe tags? The answer is yes, for two reasons.

Firstly, the number of safety tags can in principle be unlimited, depending on the requirements, which is comparable to the number of safety properties. 
Nevertheless, each crate only needs a finite set of safety tags.

Secondly, safety tags serve as abbreviations of safety properties, which have already been shown to be expressible in natural language, as illustrated by those in the Rust standard library. This expressibility is possible because each function represents a meaningful abstraction rather than arbitrary randomized code. For example, Rust’s `String`-related features generally require the memory content to be valid [`UTF-8`](https://github.com/Artisan-Lab/tag-std/blob/main/primitive-sp.md).
`UTF-8` format is a well-known high-level abstraction.
This also explains why some safety properties are not explicit enough, such as those associated with certain system-related APIs (_e.g.,_ [env::set_var()](https://doc.rust-lang.org/nightly/std/env/fn.set_var.html)) or other less common functions.

