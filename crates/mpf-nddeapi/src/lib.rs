//! Port of the portable logic in Wine's `dlls/nddeapi/nddeapi.c` and `dlls/nddeapi/nddeapi.spec`.
//!
//! Ported:
//! - The fixed return values of the 26 `stdcall` exports: [`NDDE_NOT_IMPLEMENTED`] (14) for the
//!   22 share, error-string, security and trusted-share functions, and `FALSE` for the four
//!   `NDdeIsValid*` functions, such as [`ndde_is_valid_share_name_a`]
//! - `nddeapi.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - `NDdeSpecialCommandA` and `NDdeSpecialCommandW` (`@ stub`) and the
//!   `__wine_spec_unimplemented_stub` record they raise: [`ndde_special_command_a`],
//!   [`ndde_special_command_w`], [`wine_spec_unimplemented_stub`], [`StubException`]
//!
//! Skipped:
//! - The `TRACE` and `FIXME` output, and the arguments of every export. The C bodies only log
//!   them and never read them, so the ports take no arguments. `NDdeGetErrorString*` never writes
//!   its buffer either, so the port has no buffer to write.
//! - The `WINAPI` and `__cdecl` calling conventions and the Windows types in the signatures.
//! - The `WINE_DEFAULT_DEBUG_CHANNEL` registration.
//! - Any share, topic or security logic. Network DDE was removed in Windows Vista, and Wine's
//!   source has none: its only behaviour is the fixed return values above.

mod exports;
mod is_valid;
mod not_implemented;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use is_valid::{
    ndde_is_valid_app_topic_list_a, ndde_is_valid_app_topic_list_w, ndde_is_valid_share_name_a,
    ndde_is_valid_share_name_w,
};
pub use not_implemented::{
    ndde_get_error_string_a, ndde_get_error_string_w, ndde_get_share_security_a,
    ndde_get_share_security_w, ndde_get_trusted_share_a, ndde_get_trusted_share_w,
    ndde_set_share_security_a, ndde_set_share_security_w, ndde_set_trusted_share_a,
    ndde_set_trusted_share_w, ndde_share_add_a, ndde_share_add_w, ndde_share_del_a,
    ndde_share_del_w, ndde_share_enum_a, ndde_share_enum_w, ndde_share_get_info_a,
    ndde_share_get_info_w, ndde_share_set_info_a, ndde_share_set_info_w, ndde_trusted_share_enum_a,
    ndde_trusted_share_enum_w, NDDE_NOT_IMPLEMENTED,
};
pub use stub::{
    ndde_special_command_a, ndde_special_command_w, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
