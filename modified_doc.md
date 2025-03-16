|Num| API | Missing SP | PR num |PR |PR Status |
|---|-----|--------------|--------|--------|--------|
|1| Arc::from_raw | AllocatorConsistency | 1 | [PR_134496](https://github.com/rust-lang/rust/pull/134496) | Merged |
|2| Arc::increment_strong_count | AllocatorConsistency | 1 | [ PR_134496](https://github.com/rust-lang/rust/pull/134496) | Merged |
|3| Arc::decrement_strong_count | AllocatorConsistency | 1 | [PR_134496](https://github.com/rust-lang/rust/pull/134496) | Merged |
|4| ptr::read_unaligned | !NonNull | 2 | [PR_134953](https://github.com/rust-lang/rust/pull/134953) | Merged |
|5| ptr::write_unaligned | !NonNull | 2 | [PR_134953](https://github.com/rust-lang/rust/pull/134953) | Merged |
|6| Box::from_raw | AllocatorConsistency | 3 | [PR_135009](https://github.com/rust-lang/rust/pull/135009) | Merged |
|7| Box::from_raw_in | AllocatorConsistency | 3 | [PR_135009](https://github.com/rust-lang/rust/pull/135009) | Merged |
|8| Box::from_non_null | AllocatorConsistency | 4 | [PR_135805](https://github.com/rust-lang/rust/pull/135805) | Merged |
|9| Box::from_non_null_in | AllocatorConsistency | 4 | [PR_135805](https://github.com/rust-lang/rust/pull/135805) | Merged |
|10| Weak::from_raw | AllocatorConsistency | 4 | [PR_135805](https://github.com/rust-lang/rust/pull/135805) | Merged |
|11| intrinsic::volatile_copy_nonoverlapping_memory | Volatile / ValidPtr / Aligned / NonOverlap / Alias / CopyTrait | 5 | [PR_138309](https://github.com/rust-lang/rust/pull/138309) | Merged |
|12| intrinsic::volatile_set_memory | Volatile / Typed / ValidPtr / Aligned | 5 | [PR_138309](https://github.com/rust-lang/rust/pull/138309) | Merged |
|13| intrinsic::typed_swap_nonoverlapping | Typed / ValidPtr / Aligned | 5 |[PR_138309](https://github.com/rust-lang/rust/pull/138309) | Merged |
|14| VaListImpl::arg | Bounded / Typed / Init | 6 | [PR_136969](https://github.com/rust-lang/rust/pull/136969) | Pending |
|15| alloc::ffi::CStr::from_raw | Alias / Owning / Dangling | 7 | [PR_137714](https://github.com/rust-lang/rust/pull/137714) | Confirmed |
|16| alloc::str::from_boxed_utf8_unchecked | ValidStr | 7 | [PR_137714](https://github.com/rust-lang/rust/pull/137714) | Confirmed |
|17| Arc::increment_strong_count | Typed | 8 | [ PR_138303](https://github.com/rust-lang/rust/pull/138303) | Merged |
|18| Arc::decrement_strong_count | Typed | 8 | [ PR_138303](https://github.com/rust-lang/rust/pull/138303) | Merged |
|19| Arc::increment_strong_count_in | Typed | 8 | [ PR_138303](https://github.com/rust-lang/rust/pull/138303) | Merged |
|20| Arc::decrement_strong_count_in | Typed | 8 | [ PR_138303](https://github.com/rust-lang/rust/pull/138303) | Merged |
|21| Rc::increment_strong_count | Typed | 8 | [ PR_138303](https://github.com/rust-lang/rust/pull/138303) | Merged |
|22| Rc::decrement_strong_count | Typed | 8 | [ PR_138303](https://github.com/rust-lang/rust/pull/138303) | Merged |
|23| Rc::increment_strong_count_in | Typed | 8 | [ PR_138303](https://github.com/rust-lang/rust/pull/138303) | Merged |
|24| Rc::decrement_strong_count_in | Typed | 8 | [ PR_138303](https://github.com/rust-lang/rust/pull/138303) | Merged |
