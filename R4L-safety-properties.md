# Safety Properties for Rust-for-Linux

This document describes the safety properties used in Rust-for-Linux kernel code to specify safety requirements for unsafe APIs.

## 1. Synchronization

### 1.1 LockHold(lock, a)

The corresponding lock `lock` is held for the duration of lifetime `'a`.

**Formal Description**:

$$
\forall t \in \text{lifetime}('a), \text{held}(\text{lock}, t) = \text{true}
$$

**Usage**: precondition

This property describes the requirement that a specific lock is held throughout the specific lifetime. It is critical for maintaining mutual exclusion and ensuring thread-safe access to shared data structures.

**Example APIs**: [File::from_raw_file](https://rust.docs.kernel.org/kernel/fs/file/struct.File.html#method.from_raw_file), [VmaRef::from_raw](https://rust.docs.kernel.org/kernel/mm/virt/struct.VmaRef.html#method.from_raw), [VmaMixedMap::from_raw](https://rust.docs.kernel.org/kernel/mm/virt/struct.VmaMixedMap.html#method.from_raw), [Guard::new](https://rust.docs.kernel.org/kernel/sync/lock/struct.Guard.html#method.new)

### 1.2 NonData_race(loc)

The memory block `loc` should not have data races.

**Formal Description**:

$$
\nexists \text{thread}_1, \text{thread}_2 \text{ such that } (\text{thread}_1.\text{access}(\text{loc}) \land \text{thread}_2.\text{access}(\text{loc}) \land (\text{thread}_1.\text{write} \lor \text{thread}_2.\text{write}))
$$

**Usage**: precondition

This property describes the requirement that concurrent access to a memory location does not result in data races. At least one of the access must be a write operation, and the access must be properly synchronized. This requires that reading from a page doesn't race with concurrent writes to the same page.

**Example APIs**: [Page::read_raw](https://rust.docs.kernel.org/kernel/page/struct.Page.html#method.read_raw), [Page::write_raw](https://rust.docs.kernel.org/kernel/page/struct.Page.html#method.write_raw), [Page::fill_zero_raw](https://rust.docs.kernel.org/kernel/page/struct.Page.html#method.fill_zero_raw), [Page::copy_from_user_slice_raw](https://rust.docs.kernel.org/kernel/page/struct.Page.html#method.copy_from_user_slice_raw)

## 2. Non-Condition

### 2.1 NonDropped(val, event)

The value `val` is not dropped when it is used in the context of `event`.

**Formal Description**:

$$
\forall t \in \text{time}(\text{event}), \text{dropped}(\text{val}, t) = \text{false}
$$

**Usage**: precondition

This property describes the requirement that a value remains valid (not dropped) during a specific event or operation. It's crucial for preventing use-after-free errors.

**Example APIs**: [Policy::set_clk](https://rust.docs.kernel.org/kernel/cpufreq/struct.Policy.html#method.set_clk), [Policy::set_freq_table](https://rust.docs.kernel.org/kernel/cpufreq/struct.Policy.html#method.set_freq_table)

### 2.2 NonMutate(ptr, val)

When value `val` is alive, the memory pointed at by `ptr` must not be mutated.

**Formal Description**:

$$
\forall t \in \text{lifetime}(\text{val}), \text{mutated}(\text{ptr}, t) = \text{false}
$$

**Usage**: hazard

This property enforces immutability of a memory location for the duration of another value's lifetime. This can be used when the memory location `ptr` points at can be mutated by both `val` and `ptr`, and then ensures the exclusive mutable access.

**Example APIs**: [CStr::from_char_ptr](https://rust.docs.kernel.org/kernel/str/struct.CStr.html#method.from_char_ptr)

### 2.3 NonZero(val, a)

The value `val` remains non-zero for the duration of the lifetime `a`.

**Formal Description**:

$$
\forall t \in \text{lifetime}(a), \text{val}_t \neq 0
$$

**Usage**: precondition

This property describes the requirement that a value (typically a reference count or numeric value) does not become zero during a specific lifetime, preventing premature deallocation or invalid state.

**Example APIs**: [Device::get_device](https://rust.docs.kernel.org/kernel/device/struct.Device.html#method.get_device), [MmWithUser::from_raw](https://rust.docs.kernel.org/kernel/mm/struct.MmWithUser.html#method.from_raw), [LocalFile::from_raw_file](https://rust.docs.kernel.org/kernel/fs/file/struct.LocalFile.html#method.from_raw_file), [ArcBorrow::from_raw](https://rust.docs.kernel.org/kernel/sync/struct.ArcBorrow.html#method.from_raw)

### 2.4 NonMutRefNonMutRef(val)

There is no mutable reference to `val` created.

**Formal Description**:

$$
\nexists \text{ref}: \&\text{mut } T, \text{ref} \to \text{val}
$$

**Usage**: precondition

This property describes the requirement that no mutable references exist to a value. This is used in the scene where the return value has exclusive mutable access or should keep the `val` immutable .

**Example APIs**:  [ArcBorrow::new](https://rust.docs.kernel.org/src/kernel/sync/arc.rs.html#577)

### 2.5 NonConcurrent(val)

The value `val` should not be accessed concurrently by multiple users.

**Formal Description**:

$$
\nexists t, \text{users}(\text{val}, t) > 1
$$

**Usage**: precondition

This property requires exclusive access to a value, ensuring that it is not being accessed by multiple threads or contexts simultaneously.

**Example APIs**: [Revocable::revoke_nosync](https://rust.docs.kernel.org/kernel/revocable/struct.Revocable.html#method.revoke_nosync)

### 2.6 NonUsed(val)

This value `val` must not be used as an argument in any other function.

**Formal Description**:

$$
\nexists f , \text{val} \in \text{args}(f)
$$

**Usage**: precondition

This property describes the requirement that a value is used exclusively in a specific context and should not been passed to other functions, preventing double-use.

**Example APIs**: [GlobalLock::new](https://rust.docs.kernel.org/kernel/sync/lock/struct.GlobalLock.html#method.new)

### 2.7 NonAccessable(ptr)

The pointee of `ptr` must not be accessed after this call.

**Formal Description**:

$$
\forall t > t_{\text{call}}, \text{access}(\text{ptr}, t) = \text{false}
$$

**Usage**: precondition

This property describes the requirement that the memory pointed to by `ptr` must not be accessed after the function call returns. For now, this property is used to describe that the function will destroy the pointee of pointer and the pointee must not be accessed after the function returns.

**Example APIs**: [ItemOperationsVTable](https://rust.docs.kernel.org/src/kernel/configfs.rs.html#446)

### 2.8 NonInstance(val, a)

For the duration of the lifetime `'a`, there must not exist a value `val`.

**Formal Description**:

$$
\forall t \in \text{lifetime}('a), \exists(\text{val}, t) = \text{false}
$$

**Usage**: precondition

This property describes the requirement that a particular value or instance does not exist during a specific lifetime,  preventing duplicate instances.

**Example APIs**: [ArcBorrow::from_raw](https://rust.docs.kernel.org/kernel/sync/struct.ArcBorrow.html#method.from_raw)

## 3. Validity

This part expresses the requirement of function parameters. When values are passed as parameters, they need to be ensured that some conditions are met, and we use validity to describe this property.

### 3.1 Valid(val, a)

The value `val` must be valid for the duration of lifetime `'a`.

**Usage**: precondition

This property describes the general requirement that a value/pointer remains valid throughout a specified lifetime. In practical usage, we use concrete secondary safety properties to describe specific context, for example: ValidVma, ValidFile, ValidMemory


### 3.2 ValidWrite(dst, len)

The pointer `dst` is valid for writing `len` bytes.

**Formal Description**:

$$
\forall i \in [0, \text{len}), \text{writable}(\text{dst} + i) = \text{true}
$$

**Usage**: precondition

This property describes the requirement that the memory region starting at `dst` with length `len` is:

- Allocated and not freed
- Writable (not read-only)
- Within bounds of a single allocation, but the allocator is insignificant
- Properly aligned for the write operation

**Example APIs**: [PolicyData::from_raw_mut](https://rust.docs.kernel.org/kernel/cpufreq/struct.PolicyData.html#method.from_raw_mut), [Policy::from_raw_mut](https://rust.docs.kernel.org/kernel/cpufreq/struct.Policy.html#method.from_raw_mut), [Cpumask::as_mut_ref](https://rust.docs.kernel.org/kernel/cpumask/struct.Cpumask.html#method.as_mut_ref), [CpumaskVar::from_raw_mut](https://rust.docs.kernel.org/kernel/cpumask/struct.CpumaskVar.html#method.from_raw_mut)

### 3.3 ValidRead(src, len)

The pointer `src` is valid for reading `len` bytes.

**Formal Description**:

$$
\forall i \in [0, \text{len}), \text{readable}(\text{src} + i) = \text{true}
$$

**Usage**: precondition

This property describes the requirement that the memory region starting at `src` with length `len` is:

- Allocated and not freed
- Readable
- Within bounds of a single allocation, but the allocator is insignificant
- Points to initialized memory (if required by context)

**Example APIs**: [CpumaskVar::from_raw](https://rust.docs.kernel.org/kernel/cpumask/struct.CpumaskVar.html#method.from_raw), [Table::from_raw](https://rust.docs.kernel.org/kernel/cpufreq/struct.Table.html#method.from_raw), [Policy::from_raw](https://rust.docs.kernel.org/kernel/cpufreq/struct.Policy.html#method.from_raw)

### 3.4 MayInvalid(v)

The value `v` may become invalid during later usage.

**Usage**: hazard

This property highlights a potential hazard state where a value that is currently valid may become invalid in future usage. For now, this safety property is used in module `cpu`. There is a public function `from_cpu` which get a corresponding `Device` from given `CpuId`. But there is possible that the returned `Device` has been unregistered but the associated memory is not freed. So this `Device` may be invalid in later usage.This is a warning to callers that they must handle potential invalidation.

**Example APIs**: [from_cpu](https://rust.docs.kernel.org/kernel/cpu/fn.from_cpu.html)

### 3.5 ValidVma(v, l)

The VMA `v` must be valid for the duration of lifetime `'l`.

**Usage**: precondition

This property describes the requirement that a Virtual Memory Area (VMA) is valid throughout a specified lifetime. When a `VMA` is passed as argument, either it has specific usage or is used for wrapping corresponding C struct in Rust. For the former, a valid `VMA` need to ensure that it satisfies specific requirement. For the latter, the validity of `VMA` need the C-side to ensure, such as:

- The VMA has been properly initialized
- The VMA has not been freed or destroyed
- The underlying `vm_area_struct` remains accessible

**Example APIs**: [VmaRef::from_raw](https://rust.docs.kernel.org/kernel/mm/virt/struct.VmaRef.html#method.from_raw), [VmaMixedMap::from_raw](https://rust.docs.kernel.org/kernel/mm/virt/struct.VmaMixedMap.html#method.from_raw)

### 3.6 ValidFile(f)

The pointer `f` must point to a valid file.

**Formal Description**:

$$
\text{opened}(f) \land \neg\text{closed}(f) \land \text{valid\_ptr}(f)
$$

**Usage**: precondition

This property describes the requirement that a file pointer points to a valid file. In Rust-for-Linux, the `file` pointer originates from C-side, used in FFI or passed as argument to generate a C-struct wrapper. A valid file means:

- The file has been opened successfully
- The file has not been closed
- The pointer is non-null and not dangling

**Example APIs**: [File::from_raw_file](https://rust.docs.kernel.org/kernel/fs/file/struct.File.html#method.from_raw_file), [LocalFile::from_raw_file](https://rust.docs.kernel.org/kernel/fs/file/struct.LocalFile.html#method.from_raw_file)

### 3.7 ValidMemory(addr, s)

The address `addr` is the start of a valid memory region of size `s`.

**Formal Description**:

$$
\forall i \in [0, s), \text{allocated}(\text{addr} + i) = \text{true}
$$

**Usage**: precondition

This property describes the requirement that a memory region is valid for access. A valid memory region means:

- The memory has been allocated (or is a valid MMIO region)
- The memory has not been freed
- The memory is accessible

**Example APIs**: [Io::from_raw](https://rust.docs.kernel.org/kernel/io/struct.Io.html#method.from_raw)

### 3.8 ValidCast(U, T)

The pointer of type `U` can be castbale to type `T` ,  and any value of type `T` written through such a pointer must result in a valid value.

Usage: precondition

This property describes the requirement that the momory region of type `U` can be treated as type `T`, but the write operation about this memory will generate a value of type `U`. The write operation thinks that it writes value into a memory region of `T` but actually `U`, so it also needs the guarantee that the value of `T` can result in a valid `U`.

Example APIs: [pin_init::cast_init](https://rust.docs.kernel.org/pin_init/fn.cast_init.html), [pin_init::cast_pin_init](https://rust.docs.kernel.org/pin_init/fn.cast_pin_init.html)

### 3.9 ValidInstance(v)

The value `v` must be a valid instance for usage.

**Usage**: precondition

This property describes the requirement that a value is a valid instance of its type. This is a general validity requirement. This safety property is used when we need to annotate an input as `Valid` but the requirement is hard to describe. Then we can give this safety property a customized comment.

**Example APIs**:

- [ArcBorrow::new](https://rust.docs.kernel.org/src/kernel/sync/arc.rs.html#577) - requires valid Arc instance
- [call_printk](https://rust.docs.kernel.org/src/kernel/print.rs.html#100) - requires valid format string instance

## 4. Control Flow

This section describes safety properties related to function call contexts, execution order, and control flow constraints.

### 4.1 CalledBy(env)

This function is only called by the specified environment `env`.

**Usage**: precondition

**Description**:
This property describes the requirement that a function can only be invoked from a specific context or environment. The environment could be:

- Procedural macro
- The C side of a C/Rust FFI boundary

This constraint reminds that the function is only called when certain preconditions are met by the calling environment, and helps prevent misuse from unexpected contexts.

**Example APIs**:

- [CoherentAllocation::field_read](https://rust.docs.kernel.org/src/kernel/dma.rs.html#591) - CalledBy(dma_read_macro)
- [CoherentAllocation::field_write](https://rust.docs.kernel.org/src/kernel/dma.rs.html#614) - CalledBy(dma_writ_macro)
- [AttributeList::new](https://rust.docs.kernel.org/src/kernel/configfs.rs.html#700) - CalledBy(kernel::configfs_attrs_macro)

### 4.2 CallOnce()

This function can only be called once.

**Usage**: precondition

This property describes the requirement that a function must be called at most once during the lifetime of the system or a specific object. This is typically used for:

- Initialization functions that set up global state
- Functions that consume ownership of a unique resource
- Functions that transition an object into a one-way state

Calling such a function more than once would violate invariants or cause undefined behavior, such as double-initialization or resource leaks.

**Example APIs**: [GlobalLock::init](https://rust.docs.kernel.org/kernel/sync/lock/global/struct.GlobalLock.html#method.init),  [Arc::from_raw](https://rust.docs.kernel.org/kernel/sync/struct.Arc.html#method.from_raw),  [GlobalLock::init](https://rust.docs.kernel.org/kernel/sync/struct.GlobalLock.html#method.init)

### 4.3 PostToFunc(fn)

The function tagged by this property can only be called after `fn` has been called.

**Usage**: precondition

**Description**:
This property describes an ordering constraint where one function must be called before another. This enforces a sequencing dependency, typically used for:

- Initialization or some specific operation before usage (e.g., must call `init()` before `use()`, must open a file before use)
- Resource acquisition before access

**Example APIs**:[File::as_ref](https://rust.docs.kernel.org/src/kernel/drm/file.rs.html#35)

### 4.4 OriginateFrom(val, fn)

The value `val` should originate from a call to function `fn`.

**Formal Description**:

$$


$$

**Usage**: precondition

This property describes the requirement that the value should originate from specific function (as its return value or processed by the function/macro). This is used in callback function and Arc-related implementation.

**Example APIs**: [Adapter::soft_reset_callback](https://rust.docs.kernel.org/src/kernel/net/phy.rs.html#312), [Arc::from_raw](https://rust.docs.kernel.org/kernel/sync/struct.Arc.html#method.from_raw),  [ArcBorrow::from_raw](https://rust.docs.kernel.org/kernel/sync/struct.ArcBorrow.html#method.from_raw)

### 4.5 AnyThread(fn)

Function `fn` can be called from any thread.

**Usage**: precondition

This property describes that a function is thread-safe and can be safely invoked from any thread context.

**Example APIs**: [Device::get_device](https://rust.docs.kernel.org/kernel/device/struct.Device.html#method.get_device)

### 4.6 CurThread(input)

If `input` is a function, `input` can only be called on the current thread. If `input` is an instance, `input` can only be accessed on the current thread.

**Usage**: precondition

This property describes the requirement that a function or an instance must be used on the current thread and cannot be used from other threads.

**Example APIs**: [SeqFile::from_raw](https://rust.docs.kernel.org/kernel/seq_file/struct.SeqFile.html#method.from_raw), [LocalFile::from_raw_file](https://rust.docs.kernel.org/kernel/fs/file/struct.LocalFile.html#method.from_raw_file)

## 5. Miscellaneous

This section describes a miscellany of safety properties, which contains different usage. Each safety property represents a specific requirement.

### 5.1 RefTransfer(ptr, ret)

The caller must own a refcount on `ptr` that is transferred to the returned `ret`.

**Usage**: precondition

This property describes the requirement that the caller owns a reference count on a pointer, and this ownership is transferred to the returned value. This is a critical property for managing reference-counted objects:

- The pointer `ptr` passed as parameter has a reference count, and this reference count is transferred by the return value `ret`
- After this function is done, try to use the pointer `ptr` again or drop it is dangerous, as the returned object takes over the refcount increment on the underlying object, but this pointer is still can be used.

In Rust-for-Linux, this safety property is used when converting raw pointers to owned reference wrappers.

**Example APIs**: [OPP::from_raw_opp_owned](https://rust.docs.kernel.org/kernel/opp/struct.OPP.html#method.from_raw_opp_owned), [ARef::from_raw](https://rust.docs.kernel.org/kernel/sync/aref/struct.ARef.html#method.from_raw), [Request::aref_from_raw](https://rust.docs.kernel.org/src/kernel/block/mq/request.rs.html#68)

### 5.2 Invariant(ptr)

The type invariants for `*ptr` must hold for the pointee of `ptr`.

**Usage**: precondition

This property describes the requirement that all type-specific invariants must hold for the object pointed to by a pointer. Type invariants are properties that must always hold for a valid instance of a type。

**Example APIs**:[Request::aref_from_raw](https://rust.docs.kernel.org/src/kernel/block/mq/request.rs.html#68)

### 5.3 ActiveContext(val)

The returned object is only used to access `val` within the task context that was active when this function was called.

**Usage**: precondition

This property describes the requirement that an object must only be used within the specific task context that was active when it was created. This property is a conclusion of safety requirement, as for now this property is just used to tag one unsafe function.

**Example APIs**: [Task::current](https://rust.docs.kernel.org/kernel/task/struct.Task.html#method.current)

### 5.4 Associated(val, T)

The value `val` is associated with a value of type `T`.

**Formal Description**:

$$
\exists v: T, \text{associated}(\text{val}, v)
$$

**Usage**: precondition

This property describes the requirement that a value must be associated with another value of a specific type. For now this property is used in module `miscdevice` to express the relationship between `file` and `MiscDeviceRegistration`

**Example APIs**:[MiscdeviceVTable::open](https://rust.docs.kernel.org/src/kernel/miscdevice.rs.html#204), [MiscdeviceVTable::release](https://rust.docs.kernel.org/src/kernel/miscdevice.rs.html#245), [MiscdeviceVTable::mmap](https://rust.docs.kernel.org/src/kernel/miscdevice.rs.html#303)

### 5.5 ContainerOf(p, C, f)

The pointer `p` points at a field `f` of container `C`.

**Formal Description**:

$$
\exists c: C, p = \&c.f
$$

**Usage**: precondition

This property describes the requirement that a pointer must point to a specific field within a container structure. This property originates from a macro `container_of!` which checks whether the memory location `p` points at a field `f` of container `c`. If true, return the address of corresponding `c`. This macro is widely used across Rust-for-Linux and we refine this requirement as **ContainerOf**.

**Example APIs**: [Device::from_drm_device](https://rust.docs.kernel.org/src/kernel/drm/device.rs.html#145), [Attribute::show](https://rust.docs.kernel.org/src/kernel/configfs.rs.html#550), [Attribute::store](https://rust.docs.kernel.org/src/kernel/configfs.rs.html#583)

### 5.6 FlagSet(flag)

The flag `flag` must already have been set.

**Usage**: precondition

This property describes the requirement that a specific flag or bit must be set in a flags field or bitmask. For now this property is used in module `virt`. In this module there are several different structs need to be used according to the value of `vm_flags_t`

**Example APIs**: [VmaMixedMap::from_raw](https://rust.docs.kernel.org/kernel/mm/virt/struct.VmaMixedMap.html#method.from_raw)

## 6. List Operations

This section describes safety properties specific to linked list. The `list` module in Rust-for-Linux implements an **intrusive doubly-linked list**,  which use some `unsafe` markers to imply the possibly illegal operation.

### 6.1 Empty(l)

The container `l` must be empty.

**Formal Description**:

$$
\text{l.len()} = 0
$$

**Usage**: precondition

This property describes the requirement that a container must be empty before performing certain operations. This property can be used when the current operation need to satisfy the `empty container` requirement before executing.

**Example APIs**: [List::insert_inner](https://rust.docs.kernel.org/src/kernel/list.rs.html#489)

### 6.2 InList(list, val)

The value `val` must be in the list `list`.

**Formal Description**:

$$
\text{val} \in \text{list}
$$

**Usage**: precondition

This property describes the requirement that a specific value must be a member of a particular list before performing operations on it. Before insertion operation, we need to ensure that the inserted item has a right position in the list, so we need to ensure the related items already in the list. Likewise, before deletion operation, we need to ensure that the item needed to be deleted has already in the list.

**Example APIs**: [List::remove_internal](https://rust.docs.kernel.org/src/kernel/list.rs.html#623), [List::remove_internal_inner](https://rust.docs.kernel.org/src/kernel/list.rs.html#637), [List::insert_inner](https://rust.docs.kernel.org/src/kernel/list.rs.html#489)

### 6.3 NonInList(list, val)

The value `val` must not be in the list `list`.

**Formal Description**:

$$
\text{val} \notin \text{list}
$$

**Usage**: precondition

This property describes the requirement that a specific value must not be a member of a list. We use this property to denote the uniqueness of an item in this list(with the same id) which should not exist in other list.

**Example APIs**: [List::remove](https://rust.docs.kernel.org/kernel/list/struct.List.html#method.remove)

### 6.4 Equal(l, r)

The left value `l` must be equal to the right value `r`.

**Formal Description**:

$$
l = r
$$

**Usage**: precondition

This property describes the requirement that two values must be equal. In the context of linked list operations, this is used to verify pointer relationships, such as `(*item).prev == prev` and `(*item).next == next`

**Example APIs**: [List::remove_internal_inner](https://rust.docs.kernel.org/src/kernel/list.rs.html#637)

### 6.5 Null(p)

The pointer `p` must be null.

**Formal Description**:

$$
p = \text{null}
$$

**Usage**: precondition

This property describes the requirement that a pointer must be null.

**Example APIs**: [List::insert_inner](https://rust.docs.kernel.org/src/kernel/list.rs.html#489)

### 6.6 NonExist(T, val)

An instance of type `T` must not exist for value `val`.

**Usage**: precondition

This property describes the requirement that no instance of a specific type wrapping a value already exist. This property ensures the uniqueness when creating a `ListArc`.

**Example APIs**: [ListArc::from_raw](https://rust.docs.kernel.org/kernel/list/struct.ListArc.html#method.from_raw)

### 6.7 Think_Exist(target, type)

The tracking inside `type` must think that there is a `target` reference.

**Usage**: precondition

This property originates from the `arc` module. There is a concept called `the tracking inside T` in this module, which denotes that there should be some kind of logic for keeping tracking of whether a `ListArc` exist or not. We refine this safety requirement as **Think_Exist**.

**Example APIs**: [ListArc::from_raw](https://rust.docs.kernel.org/kernel/list/struct.ListArc.html#method.from_raw)

### 6.8 Access(T, ret_val, type)

The caller must have `type` access to `T` for the duration of `ret_val`.

**Usage**: precondition

This property describes the requirement that the caller must maintain a specific type of access (immutable, mutable) to a value for the duration of a returned object. In the `arc_field.rs`, the struct `ListArcField` is used as a field of `ListArc`, so the operation requires that the caller has the same access rights as return value.

**Example APIs**: [ListArcField::assert_ref](https://rust.docs.kernel.org/kernel/list/struct.ListArcField.html#method.assert_ref), [ListArcField::assert_mut](https://rust.docs.kernel.org/kernel/list/struct.ListArcField.html#method.assert_mut)
