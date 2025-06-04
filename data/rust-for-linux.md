## Introduction
This is a temporary document summarizing our analysis of the unsafe APIs in [rust-for-linux](https://github.com/Artisan-Lab/tag-rust-for-linux). 
Our initial analysis is based on the official [doc](https://rust.docs.kernel.org/kernel/) provided by the project.

### Module: [List](https://rust.docs.kernel.org/kernel/list/index.html) 

```rust
struct List<T: ?Sized + ListItem<ID>, const ID: u64 = 0> {
    first: *mut ListLinksFields,
    _ty: PhantomData<ListArc<T, ID>>, // struct ListArc <==> Trait ListArcSafe
}
struct IntoIter // owned List
struct Iter // ref to List (current node) 
struct ListLinks // only pointers of the list, no PhantomData, <==> Trait HasListLinks
struct ListLinksSelfPtr<T: ?Sized, const ID: u64 = 0> { // <==> Trait HasSelfPtr
    pub inner: ListLinks<ID>,
    self_ptr: Opaque<*const T>,
}
```

#### Struct: [AtomicTracker](https://rust.docs.kernel.org/kernel/list/struct.AtomicTracker.html)
There are two unsafe APIs.
```rust
///Safety
///Must not be called if a ListArc already exist for this value.
unsafe fn on_create_list_arc_from_unique(self: Pin<&mut Self>) // a method in Trait kernel::list::ListArcSafe
///Safety
///Must only be called if there is no ListArc reference, but the tracking thinks there is.
unsafe fn on_drop_list_arc(&self) //a method in Trait kernel::list::ListArcSafe
```
#### Struct: [Cursor](https://rust.docs.kernel.org/kernel/list/struct.Cursor.html)
There are two unsafe APIs:
```rust
///Safety
///slot is a valid pointer to uninitialized memory.
///the caller does not touch slot when Err is returned, they are only permitted to deallocate.
unsafe fn __init(self, slot: *mut T) -> Result<(), E> // A method in Trait kernel::prelude::Init

///Safety
///slot is a valid pointer to uninitialized memory.
///the caller does not touch slot when Err is returned, they are only permitted to deallocate.
//slot will not move until it is dropped, i.e. it will be pinned.
unsafe fn __pinned_init(self, slot: *mut T) -> Result<(), E> // A method in Trait kernel::prelude::PinInit
```
#### Struct: [CursorPeek](https://rust.docs.kernel.org/kernel/list/struct.CursorPeek.html)

#### Struct: [IntoIter](https://rust.docs.kernel.org/kernel/list/struct.IntoIter.html)

#### Struct: [Iter](https://rust.docs.kernel.org/kernel/list/struct.Iter.html)

#### Struct: [List](https://rust.docs.kernel.org/kernel/list/struct.List.html)
There are three unsafe APIs:
```rust
///Safety
///item must not be in a different linked list (with the same id).
pub unsafe fn remove(&mut self, item: &T) -> Option<ListArc<T, ID>>

///Safety
///slot is a valid pointer to uninitialized memory.
///the caller does not touch slot when Err is returned, they are only permitted to deallocate.
unsafe fn __init(self, slot: *mut T) -> Result<(), E> // A method in Trait kernel::prelude::Init

///Safety
///slot is a valid pointer to uninitialized memory.
///the caller does not touch slot when Err is returned, they are only permitted to deallocate.
//slot will not move until it is dropped, i.e. it will be pinned.
unsafe fn __pinned_init(self, slot: *mut T) -> Result<(), E> // A method in Trait kernel::prelude::PinInit
```

#### Struct: [ListArc](https://rust.docs.kernel.org/kernel/list/struct.ListArc.html)

#### Struct: [ListArcField](https://rust.docs.kernel.org/kernel/list/struct.ListArcField.html)

#### Struct: [ListLinks](https://rust.docs.kernel.org/kernel/list/struct.ListLinks.html)

#### Struct: [ListLinksSelfPtr](https://rust.docs.kernel.org/kernel/list/struct.ListLinksSelfPtr.html)

#### Trait: [HasListLinks](https://rust.docs.kernel.org/kernel/list/trait.HasListLinks.html)

#### Trait: [HasSelfPtr](https://rust.docs.kernel.org/kernel/list/trait.HasSelfPtr.html)

#### Trait: [ListArcSafe](https://rust.docs.kernel.org/kernel/list/trait.ListArcSafe.html)

#### Trait: [ListItem](https://rust.docs.kernel.org/kernel/list/trait.ListItem.html)

#### Trait: [TryNewListArc](https://rust.docs.kernel.org/kernel/list/trait.TryNewListArc.html)
