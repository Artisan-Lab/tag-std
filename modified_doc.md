|Num| API | Missing SP | PR Status |
|---|-----|--------------|--------|
|1| Arc::from_raw | AllocatorConsistency | [Merged 1: PR_134496](https://github.com/rust-lang/rust/pull/134496) |
|2| Arc::increment_strong_count | AllocatorConsistency | [Merged 1: PR_134496](https://github.com/rust-lang/rust/pull/134496) |
|3| Arc::decrement_strong_count | AllocatorConsistency | [Merged 1: PR_134496](https://github.com/rust-lang/rust/pull/134496) |
|4| ptr::read_unaligned | !NonNull | [Merged 2: PR_134953](https://github.com/rust-lang/rust/pull/134953) |
|5| ptr::write_unaligned | !NonNull | [Merged 2: PR_134953](https://github.com/rust-lang/rust/pull/134953) |
|6| Box::from_raw | AllocatorConsistency | [Merged 3: PR_135009](https://github.com/rust-lang/rust/pull/135009) |
|7| Box::from_raw_in | AllocatorConsistency | [Merged 3: PR_135009](https://github.com/rust-lang/rust/pull/135009) |
|8| Box::from_non_null | AllocatorConsistency | [Merged 4: PR_135805](https://github.com/rust-lang/rust/pull/135805) |
|9| Box::from_non_null_in | AllocatorConsistency | [Merged 4: PR_135805](https://github.com/rust-lang/rust/pull/135805) |
|10| Weak::from_raw | AllocatorConsistency | [Merged 4: PR_135805](https://github.com/rust-lang/rust/pull/135805) |
|11| intrinsic::volatile_copy_nonoverlapping_memory | NonVolatile / ValidPtr / Aligned / NonOverlap / Alias / CopyTrait | [Pending 5: PR_135334](https://github.com/rust-lang/rust/pull/135334) |
|12| intrinsic::volatile_set_memory | ValidPtr / Aligned | [Pending 5: PR_135334](https://github.com/rust-lang/rust/pull/135334) |
|13| intrinsic::typed_swap_nonoverlapping | ValidPtr / Aligned | [Pending 5: PR_135334](https://github.com/rust-lang/rust/pull/135334) |
|14| VaListImpl::arg | Bounded / Typed / Init | [Pending 6: PR_136969](https://github.com/rust-lang/rust/pull/136969) |
|15| alloc::ffi::CStr::from_raw | Alias / Owning / Dangling | [Pending 7: PR_137714](https://github.com/rust-lang/rust/pull/137714) |
|16| alloc::str::from_boxed_utf8_unchecked | ValidStr / AllocatorConsistency | [Pending 7: PR_137714](https://github.com/rust-lang/rust/pull/137714) |


