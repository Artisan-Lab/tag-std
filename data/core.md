## Core APIs (no-std)

### Module [num](https://doc.rust-lang.org/nightly/core/num/index.html)
| Namespace | API | Precondition | Hazard | Option | Status |
|-----------|-----|--------------|--------|--------------|--------|
|core::intrinsics|[unchecked_add<T: Copy>(_x: T, _y: T) -> T](https://doc.rust-lang.org/nightly/core/intrinsics/fn.unchecked_add.html)| $\text{ValidInt}(add, \\_x, \\_y, T)$ |-|-|done|
|core::intrinsics|[unchecked_sub<T: Copy>(_x: T, _y: T) -> T](https://doc.rust-lang.org/nightly/core/intrinsics/fn.unchecked_sub.html)| $\text{ValidInt}(sub, \\_x, \\_y, T)$ |-|-|done|
|core::intrinsics|[unchecked_mul<T: Copy>(_x: T, _y: T) -> T](https://doc.rust-lang.org/nightly/core/intrinsics/fn.unchecked_mul.html)| $\text{ValidInt}(mul, \\_x, \\_y, T)$ |-|-|done|
|core::intrinsics|[unchecked_div<T: Copy>(_x: T, _y: T) -> T](https://doc.rust-lang.org/nightly/core/intrinsics/fn.unchecked_div.html)| $\text{ValidInt}(\\_y, T, !0)$, $!(\text{ValidInt}(\\_x, T, T::MIN) \land \text{ValidInt}(\\_y, T, -1))$|-|-|done|
|core::intrinsics|[unchecked_rem<T: Copy>(_x: T, _y: T) -> T](https://doc.rust-lang.org/nightly/core/intrinsics/fn.unchecked_rem.html)|$\text{ValidInt}(\\_y, T, !0)$, $!(\text{ValidInt}(\\_x, T, T::MIN) \land \text{ValidInt}(\\_y, T, -1))$|-|-|done|
|core::intrinsics|[unchecked_shl<T: Copy, U: Copy>(_x: T, _y: U) -> T](https://doc.rust-lang.org/nightly/core/intrinsics/fn.unchecked_shl.html)| $\text{ValidInt}(\\_y, T, U, \ge 0) \lor ValidInt(\\_y, T, U, <sizeof(T)*8)$|-|-|done|
|core::intrinsics|[unchecked_shr<T: Copy, U: Copy>(_x: T, _y: U) -> T](https://doc.rust-lang.org/nightly/core/intrinsics/fn.unchecked_shr.html)| $\text{ValidInt}(\\_y, T, U, \ge 0) \lor ValidInt(\\_y, T, U, <sizeof(T)*8)$|-|-|done|


### Core Intrinsics with Raw pointers
| Namespace | API | Precondition | Hazard | Option | Status |
|-----------|-----|-----|--------------|--------|--------------|
|core::intrinsics|[typed_swap_nonoverlapping<T>(x: *mut T, y: *mut T)](https://doc.rust-lang.org/nightly/core/intrinsics/fn.typed_swap_nonoverlapping.html)| $\text{PtrValid}(x,T)$, $\text{PtrValid}(y,T)$, $\text{NonOverlap}(x, y, T)$, $\text{Aligned}(x,T)$, $\text{Aligned}(y,T)$, $\text{Init}(x,T)$, $\text{Init}(y,T)$,|-|-|done|
|core::intrinsics|[vtable_size(ptr: *const ()) -> usize](https://doc.rust-lang.org/nightly/core/intrinsics/fn.vtable_size.html)|$\text{Bounded}(\\ptr, vtable)$, $\text{Init}(\\ptr,vtable)$ ||| ? |
|core::intrinsics|[vtable_align(ptr: *const ()) -> usize](https://doc.rust-lang.org/nightly/core/intrinsics/fn.vtable_align.html)|$\text{Bounded}(\\ptr, vtable)$, $\text{Init}(\\ptr,vtable) $ ||| ?|
|core::intrinsics|[copy<T>(src: *const T, dst: *mut T, count: usize)](https://doc.rust-lang.org/nightly/core/intrinsics/fn.copy.html)| $\text{ValidPtr}(src,T,count)$, $\text{ValidPtr}(dst,T,count)$, $\text{NonOverlap}(src, dst, T)$, $\text{Aligned}(src,T)$, $\text{Aligned}(dst,T)$, $\text{NonVolatile}(src)$, $\text{NonVolatile}(dst)$ | $\text{Alias}(*src, *dst)$  | $Copy\in Trait(T)$ | ? NonVolatile |
|core::intrinsics|[copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize)](https://doc.rust-lang.org/nightly/core/intrinsics/fn.copy_nonoverlapping.html)|  $\text{ValidPtr}(src,T,count)$, $\text{ValidPtr}(dst,T,count)$,  $\text{NonOverlap}(src, dst, T, count)$,  $\text{Aligned}(src,T)$,  $\text{Aligned}(dst,T)$,  $\text{NonVolatile}(src)$,  $\text{NonVolatile}(dst)$ |  $\text{Alias}(*src, *dst)$ | $Copy\in Trait(T)$ | To ?NonVolatile |
|core::intrinsics|[write_bytes<T>(dst: *mut T, val: u8, count: usize)](https://doc.rust-lang.org/nightly/core/intrinsics/fn.write_bytes.html)| $\text{ValidPtr}(dst,T,count)$,  $\text{Aligned}(dst,T)$,  $\text{NonVolatile}(dst)$ | - | - | ?NonVolatile |
|core::intrinsics|[size_of_val<T: ?Sized>(ptr: *const T) -> usize](https://doc.rust-lang.org/nightly/core/intrinsics/fn.size_of_val.html)|TO LABEL||||
|core::intrinsics|[arith_offset<T>(dst: *const T, _offset: isize) -> *const T](https://doc.rust-lang.org/nightly/core/intrinsics/fn.arith_offset.html)|TO LABEL(need a hazard label?)||| ? |
|core::intrinsics|[volatile_copy_nonoverlapping_memory<T>(dst: *mut T, src: *const T, _count: usize)](https://doc.rust-lang.org/nightly/core/intrinsics/fn.volatile_copy_nonoverlapping_memory.html)| $\text{PtrValid}(src,T)$, $\text{PtrValid}(dst,T)$, $\text{NonOverlap}(src, dst, T, count)$, $\text{Aligned}(src,T)$, $\text{Aligned}(dst,T)$,| $\text{Alias}(*src, *dst)$  | $Copy\in Trait(T)$ |  |
|core::intrinsics|[volatile_copy_memory<T>(dst: *mut T, src: *const T, _count: usize)](https://doc.rust-lang.org/nightly/core/intrinsics/fn.volatile_copy_memory.html)| $\text{ValidPtr}(\\src,T)$, $\text{PtrValid}(\\dst,T)$, $\text{NonOverlap}(\\src, \\dst, T)$, $\text{Aligned}(\\src,T)$, $\text{Aligned}(\\dst,T)$ | $\text{Alias}(*\\src, *\\dst)$ | $Copy\in Trait(T)$ | ? |
|core::intrinsics|[volatile_set_memory<T>(dst: *mut T, _val: u8, _count: usize)](https://doc.rust-lang.org/nightly/core/intrinsics/fn.volatile_set_memory.html)|$\text{ValidPtr}(\\dst,T,\\_count)$|||? |
|core::intrinsics|[volatile_load<T>(src: *const T) -> T](https://doc.rust-lang.org/nightly/core/intrinsics/fn.volatile_load.html)| $\text{ValidPtr}(\\src,T)$, $\text{Aligned}(\\src,T)$, $\text{Init}(\\src, T)$ ||$Copy\in Trait(T)$ ||
|core::intrinsics|[unaligned_volatile_load<T>(src: *const T) -> T](https://doc.rust-lang.org/nightly/core/intrinsics/fn.unaligned_volatile_load.html)| $\text{ValidPtr}(\\src,T)$, $\text{Init}(\\src, T)$ || $Copy\in Trait(T)$ ||
|core::intrinsics|[volatile_store<T>(dst: *mut T, _val: T)](https://doc.rust-lang.org/nightly/core/intrinsics/fn.volatile_store.html)| $\text{ValidPtr}(\\dst,T)$, $\text{Aligned}(\\dst,T)$ ||$Copy\in Trait(T)$ ||
|core::intrinsics|[unaligned_volatile_store<T>(dst: *mut T, _val: T)](https://doc.rust-lang.org/nightly/core/intrinsics/fn.unaligned_volatile_store.html)|$\text{ValidPtr}(\\dst,T)$ ||$Copy\in Trait(T)$ ||
|core::intrinsics|[compare_bytes(_left: *const u8, _right: *const u8, _bytes: usize) -> i32](https://doc.rust-lang.org/nightly/core/intrinsics/fn.compare_bytes.html)| $\text{ValidPtr}(\\_left,T)$, $\text{ValidPtr}(\\_right,T)$ ||||
|core::intrinsics|[min_align_of_val<T: ?Sized>(ptr: *const T) -> usize](https://doc.rust-lang.org/nightly/core/intrinsics/fn.min_align_of_val.html)|TO LABEL||||
|core::intrinsics|[ptr_offset_from<T>(ptr: *const T, _base: *const T) -> isize](https://doc.rust-lang.org/nightly/core/intrinsics/fn.ptr_offset_from.html)| $\text{NonZst}(T)$ ||| ? |
|core::intrinsics|[ptr_offset_from_unsigned<T>(ptr: *const T, _base: *const T) -> usize](https://doc.rust-lang.org/nightly/core/intrinsics/fn.ptr_offset_from_unsigned.html)| $\text{NonZst}(T)$ ||| ? |
|core::intrinsics|[read_via_copy<T>(ptr: *const T) -> T](https://doc.rust-lang.org/nightly/core/intrinsics/fn.read_via_copy.html)| $\text{ValidPtr}(\\ptr,T)$, $\text{Aligned}(\\ptr,T)$, $\text{Init}(\\ptr, T)$, $\text{NonVolatile}(\\ptr)$||$Copy\in Trait(T)$| ? |
|core::intrinsics|[write_via_move<T>(ptr: *mut T, _value: T)](https://doc.rust-lang.org/nightly/core/intrinsics/fn.write_via_move.html)|$\text{ValidPtr}(\\ptr,T)$, $\text{Aligned}(\\ptr,T)$, $\text{NonVolatile}(\\ptr)$|||? |

### Module [ptr](https://doc.rust-lang.org/nightly/core/ptr/index.html)

| Namespace | API | Precondition | Hazard | Option | Status |
|-----------|-----|-----|--------------|--------|--------------|
| core::ptr | [copy<T>(src: *const T, dst: *mut T, count: usize)](https://doc.rust-lang.org/nightly/core/ptr/fn.copy.html) |Aligned(src, T), Aligned(dst, T), Bounded(src, T, count), Bounded(dst, T, count), NonOverlap(dst, src, T) | Alias(*src, *dst) | $Copy\in Trait(T)$ | |
| core::ptr | [copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize)](https://doc.rust-lang.org/nightly/core/ptr/fn.copy_nonoverlapping.html)  | Aligned(src, T), Aligned(dst, T), Bounded(src, T, count), Bounded(dst, T, count), NonOverlap(dst, src, T, count)  | Alias(*src, *dst)  | $Copy\in Trait(T)$ |   |
| core::ptr | [drop_in_place<T: ?Sized>(to_drop: *mut T)](https://doc.rust-lang.org/nightly/core/ptr/fn.drop_in_place.html) |             |        |             |        |
| core::ptr | [read<T>(src: *const T) -> T](https://doc.rust-lang.org/nightly/core/ptr/fn.read.html) |        |        |             |        |
| core::ptr | [read_unaligned<T>(src: *const T) -> T](https://doc.rust-lang.org/nightly/core/ptr/fn.read_unaligned.html) |          |        |             |        |
| core::ptr | [read_volatile<T>(src: *const T) -> T](https://doc.rust-lang.org/nightly/core/ptr/fn.read_volatile.html) |              |        |             |        |
| core::ptr | [replace<T>(dst: *mut T, src: T) -> T](https://doc.rust-lang.org/nightly/core/ptr/fn.replace.html) |             |        |             |        |
| core::ptr | [swap<T>(x: *mut T, y: *mut T)](https://doc.rust-lang.org/nightly/core/ptr/fn.swap.html) |              |        |             |        |
| core::ptr | [swap_nonoverlapping<T>(x: *mut T, y: *mut T, count: usize)](https://doc.rust-lang.org/nightly/core/ptr/fn.swap_nonoverlapping.html) |  |              |        |             |        |
| core::ptr | [write<T>(dst: *mut T, src: T)](https://doc.rust-lang.org/nightly/core/ptr/fn.write.html) |            |        |             |        |
| core::ptr | [write_bytes<T>(dst: *mut T, val: u8, count: usize)](https://doc.rust-lang.org/nightly/core/ptr/fn.write_bytes.html) |            |        |             |        |
| core::ptr | [write_unaligned<T>(dst: *mut T, src: T)](https://doc.rust-lang.org/nightly/core/ptr/fn.write_unaligned.html) |             |        |             |        |
| core::ptr | [write_volatile<T>(dst: *mut T, src: T)](https://doc.rust-lang.org/nightly/core/ptr/fn.write_volatile.html) |              |        |             |        |
