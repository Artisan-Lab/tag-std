## Introduction
This is a temporary document summarizing our analysis of the unsafe APIs in [rust-for-linux](https://github.com/Artisan-Lab/tag-rust-for-linux). 
Our initial analysis is based on the official [doc](https://rust.docs.kernel.org/kernel/) provided by the project.

### Module: [kernel::list](https://rust.docs.kernel.org/kernel/list/index.html) 

#### Overview of the Module

[List](https://rust.docs.kernel.org/kernel/list/struct.List.html) shares many similarities with [LinkedList](https://doc.rust-lang.org/std/collections/struct.LinkedList.html) from the Rust standard library. However, `LinkedList` is designed for single-threaded programs and stores list nodes using `Box`, which is insufficient for kernel programming. In contrast, `List` is intended for multi-threaded environments and manages its nodes through `ListArc` or `Arc`.

Despite their different use cases, both modules adopt similar naming conventions and type structures. In particular, they each provide the following iterator and cursor types:
- `Iter` / `IterMut`: Immutable and mutable iterators over the list. These refer to the current node, but cannot insert or remove elements, as they lack position information (e.g., a pointer to the previous node).
- `IntoIter`: Consumes the list and yields ownership of its elements.
- `Cursor` / `CursorMut`: Provide a movable cursor with position awareness, enabling insertion and removal of nodes during iteration.

Besides, the `List` module has several other strusts:
- `ListLinks`:	The prev/next pointers for an item in a linked list.
- `ListLinksSelfPtr`: Similar to `ListLinks`, it also contains a pointer to the current node.
- `AtomicTracker`:
- `CursorPeek`: 
- `ListArcField`:

Traits associated with structs: 
- `ListArcSafe`: Implemented for types used with the `ListArc` wrapper.
- `ListItem`: Inherits from ListArcSafe and adds requirements for types that can be inserted into a List.
- `TryNewListArc`: Also inherits from ListArcSafe; provides a fallible constructor for ListArc.
- `HasListLinks`: Implemented for types containing a ListLinks field.
- `HasSelfPtr`: corresponds to the struct `ListLinksSelfPtr`

#### Struct: [AtomicTracker](https://rust.docs.kernel.org/kernel/list/struct.AtomicTracker.html)
There are two unsafe APIs by implementing the trait [`ListArcSafe`](#ListArcSafe).

#### Struct: [Cursor](https://rust.docs.kernel.org/kernel/list/struct.Cursor.html)
There are two unsafe APIs by implementing the trait [`kernel::prelude::Init`](#Init) and [`kernel::prelude::PinInit`](#PinInit).

#### Struct: [CursorPeek](https://rust.docs.kernel.org/kernel/list/struct.CursorPeek.html)
There are two unsafe APIs by implementing the trait [`kernel::prelude::Init`](#Init) and [`kernel::prelude::PinInit`](#PinInit).

#### Struct: [IntoIter](https://rust.docs.kernel.org/kernel/list/struct.IntoIter.html)
There are two unsafe APIs by implementing the trait [`kernel::prelude::Init`](#Init) and [`kernel::prelude::PinInit`](#PinInit).

#### Struct: [Iter](https://rust.docs.kernel.org/kernel/list/struct.Iter.html)
There are two unsafe APIs by implementing the trait [`kernel::prelude::Init`](#Init) and [`kernel::prelude::PinInit`](#PinInit).

#### Struct: [List](https://rust.docs.kernel.org/kernel/list/struct.List.html)
There are three unsafe APIs:
```rust
/// # Safety
///item must not be in a different linked list (with the same id).
pub unsafe fn remove(&mut self, item: &T) -> Option<ListArc<T, ID>>
```
The rest two unsafe APIs are provided by the trait [`kernel::prelude::Init`](#Init) and [`kernel::prelude::PinInit`](#PinInit).

#### Struct: [ListArc](https://rust.docs.kernel.org/kernel/list/struct.ListArc.html)
There are three unsafe APIs:
```rust
/// # Safety
/// * `ptr` must satisfy the safety requirements of [`Arc::from_raw`].
/// * The value must not already have a `ListArc` reference.
/// * The tracking inside `T` must think that there is a `ListArc` reference.
pub unsafe fn from_raw(ptr: *const T) -> Self
```
The rest two unsafe APIs are provided by the trait [`kernel::prelude::Init`](#Init) and [`kernel::prelude::PinInit`](#PinInit).

#### Struct: [ListArcField](https://rust.docs.kernel.org/kernel/list/struct.ListArcField.html)
There are four unsafe APIs:
```rust
/// # Safety
/// The caller must have shared access to the `ListArc<ID>` containing the struct with this field for the duration of the returned reference.
    pub unsafe fn assert_ref(&self) -> &T

/// # Safety
/// The caller must have mutable access to the `ListArc<ID>` containing the struct with this field for the duration of the returned reference.
pub unsafe fn assert_mut(&self) -> &mut T
```
The rest two unsafe APIs are provided by the trait [`kernel::prelude::Init`](#Init) and [`kernel::prelude::PinInit`](#PinInit).

#### Struct: [ListLinks](https://rust.docs.kernel.org/kernel/list/struct.ListLinks.html)
There are two unsafe APIs by implementing the trait [`kernel::prelude::Init`](#Init) and [`kernel::prelude::PinInit`](#PinInit).

#### Struct: [ListLinksSelfPtr](https://rust.docs.kernel.org/kernel/list/struct.ListLinksSelfPtr.html)
There are two unsafe APIs by implementing the trait [`kernel::prelude::Init`](#Init) and [`kernel::prelude::PinInit`](#PinInit).

#### unsafe Trait: [HasListLinks](https://rust.docs.kernel.org/kernel/list/trait.HasListLinks.html)
The trait has one unsafe method, and one requirement for implementing the trait.
```rust
/// # Safety (Change this to Guarantees?)
///All values of this type must have a ListLinks<ID> field at the given offset.
///The behavior of raw_get_list_links must not be changed.

/// # Safety
///The provided pointer must point at a valid struct of type Self.
unsafe fn raw_get_list_links(ptr: *mut Self) -> *mut ListLinks<ID>
```

#### unsafe Trait: [HasSelfPtr](https://rust.docs.kernel.org/kernel/list/trait.HasSelfPtr.html)
The trait has no method, but implementing the trait has one safety requirement.

```rust 
/// # Safety (Change this to Guarantees?)
///The ListLinks<ID> field of this struct at the offset HasListLinks<ID>::OFFSET must be inside a ListLinksSelfPtr<T, ID>.
```

<a id="ListArcSafe"></a>
#### Trait: [ListArcSafe](https://rust.docs.kernel.org/kernel/list/trait.ListArcSafe.html)
```rust
/// # Safety
///Must not be called if a ListArc already exist for this value.
unsafe fn on_create_list_arc_from_unique(self: Pin<&mut Self>) // a method in Trait kernel::list::ListArcSafe

/// # Safety
///Must only be called if there is no ListArc reference, but the tracking thinks there is.
unsafe fn on_drop_list_arc(&self) //a method in Trait kernel::list::ListArcSafe
```

#### unsafe Trait: [ListItem](https://rust.docs.kernel.org/kernel/list/trait.ListItem.html)
There are four unsafe methods.
```rust
/// # Guarantees
/// If there is a previous call to `prepare_to_insert` and there is no call to `post_remove`
/// since the most recent such call, then this returns the same pointer as the one returned by the most recent call to `prepare_to_insert`.
/// # Safety
/// The provided pointer must point at a valid value. (It need not be in an `Arc`.)
    unsafe fn view_links(me: *const Self) -> *mut ListLinks<ID>;

/// # Guarantees
/// * Returns the same pointer as the one passed to the most recent call to `prepare_to_insert`.
/// * The returned pointer is valid until the next call to `post_remove`.
/// # Safety
/// * The provided pointer must originate from the most recent call to `prepare_to_insert`, or from a call to `view_links` that happened after the most recent call to `prepare_to_insert`.
/// * Since the most recent call to `prepare_to_insert`, the `post_remove` method must not have been called.
    unsafe fn view_value(me: *mut ListLinks<ID>) -> *const Self;

/// # Guarantees
/// The caller is granted exclusive access to the returned [`ListLinks`] until `post_remove` is called.
/// # Safety
/// * The provided pointer must point at a valid value in an [`Arc`].
/// * Calls to `prepare_to_insert` and `post_remove` on the same value must alternate.
/// * The caller must own the [`ListArc`] for this value.
/// * The caller must not give up ownership of the [`ListArc`] unless `post_remove` has been called after this call to `prepare_to_insert`.
    unsafe fn prepare_to_insert(me: *const Self) -> *mut ListLinks<ID>;

/// # Guarantees
/// The returned pointer is the pointer that was originally passed to `prepare_to_insert`.
/// # Safety
/// The provided pointer must be the pointer returned by the most recent call to `prepare_to_insert`.
    unsafe fn post_remove(me: *mut ListLinks<ID>) -> *const Self;
```
#### unsafe Trait: [TryNewListArc](https://rust.docs.kernel.org/kernel/list/trait.TryNewListArc.html)
The trait has no unsafe method, but implementing the trait has one safety requirement.
```
///Guarantees
///If this call returns true, then there is no ListArc pointing to this value. Additionally, this call will have transitioned the tracking inside Self from not thinking that a ListArc exists, to thinking that a ListArc exists.
fn try_new_list_arc(&self) -> bool
```

### Module: [kernel::prelude](https://rust.docs.kernel.org/kernel/prelude/index.html)

<a id="Init"></a>
#### Trait: [kernel::prelude::Init](https://rust.docs.kernel.org/kernel/prelude/trait.Init.html)
```rust
/// # Safety
///slot is a valid pointer to uninitialized memory.
///the caller does not touch slot when Err is returned, they are only permitted to deallocate.
unsafe fn __init(self, slot: *mut T) -> Result<(), E> // A method in Trait kernel::prelude::Init
```
<a id="PinInit"></a>
#### Trait: [kernel::prelude::PinInit](https://rust.docs.kernel.org/kernel/prelude/trait.PinInit.html)
```rust
/// # Safety
///slot is a valid pointer to uninitialized memory.
///the caller does not touch slot when Err is returned, they are only permitted to deallocate.
//slot will not move until it is dropped, i.e. it will be pinned.
unsafe fn __pinned_init(self, slot: *mut T) -> Result<(), E> // A method in Trait kernel::prelude::PinInit
```

