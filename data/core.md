## Core APIs (no-std)

### Module [num](https://doc.rust-lang.org/nightly/core/num/index.html)
| Namespace | API | Precondition | Hazard | Option | Status |
|-----------|-----|-----|--------------|--------|--------------|
|[core::intrinsics]|[unchecked_add<T: Copy>(_x: T, _y: T) -> T](https://doc.rust-lang.org/nightly/core/intrinsics/fn.unchecked_add.html)||||||
|[core::intrinsics]|[unchecked_sub<T: Copy>(_x: T, _y: T) -> T](https://doc.rust-lang.org/nightly/core/intrinsics/fn.unchecked_sub.html)||||||
|[core::intrinsics]|[unchecked_mul<T: Copy>(_x: T, _y: T) -> T](https://doc.rust-lang.org/nightly/core/intrinsics/fn.unchecked_mul.html)||||||
|[core::intrinsics]|[unchecked_rem<T: Copy>(_x: T, _y: T) -> T](https://doc.rust-lang.org/nightly/core/intrinsics/fn.unchecked_rem.html)||||||
|[core::intrinsics]|[unchecked_shl<T: Copy>(_x: T, _y: T) -> T](https://doc.rust-lang.org/nightly/core/intrinsics/fn.unchecked_shl.html)||||||
|[core::intrinsics]|[unchecked_shr<T: Copy>(_x: T, _y: T) -> T](https://doc.rust-lang.org/nightly/core/intrinsics/fn.unchecked_shr.html)||||||
|[core::intrinsics]|[unchecked_neg<T: Copy>(_x: T, _y: T) -> T]()||||||

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

### Core Intrinsics with Raw pointers
| Namespace | API | Precondition | Hazard | Option | Status |
|-----------|-----|-----|--------------|--------|--------------|
|[core::intrinsics]|[typed_swap_nonoverlapping<T>(x: *mut T, y: *mut T)](https://doc.rust-lang.org/nightly/core/intrinsics/fn.typed_swap_nonoverlapping.html)|||||
|[core::intrinsics]|[vtable_size(_ptr: *const ()) -> usize](https://doc.rust-lang.org/nightly/core/intrinsics/fn.vtable_size.html)|||||
|[core::intrinsics]|[vtable_align(_ptr: *const ()) -> usize](https://doc.rust-lang.org/nightly/core/intrinsics/fn.vtable_align.html)|||||
|[core::intrinsics]|[copy<T>(src: *const T, dst: *mut T, count: usize)](https://doc.rust-lang.org/nightly/core/intrinsics/fn.copy.html)|||||
|[core::intrinsics]|[copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize)](https://doc.rust-lang.org/nightly/core/intrinsics/fn.copy_nonoverlapping.html)|||||
|[core::intrinsics]|[write_bytes<T>(dst: *mut T, val: u8, count: usize)](https://doc.rust-lang.org/nightly/core/intrinsics/fn.write_bytes.html)|||||
|[core::intrinsics]|[size_of_val<T: ?Sized>(_ptr: *const T) -> usize](https://doc.rust-lang.org/nightly/core/intrinsics/fn.size_of_val.html)|||||
|[core::intrinsics]|[arith_offset<T>(_dst: *const T, _offset: isize) -> *const T](https://doc.rust-lang.org/nightly/core/intrinsics/fn.arith_offset.html)|||||
|[core::intrinsics]|[volatile_copy_nonoverlapping_memory<T>(_dst: *mut T, _src: *const T, _count: usize)](https://doc.rust-lang.org/nightly/core/intrinsics/fn.typed_swap_nonoverlapping.html)|||||
|[core::intrinsics]|[volatile_copy_memory<T>(_dst: *mut T, _src: *const T, _count: usize)](https://doc.rust-lang.org/nightly/core/intrinsics/fn.volatile_copy_memory.html)|||||
|[core::intrinsics]|[volatile_set_memory<T>(_dst: *mut T, _val: u8, _count: usize)](https://doc.rust-lang.org/nightly/core/intrinsics/fn.volatile_set_memory.html)|||||
|[core::intrinsics]|[volatile_load<T>(_src: *const T) -> T](https://doc.rust-lang.org/nightly/core/intrinsics/fn.volatile_load.html)|||||
|[core::intrinsics]|[unaligned_volatile_load<T>(_src: *const T) -> T](https://doc.rust-lang.org/nightly/core/intrinsics/fn.unaligned_volatile_load.html)|||||
|[core::intrinsics]|[volatile_store<T>(_dst: *mut T, _val: T)](https://doc.rust-lang.org/nightly/core/intrinsics/fn.volatile_store.html)|||||
|[core::intrinsics]|[unaligned_volatile_load<T>(_src: *const T) -> T](https://doc.rust-lang.org/nightly/core/intrinsics/fn.unaligned_volatile_load.html)|||||
|[core::intrinsics]|[compare_bytes(_left: *const u8, _right: *const u8, _bytes: usize) -> i32](https://doc.rust-lang.org/nightly/core/intrinsics/fn.compare_bytes.html)|||||
|[core::intrinsics]|[min_align_of_val<T: ?Sized>(_ptr: *const T) -> usize](https://doc.rust-lang.org/nightly/core/intrinsics/fn.min_align_of_val.html)|||||
|[core::intrinsics]|[ptr_offset_from<T>(_ptr: *const T, _base: *const T) -> isize](https://doc.rust-lang.org/nightly/core/intrinsics/fn.ptr_offset_from.html)|||||
|[core::intrinsics]|[ptr_offset_from_unsigned<T>(_ptr: *const T, _base: *const T) -> usize](https://doc.rust-lang.org/nightly/core/intrinsics/fn.unaligned_volatile_load.html)|||||
|[core::intrinsics]|[read_via_copy<T>(_ptr: *const T) -> T](https://doc.rust-lang.org/nightly/core/intrinsics/fn.read_via_copy.html)|||||
|[core::intrinsics]|[write_via_move<T>(_ptr: *mut T, _value: T)](https://doc.rust-lang.org/nightly/core/intrinsics/fn.write_via_move.html)|||||
