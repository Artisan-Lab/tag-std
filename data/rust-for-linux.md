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
There are two unsafe APIs by implementing the trait [`kernel::prelude::Init`](#kernel::prelude::Init) and [`kernel::prelude::PinInit`](#kernel::prelude::PinInit).

#### Struct: [CursorPeek](https://rust.docs.kernel.org/kernel/list/struct.CursorPeek.html)
There are two unsafe APIs by implementing the trait [`kernel::prelude::Init`](#kernel::prelude::Init) and [`kernel::prelude::PinInit`](#kernel::prelude::PinInit).

#### Struct: [IntoIter](https://rust.docs.kernel.org/kernel/list/struct.IntoIter.html)
There are two unsafe APIs by implementing the trait [`kernel::prelude::Init`](#kernel::prelude::Init) and [`kernel::prelude::PinInit`](#kernel::prelude::PinInit).

#### Struct: [Iter](https://rust.docs.kernel.org/kernel/list/struct.Iter.html)
There are two unsafe APIs by implementing the trait [`kernel::prelude::Init`](#kernel::prelude::Init) and [`kernel::prelude::PinInit`](#kernel::prelude::PinInit).

#### Struct: [List](https://rust.docs.kernel.org/kernel/list/struct.List.html)
There are three unsafe APIs:
```rust
///Safety
///item must not be in a different linked list (with the same id).
pub unsafe fn remove(&mut self, item: &T) -> Option<ListArc<T, ID>>
```
The rest two unsafe APIs are provided by the trait [`kernel::prelude::Init`](#kernel::prelude::Init) and [`kernel::prelude::PinInit`](#kernel::prelude::PinInit).

#### Struct: [ListArc](https://rust.docs.kernel.org/kernel/list/struct.ListArc.html)
There are three unsafe APIs:
```rust
///Safety
/// * `ptr` must satisfy the safety requirements of [`Arc::from_raw`].
/// * The value must not already have a `ListArc` reference.
/// * The tracking inside `T` must think that there is a `ListArc` reference.
pub unsafe fn from_raw(ptr: *const T) -> Self
```
The rest two unsafe APIs are provided by the trait [`kernel::prelude::Init`](#kernel::prelude::Init) and [`kernel::prelude::PinInit`](#kernel::prelude::PinInit).

#### Struct: [ListArcField](https://rust.docs.kernel.org/kernel/list/struct.ListArcField.html)
There are four unsafe APIs:
```rust
/// Safety
/// The caller must have shared access to the `ListArc<ID>` containing the struct with this field for the duration of the returned reference.
    pub unsafe fn assert_ref(&self) -> &T

/// Safety
/// The caller must have mutable access to the `ListArc<ID>` containing the struct with this field for the duration of the returned reference.
pub unsafe fn assert_mut(&self) -> &mut T
```
The rest two unsafe APIs are provided by the trait [`kernel::prelude::Init`](#kernel::prelude::Init) and [`kernel::prelude::PinInit`](#kernel::prelude::PinInit).

#### Struct: [ListLinks](https://rust.docs.kernel.org/kernel/list/struct.ListLinks.html)
There are two unsafe APIs by implementing the trait [`kernel::prelude::Init`](#kernel::prelude::Init) and [`kernel::prelude::PinInit`](#kernel::prelude::PinInit).

#### Struct: [ListLinksSelfPtr](https://rust.docs.kernel.org/kernel/list/struct.ListLinksSelfPtr.html)
There are two unsafe APIs by implementing the trait [`kernel::prelude::Init`](#kernel::prelude::Init) and [`kernel::prelude::PinInit`](#kernel::prelude::PinInit).

#### unsafe Trait: [HasListLinks](https://rust.docs.kernel.org/kernel/list/trait.HasListLinks.html)

#### unsafe Trait: [HasSelfPtr](https://rust.docs.kernel.org/kernel/list/trait.HasSelfPtr.html)

<a id="ListArcSafe"></a>
#### Trait: [ListArcSafe](https://rust.docs.kernel.org/kernel/list/trait.ListArcSafe.html)
```rust
///Safety
///Must not be called if a ListArc already exist for this value.
unsafe fn on_create_list_arc_from_unique(self: Pin<&mut Self>) // a method in Trait kernel::list::ListArcSafe
///Safety
///Must only be called if there is no ListArc reference, but the tracking thinks there is.
unsafe fn on_drop_list_arc(&self) //a method in Trait kernel::list::ListArcSafe
```

#### unsafe Trait: [ListItem](https://rust.docs.kernel.org/kernel/list/trait.ListItem.html)

#### unsafe Trait: [TryNewListArc](https://rust.docs.kernel.org/kernel/list/trait.TryNewListArc.html)

### Module: [kernel::prelude](https://rust.docs.kernel.org/kernel/prelude/index.html)

<a id="kernel::prelude::Init"></a>
#### Trait: [kernel::prelude::Init](https://rust.docs.kernel.org/kernel/prelude/trait.Init.html)
```rust
///Safety
///slot is a valid pointer to uninitialized memory.
///the caller does not touch slot when Err is returned, they are only permitted to deallocate.
unsafe fn __init(self, slot: *mut T) -> Result<(), E> // A method in Trait kernel::prelude::Init
```
<a id="kernel::prelude::PinInit"></a>
#### Trait: [kernel::prelude::PinInit](https://rust.docs.kernel.org/kernel/prelude/trait.PinInit.html)
```rust
///Safety
///slot is a valid pointer to uninitialized memory.
///the caller does not touch slot when Err is returned, they are only permitted to deallocate.
//slot will not move until it is dropped, i.e. it will be pinned.
unsafe fn __pinned_init(self, slot: *mut T) -> Result<(), E> // A method in Trait kernel::prelude::PinInit
```

