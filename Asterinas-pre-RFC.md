### Summary

### Motivation

#### Missing or Incorrect Safety Comments

As a rising operating system emphasizing its special isolation method for unsafe factors, Asterinas puts significant effort into correctly and comprehensively specifying safety requirements for its unsafe APIs. We are pleased to observe that most unsafe APIs have clearly listed safety requirements and explanations of why they are safe to use at the call site. However, two main issues remain:

- Some unsafe APIs still **lack explicit safety descriptions**. e.g., in [ostd::arch::kernel::apic::x2apic::X2Apic::send_ipi()](https://github.com/asterinas/asterinas/blob/v0.16.1/ostd/src/arch/x86/kernel/apic/x2apic.rs#L78), the inner unsafe block requires the caller to ensure safety, but the caller lacks any safety comments.

  ```rust
  impl super::Apic for X2Apic {
      unsafe fn send_ipi(&self, icr: super::Icr) {
          let _guard = crate::trap::irq::disable_local();
          // SAFETY: These `rdmsr` and `wrmsr` instructions write the interrupt command to APIC and wait for results. The caller guarantees it's safe to execute this interrupt command.
          unsafe {
              wrmsr(IA32_X2APIC_ESR, 0);
  ```
- The provided safety comments may still be **incomplete or incorrect**, leading to potential misuse or misunderstanding. e.g., in [ostd::mm::frame::allocator::init()](https://github.com/asterinas/asterinas/blob/v0.16.1/ostd/src/mm/frame/allocator.rs#L199), the requirement is incomplete, and it must specify that the function must be called after `init_early_allocator`.

  ```rust
  /// Initializes the global frame allocator.
  ///
  /// It just does adds the frames to the global frame allocator. Calling it multiple times would be not safe.
  ///
  /// # Safety
  ///
  /// This function should be called only once.
  pub(crate) unsafe fn init() {
      ...
      let early_allocator = EARLY_ALLOCATOR.lock().take().unwrap();
  ```

#### Textual description: extensive and repetitive

The current reliance on free-form comments for safety requirements has **maintainability and consistency challenges**. This approach sometimes results in verbose and repetitive documentation which introduce a significant maintenance burden. Any future change to a common safety invariant necessitates error-prone, manual updates to all affected comments, creating a risk that the documentation will fall out of sync with the code.

#### More Precision: granularity and contracts

The current comment-based approach often **lacks the precision required for rigorous safety reasoning.** Safety requirements are frequently documented in broad, coarse-grained statements, which can obscure multiple distinct obligations within a single point.

By moving towards a more structured system, we can:

- Decompose these complex requirements into discrete, granular contracts, thereby enhancing clarity and auditability.
- Formally specify each contract, enabling the use of automated tools to verify adherence and catch violations early in the development period.

### Design

We propose checkable safety tags with a feasible safety tool to address the issues with three concrete gains:

1. **Lightweight checking**. Our tool can check the unsafe APIs whether the safety requirements are fully provided and correctly constructed and whether all the safety requirements are  (with the help of discharge grammar).
2. **Semantic granularity and reusability**. Each safety tag represents a single, precise safety primitive. This fine-grained approach makes safety contracts more explicit, easier to understand, and simpler to verify.  The tagging system also enables developers to reuse standardized safety primitives across different APIs, reducing duplication and ensuring consistent safety reasoning throughout the codebase.
3. **Automatic document generation**. By automatically parsing the safety tags, our tool can produce comprehensive human-readable descriptions of API safety requirements, eliminating the maintenance burden of manual documentation while ensuring accuracy and consistency across the codebase.

#### Safety Comments and Tags

In the following document, we use the term **safety comments** to refer to informal textual descriptions of safety properties or safety requirements that must be satisfied to ensure safety when using an unsafe API. This is the current form of safety descriptions used in Rust.

In contrast, **safety tags** represent safety properties using a formal language, i.e., a
[tool attribute] written in the form `#[safety { Prop: "reason" }]` where

- `safety` is proc-macro,
- `type` is one of `{precond, hazard, option}`,
  - precond denotes a safety requirement that must be satisfied before invoking an unsafe API. Most unsafe APIs carry at least one precondition.
  - hazard denotes invoking the unsafe API may temporarily leave the program in a vulnerable state.
  - option denotes an optional precondition for an unsafe API—conditions that are sufficient but not necessary to uphold the safety invariant.
- `Prop` is a safety property (SP) instance. Multiple SPs can be grouped together by separating them with commas, such as `SP1, SP2`.
- `: "reason"` is an *optional* string to clarify what SP means in the context.
  - when a reason string appears, use `;` to separate props like `SP1: ""; Sp2: ""`.

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

#### Turn Safety Comments into Safety Tags

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

1. They will be expanded into `#[doc]` comments, which will be rendered through rustdoc on HTML pages.
2. They will be collected and analyzed by a linter tool. If no safety tags are provided for an unsafe API, lints should be emitted to remind developers to provide safety requirements. If a safety tag is declared for an unsafe API but not discharged at a call site, lints should be emitted to alert developers about potentially overlooked safety requirements.

#### Define Safety Properties in Toml Configuration

SPs can be defined in TOML files  to perform checks on user inputs and generate doc comments.

An example definition of an SP is as follows:

```toml
[tag.ValidBaseAddr]
args = [ "addr", "hardware" ]
desc = "`{addr}` should be a valid base address of `{hardware}`."
```

We defined a property called `ValidBaseAddr`, which includes two arguments and a dynamic description derived from user input.

When `#[safety { ValidBaseAddr(vaddr, device) }]` is used, a corresponding doc comment is generated:

```rust
#[doc = "`vaddr` should be a valid base address of `device`."]
```

### Drawbacks

* This proposal applies to most unsafe APIs and requires significant effort to replace existing safety comments with safety tags. However, it can be implemented incrementally.
* It is unclear whether all safety properties are composable, and some properties may change frequently in the early stages. Our initial investigation shows that the idea works well for the standard library.
* Safety tags may be less readable than the original safety comments. However, their readability should be comparable when rendered in rustdoc or surfaced through the LSP server.
