## Core APIs (no-std)

### [ptr](https://doc.rust-lang.org/nightly/core/ptr/index.html)

| Namespace | API | Tags of Primitive SP | Precondition | Hazard | Option | Status |
|-----------|-----|-----|--------------|--------|--------------|--------|
| core::ptr | [copy<T>(src: *const T, dst: *mut T, count: usize)](https://doc.rust-lang.org/nightly/core/ptr/fn.copy.html) | Aligned, Bounded, NonOverlap, +Alias, Trait- | Aligned(src, T), Aligned(dst, T), Bounded(src, T, count), Bounded(dst, T, count), NonOverlap(dst, src, T) | Alias(*src, *dst) | $Copy\in Trait(T)$ | |
| core::ptr | [copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize)](https://doc.rust-lang.org/nightly/core/ptr/fn.copy_nonoverlapping.html)  | Aligned, Bounded, NonOverlap, +Alias, Trait- | Aligned(src, T), Aligned(dst, T), Bounded(src, T, count), Bounded(dst, T, count), NonOverlap(dst, src, T, count)  | Alias(*src, *dst)  | $Copy\in Trait(T)$ |   |
| core::ptr | [drop_in_place<T: ?Sized>(to_drop: *mut T)](https://doc.rust-lang.org/nightly/core/ptr/fn.drop_in_place.html) | Aligned, NonDangling,   |              |        |             |        |
| core::ptr | [read<T>(src: *const T) -> T](https://doc.rust-lang.org/nightly/core/ptr/fn.read.html) |  |              |        |             |        |
| core::ptr | [read_unaligned<T>(src: *const T) -> T](https://doc.rust-lang.org/nightly/core/ptr/fn.read_unaligned.html) |  |              |        |             |        |
| core::ptr | [read_volatile<T>(src: *const T) -> T](https://doc.rust-lang.org/nightly/core/ptr/fn.read_volatile.html) |  |              |        |             |        |
| core::ptr | [replace<T>(dst: *mut T, src: T) -> T](https://doc.rust-lang.org/nightly/core/ptr/fn.replace.html) |  |              |        |             |        |
| core::ptr | [swap<T>(x: *mut T, y: *mut T)](https://doc.rust-lang.org/nightly/core/ptr/fn.swap.html) |  |              |        |             |        |
| core::ptr | [swap_nonoverlapping<T>(x: *mut T, y: *mut T, count: usize)](https://doc.rust-lang.org/nightly/core/ptr/fn.swap_nonoverlapping.html) |  |              |        |             |        |
| core::ptr | [write<T>(dst: *mut T, src: T)](https://doc.rust-lang.org/nightly/core/ptr/fn.write.html) |  |              |        |             |        |
| core::ptr | [write_bytes<T>(dst: *mut T, val: u8, count: usize)](https://doc.rust-lang.org/nightly/core/ptr/fn.write_bytes.html) |  |              |        |             |        |
| core::ptr | [write_unaligned<T>(dst: *mut T, src: T)](https://doc.rust-lang.org/nightly/core/ptr/fn.write_unaligned.html) |  |              |        |             |        |
| core::ptr | [write_volatile<T>(dst: *mut T, src: T)](https://doc.rust-lang.org/nightly/core/ptr/fn.write_volatile.html) |  |              |        |             |        |
