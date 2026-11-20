//! cfgmgr32 subset for MPF: the deterministic logic of Wine's `dlls/cfgmgr32`.
//!
//! Properties are `CompKey` + type + buffer values and filters are `Filter` values.
//! No registry, device, notification or COM state is involved.
//!
//! Ported (rows recorded from the verbatim Wine C code in `oracle_checks.rs`, edge
//! cases in `edge_checks.rs`):
//! - `configret`: the `CR_*` and `ERROR_*` values that the mapping uses,
//!   `cr_to_win32_error` (`CM_MapCrToWin32Err`), `map_error` and `map_error_node`.
//! - `devprop`: `type_validate` (`devprop_type_validate` in `devobject.c`).
//! - `filter`: `filters_validate` (`devprop_filters_validate`) and `filters_match`
//!   (`devprop_filter_matches_properties`), including the comparison operators and
//!   the group nesting.
//! - `comp_key_eq`: the key identity that `devproperty_compare` induces. It is the
//!   only part of `devproperty_compare` that the filter engine needs.
//!
//! Divergences from the C bodies:
//! - Only equality of keys is ported. The C `bsearch` over sorted properties becomes a
//!   linear search, so `filters_match` takes unsorted input. With duplicate keys C
//!   returns whichever match the search lands on, and this port returns the first.
//! - Locale names are `&str` and compare with ASCII case folding. The oracle's
//!   `wcsicmp` shim folds ASCII only, so non-ASCII folding is not verified.
//! - Integers are 32 bits (`u32`), and sizes are `usize`. `map_error` takes the
//!   `LSTATUS` as its `u32` bit pattern.
//! - Where C reads past a buffer, the port does not. A string without a NUL unit ends
//!   at the end of its buffer. A trailing odd byte of a string is ignored.
//! - `filters_match` only evaluates. The caller validates with `filters_validate`
//!   first, as `devobject.c` does for non-NULL filter lists. An empty list matches
//!   everything, as a NULL list does. For a list that fails validation the result is
//!   unspecified (C asserts or reads past the list), but the port never panics.
//!
//! Operator quirks that are kept on purpose (the oracle rows record them):
//! - `GREATER_THAN` (3) tests the `EQUALS` bit, so it acts as `EQUALS`.
//!   `GREATER_THAN_EQUALS` (5) tests the `LESS_THAN` bit, so it acts as `LESS_THAN`.
//!   `LESS_THAN_EQUALS` (6) tests the `EQUALS` bit, so it acts as `EQUALS`.
//! - A GUID compares equal only under `EQUALS`. Any other GUID operator compares as
//!   equal (0), so `NOT_LESS_THAN` on a GUID is true.
//! - `NOT_EQUALS_IGNORE_CASE` fails validation. Its modifier bits are 0x30000 (both
//!   `NOT` and `IGNORE_CASE`), which is not one of the accepted modifiers.
//! - Unsupported operators (bitwise, begins/ends/contains) do not match. `NOT` is not
//!   applied to them.
//!
//! Skipped (needs the OS, the registry, COM or `swscanf`/`swprintf` formatting):
//! - `guid_from_string` and `propkey_from_string` call `RtlGUIDFromString` and
//!   `swscanf`. `guid_string` and `propkey_string` call `swprintf`, which the oracle
//!   does not reproduce.
//! - All registry, device enumeration, interface, notification (`notification.c`),
//!   machine connection and handle code, including the `copy_*` and `select_*`
//!   helpers that allocate and read the registry.

pub mod configret;
pub mod devprop;
pub mod filter;

#[cfg(test)]
mod edge_checks;

#[cfg(test)]
mod oracle_checks;

/// A `DEVPROPCOMPKEY`: the GUID, property id, store and optional locale of a property.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CompKey<'a> {
    /// The `fmtid` GUID bytes in memory order.
    pub fmtid: [u8; 16],
    pub pid: u32,
    pub store: u32,
    /// `None` is a NULL `LocaleName`, which is not the same key as `Some("")`.
    pub locale: Option<&'a str>,
}

/// A `DEVPROPERTY`: a key, a type and a value buffer. `None` is a NULL buffer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Property<'a> {
    pub comp_key: CompKey<'a>,
    pub ty: u32,
    pub buffer: Option<&'a [u8]>,
}

impl Property<'_> {
    /// The `BufferSize` of the property, which is the length of its buffer.
    pub fn size(&self) -> usize {
        self.buffer.unwrap_or_default().len()
    }
}

/// A `DEVPROP_FILTER_EXPRESSION`: an operator and the property it tests.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Filter<'a> {
    pub operator: u32,
    pub property: Property<'a>,
}

/// Whether two keys are equal under `devproperty_compare`: the fmtid, pid and store
/// bytes are equal, and the locales are both NULL or equal ignoring ASCII case.
pub fn comp_key_eq(a: &CompKey, b: &CompKey) -> bool {
    a.fmtid == b.fmtid
        && a.pid == b.pid
        && a.store == b.store
        && match (a.locale, b.locale) {
            (Some(x), Some(y)) => x.eq_ignore_ascii_case(y),
            (None, None) => true,
            _ => false,
        }
}
