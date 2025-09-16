### Execution Flow

#### PostToFunc(func)

This sp describes the order between function calls. The function taged by this sp can only be called after `func` has been called. For example, the call to the function is safe only if the related variables or memory sections are correctly initialized by `func`.

$$
\textsf{P} Call(func)
$$

Example APIs: [Asterinas-arch::trap::init()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/arch/x86/trap/syscall.rs#L45)


#### NotPostToFunc(func)

This sp requires the tagged function should not be called post to the given `func`. The situation usually happens when several functions have the same access to specific unsafe operations and the operations are mutual exclusive.

$$
\neg \textsf{P} Call(func)
$$

Example: [Asterinas-mm::page_table::cursor::locking::dfs_release_lock](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/mm/page_table/cursor/locking.rs#L208)

#### NotPriorToFunc(func)

This sp requires the tagged function should not be called prior to the given `func`. The situation usually happens when the function lead to a hazard state and unexpected results can be caused by `func`.

$$
\neg \Box(Call(func))
$$

Example: [Asterinas-mm:page_table::boot_pt::dismiss()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/mm/page_table/boot_pt.rs#L73)

#### CallOnce(scope)

This sp describes the requirement of a funtion to be called once and only once in the specific scope. For example, during the boot phase of the OS, we usually need to initialize some module or important global variables. In this case, `scope` can be `system` or other words that represents for the whole lifetime of the system.

$$
\textsf{F} Call(self) \wedge \Box(Call(self)\to\Box\neg Call(self))
$$

Example APIs: [Adterinas-crate::init()](https://github.com/asterinas/asterinas/blob/v0.16.0/ostd/src/lib.rs#L82)
