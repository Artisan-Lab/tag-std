|Num| API | Missing SP | Merged PR |
|---|-----|--------------|--------|
|1| Arc::from_raw | AllocatorConsistency | [1: PR_134496](https://github.com/rust-lang/rust/pull/134496) |
|2| Arc::increment_strong_count | AllocatorConsistency | [1: PR_134496](https://github.com/rust-lang/rust/pull/134496) |
|3| Arc::decrement_strong_count | AllocatorConsistency | [1: PR_134496](https://github.com/rust-lang/rust/pull/134496) |
|4| ptr::read_unaligned | !NonNull | [2: PR_134953](https://github.com/rust-lang/rust/pull/134953) |
|5| ptr::write_unaligned | !NonNull | [2: PR_134953](https://github.com/rust-lang/rust/pull/134953) |
|6| Box::from_raw | AllocatorConsistency | [3: PR_135009](https://github.com/rust-lang/rust/pull/135009) |
|7| Box::from_raw_in | AllocatorConsistency | [3: PR_135009](https://github.com/rust-lang/rust/pull/135009) |
|8| Box::from_non_null | AllocatorConsistency | [4: PR_135805](https://github.com/rust-lang/rust/pull/135805) |
|9| Box::from_non_null_in | AllocatorConsistency | [4: PR_135805](https://github.com/rust-lang/rust/pull/135805) |
|10| Weak::from_raw | AllocatorConsistency | [4: PR_135805](https://github.com/rust-lang/rust/pull/135805) |