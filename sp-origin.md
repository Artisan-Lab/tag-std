## Origin of Safety Properties
This article explains the origin of safety properties, _i.e.,_ why there are so many safety properties that are not directly related to memory safety.
In brief, there exists a core set of safety properties. 
These properties give rise to other, derived properties depending on program semantics,which aligns with our
[tracing-based verification methodology](https://hxuhack.github.io/writting/unsafe-tracing).

### 1. Core Safety Properties
Rust is primarily concerned with memory safety.
Accordingly, the unsafe intrinsic features of Rust, such as raw pointer dereference, 
turning a raw pointer into a reference, access to static mutable variables, focus on memory access safety.
These features give rise to safety properties specifically related to [`pointer validity`](https://github.com/Artisan-Lab/tag-std/blob/main/primitive-sp.md#32-pointer-validity), [`layout`](https://github.com/Artisan-Lab/tag-std/blob/main/primitive-sp.md#31-layout), and [alias](https://github.com/Artisan-Lab/tag-std/blob/main/primitive-sp.md#34-alias). 
Other safety properties of the standard library defined in [primitive-sp](https://github.com/Artisan-Lab/tag-std/blob/main/primitive-sp.md) may also be considered core safety properties. 
For example, `ValidNum` is unrelated to memory access, but it can serve as a core safety property when used to prevent integer overflow (_e.g.,_ [unchecked_add](https://doc.rust-lang.org/std/primitive.usize.html#method.unchecked_add))
Additionally, `ValidNum` can be used bound to memory access, which is considered a derived safety property. 
Therefore, a safety property can be both a core safety property and a derived safety property.

Furthermore, when developing operating systems, new core safety properties are introduced, particularly those related to hardware access safety.
Other safety properties can be derived from these core properties. 

### 2. Transformation of Safety Properties
For example, raw pointer dereference as the left value requires the safety properties of `Dereferenceable` . 
The following program introduces a new property `ValidNum` based on `Dereferenceable`, because `x < 0` ensures no undefined behavior. 
```rust
#[safety::requires(ValidNum)]
unsafe fn foo(p: *mut i32, x: i32) {
    if x > 0 {
        #[safety::transform(Deref => ValidNum)]
        unsafe { *p = x }
    }
}
```
In this way, we can transform a safety property into any other properties that are computable by a Turing machine.

### 
