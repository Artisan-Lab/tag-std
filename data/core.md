## Core APIs (no-std)

### [ptr](https://doc.rust-lang.org/nightly/core/ptr/index.html)

| Namespace | API | Tags of Primitive SP | Precondition | Hazard | Option | Status |
|-----------|-----|-----|--------------|--------|--------------|--------|
| core::ptr | [pub const unsafe fn copy<T>(src: *const T, dst: *mut T, count: usize)](https://doc.rust-lang.org/nightly/core/ptr/fn.copy.html) | Aligned, Bounded, NonOverlap, +Alias, Trait- | Aligned(src, T), Aligned(dst, T), Bounded(src, T, count), Bounded(dst, T, count), NonOverlap(dst, src, T) | Alias(*src, *dst) | $Copy\int Trait(T)$ | |
| core::ptr | [pub const unsafe fn copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize)](https://doc.rust-lang.org/nightly/core/ptr/fn.copy_nonoverlapping.html)  | Aligned, Bounded, NonOverlap, +Alias, Trait- | Aligned(src, T), Aligned(dst, T), Bounded(src, T, count), Bounded(dst, T, count), NonOverlap(dst, src, T, count)  | Alias(*src, *dst)  | $Copy\int Trait(T)$ |   |
| core::ptr |     |     |              |        |
