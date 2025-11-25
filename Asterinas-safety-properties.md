## Safety Properties for Asterinas

### 1 Execution Flow

#### 1.1 PostToFunc(func)

This sp describes the order between function calls. The function taged by this sp can only be called after `func` has been called. For example, the call to the function is safe only if the related variables or memory sections are correctly initialized by `func`.

**Formal Description:**

$$
\textsf{P} Call(func)
$$

**Usage:** precondition

Example APIs: [Asterinas-arch::trap::init()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/arch/x86/trap/syscall.rs#L45)


#### 1.2 NotPostToFunc(func)

This sp requires the tagged function should not be called post to the given `func`. The situation usually happens when several functions have the same access to specific unsafe operations and the operations are mutual exclusive.

**Formal Description:**

$$
\neg \textsf{P} Call(func)
$$

**Usage:** precondition

Example: [Asterinas-mm::page_table::cursor::locking::dfs_release_lock](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/mm/page_table/cursor/locking.rs#L208)

#### 1.3 NotPriorToFunc(func)

This sp requires the tagged function should not be called prior to the given `func`. The situation usually happens when the function lead to a hazard state and unexpected results can be caused by `func`.

**Formal Description:**

$$
\neg \Box(Call(func))
$$

**Usage:** hazard

Example: [Asterinas-mm:page_table::boot_pt::dismiss()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/mm/page_table/boot_pt.rs#L73)

#### 1.4 CallOnce(scope)

This sp describes the requirement of a funtion to be called once and only once in the specific scope. For example, during the boot phase of the OS, we usually need to initialize some module or important global variables. In this case, `scope` can be `system` or other words that represents for the whole lifetime of the system.

**Formal Description:**

$$
\textsf{F} Call(self) \wedge \Box(Call(self)\to\Box\neg Call(self))
$$

**Usage:** hazard

Example APIs: [Adterinas-crate::init()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/lib.rs#L82)

#### 1.5 OriginateFrom(val, func)

This sp describes the requirement that `val` should originate from `func` (as its return value or processed by the function/macro). In addition to the order of execution, there are also dependencies of data here.

**Formal Description:**

**Usage:** precondition

Example APIs: [Asterinas-mm::ChildRef::from_pte()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/mm/page_table/node/child.rs#L93)

### 2 Value

#### 2.1 Eq(lhs, rhs), Ne(lhs, rhs), Ge(lhs, rhs)

The sps describe the numerical magnitude relationship between `lhs` and `rhs`.

**Formal Description:** 

$$
lhs = rhs \\
lhs != rhs \\
lhs >= rhs
$$

**Usage:** precondition

Example APIs: [Asterinas-mm::MetaSlot::drop_last_in_place()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/mm/frame/meta.rs#L390)

### 3 Valid

#### 3.1 ValidAccessAddr(addr, access)

This sp describes the requirement that `access` to `addr` should be valid. Here `access` could be reading or writing at the address.

**Formal Description:**

$$
\begin{cases}
ValidRead(addr), & \text{access is reading} \\
ValidWrite(addr), & \text{access is writing}
\end{cases}
$$

**Usage:** precondition

Example APIs: [Asterinas-fill_boot_info_ptr()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/arch/x86/boot/smp.rs#L176)

#### 3.2 ValidBaseAddr(addr, hardware)

This sp describes the requirement that `addr` should be the valid base address of `hardware` such as an IOMMU, IO APIC, or HPET MMIO. Otherwise, any access to the hardware device can be unsafe.

**Formal Description:**

$$
addr = BaseAddr(hardware)
$$

**Usage:** precondtion

Example APIs: [Asterinas-arch::iommu::fault::init()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/arch/x86/iommu/fault.rs#L239)

#### 3.3 ValidInstanceAddr(val, type)

This sp describes the requirement that `addr` should point to a valid instance of `type`. The function usually directly manipulates the data at the address. Thus it is necessary to ensure the safety.

**Formal Description:**

$$
\text{typeof}(*addr) = type
$$

**Usage:** precondition

Example APIs: [Asterinas-mm::inc_frame_ref_count()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/mm/frame/mod.rs#L325)


### 4 Reference and Ownership

In this section, the reference and ownership do not represent Rust's concepts, but represent similar abstract functions implemented in the OS. e.g., manage resources by conceptual reference and ownership.

#### 4.1 RefHeld(val), RefUnheld(val)

These sps describe the requirement that the abstract reference to `val` should have been held or not. The reference is usually implemented with reference count or just logically corresponding relation.

**Formal Description:**

**Usage:** precondition

Example APIs: [Asterinas-mm::inc_frame_ref_count()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/mm/frame/mod.rs#L325), [Asterinas-mm::page_table::node:\:child::Child::from_pte()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/mm/page_table/node/child.rs#L93)

#### 4.2 OwnedResource(val)

This sp describes the requirement that `val` as a resource should be conceptually owned exclusively. Such resource can be port or other entities in the OS.

**Formal Description:**

**Usage:** precondition

Example APIs: [Asterinas-arch::iommu::fault::init()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/arch/x86/iommu/fault.rs#L239)


### 5 Memory

#### 5.1 UserSpace(start, end)

This sp describes the requirement that the given memory range (`start`, `end`) should locate within the user space.

**Formal Description:**

$$
(start, end) \subset UserSpace
$$

**Usage:** precondition

Example APIs: [Asterinas-mm::io::VmReader::from_user_space](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/mm/io.rs#L572)

#### 5.2 KernelMemorySafe(operation)

This sp describes the requirement that the operation should not affect kernel's memory safety. The sp is usually used for page table APIs where mapping or unmapping `operation` can cause memory problems. 

**Formal Description:**

**Usage:** hazard

Example APIs: [Asterinas-mm::page_table::boot_pt::BootPageTable::map_base_page()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/mm/page_table/boot_pt.rs#L164)

#### 5.3 Section(val, section)

This sp describes the reqirement that `val` should reside in the elf `section` within the memory layout.

**Formal Description:** 

**Usage:** hazard

Example APIs: [Asterinas-cpu::local::cell::CpuLocalCell::__new()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/cpu/local/cell.rs#L93)


### MISC

#### Forgotten

This sp describe the requirement that the reference to `val` should be forgotten. Usually the operation is done by calling `into_raw`, `core::mem::forget` or `ManuallyDrop`.

**Formal Description:**

**Usage:** precondition

Example APIs: [Asterinas-mm::frame::segment::Segment::from_raw()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/mm/frame/segment.rs#L118https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/mm/frame/segment.rs#L118)

#### MutAccess(val)

This sp describes the requirement that the access to `val` should be mutual exclusive. e.g., exclusive writing.

**Formal Description:**

**Usage:** hazard

Example APIs: [Asterinas-mm::frame::meta::MetaSlot::write_meta()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/mm/frame/meta.rs#L366)

#### NonModifying(val)

This sp describes the requirement that `val` should not be modified after calling the function. Usually the limitation happens to OS registers and flags.

> ! Maybe an constraint period should be specified additionally.

**Formal Description:**

**Usage:** hazard

Example APIs: [Asterinas-mm::page_table::cursor::CursorMut::protect_next()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/mm/page_table/cursor/mod.rs#L559)

#### Unaccessed(val)

This sp describes the requirement that `val` should not be accessed whether read or written. This sp is used to protect cpu local data constancy (any type of access to the data may break the constancy and cause UB).

**Formal Description:**

**Usage:** precondition

Example APIs: [Asterinas-cpu::local::copy_bsp_for_ap()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/cpu/local/mod.rs#L204)

#### Bounded(val, bound)

This sp describes the requirement that `val` should be bounded by `bound` which can not be represented by a literal number e.g., system settings or architecture limits.

**Formal Description:**

**Usage:** precondition

Example APIs: [Asterinas-io::io_port::allocator::init()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/io/io_port/allocator.rs#L69)

#### LockHeld(val)

This sp describes the requirement that `val` should already have held the lock **logically**.

**Formal Description:**

**Usage:** precondition

Example APIs: [Asterinas-mm::page_table::node::FrameRef::make_guard_unchecked()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/mm/page_table/node/mod.rs)
