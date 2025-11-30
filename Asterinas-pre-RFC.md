# RFC-0000: Safety Tags in Asterinas

* Status: Draft
* Pull request: (link to PR)
* Date submitted: YYYY-MM-DD
* Date approved: YYYY-MM-DD

## Summary

## Motivation

As an emerging operating system, Asterinas has made significant efforts to ensure memory
safety by uniquely designing a minimal and sound Trusted Computing Base (TCB) that
completely segregates unsafe code into the ostd crate.

However, Asterinas developers face several challenges when maintaining and reviewing the
unsafe code in ostd.

## Safety Requirements Are Easily Forgotten, Incomplete or Incorrect


Some unsafe APIs **lack explicit safety comments**. For example, [`send_ipi`] has
no safety requirement, but its inner unsafe block requires the caller to ensure safety.

```rust
impl super::Apic for X2Apic {
    unsafe fn send_ipi(&self, icr: super::Icr) {
        let _guard = crate::trap::irq::disable_local();
        // SAFETY: These `rdmsr` and `wrmsr` instructions write the interrupt command to APIC and wait for results. The caller guarantees it's safe to execute this interrupt command.
        unsafe {
            wrmsr(IA32_X2APIC_ESR, 0);
```

Safety comments can be incomplete or even incorrect, leading to potential misunderstanding
or misuse. For example, [`mm::frame::allocator::init`] requires that it should be called
only once, but logically calling it depends on the other parts to be initializated
beforehand. The call order matters, but the comments on the API doesn't mention that.

```rust
/// Initializes the global frame allocator.
///
/// It just does adds the frames to the global frame allocator. Calling it multiple times would be not safe.
///
/// # Safety
///

/// This function should be called only once.
pub(crate) unsafe fn init() { // mm::frame::allocator::init
    ...
    let early_allocator = EARLY_ALLOCATOR.lock().take().unwrap();
```
  
Fortunately, some comments in the the top level [`init`] clarifies this order should be
upheld:  call `init_early_allocator` first, followed by `mm::frame::meta::init`, and then
`mm::frame::allocator::init`:
  
```rust
// SAFETY: This function is called only once, before `allocator::init`
// and after memory regions are initialized.

unsafe { mm::frame::allocator::init_early_allocator() };

...

// SAFETY: We are on the BSP and APs are not yet started.
let meta_pages = unsafe { mm::frame::meta::init() };
// The frame allocator should be initialized immediately after the metadata
// is initialized. Otherwise the boot page table can't allocate frames.
// SAFETY: This function is called only once.
unsafe { mm::frame::allocator::init() };
```
  
Another case of incorrect safety comments in Asterinas is [PR#2587]. It was raised,
because the `send_ipi` function had evolved, but the variables mentioned in the safety
comments no longer existed, leaving the entire requirement outdated.

[PR#2587]: https://github.com/asterinas/asterinas/pull/2587
[`send_ipi`]: https://github.com/asterinas/asterinas/blob/v0.16.1/ostd/src/arch/x86/kernel/apic/x2apic.rs#L78
[`init`]: https://github.com/asterinas/asterinas/blob/48c7c37f50fe80689a04e14d7075c56a17ca6d52/ostd/src/lib.rs#L83
[`mm::frame::allocator::init`]: https://github.com/asterinas/asterinas/blob/v0.16.1/ostd/src/mm/frame/allocator.rs#L199

### Textual Comments and Documentation Can Be Repetitive and Inconsistent

Safety comments often exhibit similarities in wording, phrasing, and sentence structure.
But there is a conflict:
* code authors tend to keep content concise or have different writing styles
* whereas code readers crave a more comprehensive understanding

Rust for Linux has been aware of the problem, therefore the crew sets an objective titled
"Rust Safety Standard - Increasing the Correctness of unsafe Code". You can find some
links of related discussions, LWN articles, and slides [here][tag-std#rfl].

I'll quote from a [LWN article] documenting the discussion at Kangrejos 2024, which also
applies to Asterinas:

> Ideally, all of the comments would be correct, complete, and easy to understand. That's
> easier to accomplish if there's a shared vocabulary for common conventions — an author
> shouldn't need to write "valid, non-null" when just "valid" will do. Lossin suggested
> that they might want to standardize a dictionary of common terms, so that authors can
> write as little as possible, but readers will still be able to understand. Plus, having
> an explicit resource saying how to read safety documentation will make it easier for
> learners to come up to speed, and reduce the chances of misunderstandings between
> maintainers.

[tag-std#rfl]: https://github.com/Artisan-Lab/tag-std/issues/3
[LWN article]: https://lwn.net/Articles/990273/

### In the Pursuit of Sustainable Maintainability on Unsafe Code

Asterinas is constantly undergoing refinement, but accidental bugs may be inevitably
introduce like the ones that [PR#2587] resolved. Although the problem of outdated safety
comments is far less catastrophic, it sows the seeds of hidden risks that undermines the
trustworthiness and security of unsafe Rust code, ultimately of the whole operating system
as per the [Lehman's laws of software evolution].

There are few tools for Rust projects to help developers maintain unsafe code. Code author
and reviewers must keep a close eye on changes surrounding unsafe code and meticulously
inspect adherence to soundness guarantees. These change can include relaxing or
strengthening of a safety requirement, modifying the implementation on an unsafe function,
mutating the concrete arguments passed into it, or even changes to other items referenced
by a safety requirement.

The pursuit of sustainable maintainability demands a standardized process to identify and
alert developers to sites that may jeopardize the assurance and soundness of unsafe code.

[Lehman's laws of software evolution]:https://en.wikipedia.org/wiki/Lehman%27s_laws_of_software_evolution

### General Motivations

Rust [RFC#3842] outlines some broader reasons for having safety tags, which are not
specific to Asterinas:
* Safety Invariants Have No Semver
* Granular Unsafe: How Small Is Too Small?
* Formal Contracts, Casual Burden

[RFC#3842]: https://github.com/rust-lang/rfcs/pull/3842

## Design


* SP 是什么：定义、使用方式（语法）、
* 我们的工具能够检查什么问题？
* 如何检查这些问题
* 集成到星绽的步骤：调研（总体介绍星绽的安全属性）、初期（哪些属性先应用）、评估效果（我们会关注安全属性方面的星绽 PR 审查）、迭代
* 实践前后的差异、审计流程的变化

### Introduce Safety Tags

We propose checkable safety tags with a feasible [safety-tool].


[safety-tool]: https://github.com/Artisan-Lab/tag-std/tree/main/safety-tool

#### Terminology


We use the following concepts and terminologies in the RFC:
* Safety requirements are texts written on unsafe code to demostrate what responsibilities
of the caller are in using the unsafe function correctly and how the callee fulfills these
safety responsibilities.
* A safety tag is a safety requirement that is structural and machine-checkable, in the
Rust's attribute syntax `#[safety::<predicate>(TagName(arguments))]`. We'll delve into the
syntax and mechanism below.
* A safety property (SP) refers to the meaning of a safety requirement, by virtue of a tag
name with optional arguments.


#### Syntax

The syntax essentially represents a valid Rust [attribute], in the form of:

[attribute]: https://doc.rust-lang.org/reference/attributes.html#attributes

```
SafetyTags -> `#` `[` `safety::` Predicate `(` Tags `)` `]`

Predicate -> `requires` | `checked` | `delegated`

Tags -> Tag (`,` Tag)* `,`?

Tag -> TagName ( TagArguments )? (`:` LiteralString)?

TagName -> SingleIdent

TagArguments -> `(` TagArgument (, TagArgument)* `)`

TagArgument -> RustExpression
```

Specifically, we have 3 safety attribute macros
* `#[safety::requires]` is placed on an unsafe function’s signature to state the safety
requirements that callers must uphold. It's a direct replacement for the safety section in
doc comments.
* `#[safety::checked]` is placed on an expression that wraps an unsafe operation like
calling an unsafe function. It's a direct replacement for inline safety comments written
above an unsafe block.
* `#[safety::delegated]` acts like `checked`, but delivers improved ergonomics and
supplementrary checks when transferring safety responsiblities from a function body to its
signature.
  




1. **Lightweight checking**. Our tool can check the unsafe APIs whether the safety
   requirements are fully provided and correctly constructed and whether all the safety
requirements are  (with the help of discharge grammar).
2. **Semantic granularity and reusability**. Each safety tag represents a single, precise
   safety primitive. This fine-grained approach makes safety contracts more explicit,
easier to understand, and simpler to verify.  The tagging system also enables developers
to reuse standardized safety primitives across different APIs, reducing duplication and
ensuring consistent safety reasoning throughout the codebase.
3. **Automatic document generation**. By automatically parsing the safety tags, our tool
   can produce comprehensive human-readable descriptions of API safety requirements,
eliminating the maintenance burden of manual documentation while ensuring accuracy and
consistency across the codebase.



## Progress with Asterinas

At present, we have preliminarily completed the annotation of most unsafe functions in
asterinas, forming over 20 different safety primitives. The document can be found at
[Asterinas-safety-properties](https://github.com/Artisan-Lab/tag-std/blob/main/Asterinas-safety-properties.md). 

<!-- Any future change to a common safety requirement necessitates error-prone, manual updates to all affected comments,
creating a risk that the documentation will fall out of sync with the code.  -->



### Safety Comments and Tags

In the following document, we use the term **safety comments** to refer to informal
textual descriptions of safety properties or safety requirements that must be satisfied to
ensure safety when using an unsafe API. This is the current form of safety descriptions
used in Rust.

In contrast, **safety tags** represent safety properties using a formal language, i.e., a
[tool attribute] written in the form `#[safety { Prop: "reason" }]` where

- `safety` is proc-macro,
- `type` is one of `{precond, hazard, option}`,
  - precond denotes a safety requirement that must be satisfied before invoking an unsafe
  API. Most unsafe APIs carry at least one precondition.
  - hazard denotes invoking the unsafe API may temporarily leave the program in a
  vulnerable state.
  - option denotes an optional precondition for an unsafe API—conditions that are
  sufficient but not necessary to uphold the safety invariant. 
- `Prop` is a safety property (SP) instance. Multiple SPs can be grouped together by
separating them with commas, such as `SP1, SP2`.
- `: "reason"` is an *optional* string to clarify what SP means in the context.
  -  when a reason string appears, use `;` to separate props like `SP1: ""; Sp2: ""`.

Here are some basic syntax examples:

```rust
#[safety { SP }]
#[safety { SP1, SP2 }]

#[safety { SP1: "reason" }]
#[safety { SP1: "reason"; SP2: "reason" }]

#[safety { SP1, SP2: "shared reason for the two SPs" }]
#[safety { SP1, SP2: "shared reason for the two SPs"; SP3 }]
#[safety { SP3; SP1, SP2: "shared reason for the two SPs" }]
```

### Turn Safety Comments into Safety Tags

Consider safety comments on [ostd::arch::iommu::fault::FaultEventRegisters::new()](https://github.com/asterinas/asterinas/blob/v0.16.1/ostd/src/arch/x86/iommu/fault.rs#L42)

```rust
impl FaultEventRegisters {
    /// Creates an instance from the IOMMU base address.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the base address is a valid IOMMU base address and that it has exclusive ownership of the IOMMU fault event registers.
    unsafe fn new(base_register_vaddr: NonNull<u8>) -> Self {
```

We can extract safety requirements above into two properties:

| Type    | Property      | Arguments      | Description                                          |
| ------- | ------------- | -------------- | ---------------------------------------------------- |
| Precond | ValidBaseAddr | addr, hardware | `addr` should be a valid base address of `hardware`. |
| Precond | OwnedResource | resource       | `resource` shoule be exclusively owned.              |

We can represent these safety requirements using safety tags as shown below.

```rust
#[safety {

    ValidBaseAddr(base_register_vaddr, "IOMMU"),
    OwnedResource("The IOMMU fault event registers")
}]
unsafe fn new(base_register_vaddr: NonNull<u8>) -> Self {
```


Safety tags will take effect in two ways:

1. They will be expanded into `#[doc]` comments, which will be rendered through rustdoc on
   HTML pages.

2. They will be collected and analyzed by a linter tool. If no safety tags are provided
   for an unsafe API, lints should be emitted to remind developers to provide safety
   requirements. If a safety tag is declared for an unsafe API but not discharged at a
   call site, lints should be emitted to alert developers about potentially overlooked
   safety requirements.

### Define Safety Properties in Toml Configuration

SPs can be defined in TOML files  to perform checks on user inputs and generate doc
comments.

An example definition of an SP is as follows:

```toml
[tag.ValidBaseAddr]
args = [ "addr", "hardware" ]
desc = "`{addr}` should be a valid base address of `{hardware}`."
```

We defined a property called `ValidBaseAddr`, which includes two arguments and a dynamic
description derived from user input.

When `#[safety { ValidBaseAddr(vaddr, device) }]` is used, a corresponding doc comment is
generated:

```rust
#[doc = "`vaddr` should be a valid base address of `device`."]
```


## Drawbacks, Alternatives, and Unknown

### Drawbacks

* This proposal applies to most unsafe APIs and requires significant effort to replace
existing safety comments with safety tags. However, it can be implemented incrementally.
* It is unclear whether all safety properties are composable, and some properties may
change frequently in the early stages. Our initial investigation shows that the idea works
well for the standard library.
* Safety tags may be less readable than the original safety comments. However, their
readability should be comparable when rendered in rustdoc or surfaced through the LSP
server.


## Prior Art and References

Currently, there are efforts on introducing contracts and formal verification into Rust:

* [contracts]: the lang experiment has been implemented since [rust#128044] and a [lang
team] has been set up.

* [verify-rust-std] pursues applying formal verification to libstd. Also see the Rust
Foundation [announcement][vrs#ann] and Rust project goals in [2024h2] and [2025h1].
[rust#147148] is trying to port all viable contracts from verify-rust-std to libstd.


Asterinas also has started the verification work since 2025 through [vostd] which targets
ostd’s memory management subsystem, leveraging the [Verus] verification tool.

In an ideal design, contracts will be shared across modules, crates and verification
tools. Note that libstd comprises libcore, which manifests that Asterinas will hopefully
reuse verifcation results from libcore in vostd, if Verus is available in verify-rust-std.
As far as I'm aware, the work on the Verus side is [in progress][verus-vrs].

The safety tags proposed in this RFC are far less precise and rigorous on upholding safety
invariants than formal verification. However, it bridges the gaps between safety
documentation clarity and unsafe code review enhancement.


<!-- The current comment-based approach often **lacks the precision required for rigorous
safety reasoning.** Safety requirements are frequently documented in broad, coarse-grained
statements, which can obscure multiple distinct obligations within a single point.

By moving towards a more structured system, we can:

- Decompose these complex requirements into discrete, granular contracts, thereby
enhancing clarity and auditability.
- Formally specify each contract, enabling the use of automated tools to verify adherence
and catch violations early in the development period.

They are -->

[contracts]: https://rust-lang.github.io/rust-project-goals/2024h2/Contracts-and-invariants.html
[rust#128044]: https://github.com/rust-lang/rust/issues/128044
[lang team]: https://rust-lang.zulipchat.com/#narrow/channel/544080-t-lang.2Fcontracts
[verify-rust-std]: https://github.com/model-checking/verify-rust-std
[vrs#ann]: https://foundation.rust-lang.org/news/rust-foundation-collaborates-with-aws-initiative-to-verify-rust-standard-libraries/
[2024h2]: https://rust-lang.github.io/rust-project-goals/2024h2/std-verification.html
[2025h1]: https://rust-lang.github.io/rust-project-goals/2025h1/std-contracts.html
[rust#147148]: https://github.com/rust-lang/rust/pull/147148
[vostd]: https://github.com/asterinas/vostd
[Verus]: https://github.com/verus-lang/verus
[verus-vrs]: https://rust-lang.zulipchat.com/#narrow/channel/183875-wg-formal-methods/topic/Application.20of.20the.20Kani.20model.20checker/near/503280988
