# tag-std

This project aims to standardize the safety property annotation of the Rust core and standard library. There are three steps:
- Define the [primitive safety properties](primitive-sp.md) to be used for describing the safety concerns of unsafe APIs.
- [Label the unsafe APIs](usage.md) in Rust's core and standard library with primitive safety properties.
- Detect and solve discripencies via program analysis.

Through this project, we aim to establish a foundation for Rust contract design and verification, serving as a preliminary step toward this goal.
For example, consider [Challenge 3: Verifying Raw Pointer Arithmetic Operations](https://model-checking.github.io/verify-rust-std/challenges/0003-pointer-arithmentic.html) from [Verify Rust Std Lib](https://model-checking.github.io/verify-rust-std/intro.html).  In this context, we can annotate all safety properties of unsafe APIs with primitive safety properties. For instance, the unsafe API [*const T::add()](https://doc.rust-lang.org/beta/core/primitive.pointer.html#method.add) is described as follows:

```rust
pub const unsafe fn add(self, count: usize) -> Self
where
    T: Sized,

///Safety
///If any of the following conditions are violated, the result is Undefined Behavior:
///-The offset in bytes, count * size_of::<T>(), computed on mathematical integers (without ``wrapping around''), must fit in an isize.
///-If the computed offset is non-zero, then self must be derived from a pointer to some allocated object, and the entire memory range between self and the result must be in bounds of that allocated object. In particular, this range must not “wrap around” the edge of the address space.
///Allocated objects can never be larger than isize::MAX bytes, so if the computed offset stays in bounds of the allocated object, it is guaranteed to satisfy the first requirement. This implies, for instance, that vec.as_ptr().add(vec.len()) (for vec: Vec<T>) is always safe.
///Consider using wrapping_add instead if these constraints are difficult to satisfy. The only advantage of this method is that it enables more aggressive compiler optimizations.
```

We can tag the API with the following primitive safety property:
- Primitive SP template: `ValidNum(expr, vrange)`, which means $expr \in vrange$;
    - Specific primitive SP for the API: `ValidNum((sizeof(T) * count), [isize::MIN, isize::MAX] )`.

As a result, the safety property is provided as following.
```rust
#[safety::precond::ValidNum((sizeof(T) * count), [isize::MIN, isize::MAX])]
pub const unsafe fn add(self, count: usize) -> Self
where
    T: Sized,
```

For another instance, the unsafe API [ptr::copy()](https://doc.rust-lang.org/beta/core/ptr/fn.copy.html) is described as follows:
```rust
pub const unsafe fn copy<T>(src: *const T, dst: *mut T, count: usize)

///Safety
///Behavior is undefined if any of the following conditions are violated:
///-src must be valid for reads of count * size_of::<T>() bytes, and must remain valid even when dst is written for count * size_of::<T>() bytes. (This means if the memory ranges overlap, the two pointers must not be subject to aliasing restrictions relative to each other.)
///-dst must be valid for writes of count * size_of::<T>() bytes, and must remain valid even when src is read for count * size_of::<T>() bytes.
///-Both src and dst must be properly aligned.
///Like read, copy creates a bitwise copy of T, regardless of whether T is Copy. If T is not Copy, using both the values in the region beginning at *src and the region beginning at *dst can violate memory safety.
///Note that even if the effectively copied size (count * size_of::<T>()) is 0, the pointers must be properly aligned.
```

We can tag the API with the following primitive safety property:
- Primitive SP template: `Allocated(p, T, len, A)`, which means $\forall$ i $\in$ 0..sizeof(T)*len, allocator(p+i) = A;
    - Specific primitive SP for the API: `Allocated(src, T, len, any)`
- Primitive SP template: `InBound(p, T, len)`, which means [p, p+ sizeof(T) * len) $\in$ arrange;
   - Specific primitive SP for the API: `InBound(src, T, len)`
- Primitive SP template: `Init(p, T, len)`, which means $\forall$ i $\in$ 0..len, mem(p + sizeof(T) * i, p + sizeof(T) * (i+1)) = valid(T);
    - Specific primitive SP for the API: `Init(src, T, count)`
- Primitive SP template: `!Overlap(dst, src, T, len)`, which means \|dst - src\| $\ge$ sizeof(T) * len;
    - Specific primitive SP for the API: `!Overlap(dst, src, T, 1)`
- Primitive SP template: `Align(p, T)`, which means  p \% alignment(T) = 0;
    - Specific primitive SP for the API: `Align(src, T)` and `Align(dst, T)`

These are the preconditions for calling the unsafe APIs. We need more properties to discribe the hazards when the content is not `Copy`.

- Primitive SP template: `Alias(p1, p2)`, which means $p1 = p2$;
    - Specific primitive SP for the API: `Alias(dst, src)`


As a result, the safety property is provided as following.
```rust
#[safety::precond::InBound(src, T, len)]
#[safety::precond::Init(src, T, count)]
#[safety::precond::NonOverlap(dst, src, T, 1)]
#[safety::precond::Align(src, T)]
#[safety::precond::Align(dst, T)]
#[safety::hazard::Alias(dst, src)]
pub const unsafe fn copy<T>(src: *const T, dst: *mut T, count: usize)
```

When proving the soundness of [String::remove()](https://doc.rust-lang.org/beta/alloc/string/struct.String.html#method.remove) (see the code below), it is essential to verify that the primitive safety properties of its interior unsafe APIs [ptr.add()](https://doc.rust-lang.org/beta/core/primitive.pointer.html#method.add) and [ptr::copy()](https://doc.rust-lang.org/beta/core/ptr/fn.copy.html) are met in all cases.

```rust
pub fn remove(&mut self, idx: usize) -> char {
    let ch = match self[idx..].chars().next() {
    Some(ch) => ch,
        None => panic!("cannot remove a char from the end of a string"),
    };
    let next = idx + ch.len_utf8();
    let len = self.len();
    unsafe {
        ptr::copy(self.vec.as_ptr().add(next), self.vec.as_mut_ptr().add(idx), len - next);
        self.vec.set_len(len - (next - idx));
    }
    ch
}
```

**For more details, please refer to our paper:**
- "[Annotating and Auditing the Safety Properties of Unsafe Rust](https://arxiv.org/abs/2504.21312)", Zihao Rao, Hongliang Tian, Xin Wang, **Hui Xu**, _arXiv:2504.21312_, 2025. (corresponding author)
