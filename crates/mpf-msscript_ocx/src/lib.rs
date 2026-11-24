//! Port of the deterministic logic in Wine's `dlls/msscript.ocx/msscript.c`.
//!
//! Ported:
//! - Enumerator cursors, `module_enum_Next` and `module_enum_Skip`, as [`enumerate::next`] and
//!   [`enumerate::skip`]. `procedure_enum_Skip` has the same shape; `procedure_enum_Next` has the
//!   same clamp but is COM-bound.
//! - Module collection growth and indexing, as [`module_table::needs_growth`],
//!   [`module_table::grown_capacity`] and [`module_table::module_index`]. These cover the
//!   `is_power_of_2` test in `ScriptModuleCollection_Add`, the doubled capacity, and the one-based
//!   index check in `ScriptModuleCollection_get_Item`.
//! - The `Timeout` property, as [`timeout::set_timeout`] (`ScriptControl_put_Timeout`).
//! - Procedure lookup and `FUNCDESC` fields, as [`procedure::bucket_index`],
//!   [`procedure::find_in_bucket`], [`procedure::num_args`] and [`procedure::has_return_value`].
//! - The fill-once source-position latch of `fill_error_pos`, as [`error_fill::PosFill`].
//!
//! Skipped: COM plumbing, the typelib and `IActiveScript` calls, the hash function
//! (`LHashValOfNameSys` is an OS service, so the hash is an input), and the FIXME stubs.
//!
//! Divergences from C:
//! - NULL out-pointer checks (`E_POINTER`) are not reproduced, because Rust references are never
//!   null.
//! - Enumerator and index arithmetic uses wrapping `u32` operations, which match the reference's
//!   unsigned `UINT` and `ULONG` semantics on Windows.

pub mod enumerate;
pub mod error_fill;
pub mod module_table;
pub mod procedure;
pub mod timeout;

/// `HRESULT`, as its 32-bit pattern.
pub type HResult = u32;

pub const S_OK: HResult = 0;
pub const S_FALSE: HResult = 1;

/// `FAILED(hr)`: the severity bit is set.
pub const fn failed(hr: HResult) -> bool {
    (hr as i32) < 0
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
