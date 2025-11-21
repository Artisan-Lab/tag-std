### Execution Flow

#### PostToFunc(func)

This sp describes the order between function calls. The function taged by this sp can only be called after `func` has been called. For example, the call to the function is safe only if the related variables or memory sections are correctly initialized by `func`.

**Formal Description:**

$$
\textsf{P} Call(func)
$$

**Usage:** precondition

Example APIs: [Asterinas-arch::trap::init()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/arch/x86/trap/syscall.rs#L45)


#### NotPostToFunc(func)

This sp requires the tagged function should not be called post to the given `func`. The situation usually happens when several functions have the same access to specific unsafe operations and the operations are mutual exclusive.

**Formal Description:**

$$
\neg \textsf{P} Call(func)
$$

**Usage:** precondition

Example: [Asterinas-mm::page_table::cursor::locking::dfs_release_lock](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/mm/page_table/cursor/locking.rs#L208)

#### NotPriorToFunc(func)

This sp requires the tagged function should not be called prior to the given `func`. The situation usually happens when the function lead to a hazard state and unexpected results can be caused by `func`.

**Formal Description:**

$$
\neg \Box(Call(func))
$$

**Usage:** hazard

Example: [Asterinas-mm:page_table::boot_pt::dismiss()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/mm/page_table/boot_pt.rs#L73)

#### CallOnce(scope)

This sp describes the requirement of a funtion to be called once and only once in the specific scope. For example, during the boot phase of the OS, we usually need to initialize some module or important global variables. In this case, `scope` can be `system` or other words that represents for the whole lifetime of the system.

**Formal Description:**

$$
\textsf{F} Call(self) \wedge \Box(Call(self)\to\Box\neg Call(self))
$$

**Usage:** hazard

Example APIs: [Adterinas-crate::init()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/lib.rs#L82)

### Data Dependency

#### OriginateFrom(val, func)

This sp describes the requirement that `val` should originate from `func` (as its return value or processed by the function/macro). In addition to the order of execution, there are also dependencies of data here.

**Formal Description:**

**Usage:** precondition

Example APIs: [Asterinas-mm::ChildRef::from_pte()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/mm/page_table/node/child.rs#L93)

### Value

#### Eq(lhs, rhs), Ne(lhs, rhs), Ge(lhs, rhs)

The sps describe the numerical magnitude relationship between `lhs` and `rhs`.

**Formal Description:** 

$$
lhs = rhs \\
lhs != rhs \\
lhs >= rhs
$$

**Usage:** precondition

Example APIs: [Asterinas-mm::MetaSlot::drop_last_in_place()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/mm/frame/meta.rs#L390)

### Valid

#### ValidAccessAddr(addr, access)

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

#### ValidBaseAddr(addr, hardware)

This sp describes the requirement that `addr` should be the valid base address of `hardware` such as an IOMMU, IO APIC, or HPET MMIO. Otherwise, any access to the hardware device can be unsafe.

**Formal Description:**

$$
addr = BaseAddr(hardware)
$$

**Usage:** precondtion

Example APIs: [Asterinas-arch::iommu::fault::init()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/arch/x86/iommu/fault.rs#L239)

#### ValidInstanceAddr(val, type)

This sp describes the requirement that `addr` should point to a valid instance of `type`. The function usually directly manipulates the data at the address. Thus it is necessary to ensure the safety.

**Formal Description:**

$$
\text{typeof}(*addr) = type
$$

**Usage:** precondition

Example APIs: [Asterinas-mm::inc_frame_ref_count()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/mm/frame/mod.rs#L325)


### Reference and Ownership

In this section, the reference and ownership do not represent Rust's concepts, but represent similar abstract functions implemented in the OS. e.g., manage resources by conceptual reference and ownership.

#### RefHeld(val), RefUnheld(val)

These sps describe the requirement that the abstract reference to `val` should have been held or not. The reference is usually implemented with reference count or just logically corresponding relation.

**Formal Description:**

**Usage:** precondition

Example APIs: [Asterinas-mm::inc_frame_ref_count()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/mm/frame/mod.rs#L325), [Asterinas-mm::page_table::node:\:child::Child::from_pte()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/mm/page_table/node/child.rs#L93)

#### OwnedResource(val)

This sp describes the requirement that `val` as a resource should be conceptually owned exclusively. Such resource can be port or other entities in the OS.

**Formal Description:**

**Usage:** precondition

Example APIs: [Asterinas-arch::iommu::fault::init()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/arch/x86/iommu/fault.rs#L239)
