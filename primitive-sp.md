# Privimitive Safety Properties for Rust Contract Design (Draft)

This document presents a draft outlining the fundamental safety properties essential for contract definition. The current documentation on API safety descriptions in the standard library remains ad hoc. For example, the term `valid pointer' is frequently used, but the validity of a pointer depends on the context. In practice, a valid pointer may need to satisfy one or more fundamental conditions, such as being non-null, not dangling, and pointing to memory properly aligned and initialized for type T. It is worth noting that the Rust community is making progress toward standardizing contract design, as highlighted in the links below. We believe this proposal will contribute to the development and refinement of contract specifications.

[Rust Contracts RFC (draft)](https://github.com/rust-lang/lang-team/blob/master/design-meeting-minutes/2022-11-25-contracts.md)  
[MCP759](https://github.com/rust-lang/compiler-team/issues/759)  
[std-contracts-2025h1](https://rust-lang.github.io/rust-project-goals/2025h1/std-contracts.html)  

## 1 Overall Idea
In contract design, safety properties can be categorized into two types:

**Precondition**: Safety requirements that must be satisfied before invoking an unsafe API. These represent the fundamental conditions for safely using the API.

**Postcondition**: Traditionally, this refers to properties the system must satisfy after the API call. In Rust, it implies that the program must not violate Rust's safety requirements, such as exclusive mutability, after the API is executed. If an API has postconditions, it indicates that satisfying the preconditions alone does not guarantee program safety. Developers must also carefully analyze the API's implementation and its usage context.

While preconditions and postconditions are foundational to safety reasoning, they may not always be sufficient in Rust. For instance, an API may include optional preconditions. If these conditions are satisfied, the Rust compiler can guarantee that the postconditions will hold. However, meeting these optional requirements is not mandatory. For example, in the case of [ptr::read()](https://doc.rust-lang.org/std/ptr/fn.read.html), specifying that the parameter implements the Copy trait can help avoid undefined behavior related to exclusive mutability. By meeting this optional precondition, developers can ensure safe use of the API while still having the flexibility to omit it when not needed.

**Option (new)**: Optional preconditions for an unsafe API. If satisfying such conditions, it can guarantee that the post condition can be satisfied.

Besides optional preconditions, we also need to document potential hazards of Unsafe APIs. For instance, certain scenarios such as implementing a doubly linked list or the internals of [Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html) and [RefCell](https://doc.rust-lang.org/std/cell/struct.RefCell.html) require temporarily violating postconditions. In such cases, it is crucial to document how the program state deviates from Rust's safety principles and whether these vulnerabilities are eventually resolved.

**Hazard (new)**: Invoking an unsafe API may temporarily leave the program in a vulnerable or inconsistent state with respect to safe Rust. 

In practice, a safety property may correspond to a precondition, optional precondition, or hazard. To address the ambiguity of certain high-level or ad hoc safety property descriptions, we propose breaking them down into primitive safety requirements. By collecting and analyzing commonly used safety descriptions, we aim to provide a clearer framework for understanding and documenting these properties. The following sections will elaborate on these details.

<span style="color: red;"> **In short, while preconditions must be satisfied, optional preconditions are not mandatory. Hazards highlight vulnerabilities that deviate from Rust's safety principles. Meeting optional preconditions can help avoid certain types of hazards.** </span>

## 2 Summary of Primitive SPs

| ID  | Primitive SP | Usage | Example API |
|---|---|---|---|
| 1  | Aligned(p, T) | precond  | [ptr::read()](https://doc.rust-lang.org/nightly/std/ptr/fn.read.html) | 
| 2  | NonZST(T) | precond | [NonNull.offset_from](https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.offset_from)  | 
| 3  | NoPadding(T)  | precond  | [raw_eq()](https://doc.rust-lang.org/std/intrinsics/fn.raw_eq.html) |
| 4  | NonNull(p) | precond  | [NonNull::new_unchecked()](https://doc.rust-lang.org/std/ptr/struct.NonNull.html#method.new_unchecked) |
| 5  | NonDangling(p, T) | precond| [ptr::offset()](https://doc.rust-lang.org/beta/std/primitive.pointer.html#method.offset) |
| 6.1  | AllocatorConsistency(p, A) | precond | [Box::from_raw_in()](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_raw_in) |
| 6.2  | AllocatorConsistency(p) | precond | [Box::from_raw()](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_raw) |
| 7  | Pointee(p, T)  | precond  | [ptr::read()](https://doc.rust-lang.org/beta/std/primitive.pointer.html#method.read)  |
| 8  | Bounded(p, T, offset)  | precond | [ptr::offset()](https://doc.rust-lang.org/std/primitive.pointer.html#method.offset)  |
| 9.1  | NonOverlap(dst, src, T, count) | precond | [ptr::copy_nonoverlapping()](https://doc.rust-lang.org/std/ptr/fn.copy_nonoverlapping.html)  |
| 9.2  | NonOverlap(dst, src, T) | precond | [ptr::copy()](https://doc.rust-lang.org/std/ptr/fn.copy.html) |
| 11.1  | ValidInt(x, T)  | precond | [f32.to_int_unchecked()](https://doc.rust-lang.org/std/primitive.f32.html#method.to_int_unchecked)  |
| 11.2  | ValidInt(x, T, range)  | precond | [NonZero::from_mut_unchecked()](https://doc.rust-lang.org/beta/std/num/struct.NonZero.html#tymethod.from_mut_unchecked) |
| 11.4  | ValidInt(binop, x, y, T)  | precond | [usize.add()](https://doc.rust-lang.org/std/primitive.usize.html#method.unchecked_add)  |
| 11.5  | ValidInt(uop, x, T)  | precond | [unchecked_neg()](https://doc.rust-lang.org/nightly/core/primitive.isize.html#method.unchecked_neg) |
| 12.1  | ValidString(v) | precond | [String::from_utf8_unchecked()](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_unchecked) |
|     | ValidString(v) | hazard | [String.as_bytes_mut()](https://doc.rust-lang.org/std/string/struct.String.html#method.as_bytes_mut) |
| 12.2  | ValidString(p, len) | precond | [String::from_raw_parts()](https://doc.rust-lang.org/std/string/struct.String.html#method.from_raw_parts) |
| 12.3  | ValidString(s, I) | precond | [String.get_unchecked()](https://doc.rust-lang.org/std/string/struct.String.html#method.get_unchecked) |
| 12.4  | ValidString(s, begin, end) | precond | [String.slice_unchecked()](https://doc.rust-lang.org/std/string/struct.String.html#method.slice_unchecked) |
| 13  | ValidCStr(p, len) |  precond|  [CStr::from_bytes_with_nul_unchecked()](https://doc.rust-lang.org/std/ffi/struct.CStr.html#method.from_bytes_with_nul_unchecked)  |
| 14  | Init(p, T)  | precond | [Box::assume_init()](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.assume_init)  |
| 15  | Unwrap(x, T)  | precond | [Option::unwrap_unchecked()](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_unchecked)  |
| 16  | NonOwned(p)  | precond | [Box::from_raw()](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_raw)  |
| 17  | Owned(p)  | precond | [trait.FromRawFd::from_raw_fd()](https://doc.rust-lang.org/std/os/fd/trait.FromRawFd.html#tymethod.from_raw_fd)  |
| 18  | Alias(p1, p2)  | hazard | [pointer.as_mut()](https://doc.rust-lang.org/std/primitive.pointer.html#method.as_mut) |
| 19  | Lifetime(p, 'a)  | precond | [AtomicPtr::from_ptr()](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicPtr.html#method.from_ptr)  |
| 20  | Trait(T)  | option | [ptr::read()](https://doc.rust-lang.org/std/ptr/fn.read.html)  |
| 21  | Send(T, NoRc)  | option | [Send](https://doc.rust-lang.org/std/marker/trait.Send.html) |
| 22  | Sync(T, NoInteriorMut)  | option | [Sync](https://doc.rust-lang.org/std/marker/trait.Sync.html) |
| 23  | Pinned(p)  | hazard | [Pin::new_unchecked()](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.new_unchecked)  |
| 24  | Opened(fd) | precond | [trait.FromRawFd::from_raw_fd()](https://doc.rust-lang.org/std/os/fd/trait.FromRawFd.html#tymethod.from_raw_fd)  |
| 25  | NonVolatile(p) | precond | [ptr::read()](https://doc.rust-lang.org/std/ptr/fn.read.html) |

**Note**: These primitives are not yet complete. New proposals are always welcome.

## 3 Safety Property Analysis

### 3.1 Layout
Refer to the document of [type-layout](https://doc.rust-lang.org/reference/type-layout.html), there are three components related to layout: alignment, size, and padding.

#### 3.1.1 Alignment
Alignment is measured in bytes. It must be at least 1 and is always a power of 2. This can be expressed as $2^x s.t. x\ge 0$. A memory address of type `T` is considered aligned if the address is a multiple of alignment(T). The alignment requirement can be formalized as $\text{addressof}(\text{instance}(T)) \\% \text{alignment}(T) = 0$

In practice, we generally require a pointer `p` of type `*T` to be aligned. This property can be formalized as:

**psp 1. Aligned(p, T)**: 

$$p \\% \text{alignment}(T) = 0$$ 

Example APIs: [ptr::read()](https://doc.rust-lang.org/nightly/std/ptr/fn.read.html), [ptr::write()](https://doc.rust-lang.org/std/ptr/fn.write.html), [Vec::from_raw_parts()](https://doc.rust-lang.org/beta/std/vec/struct.Vec.html#method.from_raw_parts)

#### 3.1.2 Size 
The size of a value is the offset in bytes between successive elements in an array with that item type including alignment padding. It is always a multiple of its alignment (including 0), i.e., $\text{sizeof}(T) \\% \text{alignment}(T)=0$. 

A safety property may require the size of a type `T` cannot be zero. We can formulate the requirement as 

**psp 2. NonZST(T)**:

$$\text{sizeof}(T) > 0$$

Example API: [NonNull.offset_from()](https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.offset_from), [pointer.sub_ptr()](https://doc.rust-lang.org/beta/std/primitive.pointer.html#method.sub_ptr)

#### 3.1.3 Padding 
Padding refers to the unused space inserted between successive elements in an array to ensure proper alignment. Padding is taken into account when calculating the size of each element. For example, the following data structure includes 1 byte of padding, resulting in a total size of 4 bytes.
```rust
struct MyStruct { a: u16,  b: u8 } // alignment: 2; padding 1
mem::size_of::<MyStruct>(); // size: 4
```

A safety property may require the type `T` has no padding. We can formulate the requirement as 

**psp 3. Padding(T)**:

$$\text{padding}(T)=0$$

Example API: intrinsic [raw_eq()](https://doc.rust-lang.org/std/intrinsics/fn.raw_eq.html)

### 3.2 Pointer Validity

Referring to the [pointer validity](https://doc.rust-lang.org/std/ptr/index.html#safety) documentation, whether a pointer is valid depends on the context of its usage, and the criteria vary across different APIs. To better describe pointer validity and reduce ambiguity, we break down the concept into several primitive components.

#### Address
The memory address that the pointer refers to is critical. A safety property may require the pointer `p` to be non-null, as the behavior of dereferencing a null pointer is undefined. This property can be formalized as:

**psp 4. NonNull(p)**:

$$p != \text{null}$$

Example APIs: [NonNull::new_unchecked()](https://doc.rust-lang.org/std/ptr/struct.NonNull.html#method.new_unchecked), [Box::from_non_null()](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_non_null)

#### 3.2.1 Allocation
To determine whether the memory address referenced by a pointer is available for use or has been allocated by the system (either on the heap or the stack), we consider the related safety requirement: non-dangling. This means the pointer must refer to a valid memory address that has not been deallocated on the heap or remains valid on the stack.

In practice, an API may enforce that a pointer `p` to a type `T` must satisfy the non-dangling property.

**psp 5. NonDangling(p, T)**: 

$$\text{allocator}(p) = x, s.t. \ x \in \lbrace \text{GlobalAllocator}, \text{OtherAllocator}, \text{stack} \rbrace\ ||\ \text{sizeof}(T) = 0 $$ 

**Proposition 1** (NOT SURE): NonDangling(p, T) implies NonNull(p).

Example APIs: [ptr::offset()](https://doc.rust-lang.org/beta/std/primitive.pointer.html#method.offset), [Box::from_raw()](https://doc.rust-lang.org/beta/std/boxed/struct.Box.html#method.from_raw)

Besides, some properties may require the allocator to be consistent, i.e., the memory address pointed by the pointer `p` should be allocated by a specific allocator `A`.

**psp 6.1. AllocatorConsistency(p, A)**: 

$$\text{allocator}(p) = A $$

Example APIs: [Arc::from_raw_in()](https://doc.rust-lang.org/std/sync/struct.Arc.html#method.from_raw_in), [Box::from_raw_in()](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_raw_in)

If the allocator `A` is unspecified, it typically defaults to the global allocator.

**psp 6.2. AllocatorConsistency(p)**: 

$$\text{allocator}(p) = \text{GlobalAllocator} $$

Example APIs: [Arc::from_raw()](https://doc.rust-lang.org/std/sync/struct.Arc.html#method.from_raw), [Box::from_raw()](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_raw)

#### 3.2.2 Pointee

A safety property may require that a pointer `p` refers to a value of a specific type `T`. This property can be formalized as:

**psp 7. Pointee(p, T)**: 

$$\text{typeof}(*p) = T $$

**Proposition 2** (NOT SURE): Pointee(p, T) implies NonDangling(p, T) and  NonNull(p).

Example APIs: [ptr::read()](https://doc.rust-lang.org/beta/std/primitive.pointer.html#method.read), [ptr::offset()](https://doc.rust-lang.org/beta/std/primitive.pointer.html#method.offset)

#### 3.2.3 Derived Safety Properties
There are two useful derived safety properties based on the previous components.

The first one is bounded access, which requires that the pointer access with respet to an offset stays within the bound. This ensures that dereferencing the pointer results in a value of the expected type T.

**psp 8. Bounded(p, T, offset)**: 

$$\text{typeof}(*(p + \text{sizeof}(T) * offset))  = T $$

Example APIs: [ptr::offset()](https://doc.rust-lang.org/std/primitive.pointer.html#method.offset), [ptr::copy()](https://doc.rust-lang.org/std/ptr/fn.copy.html) 

A safety property may require the two pointers do not overlap with respect to `T`: 

**psp 9.1. NonOverlap(dst, src, T)**: 

$$|dst - src| > \text{sizeof}(T)$$

(This requirement might be more regid than Rustdoc.)

Example APIs: [ptr::copy_from()](https://doc.rust-lang.org/std/ptr/fn.copy.html), [ptr.copy()](https://doc.rust-lang.org/std/ptr/fn.copy_from.html) 

It may also require the two pointers do not overlap with respect to $T*count$: 

**psp 9.2. NonOverlap(dst, src, T, count)**: 

$$|dst - src| > \text{sizeof}(T) * count $$

Example APIs: [ptr::copy_nonoverlapping()](https://doc.rust-lang.org/std/ptr/fn.copy_nonoverlapping.html), [ptr.copy_from_nonoverlapping](https://doc.rust-lang.org/core/primitive.pointer.html#method.copy_from_nonoverlapping)
 
### 3.3. Content

#### 3.3.1 Integer

When converting a value `x` to an interger, the value should not be greater than the max or less the min value that can be represented by the integer type `T`.

**psp 11.1. ValidInt(x, T)**: 

$$\text{T::MAX} \geq x \geq \text{T::MIN} $$

Example APIs: [f32.to_int_unchecked()](https://doc.rust-lang.org/std/primitive.f32.html#method.to_int_unchecked), [SimdFloat.to_int_unchecked()](https://doc.rust-lang.org/std/simd/num/trait.SimdFloat.html#tymethod.to_int_unchecked)

The result of interger arithmatic of two values `x` and `y` of type `T` should not overflow the max or the min value.

**psp 11.2. ValidInt(x, T, range)**: 

Example APIs: [NonZero::from_mut_unchecked()](https://doc.rust-lang.org/beta/std/num/struct.NonZero.html#tymethod.from_mut_unchecked), [isize.unchecked_div()](https://doc.rust-lang.org/nightly/core/intrinsics/fn.unchecked_div.html) |

**psp 11.3. ValidInt(binop, x, y, T)**:

$$\text{T::MAX} \geq \text{binop}(x, y) \geq \text{T::MIN} $$

Example APIs: [isize.add()](https://doc.rust-lang.org/std/primitive.isize.html#method.unchecked_add), [usize.add()](https://doc.rust-lang.org/std/primitive.usize.html#method.unchecked_add), [pointer.add(usize.add())](https://doc.rust-lang.org/std/primitive.pointer.html#method.add)

Unary arithmatic operations have similar requirements.

**psp 11.4. ValidInt(uop, x, T)**:

$$\text{T::MAX} \geq \text{uop}(x) \geq \text{T::MIN} $$

Some APIs may require the value `x` of an integer type should not be zero.

Example API: [isize.unchecked_neg()](https://doc.rust-lang.org/nightly/core/primitive.isize.html#method.unchecked_neg)

#### 3.3.2 String
There are two types of string in Rust, [String](https://doc.rust-lang.org/std/string/struct.String.htm) which requires valid utf-8 format, and [CStr](https://doc.rust-lang.org/std/ffi/struct.CStr.html) for interacting with foreign functions.

The safety properties of String requires the bytes contained in a vector `v` should be a valid utf-8.

**psp 12.1. ValidString(v)**:

$$v\in \text{utf-8}$$

Example APIs: [String::from_utf8_unchecked()](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_unchecked), [String.as_bytes_mut()](https://doc.rust-lang.org/std/string/struct.String.html#method.as_bytes_mut), [String.as_mut_vec()](https://doc.rust-lang.org/std/string/struct.String.html#method.as_mut_vec).

We have to label the hazard of the APIs [String.as_bytes_mut()](https://doc.rust-lang.org/std/string/struct.String.html#method.as_bytes_mut) and [String.as_mut_vec()](https://doc.rust-lang.org/std/string/struct.String.html#method.as_mut_vec) with ValidString(v) because mutating the resulting bytes or vector may lead to invalid utf-8.

The safety properties of String requires the content pointed by a pointer `p` of length `len` should be a valid utf-8.

**psp 12.2. ValidString(p, len)**: 

$$*(p..p+len) \in \text{utf-8}$$

Example API: [String::from_raw_parts()](https://doc.rust-lang.org/std/string/struct.String.html#method.from_raw_parts).

The content extracted from String `s` using slice index `I` must remain valid UTF-8.

**psp 12.3. ValidString(s, I)**: 

$$s[I] \in \text{utf-8}$$

Example APIs: [String.get_unchecked()](https://doc.rust-lang.org/std/string/struct.String.html#method.get_unchecked), [String.get_unchecked_mut()](https://doc.rust-lang.org/std/string/struct.String.html#method.get_unchecked_mut)

The slice content ranging from `begin` to `end` within String s must be valid UTF-8.

**psp 12.4. ValidString(s, begin, end)**: 

$$s[begin..end] \in \text{utf-8}$$

Example APIs: [String.slice_unchecked()](https://doc.rust-lang.org/std/string/struct.String.html#method.slice_unchecked), [String.slice_mut_unchecked()](https://doc.rust-lang.org/std/string/struct.String.html#method.slice_mut_unchecked)

The safety property of CString generally requires the bytes of a u8 slice or pointed by a pointer `p` shoule contains a null terminator within isize::MAX from `p`.

**psp 13. ValidCStr(p, len)**:

$$\exists offset,\ s.t., *(p+offset) = \text{null}\ \\&\\&\ \text{ValidInt}(offset, isize) $$ 

Example APIs: [CStr::from_bytes_with_nul_unchecked()](https://doc.rust-lang.org/std/ffi/struct.CStr.html#method.from_bytes_with_nul_unchecked), [CStr::from_ptr()](https://doc.rust-lang.org/std/ffi/struct.CStr.html#method.from_ptr)

#### 3.3.3 Initialization
A safety property may require the memory of type `T` pointed by a pointer `p` is initialized.

**psp 14. Init(p, T)**:

$$\text{init}(*p, T) = true $$

Example APIs: [MaybeUninit.assume_init()](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html#method.assume_init), [Box::assume_init()](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.assume_init)

#### 3.3.4 Unwrap

Such safety properties relate to the monadic types, including [Option](https://doc.rust-lang.org/std/option/enum.Option.html) and [Result](https://doc.rust-lang.org/std/result/enum.Result.html), and they require the value after unwarpping should be of a particular type.

**psp 15. Unwrap(x, T)**:

$$\text{unwrap}(r) = x,\ s.t., \text{typeof}(x) \in \lbrace \text{Ok}, \text{Err}, \text{Some}, \text{None} \rbrace $$

Example APIs: [Option::unwrap_unchecked()](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_unchecked), [Result::unwrap_unchecked()](https://doc.rust-lang.org/core/result/enum.Result.html#method.unwrap_unchecked), [Result::unwrap_err_unchecked()](https://doc.rust-lang.org/core/result/enum.Result.html#method.unwrap_err_unchecked)

### 3.4 Aliases
This category relates to the core mechanism of Rust which aims to avoid shared mutable aliases and achieve automated memory deallocation. 

#### 3.4.1 Onwership
Let one value has two owners at the same program point is vulnerable to double free. Refer to the traidional vulnerbility of [mem::forget()](https://doc.rust-lang.org/std/mem/fn.forget.html) compared to [ManuallyDrop](https://doc.rust-lang.org/std/mem/struct.ManuallyDrop.html). The property generally relates to convert a raw pointer to an ownership, and it can be represented as:

**psp 16. NonOwned(p)**:

$$\text{hasowner}(*p) = false $$

Example APIs: [Box::from_raw()](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_raw), [ptr::read()](https://doc.rust-lang.org/std/ptr/fn.read.html), [ptr::read_volatile()](https://doc.rust-lang.org/std/ptr/fn.read_volatile.html),

**psp 17. Owned(p)**:

$$\text{hasowner}(*p) = true $$

Example APIs: [trait.FromRawFd::from_raw_fd()](https://doc.rust-lang.org/std/os/fd/trait.FromRawFd.html#tymethod.from_raw_fd), [UdpSocket::from_raw_socket()](https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.from_raw_socket)

(TO FIX: there should be similar issues for other RAII resources, we may not need this because FFI memories cannot require owned.)

#### 3.4.2 Alias
There are six types of pointers to a value x, depending on the mutabality and ownership, i.e., owner, mutable owner, reference, mutable reference, raw pointer, mutable raw pointer. The exclusive mutability principle of Rust requires that if a value has a mutable alias at one program point, it must not have other aliases at that program point. Otherwise, it may incur unsafe status. We need to track the particular unsafe status and avoid unsafe behaviors.

**psp 18. Alias(p1, p2)**:

$$*p1 = *p2 $$

Example APIs: [pointer.as_mut()](https://doc.rust-lang.org/std/primitive.pointer.html#method.as_mut), [pointer.as_ref()](https://doc.rust-lang.org/std/primitive.pointer.html#method.as_ref-1), [pointer.as_ref_unchecked()](https://doc.rust-lang.org/std/primitive.pointer.html#method.as_ref_unchecked-1)

#### 3.4.3 Lifetime

The property generally requires the lifetime of a raw pointer `p` must be valid for both reads and writes for the whole lifetime 'a.

**psp 19. Lifetime(p, 'a)**:

$$\text{lifetime}(*p)>\'a$$

Example APIs: [AtomicPtr::from_ptr()](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicPtr.html#method.from_ptr), [AtomicBool::from_ptr()](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicBool.html#method.from_ptr), [CStr::from_ptr()](https://doc.rust-lang.org/std/ffi/struct.CStr.html#method.from_ptr)

### 3.5 More

#### 3.5.1 Trait

If a parameter type `T` implements certain traits, it can guarantee safety or mitigate specific hazards

**psp 20. Trait(T)**:

$$t \in \text{trait}(T),\ t \in \lbrace \text{Copy}, \text{Unpin} \rbrace $$

In particular, $\text{Copy} \in \text{trait}(T)$ ensures that alias issues or Alias(p) are mitigated, and $\text{Unpin} \in \text{trait}(T)$ avoids the hazard associated with pinned data or Pinned(p).

Example APIs: [ptr::read()](https://doc.rust-lang.org/std/ptr/fn.read.html), [ptr::read_volatile()](https://doc.rust-lang.org/std/ptr/fn.read_volatile.html), [Pin::new_unchecked()](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.new_unchecked)


#### 3.5.2 Thread-Safe (Atomicity)
Refer to the [Rustnomicon](https://doc.rust-lang.org/nomicon/send-and-sync.html), it generally relates to the implementation of the Send/Sync attribute that requires update operations of a critical memory to be exclusive. 

Automatically verifying the correctness of the Send trait implementation for any type T is difficult. However, implementing the Send trait can be considered safe if T has no field of `Rc` type.

**psp 21. Send(T, NoRc)**:

$$\forall field \in T,\ \text{hasrefcound}(field) = false$$

The function `hasrefcount(field)` evaluates to true if the type of the field is `Rc`. If the field is also a compound type, we should recursively evaluate each sub-field.

(TO FIX: This can be defined in a recursive form.)

This is a conditional precondition for implementing the Send trait (NOT SURE: Is it sufficient?). Since implementations of Send trait must not introduce concurrency issues, we do not define corresponding hazards.

Similar to Send(T), we can define the followin optional precondition for Sync(T):

**psp 22. Sync(T, NoInteriorMut)**:

$$\forall field \in T,\ \text{interiormut}(field) = false$$

The function interiormut(field) evaluates to trues if the type of the field is `Cell`, `RefCell`, or `UnsafeCell`. If the field is also a compound type, we should recursively evaluate each sub-field.

(TO FIX: This should be changed to a recursive form.)

Example APIs: Auto trait [Send](https://doc.rust-lang.org/std/marker/trait.Send.html), [Sync](https://doc.rust-lang.org/std/marker/trait.Sync.html)

#### 3.5.3 Pin
Implementing `Pin` for `!Unpin` is also valid in Rust, developers should not move the pinned object pointed by `p` after created.

**psp 23. Pinned(p)**:

$$\forall t > \text{timeofpin},\text{addressof}(*p) = p $$

Example APIs: [Pin::new_unchecked()](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.new_unchecked),[Pin.into_inner_unchecked()](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.into_inner_unchecked), [Pin.map_unchecked()](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.map_unchecked), [Pin.get_unchecked_mut()](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.get_unchecked_mut), [Pin.map_unchecked_mut](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.map_unchecked_mut())

#### 3.5.4 File Read/Write

The file discripter `fd` must be opened.

**psp 24. Opened(fd)**:

$$\text{opened}(fd) = true$$

Example APIs: [trait.FromRawFd::from_raw_fd()](https://doc.rust-lang.org/std/os/fd/trait.FromRawFd.html#tymethod.from_raw_fd), [UdpSocket::from_raw_socket()](https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.from_raw_socket)

#### 3.5.5 Volatility

There are specific APIs for volatile memory access in std-lib, like [ptr::read_volatile()](https://doc.rust-lang.org/std/ptr/fn.read_volatile.html) and [ptr::write_volatile()](https://doc.rust-lang.org/std/ptr/fn.write_volatile.html). Other memory operations should require non-volatile by default.

**psp 25. NonVolatile(p)**:

$$\text{volatile}(*p) = false$$

Example APIs: [ptr::read()](https://doc.rust-lang.org/std/ptr/fn.read.html), [ptr::write()](https://doc.rust-lang.org/std/ptr/fn.write.html)

### 4 Primitive Properties Yet to Be Considered

- [GlobalAlloc](https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html)
