|Num| API | Missing SP | PR num |PR |PR Status |
|---|-----|--------------|--------|--------|--------|
|1| Arc::from_raw | AllocatorConsistency | 1 | [PR_134496](https://github.com/rust-lang/rust/pull/134496) | <span style="color:white;background:green;font-weight:bold">Merged</span> |
|2| Arc::increment_strong_count | AllocatorConsistency | 1 | [ PR_134496](https://github.com/rust-lang/rust/pull/134496) | Merged |
|3| Arc::decrement_strong_count | AllocatorConsistency | 1 | [PR_134496](https://github.com/rust-lang/rust/pull/134496) | Merged |
|4| ptr::read_unaligned | !NonNull | 2 | [PR_134953](https://github.com/rust-lang/rust/pull/134953) | Merged |
|5| ptr::write_unaligned | !NonNull | 2 | [PR_134953](https://github.com/rust-lang/rust/pull/134953) | Merged |
|6| Box::from_raw | AllocatorConsistency | 3 | [PR_135009](https://github.com/rust-lang/rust/pull/135009) | Merged |
|7| Box::from_raw_in | AllocatorConsistency | 3 | [PR_135009](https://github.com/rust-lang/rust/pull/135009) | Merged |
|8| Box::from_non_null | AllocatorConsistency | 4 | [PR_135805](https://github.com/rust-lang/rust/pull/135805) | Merged |
|9| Box::from_non_null_in | AllocatorConsistency | 4 | [PR_135805](https://github.com/rust-lang/rust/pull/135805) | Merged |
|10| Weak::from_raw | AllocatorConsistency | 4 | [PR_135805](https://github.com/rust-lang/rust/pull/135805) | Merged |
|11| intrinsic::volatile_copy_nonoverlapping_memory | NonVolatile / ValidPtr / Aligned / NonOverlap / Alias / CopyTrait | 5 | [PR_135334](https://github.com/rust-lang/rust/pull/135334) | Pending |
|12| intrinsic::volatile_set_memory | ValidPtr / Aligned | 5 | [PR_135334](https://github.com/rust-lang/rust/pull/135334) | Pending |
|13| intrinsic::typed_swap_nonoverlapping | ValidPtr / Aligned | 5 |[PR_135334](https://github.com/rust-lang/rust/pull/135334) | Pending |
|14| VaListImpl::arg | Bounded / Typed / Init | 6 | [PR_136969](https://github.com/rust-lang/rust/pull/136969) | Pending |
|15| alloc::ffi::CStr::from_raw | Alias / Owning / Dangling | 7 | [PR_137714](https://github.com/rust-lang/rust/pull/137714) | Confirmed |
|16| alloc::str::from_boxed_utf8_unchecked | ValidStr | 7 | [PR_137714](https://github.com/rust-lang/rust/pull/137714) | Confirmed |


