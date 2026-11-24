//! Headless port of the deterministic helpers in Wine's `dlls/msdasql`.
//!
//! The DLL is an OLE DB provider over ODBC. Most of its code is COM dispatch, ODBC calls, registry access and
//! static property tables. The functions below run on plain values and keep the C control flow.
//!
//! Ported (names in `dlls/msdasql/session.c` and `dlls/msdasql/msdasql_main.c`):
//! - `sqltype_to_bindtype`, `is_variable_length`, `is_fixed_length`: [sqltype_to_bindtype], [is_variable_length], [is_fixed_length].
//! - `debugstr_sqltype`, `debugstr_dbtype`: [debugstr_sqltype], [debugstr_dbtype].
//! - `convert_dbproperty_mode`: [convert_dbproperty_mode].
//! - `get_property_count`: [get_property_count].
//!
//! Skipped: the COM interface methods, the ODBC handle calls, `dump_sql_diag_records`, the registry and the
//! static property tables.
//!
//! Limits of the port:
//! - `convert_dbproperty_mode` folds ASCII letters only. Wine's `wcsicmp` also folds non-ASCII letters.
//! - The `FIXME` trace in the default branch of `sqltype_to_bindtype` is not emitted. The `DBTYPE_I4` fallback is.
//! - `sqltype_to_bindtype` returns a `DBTYPE` as `u16`. The C signature says `SQLSMALLINT`.

mod mode;
mod names;
mod types;

pub use mode::convert_dbproperty_mode;
pub use names::{debugstr_dbtype, debugstr_sqltype};
pub use types::{is_fixed_length, is_variable_length, sqltype_to_bindtype};

/// Ports `get_property_count`: counts the properties whose flags share at least one bit with `flag`.
pub fn get_property_count(flag: u32, property_flags: &[u32]) -> usize {
    property_flags
        .iter()
        .filter(|&&flags| (flags & flag) != 0)
        .count()
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
