//! Port of the portable logic in Wine's `dlls/tapi32/` (`line.c`, `phone.c`, `assisted.c` and
//! `tapi32.spec`).
//!
//! Ported:
//! - `lineGetCountryA` and `lineGetCountryW` country list: [`line_get_country`], [`CountryKey`],
//!   [`CountryRules`], [`country_id_of`]
//! - `tapiGetLocationInfoA` and `tapiGetLocationInfoW`: [`tapi_get_location_info_a`],
//!   [`tapi_get_location_info_w`], [`LocationRegistry`], [`RegData`], [`LocationInfo`],
//!   [`AnsiLocationInfo`]
//! - Out-parameter exports that return a constant: [`line_negotiate_api_version`],
//!   [`phone_initialize_ex`]
//! - `tapi32.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`], and the
//!   `LINEERR_OPERATIONFAILED` constant
//! - `@ stub` exports `tapiRequestDrop` and `tapiRequestMediaCall`, and the
//!   `__wine_spec_unimplemented_stub` record they raise: [`tapi_request_drop`],
//!   [`tapi_request_media_call`], [`wine_spec_unimplemented_stub`], [`StubException`]
//!
//! Skipped:
//! - `lineGetTranslateCaps` and `lineGetTranslateCapsA`. They walk the registry in two passes and
//!   have no oracle rows, so the export table marks them [`ExportKind::Unported`].
//! - The NULL checks of the C bodies. The port takes references, or `want_*` flags for the
//!   optional out-parameters of the location lookup.
//! - Registry enumeration and info failure codes, and `LINEERR_NOMEM` from the heap allocation.
//! - ANSI code page conversion. [`tapi_get_location_info_a`] maps ASCII only.
//! - The `FIXME` and `TRACE` output, and the `warn_once` counters.
//! - Undefined behaviour in the C bodies: writes past the caller's buffer, and the size of a
//!   missing registry value.
//! - The `WINAPI` calling convention, and the handle, device ID, version and extension parameters
//!   that the C bodies never read.

mod country;
mod exports;
mod location;
mod out_param;
mod stub;

pub use country::{country_id_of, line_get_country, CountryKey, CountryRules};
pub use exports::{export_kind, ExportKind, EXPORTS, LINEERR_OPERATIONFAILED};
pub use location::{
    tapi_get_location_info_a, tapi_get_location_info_w, AnsiLocationInfo, LocationInfo,
    LocationRegistry, RegData,
};
pub use out_param::{line_negotiate_api_version, phone_initialize_ex};
pub use stub::{
    tapi_request_drop, tapi_request_media_call, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
