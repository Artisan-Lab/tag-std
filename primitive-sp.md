# Privimitive Safety Properties for Rust Contract Design (Draft)

This document presents a draft outlining the fundamental safety properties essential for contract definition. The current documentation on API safety descriptions in the standard library remains ad hoc. For example, the term `valid pointer` is frequently used, but the validity of a pointer depends on the context, as explained in [Rustdoc](https://doc.rust-lang.org/std/ptr/index.html), posing difficulties for developers in interpreting the exact safety requirements of an unsafe API if they are unfamiliar with the background. We hope to provide explicit and non-ambiguous safety descriptions for developers. Using pointer validity as an example, a pointer may need to satisfy several fundamental requirements (which cannot be further broken down) to be valid, such as being non-null, not dangling, and pointing to memory that is properly aligned and initialized for type T. It is worth noting that the Rust community is making progress toward standardizing contract design, as highlighted in the links below. We believe this proposal will contribute to the development and refinement of contract specifications.

[std-contracts-2025h1](https://rust-lang.github.io/rust-project-goals/2025h1/std-contracts.html)  
[MCP759](https://github.com/rust-lang/compiler-team/issues/759)  
[Rust Contracts RFC (draft)](https://github.com/rust-lang/lang-team/blob/master/design-meeting-minutes/2022-11-25-contracts.md)  


## 1 Overall Idea
Traditional contract enforces two types of safety invariant: precondition and postcondition, but they do not fully align with Rust’s approach to safety property descriptions. 

**Precondition**: Safety requirements that must be satisfied before invoking an unsafe API. These represent the fundamental conditions for safely using the API.

**Postcondition**: This refers to a set of properties the system must satisfy after an API call. It is mainly used as the constraint to verify the implementation correctness of the API.

In Rust, most safety properties for unsafe APIs are preconditions. In comparison, postconditions are not generally required (expect when implimenting unsafe traits) because API users do not need to be concerned with the correctness of the API’s implementation. However, there are some safety properties which are not preconditions but instead highlight potential hazards of unsafe APIs. For instance, certain scenarios such as implementing a doubly linked list require temporarily violating the safety invariant of Rust, e.g., via [pointer::as_mut()](https://doc.rust-lang.org/std/primitive.pointer.html#method.as_mut). In such cases, it is crucial to document how the program state deviates from Rust's safety principles and whether these vulnerabilities are eventually resolved.

**Hazard (new)**: Invoking an unsafe API may temporarily leave the program in a vulnerable state with respect to the safety invariant of Rust. 

Besides, there are also optional preconditions in Rustdoc. If these conditions are satisfied, the Rust compiler can guarantee that the safety invariant will hold. However, meeting these optional requirements is not mandatory. For example, [ptr::read()](https://doc.rust-lang.org/std/ptr/fn.read.html) specifies that the parameter implements the Copy trait can help avoid undefined behaviors related to exclusive mutability. By meeting this optional precondition, developers can ensure safe use of the API while still having the flexibility to omit it when not needed. Optional preconditions exist because exist because enumerating all possible safe usages can be difficult, and they can provide a shortcut for safety assurance.

**Option (new)**: Optional preconditions for an unsafe API. If such conditions are satisfied, they can ensure the safety invariant of Rust.

<span style="color: red;"> **In short, while preconditions must be satisfied, optional preconditions are not mandatory. Hazards highlight vulnerabilities that deviate from Rust's safety principles. Meeting optional preconditions can help avoid certain types of hazards.** </span>

In practice, a safety property may correspond to a precondition, an optional precondition, or a hazard. To eliminate ambiguity in high-level or ad hoc safety property descriptions, we propose breaking them down into primitive safety requirements. The following sections will elaborate on these details.

## 2 Summary 

### 2.1 Summary of Primitive SPs

| ID  | Primitive SP | Meaning | Usage | Example API |
|---|---|---|---|---|
| I.1  | Align(p, T) | p \% alignment(T) = 0 | precond | [ptr::read()](https://doc.rust-lang.org/nightly/std/ptr/fn.read.html) | 
| I.2  | Size(T, c) | sizeof(T) = c, c $\in$ \{num, unknown, any\} | option | [Layout::for_value_raw()](https://doc.rust-lang.org/nightly/std/alloc/struct.Layout.html#method.for_value_raw)  | 
|      | - | - | precond | [NonNull::offset_from](https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.offset_from)  | 
| I.3  | !Padding(T)  | padding(T) = 0 | precond  | [intrinsics::raw_eq()](https://doc.rust-lang.org/std/intrinsics/fn.raw_eq.html) |
| II.1  | !Null(p) | p!= 0 | precond  | [NonNull::new_unchecked()](https://doc.rust-lang.org/std/ptr/struct.NonNull.html#method.new_unchecked) |
| II.2 | Allocated(p, T, len, A) | $\forall$ i $\in$ 0..sizeof(T)*len, allocator(p+i) = A | precond | [Box::from_raw_in()](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_raw_in) |
| II.3  | InBound(p, T, len, arange) | [p, p+ sizeof(T) * len) $\in$ arange  | precond | [ptr::offset()](https://doc.rust-lang.org/std/primitive.pointer.html#method.offset)  |
| II.4  | !Overlap(dst, src, T, len) | \|dst - src\| $\ge$ sizeof(T) * len | precond | [ptr::copy_nonoverlapping()](https://doc.rust-lang.org/std/ptr/fn.copy_nonoverlapping.html)  |
| II.5  | Typed(p, T) | typeof(*p) = T | precond | [Rc::from_raw()](https://doc.rust-lang.org/beta/std/rc/struct.Rc.html#method.from_raw) |
| III.1  | ValidNum(exp, vrange)  | exp $\in$ vrange | precond | [usize::add()](https://doc.rust-lang.org/std/primitive.usize.html#method.unchecked_add)  |
| III.2  | ValidString(arange) | mem(arange) $\in$ utf-8 |  precond | [String::from_utf8_unchecked()](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_unchecked) |
|        | ValidString(arange) | - | hazard | [String::as_bytes_mut()](https://doc.rust-lang.org/std/string/struct.String.html#method.as_bytes_mut) |
| III.3  | ValidCStr(p, len) | mem(p+len, p+len+1) = '\0' | precond|  [CStr::from_bytes_with_nul_unchecked()](https://doc.rust-lang.org/std/ffi/struct.CStr.html#method.from_bytes_with_nul_unchecked)  |
| III.4  | Init(p, T, len)  | $\forall$ i $\in$ 0..len, mem(p + sizeof(T) * i, p + sizeof(T) * (i+1)) = valid(T) | precond | [MaybeUninit::slice_assume_init_mut()](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html#method.slice_assume_init_mut) |
|         | -  | - | hazard | [ptr::copy()](https://doc.rust-lang.org/std/ptr/fn.copy.html) |
|         | -  | - | option | [ptr::copy()](https://doc.rust-lang.org/std/ptr/fn.copy.html) |
| III.5  | Unwrap(x, T) | unwrap(x) = T | precond | [Option::unwrap_unchecked()](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_unchecked)  |
| IV.1  | Ownning(p) | ownership(*p) = none | precond | [Box::from_raw()](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_raw)  |
| IV.2  | Alias(p1, p2) | p1 = p2 | hazard | [pointer::as_mut()](https://doc.rust-lang.org/std/primitive.pointer.html#method.as_mut) |
| IV.3  | Alive(p, l) | lifetime(*p) $\ge$ l | precond | [AtomicPtr::from_ptr()](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicPtr.html#method.from_ptr)  |
| V.1  | Pinned(p, l) | $$\forall t \in 0..l, \\&(*p)_0 = p_t$$ | hazard | [Pin::new_unchecked()](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.new_unchecked)  |
| V.2  | !Volatile(p) | volatile(*p) = false | precond | [ptr::read()](https://doc.rust-lang.org/std/ptr/fn.read.html) |
| V.3  | Opened(fd) | opened(fd) = true | precond | [trait::FromRawFd::from_raw_fd()](https://doc.rust-lang.org/std/os/fd/trait.FromRawFd.html#tymethod.from_raw_fd)  |
| V.4  | Trait(T, trait) | trait $\in$ traitimpl(T) | option | [ptr::read()](https://doc.rust-lang.org/std/ptr/fn.read.html)  |
| V.5  | !Reachable(I) | sat(cond(I)) = false | precondition | [intrinsics::read()](https://doc.rust-lang.org/nightly/std/intrinsics/fn.unreachable.html) |

**Note**: These primitives are not yet complete. New proposals are always welcome. 

### 2.2 Compound SPs used in Rustdoc

| SP in Rustdoc | Compound SP | Meaning | Usage | Example API |
|---|---|---|---|---|   
| [Valid pointer](https://doc.rust-lang.org/nightly/std/ptr/index.html) | ValidPtr(p, T, len, arange) | Size(T, 0) \|\| (!Size(T,0) && Deref(p, T, len, arange) ) | precond | [ptr::read<T>()](https://doc.rust-lang.org/nightly/std/ptr/fn.read.html)  |       
| Dereferenceable | Deref(p, T, len, arange) | Allocated(p, T, len, *) && InBound(p, T, len, arange) | precond | only used to define valid pointers |
| Valid pointer to reference conversion | Ptr2Ref(p, T) | Allocated(p, T, 1, *) && Init(p, T, 1) && Align(p, T) && Alias(p, *) | precond, hazard | [ptr::as_uninit_ref()](https://doc.rust-lang.org/nightly/std/ptr/struct.NonNull.html#method.as_uninit_ref) |
| Layout Consistency | Layout(p, layout) | Align(p, layout.align) && Allocated(p, layout.size, any) | precond | [GlobalAlloc::realloc()](https://doc.rust-lang.org/nightly/std/alloc/trait.GlobalAlloc.html#method.realloc) | 

### 2.3 Synonymous SPs used in Rustdoc

| SP in Rustdoc | Primitive SP | 
|---|---|
| Non-Dangling | Allocated(p, T, len, A) |

## 3 Safety Property Details

### 3.1 Layout
Refer to the document of [type-layout](https://doc.rust-lang.org/reference/type-layout.html), there are three components related to layout: alignment, size, and padding.

#### 3.1.1 Alignment
Alignment is measured in bytes. It must be at least 1 and is always a power of 2. This can be expressed as $2^x$ s.t. $x\ge 0$. A memory address of type `T` is considered aligned if the address is a multiple of alignment(T). The alignment requirement can be formalized as $\text{addressof}(\text{instance}(T)) \\% \text{alignment}(T) = 0$

In practice, we generally require a pointer `p` of type `*T` to be aligned. This property can be formalized as:

**psp I.1 Align(p, T)**: 

$$p \\% \text{alignment}(T) = 0$$

Example APIs: [ptr::read()](https://doc.rust-lang.org/nightly/std/ptr/fn.read.html), [ptr::write()](https://doc.rust-lang.org/std/ptr/fn.write.html), [Vec::from_raw_parts()](https://doc.rust-lang.org/beta/std/vec/struct.Vec.html#method.from_raw_parts)

#### 3.1.2 Size 
The size of a value is the offset in bytes between successive elements in an array with that item type including alignment padding. It is always a multiple of its alignment (including 0), i.e., $\text{sizeof}(T) \\% \text{alignment}(T)=0$. We can represent the size-related properties as below:

**psp I.2 Size(T, c)**:

$$sizeof(T) = c\ \\&\\&\ c \in \{num, unknown, any\}$$

For example, not all types are statically sized, such as slices and trait objects. Therefore, a safety property may require the size of a type `T` can be determined during compiling time. We can represent the property as `Size(T, any)`. This is generally emplopyed as an optional property.

Example API: [Layout::for_value_raw()](https://doc.rust-lang.org/nightly/std/alloc/struct.Layout.html#method.for_value_raw)

Besides, a safety property may require the size of a type `T` cannot be zero. We can formulate the requirement as `Size(T, 0)`.

Example APIs: [NonNull::offset_from()](https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.offset_from), [pointer::sub_ptr()](https://doc.rust-lang.org/beta/std/primitive.pointer.html#method.sub_ptr)

#### 3.1.3 Padding 
Padding refers to the unused space inserted between successive elements in an array to ensure proper alignment. Padding is taken into account when calculating the size of each element. For example, the following data structure includes 1 byte of padding, resulting in a total size of 4 bytes.
```rust
struct MyStruct { a: u16,  b: u8 } // alignment: 2; padding 1
mem::size_of::<MyStruct>(); // size: 4
```

A safety property may require the type `T` has no padding. We can formulate the requirement as 

**psp I.3 Padding(T, false)**:

$$\text{padding}(T) = 0$$

Example API: [intrinsics::raw_eq()](https://doc.rust-lang.org/std/intrinsics/fn.raw_eq.html)

### 3.2 Pointer Validity

Referring to the [pointer validity](https://doc.rust-lang.org/std/ptr/index.html#safety) documentation, whether a pointer is valid depends on the context of its usage, and the criteria vary across different APIs. To better describe pointer validity and reduce ambiguity, we break down the concept into several primitive components.

#### Address
The memory address that the pointer refers to is critical. A safety property may require the pointer `p` to be non-null, as the behavior of dereferencing a null pointer is undefined. This property can be formalized as:

**psp II.1 !Null(p)**:

$$p != 0$$

Example APIs: [NonNull::new_unchecked()](https://doc.rust-lang.org/std/ptr/struct.NonNull.html#method.new_unchecked), [Box::from_non_null()](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_non_null)

#### 3.2.1 Allocation
To determine whether the memory address referenced by a pointer is available for use or has been allocated by the system (either on the heap or the stack), we consider the related safety requirement: non-dangling or allocated. This means the pointer must refer to a valid memory address that has not been deallocated on the heap or remains valid on the stack. In practice, an API may require that the entire memory region of length `len` for type `T`, as pointed to by `p` of type `*mut T`, be allocated. Besides, some APIs may require the allocator to be consistent, i.e., the memory address pointed by the pointer `p` should be allocated by a specific allocator `A`.

**psp II.2 Allocated(p, T, len, A)**: 

$$\forall i \in 0..sizeof(T)*len, allocator(p+i) = A $$

Example APIs: [Arc::from_raw_in()](https://doc.rust-lang.org/std/sync/struct.Arc.html#method.from_raw_in), [Box::from_raw_in()](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_raw_in)

If the allocator `A` is unspecified, it typically defaults to the global allocator.

Example APIs: [Arc::from_raw()](https://doc.rust-lang.org/std/sync/struct.Arc.html#method.from_raw), [Box::from_raw()](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_raw), [ptr::offset()](https://doc.rust-lang.org/beta/std/primitive.pointer.html#method.offset), [Box::from_raw()](https://doc.rust-lang.org/beta/std/boxed/struct.Box.html#method.from_raw)

Bounded access requires that the pointer access with respet to an offset stays within the bound of the same allocated object. This ensures that dereferencing the pointer yields a value (which may not yet be initialized) of the expected type T. 

**psp II.3 InBound(p, T, len, arange)**: 

$$[p, p+ sizeof(T) * len) \in arange $$

Example APIs: [ptr::offset()](https://doc.rust-lang.org/std/primitive.pointer.html#method.offset), [ptr::copy()](https://doc.rust-lang.org/std/ptr/fn.copy.html) 

A safety property may require the two pointers do not overlap with respect to `T` or  $T*count$:

**psp II.4 !Overlap(dst, src, T, len)**: 

$$|dst - src| > \text{sizeof}(T) * len $$

Example APIs: [ptr::copy_from()](https://doc.rust-lang.org/std/ptr/fn.copy.html), [ptr::copy()](https://doc.rust-lang.org/std/ptr/fn.copy_from.html), [ptr::copy_nonoverlapping()](https://doc.rust-lang.org/std/ptr/fn.copy_nonoverlapping.html), [ptr::copy_from_nonoverlapping](https://doc.rust-lang.org/core/primitive.pointer.html#method.copy_from_nonoverlapping)

Besides, some APIs accepts a raw pointer as the input and requires the raw pointer must have been previously returned by a call of `into_raw` from the same module.

**psp II.5 Typed(p, T)**: 

$$\text{sizeof}(*p) = T $$

Note that this may also concern the memory space ahead of p.

Example APIs: [Rc::from_raw()](https://doc.rust-lang.org/beta/std/rc/struct.Rc.html#method.from_raw), [Arc::from_raw()](https://doc.rust-lang.org/beta/std/sync/struct.Arc.html#method.from_raw), [Weak::from_raw()](https://doc.rust-lang.org/beta/std/sync/struct.Weak.html#method.from_raw),[Thread::from_raw()](https://doc.rust-lang.org/beta/std/thread/struct.Thread.html#method.from_raw)

### 3.3. Content

#### 3.3.1 Integer

When converting a value `x` to an interger or performing integer arithmetic, the result should not be greater than the max or less the min value that can be represented by the integer type `T`.

**psp III.1 ValidNum(exp, vrange)**: 

The first parameter `exp` stands for an arithmetic expression in the form of `(binOperator, operand1, operand2)` or `(unaryOperator, operand)`, where the `operand` can also an `expression`. The second parameter `vrange` specifies the range of valid values, such as `[isize::MIN, isize::MAX]`.

Example APIs: [f32.to_int_unchecked()](https://doc.rust-lang.org/std/primitive.f32.html#method.to_int_unchecked), [SimdFloat.to_int_unchecked()](https://doc.rust-lang.org/std/simd/num/trait.SimdFloat.html#tymethod.to_int_unchecked), [NonZero::from_mut_unchecked()](https://doc.rust-lang.org/beta/std/num/struct.NonZero.html#tymethod.from_mut_unchecked), [isize.unchecked_div()](https://doc.rust-lang.org/nightly/core/intrinsics/fn.unchecked_div.html), [u32::unchecked_shl()](https://doc.rust-lang.org/nightly/core/intrinsics/fn.unchecked_shl.html), [u32::unchecked_shr()](https://doc.rust-lang.org/nightly/core/intrinsics/fn.unchecked_shr.html), [isize.unchecked_neg()](https://doc.rust-lang.org/nightly/core/primitive.isize.html#method.unchecked_neg), [isize.add()](https://doc.rust-lang.org/std/primitive.isize.html#method.unchecked_add), [usize.add()](https://doc.rust-lang.org/std/primitive.usize.html#method.unchecked_add), [pointer.add(usize.add())](https://doc.rust-lang.org/std/primitive.pointer.html#method.add), [slice::from_raw_parts()](https://doc.rust-lang.org/nightly/std/slice/fn.from_raw_parts.html) 

#### 3.3.2 String
There are two types of string in Rust, [String](https://doc.rust-lang.org/std/string/struct.String.htm) which requires valid utf-8 format, and [CStr](https://doc.rust-lang.org/std/ffi/struct.CStr.html) for interacting with foreign functions.

The safety properties of String requires the bytes contained in a vector `v` should be a valid utf-8.

**psp III.2 ValidString(arange)**:

$$mem(arange)\in \text{utf-8}$$

The parameter `arange` specifies an address range. For different APIs, the address range can be specified with `(pointer, T, length)' or a vector `v`, etc.

Example APIs: [String::from_utf8_unchecked()](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_unchecked), [String::as_bytes_mut()](https://doc.rust-lang.org/std/string/struct.String.html#method.as_bytes_mut), [String::as_mut_vec()](https://doc.rust-lang.org/std/string/struct.String.html#method.as_mut_vec), [String::from_raw_parts()](https://doc.rust-lang.org/std/string/struct.String.html#method.from_raw_parts), [String::get_unchecked()](https://doc.rust-lang.org/std/string/struct.String.html#method.get_unchecked), [String::get_unchecked_mut()](https://doc.rust-lang.org/std/string/struct.String.html#method.get_unchecked_mut), [String::slice_unchecked()](https://doc.rust-lang.org/std/string/struct.String.html#method.slice_unchecked), [String::slice_mut_unchecked()](https://doc.rust-lang.org/std/string/struct.String.html#method.slice_mut_unchecked)

We have to label the hazard of the APIs [String::as_bytes_mut()](https://doc.rust-lang.org/std/string/struct.String.html#method.as_bytes_mut) and [String::as_mut_vec()](https://doc.rust-lang.org/std/string/struct.String.html#method.as_mut_vec) with ValidString(v) because mutating the resulting bytes or vector may lead to invalid utf-8.

**psp III.3 ValidCStr(p, len)**:

$$\text{mem}(p+len, p+len+1) =$$ '\0'

Example APIs: [CStr::from_bytes_with_nul_unchecked()](https://doc.rust-lang.org/std/ffi/struct.CStr.html#method.from_bytes_with_nul_unchecked), [CStr::from_ptr()](https://doc.rust-lang.org/std/ffi/struct.CStr.html#method.from_ptr)

#### 3.3.3 Initialization
A safety property may require a range of memory pointed by a pointer `p` is initialized. This range of memory can be independent of type T.

**psp III.4 Init(p, T, len)**:

$$\forall i \in 0..len, \text{men}(p + \text{sizeof}(T) * i, p + \text{sizeof}(T) * (i+1)) = \text{valid}(T) $$

Note that this property may serve as either preconditions (e.g., [MaybeUninit::slice_assume_init_mut()](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html#method.slice_assume_init_mut)) or optional requirements and hazards (e.g., [ptr::copy()](https://doc.rust-lang.org/std/ptr/fn.copy.html).

Example APIs: [BorrowedBuf::set_init()](https://doc.rust-lang.org/nightly/std/io/struct.BorrowedBuf.html#method.set_init), [MaybeUninit::assume_init()](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html#method.assume_init), [Box::assume_init()](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.assume_init)[MaybeUninit::slice_assume_init_mut()](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html#method.slice_assume_init_mut), [ptr::copy()](https://doc.rust-lang.org/std/ptr/fn.copy.html), [ptr::copy_nonoverlapping](https://doc.rust-lang.org/std/ptr/fn.copy_nonoverlapping.html), [NonNull::copy_from](https://doc.rust-lang.org/std/ptr/struct.NonNull.html#method.copy_from)

#### 3.3.4 Unwrap

Such safety properties relate to the monadic types, including [Option](https://doc.rust-lang.org/std/option/enum.Option.html) and [Result](https://doc.rust-lang.org/std/result/enum.Result.html), and they require the value after unwarpping should be of a particular type.

**psp III.5 Unwrap(x, T, target)**:

$$\text{unwrap}(x) = target,\ s.t., \text{typeof}(target) \in \lbrace \text{Ok(T)}, \text{Err(E)}, \text{Some(T)}, \text{None} \rbrace $$

Example APIs: [Option::unwrap_unchecked()](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_unchecked), [Result::unwrap_unchecked()](https://doc.rust-lang.org/core/result/enum.Result.html#method.unwrap_unchecked), [Result::unwrap_err_unchecked()](https://doc.rust-lang.org/core/result/enum.Result.html#method.unwrap_err_unchecked)

### 3.4 Alias
This category relates to the core mechanism of Rust which aims to avoid shared mutable aliases and achieve automated memory deallocation. 

#### 3.4.1 Onwership
Let one value has two owners at the same program point is vulnerable to double free. Refer to the traidional vulnerbility of [mem::forget()](https://doc.rust-lang.org/std/mem/fn.forget.html) compared to [ManuallyDrop](https://doc.rust-lang.org/std/mem/struct.ManuallyDrop.html). The property generally relates to convert a raw pointer to an ownership, and it can be represented as:

**psp IV.1 Ownning(p)**:

$$\text{ownership}(*p) = none $$

Example APIs: [Box::from_raw()](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_raw), [ptr::read()](https://doc.rust-lang.org/std/ptr/fn.read.html), [ptr::read_volatile()](https://doc.rust-lang.org/std/ptr/fn.read_volatile.html), [FromRawFd::from_raw_fd()](https://doc.rust-lang.org/std/os/fd/trait.FromRawFd.html#tymethod.from_raw_fd), [UdpSocket::from_raw_socket()](https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.from_raw_socket)

#### 3.4.2 Alias
There are six types of pointers to a value x, depending on the mutabality and ownership, i.e., owner, mutable owner, reference, mutable reference, raw pointer, mutable raw pointer. The exclusive mutability principle of Rust requires that if a value has a mutable alias at one program point, it must not have other aliases at that program point. Otherwise, it may incur unsafe status. We need to track the particular unsafe status and avoid unsafe behaviors.

**psp IV.2 Alias(p1, p2)**:

$$p1 = p2 $$

Example APIs: [pointer.as_mut()](https://doc.rust-lang.org/std/primitive.pointer.html#method.as_mut), [pointer.as_ref()](https://doc.rust-lang.org/std/primitive.pointer.html#method.as_ref-1), [pointer.as_ref_unchecked()](https://doc.rust-lang.org/std/primitive.pointer.html#method.as_ref_unchecked-1)

#### 3.4.3 Lifetime

The property generally requires the lifetime of a raw pointer `p` must be valid for both reads and writes for the whole lifetime 'a.

**psp IV.3 Alive(p, l)**:

$$\text{lifetime}(*p) \ge l$$

Example APIs: [AtomicPtr::from_ptr()](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicPtr.html#method.from_ptr), [AtomicBool::from_ptr()](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicBool.html#method.from_ptr), [CStr::from_ptr()](https://doc.rust-lang.org/std/ffi/struct.CStr.html#method.from_ptr)

### 3.5 Misc

#### 3.5.1 Pin
Implementing `Pin` for `!Unpin` is also valid in Rust, developers should not move the pinned object pointed by `p` after created.

**psp V.1 Pinned(p)**:

$$\forall t \in 0..l, \\&(*p)_0 = p_t$$

Example APIs: [Pin::new_unchecked()](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.new_unchecked),[Pin::into_inner_unchecked()](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.into_inner_unchecked), [Pin.map_unchecked()](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.map_unchecked), [Pin::get_unchecked_mut()](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.get_unchecked_mut), [Pin.map_unchecked_mut](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.map_unchecked_mut())

#### 3.5.2 Volatility

There are specific APIs for volatile memory access in std-lib, like [ptr::read_volatile()](https://doc.rust-lang.org/std/ptr/fn.read_volatile.html) and [ptr::write_volatile()](https://doc.rust-lang.org/std/ptr/fn.write_volatile.html). Other memory operations should require non-volatile by default.

**psp V.6 !Volatile(p)**:

$$\text{volatile}(*p) = false$$

Example APIs: [ptr::read()](https://doc.rust-lang.org/std/ptr/fn.read.html), [ptr::write()](https://doc.rust-lang.org/std/ptr/fn.write.html)

#### 3.5.3 File Read/Write

The file discripter `fd` must be opened.

**psp V.3 Opened(fd)**:

$$\text{opened}(fd) = true$$

Example APIs: [FromRawFd::from_raw_fd()](https://doc.rust-lang.org/std/os/fd/trait.FromRawFd.html#tymethod.from_raw_fd), [UdpSocket::from_raw_socket()](https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.from_raw_socket)

#### 3.5.4 Trait

If a parameter type `T` implements certain traits, it can guarantee safety or mitigate specific hazards

**psp V.4 Trait(T, trait)**:

$$trait \in \text{trait}(T) $$

In particular, $\text{Copy} \in \text{trait}(T)$ ensures that alias issues or Alias(p) are mitigated, and $\text{Unpin} \in \text{trait}(T)$ avoids the hazard associated with pinned data or Pinned(p).

Example APIs: [ptr::read()](https://doc.rust-lang.org/std/ptr/fn.read.html), [ptr::read_volatile()](https://doc.rust-lang.org/std/ptr/fn.read_volatile.html), [Pin::new_unchecked()](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.new_unchecked)

#### 3.5.5 Unreachable

The current program point should not be reachable during execution.

**psp V.5 !Reachable(I)**:

$$sat(cond(I)) = false$$

Example APIs: [intrinsics::unreachable()](https://doc.rust-lang.org/nightly/std/intrinsics/fn.unreachable.html), [hint::unreachable_unchecked()](https://doc.rust-lang.org/nightly/std/hint/fn.unreachable_unchecked.html)

### 4 Primitive Properties Yet to Be Considered

- Unsafe Trait
- [GlobalAlloc](https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html)
- DSTs like slice and dynamic trait objects
