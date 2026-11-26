//! Port of the portable logic in Wine's `dlls/atlthunk/atlthunk.c`.
//!
//! Ported:
//! - Thunk machine-code layout written into `struct thunk_code` for x86-64,
//!   i386 and AArch64 (`alloc_thunk_pool`): [`encode_x86_64`], [`encode_i386`],
//!   [`encode_aarch64`]
//! - `THUNK_POOL_SIZE` per architecture: [`pool_capacity`]
//! - Slot bookkeeping of `AtlThunk_AllocateData`, `AtlThunk_FreeData` and the
//!   `DllMain` release test: [`PoolCounters`]
//! - `AtlThunk_DataToCode` index-to-code offset: [`thunk_code_offset`]
//!
//! Skipped:
//! - `VirtualAlloc`, `VirtualProtect`, `VirtualFree`: OS memory management.
//! - `CRITICAL_SECTION` statics and `Enter/LeaveCriticalSection`: locking.
//! - `InterlockedIncrement` itself: the atomic primitive is the OS's job; the
//!   increment and comparison it guards are ported in [`PoolCounters::free`].
//! - The `current_pool` global and `DllMain` attach/detach: process state.
//! - `TRACE` / `FIXME` and the unsupported-architecture stubs.

mod encode;
mod pool;

pub use encode::{encode_aarch64, encode_i386, encode_x86_64};
pub use pool::{pool_capacity, thunk_code_offset, PoolCounters};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
