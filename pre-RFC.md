# pre-RFC: Safety Property System

# Summary
[summary]: #summary

A safety property mechanism that is standardized on writing safety documentation and propeties are auto checked.

This RFC is at API-level, not compiler or language level, because we only introduce some attribute macros on 
functions and expressions, which has been able to express today, but needs a linter tool to achieve the goal.

And this PRF influences the whole crate ecosystem, especially on unsafe code practices, because we hope safety
propeties from std and sibling crates are accessible to downstream crates, even for low-level no_std crates.

# Motivation
[motivation]: #motivation

To avoid the misuse of unsafe code, Rust developers should provide clear safety comments for unsafe APIs.

While these safety comments are quite human readable, they can be interpreted differently from person to person.
Even the current best practices in the Rust standard library are ad hoc and informal in some places.

Besides, safety comments are sometimes tedious and repetitive to write, thus people just feel bothered to do or 
simply leave a short note, but reviewers may neglect some unmentioned safety requirements.

And here's a severe problem in review or audit as time goes by: when safety requirements changed on an API,
function callers and people are unaware of the change. This is quite a safety hazard and risk for downstream
crates whose safety comments are out of date since the change from upstream API.

So we need to improve the practice of writing safety comments by making it machine readable and checkable 
in the form of safety tags. We want these tags to be
* accessible to read and write for human: if a tag is defined but missing on callsites, lints will be emitted 
  to help the coder and reviewers assess safety requirements
* composable enough to piece together safety comments for readers, especially on rustdoc HTML pages as seen today
* versioned: tags are in need of semver checking to solve the problem brought by evolution of safety requirements

# Guide-level explanation
[guide-level-explanation]: #guide-level-explanation

## Terms: requirement, property, and tag

The unit of a piece of safety information is called a safety requirement, property, or tag. Nuances are
* a safety requirement is descriptive in text
* a safety propety is structured and formalized to be made of a keyword (i.e. ident) of a type, arguments and short description;
  ideally string interpolation is able to perform on it so that a property is as much reusable as possible
* a safety tag is a [tool attribute](https://doc.rust-lang.org/reference/attributes.html#tool-attributes) in the form of 
  `#[safety::type::Prop(args, ...)]` where `safety` is a crate name or tool name, `type` is one of `{precond,hazard,option}`,
  and `Prop(args, ...)` is a safety property. For safety propeties in libcore and libstd, refer to 
  [this document](https://github.com/Artisan-Lab/tag-std/blob/main/primitive-sp.md) and 
  [paper](https://arxiv.org/abs/2504.21312). For property types:
  * precond denotes a safety requirement that must be satisfied before invoking an unsafe API; most unsafe APIs may have this
  * hazard denotes invoking the unsafe API may temporarily leave the program in a vulnerable state; e.g. [`String::as_bytes_mut`]
  * option denotes optional precondition for the unsafe API; if such condition is satisfied, they can ensure the safety invariant;
    e.g. see the following example of [`ptr::read`]

[`String::as_bytes_mut`]: https://doc.rust-lang.org/std/string/struct.String.html#method.as_bytes_mut
[`ptr::read`]: https://doc.rust-lang.org/std/ptr/fn.read.html

## Turn safety requirements into propeties and tags

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
| Precond | ValidPtr | src       | `*const T` mut be [valid]                                                                |
| Precond | Aligned  | src       | `*const T` must be [aligned][alignment] to `align_of::<T>()`                             |
| Precond | Init     | src       | `*const T` must be initialized                                                           |
| Option  | Trait    | T, Copy   | it's bitwise copy even for `T: !Copy`; see "Ownership of the Returned Value" for caveats |

[valid]: https://doc.rust-lang.org/std/ptr/index.html#safety
[alignment]: https://doc.rust-lang.org/std/ptr/index.html#alignment

Thus safety tags can be written as 

```rust
/// # Safety
#[safety::precond::ValidPtr(src)]
#[safety::precond::Aligned(src)]
#[safety::precond::Init(src)]
#[safety::option::Trait("T: Copy", memo = "description")]
///
/// ## Ownership of the Returned Value
/// ...
///
/// [valid]: https://doc.rust-lang.org/std/ptr/index.html#safety
/// [aligned]: https://doc.rust-lang.org/std/ptr/index.html#alignment
pub const unsafe fn read<T>(src: *const T) -> T { ... }
```

Safety tags will brings two effects:
1. they are expanded to `#[doc]` comments, thus rendered through rustdoc on HTML pages
2. they are collected by a linter tool which sees all tags in all crates involved, and analyzes each callsite
   to emit what safety tags are missing. The tool supports property semver checking, meaning when a dependency
   is updated, and its tags are modified, there will be a report about where infected tags locates and what
   differences are w.r.t. any component in safety propeties.

## Discharge a safety property

A common practice for calling unsafe functions are to leave a small piece of 
safety comments, and repeat it or refer to the same comments. For example:

```rust
// src: rust/library/alloc/src/collections/vec_deque/into_iter.rs
// https://github.com/rust-lang/rust/blob/ebd8557637b33cc09b6ee8273f3154d5d3af6a15/library/alloc/src/collections/vec_deque/into_iter.rs#L104

// impl<T, A: Allocator> Iterator for IntoIter<T, A>

// fn try_fold<B, F, R>(&mut self, mut init: B, mut f: F) -> R
init = head.iter().map(|elem| {
    guard.consumed += 1;
    // SAFETY: Because we incremented `guard.consumed`, the
    // deque effectively forgot the element, so we can take
    // ownership
    unsafe { ptr::read(elem) }
})
.try_fold(init, &mut f)?;

tail.iter().map(|elem| {
    guard.consumed += 1;
    // SAFETY: Same as above.
    unsafe { ptr::read(elem) }
})
.try_fold(init, &mut f)

// fn try_rfold<B, F, R>(&mut self, mut init: B, mut f: F) -> R
// SAFETY: See `try_fold`'s safety comment.
unsafe { ptr::read(elem) } // head
// SAFETY: Same as above.
unsafe { ptr::read(elem) } // tail
```

There are potential issues in review or audit:
* Did the author know and confirm all safety requirements on `ptr::read` are fulfilled?
  From the above comments, we're only sure that the `option::Trait(T, Copy)` property is 
  considered, but unsure about other propeties.
* When the try_fold's safety comments changed, people might miss checking if these referrers
  are still appropriate. It depends on the author and reviewers to recall or find these places.
  It's luckily not hard to do for the above example, as `fold` and `try_fold` are quite similar,
  and both in the same module. However, it'd be really hard to find referrers across modules or 
  even crates.
* It's sad when a piece of code are changed without noticing a safety requirement relies upon it.
  The above comment "the deque effectively forgot" is actually tied to Guard's drop implementation,
  so ideally, if code inside `try_fold::Guard::drop` changes, people really ought to check these safety
  comments still hold, while there is no comments on `Guard::drop` to indicate a relation to 
  `ptr::read(elem)`. Not to mention that `try_rfold`'s safety comments refer to `try_fold`'s,
  `try_rfold` has its own `Guard::drop` impl, meaning we should check both `try_{r,}fold::Guard::drop`
  even when only single drop impl changes. 

So we put up a solution to these problems via annotating `#[discharges]` on callsites and some form of 
reference system.

```rust
// fn try_fold<B, F, R>(&mut self, mut init: B, mut f: F) -> R

// Also tag #[safety::ref::try_fold] on try_fold::Guard::drop fn declaration.

init = head.iter().map(|elem| {
    guard.consumed += 1;

    #[safety::discharges::ValidPtr(elem)]
    #[safety::discharges::Aligned(elem)]
    #[safety::discharges::Init(elem)]
    #[safety::discharges::Trait("T: Copy", memo = "
      Because we incremented `guard.consumed`, the deque 
      effectively forgot the element, so we can take ownership.
    ")]
    #[safety::referent(try_fold)]
    unsafe { ptr::read(elem) }
})
.try_fold(init, &mut f)?;
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
    = NOTE: ValidPtr 👉 https://doc.rust-lang.org/std/ptr/index.html#safety 👉 
    = NOTE: Aligned 👉 https://doc.rust-lang.org/std/ptr/index.html#alignment
    = NOTE: Init 👉 The pointer must be initialized before calling `core::ptr::read`
```

To avoid verbosity, we propose `#[referent]` for entity definition and `#[ref]` for entity
reference.

```rust
// fn try_fold<B, F, R>(&mut self, mut init: B, mut f: F) -> R

#[safety::ref::try_fold] fn try_fold::Guard::drop(&mut self) { ... }

#[safety::discharges::ValidPtr(elem)]
#[safety::discharges::Aligned(elem)]
#[safety::discharges::Init(elem)]
#[safety::discharges::Trait("T: Copy", memo = "
  Because we incremented `guard.consumed`, the deque 
  effectively forgot the element, so we can take ownership.
")]
#[safety::referent(try_fold)] // 👈 entity definition
unsafe { ptr::read(elem) } // head

#[safety::ref::try_fold]
unsafe { ptr::read(elem) } // tail

// fn try_rfold<B, F, R>(&mut self, mut init: B, mut f: F) -> R

#[safety::ref::try_fold] fn try_rfold::Guard::drop(&mut self) { ... }

#[safety::ref::try_fold]
unsafe { ptr::read(elem) } // head

#[safety::ref::try_fold]
unsafe { ptr::read(elem) } // tail
```

If referent is not defined or collides, hard error is emitted.

Once safety propeties on referent changes, we can know all relevant places, and estimate
safety requirements fulfillment on referrers.

///////////////////////////////// TODO: Below are not started yet /////////////////////////////////

Explain the proposal as if it was already included in the language and you were teaching it to another Rust programmer. That generally means:

- Introducing new named concepts.
- Explaining the feature largely in terms of examples.
- Explaining how Rust programmers should *think* about the feature, and how it should impact the way they use Rust. It should explain the impact as concretely as possible.
- If applicable, provide sample error messages, deprecation warnings, or migration guidance.
- If applicable, describe the differences between teaching this to existing Rust programmers and new Rust programmers.
- Discuss how this impacts the ability to read, understand, and maintain Rust code. Code is read and modified far more often than written; will the proposed feature make code easier to maintain?

For implementation-oriented RFCs (e.g. for compiler internals), this section should focus on how compiler contributors should think about the change, and give examples of its concrete impact. For policy RFCs, this section should provide an example-driven introduction to the policy, and explain its impact in concrete terms.

# Reference-level explanation
[reference-level-explanation]: #reference-level-explanation

This is the technical portion of the RFC. Explain the design in sufficient detail that:

- Its interaction with other features is clear.
- It is reasonably clear how the feature would be implemented.
- Corner cases are dissected by example.

The section should return to the examples given in the previous section, and explain more fully how the detailed proposal makes those examples work.

# Drawbacks
[drawbacks]: #drawbacks

Why should we *not* do this?

# Rationale and alternatives
[rationale-and-alternatives]: #rationale-and-alternatives

- Why is this design the best in the space of possible designs?
- What other designs have been considered and what is the rationale for not choosing them?
- What is the impact of not doing this?
- If this is a language proposal, could this be done in a library or macro instead? Does the proposed change make Rust code easier or harder to read, understand, and maintain?

# Prior art
[prior-art]: #prior-art

Discuss prior art, both the good and the bad, in relation to this proposal.
A few examples of what this can include are:

- For language, library, cargo, tools, and compiler proposals: Does this feature exist in other programming languages and what experience have their community had?
- For community proposals: Is this done by some other community and what were their experiences with it?
- For other teams: What lessons can we learn from what other communities have done here?
- Papers: Are there any published papers or great posts that discuss this? If you have some relevant papers to refer to, this can serve as a more detailed theoretical background.

This section is intended to encourage you as an author to think about the lessons from other languages, provide readers of your RFC with a fuller picture.
If there is no prior art, that is fine - your ideas are interesting to us whether they are brand new or if it is an adaptation from other languages.

Note that while precedent set by other languages is some motivation, it does not on its own motivate an RFC.
Please also take into consideration that rust sometimes intentionally diverges from common language features.

# Unresolved questions
[unresolved-questions]: #unresolved-questions

- What parts of the design do you expect to resolve through the RFC process before this gets merged?
- What parts of the design do you expect to resolve through the implementation of this feature before stabilization?
- What related issues do you consider out of scope for this RFC that could be addressed in the future independently of the solution that comes out of this RFC?

# Future possibilities
[future-possibilities]: #future-possibilities

## Interaction with Rust type system

About expression and trait solver.

======================

Think about what the natural extension and evolution of your proposal would
be and how it would affect the language and project as a whole in a holistic
way. Try to use this section as a tool to more fully consider all possible
interactions with the project and language in your proposal.
Also consider how this all fits into the roadmap for the project
and of the relevant sub-team.

This is also a good place to "dump ideas", if they are out of scope for the
RFC you are writing but otherwise related.

If you have tried and cannot think of any future possibilities,
you may simply state that you cannot think of anything.

Note that having something written down in the future-possibilities section
is not a reason to accept the current or a future RFC; such notes should be
in the section on motivation or rationale in this or subsequent RFCs.
The section merely provides additional information.
