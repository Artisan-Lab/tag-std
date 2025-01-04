# tag-std

This project aims to standardize the safety property annotation of the Rust core and standard library. There are three steps:
- Define the [primitive safety properties](primitive-sp.md) to be used for describing the safety concerns of unsafe APIs.
- Label the unsafe APIs in Rust's core and standard library with primitive safety properties.
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
- Primitive SP template: ValidInt(binop, x, y, T), which means $\text{T::MAX} \geq \text{binop}(x, y) \geq \text{T::MIN} $; Specific primitive SP for the API: ValidInt(mul, count, sizeof(T), isize).

This is a precondition for calling the unsafe API. When proving the soundness of [String::remove()](https://doc.rust-lang.org/beta/alloc/string/struct.String.html#method.remove) (see the code below), it is essential to verify that the primitive safety properties of its interior unsafe APIs [ptr.add()](https://doc.rust-lang.org/beta/core/primitive.pointer.html#method.add) and [ptr::copy()](https://doc.rust-lang.org/beta/core/ptr/fn.copy.html) are met in all cases.

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
