# OS-Related Safety Risk Tags

> 基于 [linux-unsafe-doc](https://safer-rust.github.io/linux-unsafe-doc)、[asterinas-unsafe-doc](https://safer-rust.github.io/asterinas-unsafe-doc)、[std-unsafe-doc](https://safer-rust.github.io/std-unsafe-doc) 的实际 API 数据自动生成

> 标签定义参考 [primitive-sp.md](https://github.com/safer-rust/safety-tags/blob/main/primitive-sp.md) [sp-core.toml](https://github.com/safer-rust/safety-tags/blob/main/safety-tool/assets/sp-core.toml) [sp-rust-for-linux.toml](https://github.com/safer-rust/safety-tags/blob/main/safety-tool/assets/sp-rust-for-linux.toml) [asterinas/sp.md](https://github.com/safer-rust/safety-tags/blob/main/target-projects/asterinas/sp.md)

**覆盖率**: linux-unsafe-doc: 93/140 (66.4%) | asterinas-unsafe-doc: 133/171 (77.8%) | std-unsafe-doc: 266/6069 (4.4%)

## 标签类型说明

| 类型 | 含义 |
|------|------|
| **Precond** | 前置条件 |
| **Hazard** | 危险 |
| **Option** | 可选条件 |

## Memory & Pointer

| Tag | 含义 | 类型 | API 示例 |
|-----|------|------|----------|
| `Align` | 对齐要求 | Precond | linux: [`new`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/alloc/allocator/iter.rs#L77) (method)<br>asterinas: [`protect_gpa_tdvm_call`](https://github.com/asterinas/asterinas/blob/main/ostd/src/arch/x86/tdx_guest.rs#L48) (function)<br>std: [`clone_to_uninit`](https://doc.rust-lang.org/nightly/alloc/bstr/ByteString/clone_to_uninit/) (method) |
| `Allocated` | 已分配内存 | Precond | linux: [`Allocator`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/alloc.rs#L159) (trait)<br>asterinas: [`boot_all_aps`](https://github.com/asterinas/asterinas/blob/main/ostd/src/boot/smp.rs#L72) (function)<br>std: [`deallocate`](https://doc.rust-lang.org/nightly/alloc/alloc/Global/deallocate/) (method) |
| `Deref` | 可解引用 | Precond | — |
| `InBound` | 边界内访问 | Precond | linux: [`from_raw_parts`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/alloc/kvec.rs#L550) (method)<br>asterinas: [`protect_gpa_tdvm_call`](https://github.com/asterinas/asterinas/blob/main/ostd/src/arch/x86/tdx_guest.rs#L48) (function)<br>std: [`from_parts`](https://doc.rust-lang.org/nightly/alloc/vec/Vec/from_parts/) (method) |
| `NonNull` | 指针非空 | Precond | linux: [`Allocator::realloc`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/alloc.rs#L230) (trait_method)<br>asterinas: [`NonNullPtr`](https://github.com/asterinas/asterinas/blob/main/ostd/src/sync/rcu/non_null/mod.rs#L25) (trait)<br>std: [`from_non_null`](https://doc.rust-lang.org/nightly/alloc/boxed/Box/from_non_null/) (method) |
| `NonOverlap` | 内存不重叠 | Precond | linux: [`ForeignOwnable::borrow_mut`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/types.rs#L126) (trait_method) |
| `Valid` | 值有效 | Precond | linux: [`Allocator`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/alloc.rs#L159) (trait)<br>asterinas: [`copy_from_mmio`](https://github.com/asterinas/asterinas/blob/main/ostd/src/arch/loongarch/io/io_mem.rs#L57) (function)<br>std: [`as_mut_vec`](https://doc.rust-lang.org/nightly/alloc/string/String/as_mut_vec/) (method) |
| `ValidMemory` | 有效内存/IO区 | Precond | — |
| `ValidPtr` | 可解引用指针 | Precond | linux: [`to_page`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/alloc/allocator.rs#L191) (method)<br>std: [`split_at_mut`](https://doc.rust-lang.org/nightly/core/pointer/split_at_mut/) (method) |
| `ValidRead` | 内存可读 | Precond | linux: [`from_raw`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/cpufreq.rs#L234) (method)<br>asterinas: [`copy_to_mmio`](https://github.com/asterinas/asterinas/blob/main/ostd/src/arch/loongarch/io/io_mem.rs#L75) (function)<br>std: [`_mm_clflushopt`](https://doc.rust-lang.org/nightly/core/core_arch/x86/clflushopt/_mm_clflushopt/) (function) |
| `ValidWrite` | 内存可写 | Precond | linux: [`from_raw_mut`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/cpufreq.rs#L140) (method)<br>asterinas: [`copy_from_mmio`](https://github.com/asterinas/asterinas/blob/main/ostd/src/arch/loongarch/io/io_mem.rs#L57) (function)<br>std: [`simd_masked_store`](https://doc.rust-lang.org/nightly/core/intrinsics/simd/simd_masked_store/) (function) |

## Init & Content

| Tag | 含义 | 类型 | API 示例 |
|-----|------|------|----------|
| `Init` | 内存已初始化 | Precond/Hazard | linux: [`assume_init`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/alloc/kbox.rs#L230) (method)<br>asterinas: [`init_on_cpu`](https://github.com/asterinas/asterinas/blob/main/ostd/src/arch/x86/trap/syscall.rs#L44) (function)<br>std: [`assume_init`](https://doc.rust-lang.org/nightly/alloc/boxed/Box/assume_init/) (method) |
| `NoPadding` | 无padding | Precond | — |
| `Typed` | 类型正确 | Precond | linux: [`drvdata_borrow`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/device.rs#L268) (method)<br>asterinas: [`map`](https://github.com/asterinas/asterinas/blob/main/ostd/src/arch/x86/iommu/dma_remapping/mod.rs#L84) (function)<br>std: [`downcast_unchecked`](https://doc.rust-lang.org/nightly/alloc/boxed/Box/downcast_unchecked/) (method) |
| `Unwrap` | 解包结果正确 | Precond | std: [`unchecked_add`](https://doc.rust-lang.org/nightly/core/i128/unchecked_add/) (method) |
| `ValidCStr` | 有效C字符串 | Precond | linux: [`call_printk`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/print.rs#L105) (function)<br>std: [`from_bytes_with_nul_unchecked`](https://doc.rust-lang.org/nightly/core/ffi/c_str/CStr/from_bytes_with_nul_unchecked/) (method) |
| `ValidNum` | 数值范围有效 | Precond | linux: [`from_raw_parts`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/alloc/kvec.rs#L550) (method)<br>asterinas: [`PageTableConfig::item_from_raw`](https://github.com/asterinas/asterinas/blob/main/ostd/src/mm/page_table/mod.rs#L146) (trait_method)<br>std: [`grow`](https://doc.rust-lang.org/nightly/alloc/alloc/Global/grow/) (method) |
| `ValidString` | 有效UTF-8 | Precond/Hazard | std: [`from_boxed_utf8_unchecked`](https://doc.rust-lang.org/nightly/alloc/str/from_boxed_utf8_unchecked/) (function) |

## Alias & Ownership

| Tag | 含义 | 类型 | API 示例 |
|-----|------|------|----------|
| `Alias` | 别名 | Hazard | asterinas: [`NonNullPtr::from_raw`](https://github.com/asterinas/asterinas/blob/main/ostd/src/sync/rcu/non_null/mod.rs#L60) (trait_method)<br>std: [`replace`](https://doc.rust-lang.org/nightly/core/cell/UnsafeCell/replace/) (method) |
| `Alive` | 生命周期有效 | Precond | linux: [`Allocator`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/alloc.rs#L159) (trait)<br>asterinas: [`borrow_paddr`](https://github.com/asterinas/asterinas/blob/main/ostd/src/mm/frame/frame_ref.rs#L29) (method)<br>std: [`copy`](https://doc.rust-lang.org/nightly/core/ptr/copy/) (function) |
| `NonMutRef` | 无可变引用 | Precond | asterinas: [`from_pte`](https://github.com/asterinas/asterinas/blob/main/ostd/src/mm/page_table/node/pte_state.rs#L64) (method) |
| `Owning` | 独占所有权 | Precond | linux: [`ListItem::prepare_to_insert`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/list.rs#L343) (trait_method)<br>asterinas: [`init`](https://github.com/asterinas/asterinas/blob/main/ostd/src/arch/x86/iommu/fault.rs#L244) (function)<br>std: [`from_raw`](https://doc.rust-lang.org/nightly/alloc/ffi/c_str/CString/from_raw/) (method) |
| `Pinned` | 不可移动 | Hazard | linux: [`RegistrationOps::register`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/driver.rs#L149) (trait_method)<br>std: [`new_unchecked`](https://doc.rust-lang.org/nightly/core/pin/Pin/new_unchecked/) (method) |

## Sync & Concurrency

| Tag | 含义 | 类型 | API 示例 |
|-----|------|------|----------|
| `LockHold` | 锁已持有 | Precond | linux: [`Backend::assert_is_held`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/sync/lock.rs#L97) (trait_method)<br>asterinas: [`make_guard_unchecked`](https://github.com/asterinas/asterinas/blob/main/ostd/src/mm/page_table/node/mod.rs#L160) (method) |
| `NonConcurrent` | 无并发访问 | Precond | linux: [`to_page`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/alloc/allocator.rs#L191) (method)<br>asterinas: [`get`](https://github.com/asterinas/asterinas/blob/main/ostd/src/task/utils.rs#L30) (method) |
| `NonData_race` | 无数据竞争 | Precond | linux: [`assume_no_fdget_pos`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/fs/file.rs#L306) (method)<br>std: [`replace`](https://doc.rust-lang.org/nightly/core/cell/UnsafeCell/replace/) (method) |
| `NonMutate` | 不可变 | Precond | linux: [`ForeignOwnable::borrow_mut`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/types.rs#L126) (trait_method)<br>std: [`from_ptr`](https://doc.rust-lang.org/nightly/core/ffi/c_str/CStr/from_ptr/) (method) |
| `NonVolatile` | 非volatile | Precond | asterinas: [`read_once`](https://github.com/asterinas/asterinas/blob/main/ostd/src/arch/loongarch/io/io_mem.rs#L28) (function)<br>std: [`volatile_copy_nonoverlapping_memory`](https://doc.rust-lang.org/nightly/core/intrinsics/volatile_copy_nonoverlapping_memory/) (function) |
| `XorAccess` | 互斥访问 | Precond | linux: [`ListItem::prepare_to_insert`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/list.rs#L343) (trait_method)<br>asterinas: [`dyn_meta_ptr`](https://github.com/asterinas/asterinas/blob/main/ostd/src/mm/frame/meta.rs#L333) (method) |

## OS Validity

| Tag | 含义 | 类型 | API 示例 |
|-----|------|------|----------|
| `Assoc` | 值关联 | Precond | linux: [`drvdata_borrow`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/device.rs#L268) (method)<br>asterinas: [`from_user_space`](https://github.com/asterinas/asterinas/blob/main/ostd/src/mm/io/mod.rs#L779) (method)<br>std: [`_mm_clflushopt`](https://doc.rust-lang.org/nightly/core/core_arch/x86/clflushopt/_mm_clflushopt/) (function) |
| `CanFail` | 可能失败 | Precond | linux: [`map_pages`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/iommu/pgtable.rs#L166) (method)<br>asterinas: [`copy_from_mmio_fallible`](https://github.com/asterinas/asterinas/blob/main/ostd/src/arch/loongarch/io/io_mem.rs#L93) (function)<br>std: [`GlobalAllocator`](https://doc.rust-lang.org/nightly/core/alloc/GlobalAllocator/) (trait) |
| `ContainerOf` | 容器指针 | Precond | linux: [`HasGroup`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/configfs.rs#L205) (trait)<br>std: [`Field`](https://doc.rust-lang.org/nightly/core/field/Field/) (trait) |
| `FlagSet` | 标志已设置 | Precond | linux: [`from_raw`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/mm/virt.rs#L185) (method)<br>asterinas: [`sync_dma_range`](https://github.com/asterinas/asterinas/blob/main/ostd/src/arch/loongarch/mm/mod.rs#L116) (function) |
| `Forgotten` | 已遗忘所有权 | Precond | linux: [`from_raw_parts`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/alloc/kvec.rs#L550) (method)<br>asterinas: [`from_raw`](https://github.com/asterinas/asterinas/blob/main/ostd/src/mm/frame/mod.rs#L206) (method)<br>std: [`from_raw`](https://doc.rust-lang.org/nightly/alloc/ffi/c_str/CString/from_raw/) (method) |
| `Invariant` | 类型不变式 | Precond | linux: [`to_page`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/alloc/allocator.rs#L191) (method)<br>std: [`drop`](https://doc.rust-lang.org/nightly/core/mem/manually_drop/ManuallyDrop/drop/) (method) |
| `MayInvalid` | 可能失效 | Hazard | asterinas: [`map`](https://github.com/asterinas/asterinas/blob/main/ostd/src/arch/loongarch/iommu/mod.rs#L18) (function)<br>std: [`from_non_null`](https://doc.rust-lang.org/nightly/alloc/boxed/Box/from_non_null/) (method) |
| `NonAccessable` | 调用后不可访问 | Precond | linux: [`Allocator::free`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/alloc.rs#L247) (trait_method)<br>asterinas: [`NonNullPtr::from_raw`](https://github.com/asterinas/asterinas/blob/main/ostd/src/sync/rcu/non_null/mod.rs#L60) (trait_method)<br>std: [`from_raw`](https://doc.rust-lang.org/nightly/alloc/ffi/c_str/CString/from_raw/) (method) |
| `NonDropped` | 不可drop | Precond | linux: [`from_raw`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/bitmap.rs#L35) (method) |
| `NonInstance` | 实例不可存在 | Precond | asterinas: [`init`](https://github.com/asterinas/asterinas/blob/main/ostd/src/io/mod.rs#L39) (function) |
| `NonZero` | 非零 | Precond | linux: [`from_raw`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/device.rs#L369) (method)<br>asterinas: [`Id`](https://github.com/asterinas/asterinas/blob/main/ostd/src/util/id_set.rs#L55) (trait)<br>std: [`from_raw_parts`](https://doc.rust-lang.org/nightly/alloc/vec/Vec/from_raw_parts/) (method) |
| `RefTransfer` | 引用计数转移 | Precond | linux: [`from_cpu`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/cpu.rs#L142) (function)<br>asterinas: [`inc_frame_ref_count`](https://github.com/asterinas/asterinas/blob/main/ostd/src/mm/frame/mod.rs#L358) (function) |

## Control Flow

| Tag | 含义 | 类型 | API 示例 |
|-----|------|------|----------|
| `CallOnce` | 仅调用一次 | Precond | linux: [`ListItem::view_value`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/list.rs#L325) (trait_method)<br>asterinas: [`init_on_ap`](https://github.com/asterinas/asterinas/blob/main/ostd/src/arch/loongarch/mod.rs#L61) (function) |
| `CalledBy` | 调用环境约束 | Precond | asterinas: [`do_inter_processor_call`](https://github.com/asterinas/asterinas/blob/main/ostd/src/smp.rs#L92) (function) |
| `CurThread` | 当前线程/CPU | Precond | linux: [`assume_no_fdget_pos`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/fs/file.rs#L306) (method)<br>asterinas: [`init_on_cpu`](https://github.com/asterinas/asterinas/blob/main/ostd/src/arch/loongarch/trap/mod.rs#L31) (function) |
| `NotPostToFunc` | 禁止后续调用 | Precond | — |
| `NotPriorToFunc` | 禁止前置调用 | Precond | — |
| `OriginateFrom` | 来源约束 | Precond | linux: [`ListItem::post_remove`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/list.rs#L355) (trait_method)<br>asterinas: [`unprepare_dma`](https://github.com/asterinas/asterinas/blob/main/ostd/src/mm/dma/util.rs#L175) (function) |
| `PostToFunc` | 调用顺序依赖 | Precond | linux: [`drvdata_borrow`](https://github.com/Rust-for-Linux/linux/blob/rust-next/rust/kernel/device.rs#L268) (method)<br>asterinas: [`init_on_ap`](https://github.com/asterinas/asterinas/blob/main/ostd/src/arch/loongarch/mod.rs#L61) (function)<br>std: [`_mm256_stream_pd`](https://doc.rust-lang.org/nightly/core/core_arch/x86/avx/_mm256_stream_pd/) (function) |

## Memory

| Tag | 含义 | 类型 | API 示例 |
|-----|------|------|----------|
| `UserSpace` | 用户空间内存 | Precond | asterinas: [`copy_from_mmio_fallible`](https://github.com/asterinas/asterinas/blob/main/ostd/src/arch/loongarch/io/io_mem.rs#L93) (function) |

