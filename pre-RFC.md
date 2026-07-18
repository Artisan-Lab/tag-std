# pre-RFC: Safety Property System

# Summary
[summary]: #summary

This RFC proposes a DSL (domain-specific language)-based mechanism for specifying safety properties,
aiming to standardize how safety descriptions are written in API documentation. On the one hand, it
seeks to improve the ergonomics of writing safety descriptions; on the other hand, these safety
properties can enable finer-grained unsafe code management and automated safety checking.

This RFC operates at the API level rather than the compiler or language level, as it merely
introduces attribute macros on functions and expressions that are already expressible today, but may
require a linter tool to realize automated check.

This RFC has influences on the entire crate ecosystem, including the standard library and downstream
crates.

# Motivation
[motivation]: #motivation

To avoid the misuse of unsafe code, Rust developers are encouraged to provide clear safety comments
for unsafe APIs. While these comments are generally human-readable, they can be ambiguous and
laborious to write. Even the current best practices in the Rust standard library are somewhat ad hoc
and informal. Moreover, safety comments are often repetitive and may be perceived as less important
than the code itself, which makes them error-prone and increases the risk that reviewers may
overlook inaccuracies or missing safety requirements.

For instance, a severe problem may arise if the safety requirements of an API change over time:
downstream users may be unaware of such changes and thus be exposed to security risks. Therefore, we
propose to improve the current practice of writing safety comments by making them checkable through
a system of safety tags. These tags are designed to be:

* Compatible with existing safety documentation: Safety tags should be expressive enough to
  represent current safety comments, especially as rendered in today's rustdoc HTML pages.
* Usable by compiler tools for safety checking: If no safety tags are provided for an unsafe API,
  lints should be emitted to remind developers to provide safety requirements. If a safety tag is
  declared for an unsafe API but not discharged at a callsite, lints should be emitted to alert
  developers about potentially overlooked safety requirements.
* Versioned: When safety tags are revised, the changes should be propagated and checked across the
  entire dependency graph to address issues caused by the evolution of safety requirements.

# Guide-level Explanation
[guide-level-explanation]: #guide-level-explanation

## Terms: Safety Comments and Tags

In the following document, we use the term **safety comments** to refer to informal textual
descriptions of safety properties or safety requirements that must be satisfied to ensure safety
when using an unsafe API. This is the current form of safety descriptions used in Rust.

In contrast, **safety tags** represent safety properties using a formal language, i.e., a
[tool attribute] namespace `safety` with sub-attributes for each contract category:

- `#[safety::requires(Prop1, Prop2, ...)]` — **preconditions**: safety requirements that must be
  satisfied before invoking an unsafe API. Most unsafe APIs carry at least one precondition.
  By default, every property in `requires` is a precondition. A property tagged with
  `kind = "hazard"` denotes that invoking the unsafe API may temporarily leave the program
  in a vulnerable state with respect to Rust's safety invariants (e.g. [`String::as_bytes_mut`]).
- `#[safety::invariant(Prop)]` — **struct invariants**: properties that must hold for every instance
  of a struct at all observable points.
- `#[safety::verify]` — marks a function as a verification entry point.
- `#[safety::ensures(Prop)]` — **postconditions**: properties guaranteed after the function returns
  (reserved for future use).
- A property group can carry an optional `kind = "..."` tag for fine-grained categorization:
  ```rust
  #[safety::requires(
    (ValidPtr(ptr, T, 1), Align(ptr, T), kind = "precond"),
    (Alias(ptr, ret), kind = "hazard"),
  )]
  ```
  When `kind` is omitted, `"precond"` is the default for `requires`, `"hazard"` for `hazard`.
- `/ *Prop */` — **inline discharge** on a callsite, discharging specific safety properties of the
  callee (see [Discharge Safety Properties](#discharge-safety-properties)).
- When multiple properties share the same kind, they are grouped with commas inside a single
  attribute rather than repeated:
  ```rust
  // Preferred (merged)
  #[safety::requires(ValidPtr(ptr, T, 1), Align(ptr, T), Init(ptr, T, 1))]
  // Equivalent but verbose
  // #[safety::requires(ValidPtr(ptr, T, 1))]
  // #[safety::requires(Align(ptr, T))]
  // #[safety::requires(Init(ptr, T, 1))]
  ```
- `Prop` is a safety property (SP) instance. For safety properties in libcore and libstd,
  refer to [this document][primitive-sp] and our ongoing [paper].

Here are some basic syntax examples:

```rust
#[safety::requires(SP)]
#[safety::requires(SP1, SP2)]
#[safety::requires((SP1, kind = "precond"), (SP2, kind = "hazard"))]
#[safety::invariant(SP1, SP2)]
#[safety::invariant(any(Null(p), (ValidPtr(p, T, 1), Align(p, T))))]
#[safety::verify]
#[safety::ensures(SP)]
```

### The `any(...)` Combinator

For properties that admit alternative states, the `any(...)` combinator expresses a logical OR
between disjuncts, where commas inside each parenthesized disjunct mean logical AND:

```text
any(D1, D2)
any(Null(p), (P1(p, ...), P2(p, ...)))
```

The primary use case is a **null guard**: when a pointer may legally be null, `any(Null(p), ...)`
declares that the conjunct properties only need to hold when `p` is non-null. This is the
raw-pointer counterpart of `Option` invariants:

```rust
// A linked list node whose `next` field may be null
#[safety::invariant(any(
    Null(self.next),
    (ValidPtr(self.next, Node, 1), Align(self.next, Node))
))]
pub struct Node {
    value: u32,
    next: *mut Node,
}
```

In the `ptr::read` example, the `Owning` precondition and `Trait(T, Copy)` advisory can be grouped
under `any` because either `T: Copy` or `Owning(src)` must hold — non-Copy types require ownership
transfer:

```rust
#[safety::requires(
    ValidPtr(src, T, 1),
    Aligned(src, T),
    Init(src, T, 1),
    Alias(src, ret),
    any(Owning(src), Trait(T, Copy)),
)]
pub const unsafe fn read<T>(src: *const T) -> T { ... }
```

We can define the annotation language with context-free grammar as follows:

```text
SafetyAnnotation => '#' '[' 'safety' '::' attr '(' SPGroups ')' ']'
attr      => 'requires' | 'invariant' | 'ensures' | 'verify'
SPGroups  => SPGroup (',' SPGroup)*
SPGroup   => '(' SPList (',' 'kind' '=' STRING)? ')'
           | SPList
SPList    => SPItem (',' SPItem)*
SPItem    => ID ('(' Arg (',' Arg)* ')')?
           | 'any' '(' Disjunct (',' Disjunct)* ')'
Disjunct  => SPItem
           | '(' SPList ')'
ID        => [A-Z][A-Za-z]*
Arg       => expression | type
```

See the following usage of `ptr::read` as a full example.

[tool attribute]: https://doc.rust-lang.org/reference/attributes.html#tool-attributes
[`String::as_bytes_mut`]: https://doc.rust-lang.org/std/string/struct.String.html#method.as_bytes_mut
[`ptr::read`]: https://doc.rust-lang.org/std/ptr/fn.read.html
[primitive-sp]: https://github.com/Artisan-Lab/tag-std/blob/main/primitive-sp.md
[paper]: https://arxiv.org/abs/2504.21312

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

| Category      | Property | Arguments   | Description                                                                                                                                         |
|---------------|----------|-------------|-----------------------------------------------------------------------------------------------------------------------------------------------------|
| `requires`    | ValidPtr | src, T, 1   | `src` must be [valid] for reads (for 1 * sizeof(T) bytes).                                                                                          |
| `requires`    | Aligned  | src, T      | `src` must be properly aligned (with T).                                                                                                            |
| `requires`    | Init     | src, T, 1   | `src` must point to a properly initialized value of type `T`.                                                                                       |
| `requires`    | Alias    | src, ret    | The return value may incur aliases between src and the return value (informational).                                                                 |
| `requires`    | any      | Owning, Trait(T, Copy) | Either `src` must be uniquely owned (non-Copy types), or `T: Copy` must hold.                                                            |

[valid]: https://doc.rust-lang.org/std/ptr/index.html#safety
[alignment]: https://doc.rust-lang.org/std/ptr/index.html#alignment
[`Copy`]: https://doc.rust-lang.org/std/marker/trait.Copy.html

We can represent these safety requirements using safety tags as shown below.

```rust
#[safety::requires(
    ValidPtr(src, T, 1),
    Aligned(src, T),
    Init(src, T, 1),
    Alias(src, ret),
    any(Owning(src), Trait(T, Copy)),
)]
pub const unsafe fn read<T>(src: *const T) -> T { ... }
```

Safety tags will take effect in two ways:
1. They will be expanded into `#[doc]` comments, which will be rendered through rustdoc on HTML
   pages.
2. They will be collected and analyzed by a linter tool. If no safety tags are provided for an
   unsafe API, lints should be emitted to remind developers to provide safety requirements. If a
    safety tag is declared for an unsafe API but not discharged at a call site, lints should be
    emitted to alert developers about potentially overlooked safety requirements.

<details>

## Define Safety Properties in Toml Configuration

SPs can be defined in TOML files to perform checks on user inputs and generate doc comments.

An example definition of an SP is as follows:

```toml
[tag.Aligned]
args = [ "p", "T" ]
desc = "pointer `{p}` must be properly aligned for type `{T}`"
expr = "p % alignment(T) = 0"
url = "https://doc.rust-lang.org/nightly/std/ptr/index.html#alignment"
```

We defined a property called `Aligned`, which includes two arguments, a dynamic description derived
from user input and some other fields. All fields are optional.

When `#[safety::requires(Aligned(src, T))]` is used, a corresponding doc comment is generated:

```rust
#[doc = "pointer `src` must be properly aligned for type `T`"]
```

For detailed usage and examples, refer to [tag-std#35].

![](https://github.com/user-attachments/assets/48ec3740-5a49-4afd-b17d-64bfc8b7e8e3)

[tag-std#35]: https://github.com/Artisan-Lab/tag-std/pull/35

## Safety Properties with Arguments for Verification

We also support SPs with arguments, which are required in verification scenarios.

```rust
#[safety::requires(
    ValidPtr(src, T, 1),
    Aligned(src, T),
    Init(src, T, 1),
    Alias(src, ret),
    any(Owning(src), Trait(T, Copy)),
)]
pub const unsafe fn read<T>(src: *const T) -> T { ... }
```

Most users do not need to write these arguments, unless they are running additional experimental
Safety Property Verification using RAPx (Rust Analysis Platform extended). For more details, see
[this chapter][RAPx-SP] of the RAPx book.

[RAPx-SP]: https://artisan-lab.github.io/RAPx-Book/6.4-unsafe.html

## Discharge Safety Properties

Currently, a common practice when calling unsafe functions is to leave a brief safety comment
explaining why it is safe to use the unsafe code. However, there is no clear guidance on safety
justifications, and this practice is not mandatory. As a result, developers may end up repeatedly
copying and pasting the same text or referring to the same comments. [For example][vec_deque]:

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

* **Lack of clarity on safety requirements**: It is unclear whether the developer has considered all
safety requirements for `ptr::read` and ensured they are satisfied. From the comments, we can see
that only the `Owning` safety property is explicitly addressed.

* **Comment dependence and maintenance burden**: When a piece of safety documentation is modified,
all places that reference it must be reconsidered and updated accordingly. In this example,
`try_rfold` refers to the safety comments inside `try_fold`. If the safety comment within `try_fold`
changes, developers might forget to verify whether the new comment still applies to `try_rfold`.
(This is not the focus of this RFC, but see [versions of a tag](#semver-tag) for our thought.)
  
* **Implicit dependence on unsafe behavior**: Developers may unknowingly change code that other
safety assumptions rely on. For instance, the comment "the deque effectively forgot the element"
depends on the behavior of Guard's Drop implementation. If `try_fold::Guard::drop` changes,
developers must check whether the associated safety comments still hold. (This RFC does not address
this problem, but see [Entity Reference System](#reference-entity) for our thought.)

To address the first issue, we propose a solution based on annotating safety tags on callsites using
inline `/* property */` discharge comments or `#[safety::verify]` for automated verification.

```rust
fn try_fold<B, F, R>(&mut self, mut init: B, mut f: F) -> R {
    ...

    init = head.iter().map(|elem| {
        guard.consumed += 1;

        // SAFETY: Because we incremented `guard.consumed`, the deque
        //         effectively forgot the element, so we can take ownership.
        /* ValidPtr(elem, T, 1), Aligned(elem, T), Init(elem, T, 1), Owning(elem) */
        unsafe { ptr::read(elem) }
    })
    .try_fold(init, &mut f)?;

    ...
}
```

`#[safety]` must correspond to each safety property on the called unsafe API, if
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

# Reference-level Explanation
[reference-level-explanation]: #reference-level-explanation

Since this RFC does not require significant changes to the Rust compiler or language, the
implementation details discussed in this section are tool-specific and primarily focus on syntax.

Take one safety tag on `ptr::read` as an example:

```rust
// The `safety` tool attribute is registered via
// #![feature(register_tool)]
// #![register_tool(safety)]
//
// or injected through RUSTFLAGS:
// RUSTFLAGS="--cfg=safety -Zcrate-attr=feature(register_tool) -Zcrate-attr=register_tool(safety)"

#[safety::requires(ValidPtr(src, T, 1), Aligned(src, T), Init(src, T, 1))]
```

The attribute can expand to multiple downstream annotations for different tools. For example,
`#[safety::requires(ValidPtr(src, T, 1), Aligned(src, T), Init(src, T, 1))]` on `ptr::read` could
generate:

```rust
#[doc = "`src` must be [valid] for reads.\n\n[valid]: https://doc.rust-lang.org/std/ptr/index.html#safety"]
#[rapx::requires(ValidPtr(src, T, 1), Aligned(src, T), Init(src, T, 1))]
#[kani::requires(kani::mem::can_dereference(src))]
```

* `#[doc]` is a safety comment, possibly with extra argument information interpolated into the text.
* `#[kani]` is a [contract]. If the safety property has a counterpart in an external verification
  tool such as kani, we hope to support this feature in the future.
* `#[rapx::requires(...)]` is a [tool attribute] processed by the RAPx verifier. The
  `register_tool` feature is needed, which can be provided via source annotation or compiler
  flags: 

[contract]: https://model-checking.github.io/kani/reference/experimental/contracts.html
[register_tool]: https://github.com/rust-lang/rfcs/pull/3808

```rust
#![feature(register_tool)]
#![register_tool(safety)]
```

or add them to [`--crate-attr`](https://github.com/rust-lang/rfcs/pull/3791) compiler flag:

```bash
rustc --crate-attr="feature(register_tool)" --crate-attr="register_tool(safety)"
```

To support inline `/* property */` discharge comments, the RAPx driver parses comments
adjacent to unsafe call sites without requiring additional unstable features.

Details of implementation on reference entity system belongs to the linter tool.

# Drawbacks
[drawbacks]: #drawbacks

* This proposal applies to most unsafe APIs and requires significant effort to replace existing
  safety comments with safety tags. However, it can be implemented incrementally.
* It is unclear whether all safety properties are composable, and some properties may change
  frequently in the early stages. Our initial investigation shows that the idea works well for the
  standard library.
* Safety tags may be less readable than the original safety comments. However, their readability
  should be comparable when rendered in rustdoc or surfaced through the LSP server.

# Rationale and alternatives
[rationale-and-alternatives]: #rationale-and-alternatives

## Alternatives from IRLO

There are alternative discussion or Pre-RFCs on IRLO:

* 2023-10: [Ability to call unsafe functions without curly brackets](https://internals.rust-lang.org/t/ability-to-call-unsafe-functions-without-curly-brackets/19635/22)
  * This is a discussion about make single unsafe call simpler, so the idea evolved into tczajka's Pre-RFC.
  * But the idea and syntax from Scottmcm's comments are very enlightening to our RFC.
* 2024-10: [Detect and Fix Overscope unsafe Block](https://internals.rust-lang.org/t/detect-and-fix-overscope-unsafe-block/21660/19) 
  * The OP is about safe code scope in big unsafe block, which is not discussed in our RFC.
  * But scottmcm's comments are good inspiration for our RFC.
* 2024-12: [Pre-RFC: Unsafe reasons](https://internals.rust-lang.org/t/pre-rfc-unsafe-reasons/22093) proposed by chrefr
  * This is a good improvement on abstracting safety comments into a single, machine-readable and
    checkable identifier. However, it doesn't specify arguments and lacks more fine-grained string
    interpolation for detailing unsafe reasons.
  * It also requests big changes on language and compiler change, while safety tags in our RFC is lightweight
* 2025-02: [Pre-RFC: Single function call `unsafe`](https://internals.rust-lang.org/t/pre-rfc-single-function-call-unsafe/22343) proposed by tczajka
  * The practice of using a single unsafe call is good, but the postfix `.unsafe` requires more
    compiler support and does not offer suggestions for improving safe comments.
  * Our RFC, however, supports annotating safety tags on any expression, including single calls.
* 2025-05: [Pre-RFC: Granular Unsafe Blocks - A more explicit and auditable approach](https://internals.rust-lang.org/t/pre-rfc-granular-unsafe-blocks-a-more-explicit-and-auditable-approach/23022) proposed by Redlintles
  * The safety categories suggested are overly broad. In contrast, the safety properties outlined in
    our RFC are more granular and semantics-specific.
* 2025-07: [Unsafe assertion invariants](https://internals.rust-lang.org/t/unsafe-assertion-invariants/23206)
  * It’s a good idea to embed safety requirements into doc comments, which aligns with one of the
    goals in our RFC.

## Alternatives from Rust for Linux

More importantly, our proposal is a big improvement to these proposals, which Rust for Linux care
more about:
* 2024-09: [Rust Safety Standard: Increasing the Correctness of unsafe Code][Rust Safety Standard]
  proposed by Benno Lossin
  * This slides are about reasons and goals for safety documentation standardization, which our
    proposal tries to achieve.
  * It doesn't mention how the standard is implemented, but Predrag (see the next line) and we
    follow the spirit.
* 2024-10: [Automated checking of unsafe code requirements](https://hackmd.io/@predrag/ByVBjIWlyx)
  proposed by Predrag
  * Our proposal is greatly inspired by Predrag's, so many of it can apply to ours, such as
    structured comments, entity reference, requirements discharge, and handling soundness hazard on
    safety rule changes. 
  * The main difference is syntax: Predrag put up new syntax within doc and line comments, which is
    pretty human and machine readable, but can be hard to implement as compiler just throws aways
    line comments so it's less handy to get safe rules on an expression than
    [`stmt_expr_attributes`](https://github.com/rust-lang/rust/issues/15701).
  * His proposal doesn't mention arguments support in safety rules, meaning we don't know how a
    pointer safety rule can apply to two pointers function arguments without ambiguity.

Originally, we only focus on libstd's common safety propeties ([paper]), but noticed the RustWeek
[meeting note] in zulipchat. Thus [tag-std#3](https://github.com/Artisan-Lab/tag-std/issues/3) is
opened to support Rust for Linux on safety standard.

[meeting note]: https://hackmd.io/@qnR1-HVLRx-dekU5dvtvkw/SyUuR6SZgx
[Rust Safety Standard]: https://kangrejos.com/2024/Rust%20Safety%20Standard.pdf
[paper]: https://arxiv.org/abs/2504.21312

# Prior art
[prior-art]: #prior-art

Currently, there are efforts on introducing contracts and formal verification into Rust:
* [contracts](https://rust-lang.github.io/rust-project-goals/2024h2/Contracts-and-invariants.html):
  the lang experiment has been implemented since
  [rust#128044](https://github.com/rust-lang/rust/issues/128044).
* [verify-rust-std] pursues applying formal verification tools to libstd. Also see Rust Foundation
  [announcement][vrs#ann], project goals during [2024h2] and [2025h1].

Our proposal "safety property system" also follows [design by contract], especially on
* A clear metaphor to guide the design process
* The connection with automatic software documentation

Nonetheless, safety property is of static semantics, unlike other verification tools which tends to
employ symbolic execution and be dynamic in some ways. Also, safety property is based on current
safety comment practices, thus Rustaceans may feel more familiar.

[design by contract]: https://en.wikipedia.org/wiki/Design_by_contract
[verify-rust-std]: https://github.com/model-checking/verify-rust-std
[2024h2]: https://rust-lang.github.io/rust-project-goals/2024h2/std-verification.html
[2025h1]: https://rust-lang.github.io/rust-project-goals/2025h1/std-contracts.html
[vrs#ann]: https://foundation.rust-lang.org/news/rust-foundation-collaborates-with-aws-initiative-to-verify-rust-standard-libraries/

# Unresolved questions
[unresolved-questions]: #unresolved-questions

* semver of safety propeties: see [versions of a tag](#semver-tag) above.
* order requirements on invocation: it's also common to clarify an unsafe operation must be
  performed once, or some unsafe operation must be followed by or precede another. Our proposal may
  well support this by extending entity reference system and control-flow analysis. Tracked in
  [tag-std#29].
* handle type erasure: we haven't think about calls through unsafe fn pointer or `dyn Trait`.

[tag-std#29]: https://github.com/Artisan-Lab/tag-std/issues/29

# Future possibilities
[future-possibilities]: #future-possibilities

## Versions of a tag

<a id="semver-tag"></a>

We should notice entity reference system handles two versions of tags from the above example!

When a tag is newly introduced on an API, discharge detection applies.

When a revised tag occurs on an API, discharge detection still applies, and a complete report on
tagged places including referencing places should be provided. If local tags are affected by the
revised tag from upstream crate, propagation analysis should extend from culprit crate to the whole
dependency graph.

It's worth noting that this is unlike [semver] checks on crate's APIs. Reason are 
* core or similar builtin libraries are not versioned. Even if these crates are tied to specific
  rust toolchain, toolchain version doesn't and is unable to reflect version of builtin libraries.
* adding a new tag breaks downstream crates due to discharge detection, while adding a new API is
  usually not a braking change.
* tags are public across all crates, if an upstream tag is removed, all downstream crates need to
  remove it accordingly.

[semver]: https://doc.rust-lang.org/cargo/reference/semver.html

So making tags versioned is a big challenge. On the one hand, we want tags to be part of APIs and
semver controlled, on the other hand, any change in tags results in high churn.

This RFC suggests reporting diffs on versions of tags, in warnings or errors at user option, but
doesn't provide any solution to churn. That's to say, it's unclear whether safety propeties should
be semver checked or not.

## Entity Reference System

<a id="reference-entity"></a>

To reduce verbosity, we propose `#[ref]` to bi-directional entity references.

For [IntoIter][vec_deque] of VecDeque:

[vec_deque]: https://github.com/rust-lang/rust/blob/ebd8557637b33cc09b6ee8273f3154d5d3af6a15/library/alloc/src/collections/vec_deque/into_iter.rs#L104

```rust
fn try_fold<B, F, R>(&mut self, mut init: B, mut f: F) -> R
    impl<'a, T, A: Allocator> Drop for Guard<'a, T, A> {
        #[ref(try_fold)] // 💡 ptr::read below relies on this drop impl
        fn drop(&mut self) { ... }
    }
    ...

    init = head.iter().map(|elem| {
        guard.consumed += 1;

        #[ref(try_fold)] // 💡
        // SAFETY: Because we incremented `guard.consumed`, the deque
        //         effectively forgot the element, so we can take ownership.
        /* ValidPtr(elem, T, 1), Aligned(elem, T), Init(elem, T, 1), Owning(elem) */
        unsafe { ptr::read(elem) }
    })
    .try_fold(init, &mut f)?;

    tail.iter().map(|elem| {
        guard.consumed += 1;

        #[ref(try_fold)] // 💡 No longer to write SAFETY: Same as above.
        unsafe { ptr::read(elem) }
    })
    .try_fold(init, &mut f)
}

fn try_rfold<B, F, R>(&mut self, mut init: B, mut f: F) -> R {
    impl<'a, T, A: Allocator> Drop for Guard<'a, T, A> {
        #[ref(try_fold)] // 💡
        fn drop(&mut self) { ... }
    }
    ...

    init = tail.iter().map(|elem| {
            guard.consumed += 1;

            #[ref(try_fold)] // 💡 No longer to write SAFETY: See `try_fold`'s safety comment.
            unsafe { ptr::read(elem) }
        })
        .try_rfold(init, &mut f)?;

    head.iter().map(|elem| {
            guard.consumed += 1;

            #[ref(try_fold)] // 💡 No longer to write SAFETY: Same as above.
            unsafe { ptr::read(elem) }
        })
        .try_rfold(init, &mut f)
}
```

These `#[ref]` annotations act as cross-references that nudge developers to inspect every linked
site. When either end or the code around it changes, reviewers are instantly aware of all affected
locations and can verify that every referenced safety requirement is still satisfied.

## Interaction with Rust type system

Arguments in a property can be any expression, and sometimes the type of argument must be known in
analysis and doc comments:

```rust
// Syntax1: type provided explicitly — verifier can check the given type is valid
#[safety::requires(Aligned(p, T))]
// Syntax2: type inferred from the function signature — simpler for the annotator
#[safety::requires(Aligned(p))]
unsafe fn read<T>(src: *const T) {}
```

The generic type `T` will be rendered in `#[doc]`, so it'd be tricky if the type needs
[normalization] or trait bounds analysis. It happens to be the case that `ptr::read` has a safety
property `Trait(T, Copy)` (informational, not a hard precondition).

[normalization]: https://rustc-dev-guide.rust-lang.org/normalization.html

Because attributes on expression are only available in HIR, is type fully normalized at this stage?
I guess no.

Trait solver may be involved, due to trait bounds analysis in safety property: if we hope to do
better on `#[option::Trait(T, Copy)]`, each call of read on non-Copy T should requires a safety
reason.

## Better experience with more tooling

We're also considering implmenting such tools for better development, review, and audit experience:
* a LSP server to analyze safety properties and offer safety attributes autocompletion
* a [SARIF](https://sarifweb.azurewebsites.net/) adaptor and code scanning workflow on Github
  PR/Security ([e.g.][sarif-rs]).

[sarif-rs]: https://psastras.github.io/sarif-rs/docs/getting-started/introduction/

