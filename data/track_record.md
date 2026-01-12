## Asterinas


| Num | PR                                                         | API                                                        | Info                     | PR Status       |
| --- | ---------------------------------------------------------- | ---------------------------------------------------------- | ------------------------ | --------------- |
| 1   | [2677](https://github.com/asterinas/asterinas/pull/2677)   | loongarch_boot,riscv_boot,ap_early_entry,kernel_task_entry | safe => unsafe           | merged          |
| 2   | [2673](https://github.com/asterinas/asterinas/pull/2673)   | IoApicAccess::{read, write}                                | no safety doc            | submitted       |
| 3   | [2707](https://github.com/asterinas/asterinas/issues/2707) | (x86, riscv) bringup_all_aps                               | vague safety requirement | issue submitted |

## Rust-for-linux


| Num | Commit Hash                                                | Email Link                                                                                                                              | API                          | Commit Title             | PR Status      |
| --- | ---------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------- | ------------------------ | --------------- |
| 1   | d37a39f607c4                                               | [Re: [PATCH v2] rust: dma: remove incorrect safety documentation](https://lore.kernel.org/all/DFARBS5X3XAV.304WNUYV2ES3Q@kernel.org/#r) | as_slice_mut                 |        rust: dma: add as_slice/write functions for CoherentAllocation                  | merged          |
| 2   |  9b90864bb42b   |      [Re: [PATCH v2] rust: device_id: replace incorrect word in safety documentation](https://lore.kernel.org/all/DFARBV6C1ITF.32UCXF6AYE2A8@kernel.org/)                                                                                                                                   | RawDeviceId  |    rust: implement `IdArray`, `IdTable` and `RawDeviceId`         | merged       |
| 3   |  |   [Re: [PATCH v3 RESEND] rust: cpumask: rename methods of Cpumask for clarity and consistency](https://lore.kernel.org/all/aWS9yf1iwWW-O0y6@google.com/)                                                                                                                                      | as_mut_ref -> from_raw_mut, as_ref -> from_raw |  | merged |

## Standard Library


| Num | API                                            | Missing SP                                                     | PR Num | PR                                                          | PR Status |
| --- | ---------------------------------------------- | -------------------------------------------------------------- | ------ | ----------------------------------------------------------- | --------- |
| 1   | Arc::from_raw                                  | Allocated                                                      | 1      | [PR_134496](https://github.com/rust-lang/rust/pull/134496)  | Merged    |
| 2   | Arc::increment_strong_count                    | Allocated                                                      | 1      | [ PR_134496](https://github.com/rust-lang/rust/pull/134496) | Merged    |
| 3   | Arc::decrement_strong_count                    | Allocated                                                      | 1      | [PR_134496](https://github.com/rust-lang/rust/pull/134496)  | Merged    |
| 4   | ptr::read_unaligned                            | !NonNull                                                       | 2      | [PR_134953](https://github.com/rust-lang/rust/pull/134953)  | Merged    |
| 5   | ptr::write_unaligned                           | !NonNull                                                       | 2      | [PR_134953](https://github.com/rust-lang/rust/pull/134953)  | Merged    |
| 6   | Box::from_raw                                  | Allocated                                                      | 3      | [PR_135009](https://github.com/rust-lang/rust/pull/135009)  | Merged    |
| 7   | Box::from_raw_in                               | Allocated                                                      | 3      | [PR_135009](https://github.com/rust-lang/rust/pull/135009)  | Merged    |
| 8   | Box::from_non_null                             | Allocated                                                      | 4      | [PR_135805](https://github.com/rust-lang/rust/pull/135805)  | Merged    |
| 9   | Box::from_non_null_in                          | Allocated                                                      | 4      | [PR_135805](https://github.com/rust-lang/rust/pull/135805)  | Merged    |
| 10  | Weak::from_raw                                 | Allocated                                                      | 4      | [PR_135805](https://github.com/rust-lang/rust/pull/135805)  | Merged    |
| 11  | intrinsic::volatile_copy_nonoverlapping_memory | Volatile / ValidPtr / Aligned / NonOverlap / Alias / CopyTrait | 5      | [PR_138309](https://github.com/rust-lang/rust/pull/138309)  | Merged    |
| 12  | intrinsic::volatile_set_memory                 | Volatile / Typed / ValidPtr / Aligned                          | 5      | [PR_138309](https://github.com/rust-lang/rust/pull/138309)  | Merged    |
| 13  | intrinsic::typed_swap_nonoverlapping           | Typed / ValidPtr / Aligned                                     | 5      | [PR_138309](https://github.com/rust-lang/rust/pull/138309)  | Merged    |
| 14  | alloc::ffi::CStr::from_raw                     | Alias / Owning / Allocated                                     | 6      | [PR_137714](https://github.com/rust-lang/rust/pull/137714)  | Merged    |
| 15  | alloc::str::from_boxed_utf8_unchecked          | ValidStr                                                       | 6      | [PR_137714](https://github.com/rust-lang/rust/pull/137714)  | Merged    |
| 16  | Arc::increment_strong_count                    | Typed                                                          | 7      | [ PR_138303](https://github.com/rust-lang/rust/pull/138303) | Merged    |
| 17  | Arc::decrement_strong_count                    | Typed                                                          | 7      | [ PR_138303](https://github.com/rust-lang/rust/pull/138303) | Merged    |
| 18  | Arc::increment_strong_count_in                 | Typed                                                          | 7      | [ PR_138303](https://github.com/rust-lang/rust/pull/138303) | Merged    |
| 19  | Arc::decrement_strong_count_in                 | Typed                                                          | 7      | [ PR_138303](https://github.com/rust-lang/rust/pull/138303) | Merged    |
| 20  | Rc::increment_strong_count                     | Typed                                                          | 7      | [ PR_138303](https://github.com/rust-lang/rust/pull/138303) | Merged    |
| 21  | Rc::decrement_strong_count                     | Typed                                                          | 7      | [ PR_138303](https://github.com/rust-lang/rust/pull/138303) | Merged    |
| 22  | Rc::increment_strong_count_in                  | Typed                                                          | 7      | [ PR_138303](https://github.com/rust-lang/rust/pull/138303) | Merged    |
| 23  | Rc::decrement_strong_count_in                  | Typed                                                          | 7      | [ PR_138303](https://github.com/rust-lang/rust/pull/138303) | Merged    |
| 24  | Box::from_raw                                  | Alias                                                          | 8      | [PR_146870](https://github.com/rust-lang/rust/pull/146870)  | Pending   |
| 25  | Box::from_raw_in                               | Alias                                                          | 8      | [PR_146870](https://github.com/rust-lang/rust/pull/146870)  | Pending   |
| 26  | Box::from_non_null                             | Alias                                                          | 8      | [PR_146870](https://github.com/rust-lang/rust/pull/146870)  | Pending   |
| 27  | Box::from_non_null_in                          | Alias                                                          | 8      | [PR_146870](https://github.com/rust-lang/rust/pull/146870)  | Pending   |
| 28  | VaListImpl::arg                                | InBound / Typed / Init                                         | 9      | [PR_146925](https://github.com/rust-lang/rust/pull/146925)  | Pending   |
| 29  | intrinsic::va_copy                             | !Null / Allocated / Alias                                      | 9      | [PR_146925](https://github.com/rust-lang/rust/pull/146925)  | Pending   |
| 30  | intrinsic::va_arg                              | InBound / Typed / Init                                         | 9      | [PR_146925](https://github.com/rust-lang/rust/pull/146925)  | Pending   |
| 31  | intrinsic::va_end                              | Allocated                                                      | 9      | [PR_146925](https://github.com/rust-lang/rust/pull/146925)  | Pending   |
