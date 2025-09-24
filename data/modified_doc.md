|Num| API | Missing SP | PR Num |PR |PR Status |
|---|-----|--------------|--------|--------|--------|
|1| Arc::from_raw | Allocated | 1 | [PR_134496](https://github.com/rust-lang/rust/pull/134496) | Merged |
|2| Arc::increment_strong_count | Allocated | 1 | [ PR_134496](https://github.com/rust-lang/rust/pull/134496) | Merged |
|3| Arc::decrement_strong_count | Allocated | 1 | [PR_134496](https://github.com/rust-lang/rust/pull/134496) | Merged |
|4| ptr::read_unaligned | !NonNull | 2 | [PR_134953](https://github.com/rust-lang/rust/pull/134953) | Merged |
|5| ptr::write_unaligned | !NonNull | 2 | [PR_134953](https://github.com/rust-lang/rust/pull/134953) | Merged |
|6| Box::from_raw | Allocated | 3 | [PR_135009](https://github.com/rust-lang/rust/pull/135009) | Merged |
|7| Box::from_raw_in | Allocated | 3 | [PR_135009](https://github.com/rust-lang/rust/pull/135009) | Merged |
|8| Box::from_non_null | Allocated | 4 | [PR_135805](https://github.com/rust-lang/rust/pull/135805) | Merged |
|9| Box::from_non_null_in | Allocated | 4 | [PR_135805](https://github.com/rust-lang/rust/pull/135805) | Merged |
|10| Weak::from_raw | Allocated | 4 | [PR_135805](https://github.com/rust-lang/rust/pull/135805) | Merged |
|11| intrinsic::volatile_copy_nonoverlapping_memory | Volatile / ValidPtr / Aligned / NonOverlap / Alias / CopyTrait | 5 | [PR_138309](https://github.com/rust-lang/rust/pull/138309) | Merged |
|12| intrinsic::volatile_set_memory | Volatile / Typed / ValidPtr / Aligned | 5 | [PR_138309](https://github.com/rust-lang/rust/pull/138309) | Merged |
|13| intrinsic::typed_swap_nonoverlapping | Typed / ValidPtr / Aligned | 5 |[PR_138309](https://github.com/rust-lang/rust/pull/138309) | Merged |
|14| alloc::ffi::CStr::from_raw | Alias / Owning / Allocated | 6 | [PR_137714](https://github.com/rust-lang/rust/pull/137714) | Merged |
|15| alloc::str::from_boxed_utf8_unchecked | ValidStr | 6 | [PR_137714](https://github.com/rust-lang/rust/pull/137714) | Merged |
|16| Arc::increment_strong_count | Typed | 7 | [ PR_138303](https://github.com/rust-lang/rust/pull/138303) | Merged |
|17| Arc::decrement_strong_count | Typed | 7 | [ PR_138303](https://github.com/rust-lang/rust/pull/138303) | Merged |
|18| Arc::increment_strong_count_in | Typed | 7 | [ PR_138303](https://github.com/rust-lang/rust/pull/138303) | Merged |
|19| Arc::decrement_strong_count_in | Typed | 7 | [ PR_138303](https://github.com/rust-lang/rust/pull/138303) | Merged |
|20| Rc::increment_strong_count | Typed | 7 | [ PR_138303](https://github.com/rust-lang/rust/pull/138303) | Merged |
|21| Rc::decrement_strong_count | Typed | 7 | [ PR_138303](https://github.com/rust-lang/rust/pull/138303) | Merged |
|22| Rc::increment_strong_count_in | Typed | 7 | [ PR_138303](https://github.com/rust-lang/rust/pull/138303) | Merged |
|23| Rc::decrement_strong_count_in | Typed | 7 | [ PR_138303](https://github.com/rust-lang/rust/pull/138303) | Merged |
|24| Box::from_raw | Alias | 8 | [PR_146870](https://github.com/rust-lang/rust/pull/146870) | Pending |
|25| Box::from_raw_in | Alias | 8 | [PR_146870](https://github.com/rust-lang/rust/pull/146870) | Pending |
|26| Box::from_non_null | Alias | 8 | [PR_146870](https://github.com/rust-lang/rust/pull/146870) | Pending |
|27| Box::from_non_null_in | Alias | 8 | [PR_146870](https://github.com/rust-lang/rust/pull/146870) | Pending |
|28| VaListImpl::arg | InBound / Typed / Init | 9 | [PR_146925](https://github.com/rust-lang/rust/pull/146925) | Pending |
|29| intrinsic::va_copy | !Null / Allocated / Alias | 9 | [PR_146925](https://github.com/rust-lang/rust/pull/146925) | Pending |
|30| intrinsic::va_arg | InBound / Typed / Init | 9 | [PR_146925](https://github.com/rust-lang/rust/pull/146925) | Pending |
|31| intrinsic::va_end | Allocated | 9 | [PR_146925](https://github.com/rust-lang/rust/pull/146925) | Pending |
