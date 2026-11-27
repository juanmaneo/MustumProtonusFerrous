//! Port of the portable logic in Wine's `dlls/comctl32`: the pointer and item arrays in `dpa.c`
//! and `dsa.c`, the `Str_GetPtr*` and `Str_SetPtr*` helpers in `string.c`, the
//! `comctl32.spec` export table, and the `@ stub` exception records.
//!
//! Ported:
//! - `dpa.c` pointer arrays: [`dpa_create`], [`dpa_create_ex`], [`dpa_grow`], [`dpa_clone`],
//!   [`dpa_clone_into`], [`dpa_get_ptr`], [`dpa_get_ptr_index`], [`dpa_insert_ptr`],
//!   [`dpa_set_ptr`], [`dpa_delete_ptr`], [`dpa_delete_all_ptrs`], [`dpa_sort`], [`dpa_search`],
//!   [`dpa_enum_callback`], [`dpa_destroy`], [`dpa_destroy_callback`], [`Dpa`], [`DPAS_SORTED`],
//!   [`DPAS_INSERTBEFORE`], [`DPAS_INSERTAFTER`]
//! - `dsa.c` item arrays: [`dsa_create`], [`dsa_get_item`], [`dsa_get_item_ptr`],
//!   [`dsa_set_item`], [`dsa_insert_item`], [`dsa_delete_item`], [`dsa_delete_all_items`],
//!   [`dsa_enum_callback`], [`dsa_destroy`], [`dsa_destroy_callback`], [`Dsa`]
//! - `string.c` pointer helpers: [`str_get_ptr_a`], [`str_set_ptr_a`], [`str_get_ptr_w`],
//!   [`str_set_ptr_w`]
//! - `comctl32.spec` export table: [`EXPORTS`], [`Export`], [`ExportKind`], [`export_kind`],
//!   [`export_by_ordinal`]
//! - `@ stub -noname` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`create_page`], [`create_proxy_page`], [`do_reader_mode`], [`sh_get_process_dword`],
//!   [`wine_spec_unimplemented_stub`], [`StubException`]
//!
//! Skipped:
//! - `DPA_Merge` in `dpa.c`. Its merge callback and its conformance rows are not written yet.
//! - `DPA_LoadStream` and `DPA_SaveStream`, which read and write an `IStream`.
//! - `Alloc`, `ReAlloc` and `Free` in `commctrl.c`, which wrap the process heap. Allocation
//!   goes through the `alloc` callback, and the heap handle is not modelled.
//! - `FreeMRUList`, `IntlStrEqWorker*`, `DllGetVersion` and the `ImageList_*` exports.
//! - The window, dialog and control exports. They stay in [`EXPORTS`] as
//!   [`ExportKind::Stdcall`], and their bodies are not ported.

mod dpa;
mod dsa;
mod exports;
mod string;
mod stub;

pub use dpa::{
    dpa_clone, dpa_clone_into, dpa_create, dpa_create_ex, dpa_delete_all_ptrs, dpa_delete_ptr,
    dpa_destroy, dpa_destroy_callback, dpa_enum_callback, dpa_get_ptr, dpa_get_ptr_index, dpa_grow,
    dpa_insert_ptr, dpa_search, dpa_set_ptr, dpa_sort, Dpa, DPAS_INSERTAFTER, DPAS_INSERTBEFORE,
    DPAS_SORTED,
};
pub use dsa::{
    dsa_create, dsa_delete_all_items, dsa_delete_item, dsa_destroy, dsa_destroy_callback,
    dsa_enum_callback, dsa_get_item, dsa_get_item_ptr, dsa_insert_item, dsa_set_item, Dsa,
};
pub use exports::{export_by_ordinal, export_kind, Export, ExportKind, EXPORTS};
pub use string::{str_get_ptr_a, str_get_ptr_w, str_set_ptr_a, str_set_ptr_w};
pub use stub::{
    create_page, create_proxy_page, do_reader_mode, sh_get_process_dword,
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
