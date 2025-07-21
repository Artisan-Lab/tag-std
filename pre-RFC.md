# pre-RFC: Safety Property System

# Summary
[summary]: #summary

This RFC proposes a DSL-based mechanism for specifying safety properties, aiming to standardize how safety descriptions are written in API documentation. On the one hand, it seeks to improve the ergonomics of writing safety descriptions; on the other hand, these safety properties can enable finer-grained unsafe code management and automated safety checking.

This RFC operates at the API level rather than the compiler or language level, as it merely introduces attribute macros on functions and expressions that are already expressible today, but may require a linter tool to realize automated check.

This RFC has influences on the entire crate ecosystem, including the standard library and downstream crates.

# Motivation
[motivation]: #motivation

To avoid the misuse of unsafe code, Rust developers are encouraged to provide clear safety comments for unsafe APIs. While these comments are generally human-readable, they can be ambiguous and laborious to write. Even the current best practices in the Rust standard library are somewhat ad hoc and informal. Moreover, safety comments are often repetitive and may be perceived as less important than the code itself, which makes them error-prone and increases the risk that reviewers may overlook inaccuracies or missing safety requirements.

For instance, a severe problem may arise if the safety requirements of an API change over time: downstream users may be unaware of such changes and thus be exposed to security risks. Therefore, we propose to improve the current practice of writing safety comments by making them checkable through a system of safety tags. These tags are designed to be:

* Compatible with existing safety documentation: Safety tags should be expressive enough to represent current safety comments, especially as rendered in today's rustdoc HTML pages.
* Usable by compiler tools for safety checking: If no safety tags are provided for an unsafe API, lints should be emitted to remind developers to provide safety requirements. If a safety tag is declared for an unsafe API but not discharged at a callsite, lints should be emitted to alert developers about potentially overlooked safety requirements.
* Versioned: When safety tags are revised, the changes should be propagated and checked across the entire dependency graph to address issues caused by the evolution of safety requirements.

# Guide-level Explanation
[guide-level-explanation]: #guide-level-explanation

## Terms: Safety Comments and Tags

In the following document, we use the term **safety comments** to refer to informal textual descriptions of safety properties or safety requirements that must be satisfied to ensure safety when using an unsafe API. This is the current form of safety descriptions used in Rust.

In contrast, **safety tags** represent safety properties using a formal language, i.e., a [tool attribute] written in the form `#[safety::type::Prop(args, ...)]` where
  - `safety` is a crate name or tool name,
  - `type` is one of `{precond, hazard, option}`,
      - precond denotes a safety requirement that must be satisfied before invoking an unsafe API. Most unsafe APIs carry at least one precondition.
      - hazard denotes invoking the unsafe API may temporarily leave the program in a vulnerable state; e.g. [`String::as_bytes_mut`].
      - option denotes an optional precondition for an unsafe API—conditions that are sufficient but not necessary to uphold the safety invariant. 
  - `Prop(args, ...)` is a safety property instance. For safety propeties in libcore and libstd, refer to [this document](https://github.com/Artisan-Lab/tag-std/blob/main/primitive-sp.md) and our ongoing [paper](https://arxiv.org/abs/2504.21312).

See the following usage of `ptr::read` as an example.

[tool attribute]: https://doc.rust-lang.org/reference/attributes.html#tool-attributes
[`String::as_bytes_mut`]: https://doc.rust-lang.org/std/string/struct.String.html#method.as_bytes_mut
[`ptr::read`]: https://doc.rust-lang.org/std/ptr/fn.read.html

## Turn Safety Comments into Safety Tags

Consider safety comments on [`ptr::read`]

```rust
/// # Safety
///
/// Behavior is undefined if any of the following conditions are violated:
///
/// * `src` must be [valid] for reads.
///
/// * `src` must be properly aligned. Use [`read_unaligned`] if this is not the
///   case.
///
/// * `src` must point to a properly initialized value of type `T`.
///
/// Note that even if `T` has size `0`, the pointer must be properly aligned.
/// 
/// ## Ownership of the Returned Value
///
/// `read` creates a bitwise copy of `T`, regardless of whether `T` is [`Copy`].
/// If `T` is not [`Copy`], using both the returned value and the value at
/// `*src` can violate memory safety. Note that assigning to `*src` counts as a
/// use because it will attempt to drop the value at `*src`.
pub const unsafe fn read<T>(src: *const T) -> T { ... }
```

We can extract safety requirements above into propeties below:

| Type    | Property | Arguments | Description                                                                              |
|---------|----------|-----------|------------------------------------------------------------------------------------------|
| Precond | ValidPtr | src, T, 1 | `src` must be [valid] for reads (for 1 * sizeof(T) bytes). |
| Precond | Aligned  | src, T    | `src` must be properly aligned (with T).  |
| Precond | Init     | src, T, 1 | `src` must point to a properly initialized value of type `T`. 
| Option  | Trait    | T, Copy   | If `T` is not [`Copy`], using both the returned value and the value at `*src` can violate memory safety. |
| Precond | NotOwned  | src       | Further clarification: The memory pointed by src must not be owned if T is not copy, or the object hold by *src should not be automatically dropped |
| Hazard | Alias  | src, ret      | Further clarification: The return value may incur aliases between src and the return value |

[valid]: https://doc.rust-lang.org/std/ptr/index.html#safety
[alignment]: https://doc.rust-lang.org/std/ptr/index.html#alignment
[`Copy`]: https://doc.rust-lang.org/std/marker/trait.Copy.html

We can represent these safety requirements using safety tags as shown below.

```rust
/// # Safety
#[safety::precond::ValidPtr(src, T, 1)]
#[safety::precond::Aligned(src, T)]
#[safety::precond::Init(src, T, 1)]
#[safety::any{
    precond::NotOwned(src),
    option::Trait(T, Copy)
}
#[safety::precond::Alias(src, ret)]
///
/// ## Ownership of the Returned Value
/// ...
///
/// [valid]: https://doc.rust-lang.org/std/ptr/index.html#safety
/// [aligned]: https://doc.rust-lang.org/std/ptr/index.html#alignment
pub const unsafe fn read<T>(src: *const T) -> T { ... }
```

Safety tags will take effect in two ways:
1. They will be expanded into `#[doc]` comments, which will be rendered through rustdoc on HTML pages.
2. They will be collected and analyzed by a linter tool. If no safety tags are provided for an unsafe API, lints should be emitted to remind developers to provide safety requirements. If a safety tag is declared for an unsafe API but not discharged at a call site, lints should be emitted to alert developers about potentially overlooked safety requirements.

## Discharge Safety Properties

Currently, a common practice when calling unsafe functions is to leave a brief safety comment explaining why it is safe to use the unsafe code. However, there is no clear guidance on safety justifications, and this practice is not mandatory. As a result, developers may end up repeatedly copying and pasting the same text or referring to the same comments. [For example][vec_deque]:

[vec_deque]: https://github.com/rust-lang/rust/blob/ebd8557637b33cc09b6ee8273f3154d5d3af6a15/library/alloc/src/collections/vec_deque/into_iter.rs#L104

```rust
// src: rust/library/alloc/src/collections/vec_deque/into_iter.rs

// impl<T, A: Allocator> Iterator for IntoIter<T, A>

fn try_fold<B, F, R>(&mut self, mut init: B, mut f: F) -> R {
    ...
    init = head.iter().map(|elem| {
        guard.consumed += 1;
        // SAFETY: Because we incremented `guard.consumed`, the deque effectively forgot the element, so we can take ownership
        unsafe { ptr::read(elem) }
    })
    .try_fold(init, &mut f)?;

    tail.iter().map(|elem| {
        guard.consumed += 1;
        // SAFETY: Same as above.
        unsafe { ptr::read(elem) }
    })
    .try_fold(init, &mut f)
}

fn try_rfold<B, F, R>(&mut self, mut init: B, mut f: F) -> R {
    ...
    init = tail.iter().map(|elem| {
        guard.consumed += 1;
        // SAFETY: See `try_fold`'s safety comment.
        unsafe { ptr::read(elem) }
    })
    .try_rfold(init, &mut f)?;

    head.iter().map(|elem| {
        guard.consumed += 1;
        // SAFETY: Same as above.
        unsafe { ptr::read(elem) }
    })
    .try_rfold(init, &mut f)
}
```

The example above demonstrates several issues:

* **Lack of clarity on safety requirements**: It is unclear whether the developer has considered all safety requirements for `ptr::read` and ensured they are satisfied. From the comments, we can see that only the `NotOwned` safety property is explicitly addressed.

* **Comment dependency and maintenance burden**: When a piece of safety documentation is modified, all places that reference it must be reconsidered and updated accordingly. In this example, `try_rfold` refers to the safety comments inside `try_fold`. If the safety comment within `try_fold` changes, developers might forget to verify whether the new comment still applies to `try_rfold`.
* **Implicit dependencies on unsafe behavior**: Developers may unknowingly change code that other safety assumptions rely on. For instance, the comment "the deque effectively forgot the element" depends on the behavior of Guard's Drop implementation. If `try_fold::Guard::drop` changes, developers must check whether the associated safety comments still hold. 

To address these issues, we propose a solution based on annotating call sites with `#[discharges]` and introducing an entity reference system.

```rust
fn try_fold<B, F, R>(&mut self, mut init: B, mut f: F) -> R {
    ...

    init = head.iter().map(|elem| {
        guard.consumed += 1;

        #[safety::discharges::ValidPtr(elem)]
        #[safety::discharges::Aligned(elem)]
        #[safety::discharges::Init(elem)]
        #[safety::discharges::Trait(T, Copy, memo = "
          Because we incremented `guard.consumed`, the deque 
          effectively forgot the element, so we can take ownership.
        ")]
        #[safety::referent(try_fold)]
        unsafe { ptr::read(elem) }
    })
    .try_fold(init, &mut f)?;

    ...
}
```

`#[discharges]` must correspond to each safety property on the called unsafe API, if
any property is missing, the linter will emit warnings or errors:

```rust
error: `ValidPtr`, `Aligned`, `Init` are not discharged,
       refer to `core::ptr::read`'s document or safety propeties for their meanings.
   --> rust/library/alloc/src/collections/vec_deque/into_iter.rs:xxx:xxx
    |
LLL | unsafe { ptr::read(elem) }
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ For this unsafe call.
    |
    = NOTE: ValidPtr 👉 https://doc.rust-lang.org/std/ptr/index.html#safety
    = NOTE: Aligned 👉 https://doc.rust-lang.org/std/ptr/index.html#alignment
    = NOTE: Init 👉 The pointer must be initialized before calling `core::ptr::read`
```

To avoid verbosity, we propose `#[referent]` for entity definition and `#[ref]` for entity
reference.

```rust
fn try_fold<B, F, R>(&mut self, mut init: B, mut f: F) -> R
    ...
    impl<'a, T, A: Allocator> Drop for Guard<'a, T, A> {
        #[safety::ref::try_fold] // 💡
        fn drop(&mut self) { ... }
    }
    
    ...

    init = head.iter().map(|elem| {
        guard.consumed += 1;

        #[safety::discharges::ValidPtr(elem)]
        #[safety::discharges::Aligned(elem)]
        #[safety::discharges::Init(elem)]
        #[safety::discharges::Trait(T, Copy, memo = "
          Because we incremented `guard.consumed`, the deque 
          effectively forgot the element, so we can take ownership.
        ")]
        #[safety::referent(try_fold)] // 👈 entity definition
        unsafe { ptr::read(elem) }
    })
    .try_fold(init, &mut f)?;

    tail.iter().map(|elem| {
        guard.consumed += 1;

        #[safety::ref::try_fold] // 💡 No longer to write SAFETY: Same as above.
        unsafe { ptr::read(elem) }
    })
    .try_fold(init, &mut f)
}

fn try_rfold<B, F, R>(&mut self, mut init: B, mut f: F) -> R {
    ...
    impl<'a, T, A: Allocator> Drop for Guard<'a, T, A> {
        #[safety::ref::try_fold] // 💡
        fn drop(&mut self) { ... }
    }
    
    ...

    init = tail.iter().map(|elem| {
            guard.consumed += 1;
            
            #[safety::ref::try_fold] // 💡 No longer to write SAFETY: See `try_fold`'s safety comment.
            unsafe { ptr::read(elem) }
        })
        .try_rfold(init, &mut f)?;

    head.iter().map(|elem| {
            guard.consumed += 1;
            
            #[safety::ref::try_fold] // 💡 No longer to write SAFETY: Same as above.
            unsafe { ptr::read(elem) }
        })
        .try_rfold(init, &mut f)
}
```

If referent is not defined or collides, hard error is emitted.

Once safety propeties on referent changes, we can know all relevant places (e.g. lines with 💡 emoji),
and estimate safety requirements fulfillment on referrers.

## Versions of a tag

<a id="semver-tag"></a>

We should notice entity reference system handles two versions of tags from the above example!

When a tag is newly introduced on an API, discharge detection applies.

When a revised tag occurs on an API, discharge detection still applies, and a complete 
report on tagged places including referencing places should be provided. If local tags
are affected by the revised tag from upstream crate, propagation analysis should extend 
from culprit crate to the whole dependency graph.

It's worth noting that this is unlike [semver] checks on crate's APIs. Reason are 
* core or similar builtin libraries are not versioned. Even if these crates are tied to 
  specific rust toolchain, toolchain version doesn't and is unable to reflect version 
  of builtin libraries.
* adding a new tag breaks downstream crates due to discharge detection, while adding a 
  new API is usually not a braking change.
* tags are public across all crates, if an upstream tag is removed, all downstream crates 
  need to remove it accordingly.

[semver]: https://doc.rust-lang.org/cargo/reference/semver.html

So making tags versioned is a big challenge. On the one hand, we want tags to be part of 
APIs and semver controlled, on the other hand, any change in tags results in high churn.

This RFC suggests reporting diffs on versions of tags, in warnings or errors at user option,
but doesn't provide any solution to churn. That's to say, it's unclear whether safety 
propeties should be semver checked or not.

# Reference-level explanation
[reference-level-explanation]: #reference-level-explanation

Since this RFC doesn't require too much new features from Rust compiler or language,
implementations in this section are tool specific and focus on syntax.

Take one of safety tag on `ptr::read` as an example:

```rust
#[safety::precond::ValidPtr(src)]
```

It's a proc-macro, but reexported in a lib crate, becuase only root path is accessible
in proc-macro crate. We have to reexport it in a nested module outside:

```rust
// proc-macro crate: safety_macro/src/lib.rs
#[proc_macro_attribute]
pub fn Precond_ValidPtr(attr: TokenStream, item: TokenStream) -> TokenStream { ... }

// normal lib crate: safety_lib/src/lib.rs
pub mod precond {
    pub use safety_macro::Precond_ValidPtr as ValidPtr;
}
```

The user can import the lib crate through Cargo.toml:

```toml
# The following means renaming safety-lib to safety as your dependency
safety = { version = "0.0.1", package = "safety-lib" }
```

`#[safety::precond::ValidPtr]` now is available in your crate, and it's autocompleted
by RA if you type `#[safety::]` or something.

Thanks to proc-macro crate being host-target only, we can also make it work in no_std projects,
and even non-Cargo projects like Rust for Linux. See [tag-std#24] if you're interested. 

[tag-std#24]: https://github.com/Artisan-Lab/tag-std/pull/24

The proc macro expands to two attributes:

```rust
#[doc = "`src` must be [valid] for reads.\n\n[valid]: https://doc.rust-lang.org/std/ptr/index.html#safety"]
#[safety_tool::inner(property = ValidPtr(src), kind = "precond")]
```

* `#[doc]` is a safety comment, possibly with extra argument infomation interpolated into the text.
* `#[safety_tool]` is a [tool attribute], and `inner(...)` all path segments following it is basically
  verbatim passed to and interpreted by linter tool.

The second attribute requires [register_tool](https://github.com/rust-lang/rfcs/pull/3808) to be stablized.
At the moment, users must add these features in root module:

```rust
#![feature(register_tool)]
#![register_tool(safety_tool)]
```

or equivalently add them to [`--crate-attr`](https://github.com/rust-lang/rfcs/pull/3791) compiler flag
which also needs to stablize:

```bash
rustc --crate-attr="feature(register_tool)" --crate="register_tool(safety_tool)"
```

For `#[discharges]`, more unstable features are required to support attributes on satements and expression:

```rust
#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]
```

Details of implementation on reference entity system belongs to the linter tool.

# Drawbacks
[drawbacks]: #drawbacks

* cover too many unsafe APIs
  * need a lot of efforts on inital safety tags
* semver compatibility churn (see [above](#semver-tag))
* inadequacy of unsafe operation semantics
  * frequent change on some safety propeties
  * not sure if all safety propeties are composable
* tools coupling
  * it's less readable in source code around safety tags, and must turn to rustdoc or LSP server 
    for help to know safety requirements in plain text

# Rationale and alternatives
[rationale-and-alternatives]: #rationale-and-alternatives

## Alternatives from URLO

There are alternative discussion or Pre-RFCs on URLO:

* 2023-10: [Ability to call unsafe functions without curly brackets](https://internals.rust-lang.org/t/ability-to-call-unsafe-functions-without-curly-brackets/19635/22)
  * this is a discussion about make single unsafe call simpler, so the idea evolved into tczajka's Pre-RFC
  * but the idea and syntax from scottmcm's comments are very enlightening to our RFC
* 2024-10: [Detect and Fix Overscope unsafe Block](https://internals.rust-lang.org/t/detect-and-fix-overscope-unsafe-block/21660/19) 
  * the OP is about safe code scope in big unsafe block, which is not discussed in our RFC
  * but scottmcm's comments are good inspiration for our RFC
* 2024-12: [Pre-RFC: Unsafe reasons](https://internals.rust-lang.org/t/pre-rfc-unsafe-reasons/22093) proposed by chrefr
  * good improvement on abstracting safety comments to single identifier that is machine readable and checkable,
    but doesn't specify arguments and string interpolation to be more fine-grained on unsafe reasons
  * big request on language and compiler change, while safety tags in our RFC is lightweight
* 2025-02: [Pre-RFC: Single function call `unsafe`](https://internals.rust-lang.org/t/pre-rfc-single-function-call-unsafe/22343) proposed by tczajka
  * single unsafe call is a good practice, but postfix `.unsafe` needs more compiler supports but doesn't suggest any improvement on safe comments 
  * our RFC supports annotating safety tags on any expression including single calls
* 2025-05: [Pre-RFC: Granular Unsafe Blocks - A more explicit and auditable approach](https://internals.rust-lang.org/t/pre-rfc-granular-unsafe-blocks-a-more-explicit-and-auditable-approach/23022) proposed by Redlintles
  * safety categories suggested are too broad
  * while safety propeties in our RFC are more granular and semantics-specifc
* 2025-07: [Unsafe assertion invariants](https://internals.rust-lang.org/t/unsafe-assertion-invariants/23206)
  * good idea to embed safety requirements/contract/information into doc comments, which is similar to one of the goals in our RFC

## Alternatives from Rust for Linux

More importantly, our proposal is a big improvement to these proposals, which Rust for Linux care more about:
* 2024-09: [Rust Safety Standard: Increasing the Correctness of unsafe Code][Rust Safety Standard] proposed by Benno Lossin
  * this slides are about reasons and goals for safety documentation standardization, which our proposal tries to achieve
  * it doesn't mention how the standard is implemented, but Predrag (see the next line) and we follow the spirit
* 2024-10: [Automated checking of unsafe code requirements](https://hackmd.io/@qnR1-HVLRx-dekU5dvtvkw/SyUuR6SZgx) proposed by Predrag
  * our proposal is greatly inspired by Predrag's, so many of it can apply to ours, such as structured comments,
    entity reference, requirements discharge, and handling soundness hazard on safety rule changes. 
  * the main difference is syntax: Predrag put up new syntax within doc and line comments, which is pretty human and machine readable,
    but can be hard to implement as compiler just throws aways line comments so it's less handy to get safe rules on an expression
    than [`stmt_expr_attributes`](https://github.com/rust-lang/rust/issues/15701).
  * his proposal doesn't mention arguments support in safety rules, meaning we don't know how a pointer safety rule can apply to two
    pointers function arguments without ambiguity.

Originally, we only focus on libstd's common safety propeties ([paper]), but noticed the RustWeek meeting note
[Function contracts and type invariants specification](https://hackmd.io/@qnR1-HVLRx-dekU5dvtvkw/SyUuR6SZgx) in zulipchat. 
Thus [tag-std#3](https://github.com/Artisan-Lab/tag-std/issues/3) is opened to support Rust for Linux on safety standard.

[Rust Safety Standard]: https://kangrejos.com/2024/Rust%20Safety%20Standard.pdf
[paper]: https://arxiv.org/abs/2504.21312

# Prior art
[prior-art]: #prior-art

Currently, there are efforts on introducing contracts and formal verification into Rust:
* [contracts](https://rust-lang.github.io/rust-project-goals/2024h2/Contracts-and-invariants.html): the lang experiment has been
  implemented since [rust#128044](https://github.com/rust-lang/rust/issues/128044).
* [verify-rust-std] pursues applying formal verification tools to libstd. Also see Rust Foundation [announcement][vrs#ann],
  project goals during [2024h2] and [2025h1].

Our proposal "safety property system" also follows [design by contract](https://en.wikipedia.org/wiki/Design_by_contract), especially on
* A clear metaphor to guide the design process
* The connection with automatic software documentation

Nonetheless, safety property is of static semantics, unlike other verification tools which tends to employ symbolic execution
and be dynamic in some ways. Also, safety property is based on current safety comment practices, thus Rustaceans may feel 
more familiar.

[verify-rust-std]: https://github.com/model-checking/verify-rust-std
[2024h2]: https://rust-lang.github.io/rust-project-goals/2024h2/std-verification.html
[2025h1]: https://rust-lang.github.io/rust-project-goals/2025h1/std-contracts.html
[vrs#ann]: https://foundation.rust-lang.org/news/rust-foundation-collaborates-with-aws-initiative-to-verify-rust-standard-libraries/

# Unresolved questions
[unresolved-questions]: #unresolved-questions

* semver of safety propeties: see [versions of a tag](#semver-tag) above.
* order requirements on invocation: it's also common to clarify an unsafe operation must be performed once,
  or some unsafe operation must be followed by or precede another. Our proposal may well support this by 
  extending entity reference system and control-flow analysis. Tracked in [tag-std#29].
* handle type erasure: we haven't think about calls through unsafe fn pointer or `dyn Trait`.

[tag-std#29]: https://github.com/Artisan-Lab/tag-std/issues/29

# Future possibilities
[future-possibilities]: #future-possibilities

## Interaction with Rust type system

Arguments in a property can be any expression, and sometimes the type of argument must be known
in analysis and doc comments:

```rust
// Syntax1: we don't need to query type if user is asked to provide it.
//          But we're responsible to check the given type is valid!
//          So this means we have to reach type systems anyway.
#[safety::precond::Aligned(p, T)]
// Syntax2: we must get type info from rustc.
#[safety::precond::Aligned(p)]
unsafe fn read<T>(src: *const T) {}
```

The generic type `T` will be rendered in `#[doc]`, so it'd be tricky if the type needs 
[normalization] or trait bounds analysis. It happens to be the case that `ptr::read`
has a safety property `#[option::Trait(T, Copy)]`.

[normalization]: https://rustc-dev-guide.rust-lang.org/normalization.html

Because attributes on expression are only available in HIR, is type fully normalized at 
this stage? I guess no.

Trait solver may be involved, due to trait bounds analysis in safety property: if we
hope to do better on `#[option::Trait(T, Copy)]`, each call of read on non-Copy T should 
requires a safety reason.

## Dynamic safety properties

The reason to have dynamically generated propeties is that we are unable to write 
an attribute library that can meet all unsafe code.

Low-level crates probably requires their own safety propeties more than libstd defines.

The core idea is a project-aware configuration file, in toml or json format, mapping
property name, arguments, and description (including string interpolation). When
compiling safety-macro, its build.rs will read the project mapping, and auto generate macros.
(Suppose we don't have [reflection and comptime][reflection-comptime] any time soon.)

[reflection-comptime]: https://github.com/rust-lang/rust-project-goals/pull/311

We're trying to experiment on this though, as Asterinas OS wants this.
Feel free to drop by [tag-std#26](https://github.com/Artisan-Lab/tag-std/issues/26).

## Better development, review, and audit experience with more toolings

We're also considering implmenting such tools for better experience:
* a LSP server to analyze safety properties and offer safety attributes autocompletion
* a [SARIF](https://sarifweb.azurewebsites.net/) adaptor and code scanning workflow
on Github PR/Security ([e.g.][sarif-rs]).

[sarif-rs]: https://psastras.github.io/sarif-rs/docs/getting-started/introduction/

