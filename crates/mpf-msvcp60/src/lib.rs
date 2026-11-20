//! Port of the deterministic logic in Wine's `dlls/msvcp60`: `char_traits` and the search and
//! comparison members of `basic_string` from `string.c`, and the `fmtflags` members of `ios_base`
//! from `ios.c`.
//!
//! Ported:
//! - `char_traits::compare`, `length`, `_Copy_s`, `copy`, `find` and `assignn`, for `char` and
//!   `wchar_t`: [`char_traits`].
//! - `basic_string` `find`, `rfind`, `find_first_of`, `find_first_not_of`, `find_last_of` and
//!   `find_last_not_of`, in their `cstr_substr` forms: [`basic_string`].
//! - `basic_string` `compare`, `compare_cstr`, `compare_substr`, `compare_substr_substr` and
//!   `compare_substr_cstr_len`, and its 18 comparison operators: [`basic_string`].
//! - `ios_base` `flags_set`, `flags_get`, `setf`, `setf_mask` and `unsetf`: [`ios_base`].
//!
//! Not ported in this pass:
//! - `char_traits::move` and `_Move_s`: they are `memmove`-style copies between buffers, and need
//!   a single-buffer API that this pass does not add.
//! - `basic_string` mutators and constructors (`assign`, `append`, `insert`, `replace`, `erase`,
//!   `substr`, `resize`, `ctor_*`): they grow storage through `_Grow` and `MSVCP_allocator_*`,
//!   and this port does not model allocation.
//! - The `_ch`, `_off` and `_cstr` overloads of the search members: each forwards to the
//!   functions above with a different needle.
//! - `ios_base` stream and locale members, and locking: they depend on runtime state.
//! - `_Xran`: the out-of-range `None` results of the comparisons. `_Copy_s`'s invalid parameter is
//!   the `Err` result of `copy_s` and `copy`.

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub mod basic_string;
pub mod char_traits;
pub mod ios_base;

/// `basic_string::npos`, which is `size_t(-1)`.
pub const NPOS: usize = usize::MAX;
