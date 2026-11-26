//! Port of the portable logic in Wine's `dlls/authz/authz.c`.
//!
//! Ported:
//! - The seven exports defined in `authz.c`. Each is a stub that calls
//!   `SetLastError(ERROR_CALL_NOT_IMPLEMENTED)` and returns `FALSE`:
//!   [`authz_initialize_resource_manager`], [`authz_free_resource_manager`],
//!   [`authz_install_security_event_source`], [`authz_access_check`],
//!   [`authz_free_context`], [`authz_initialize_context_from_sid`],
//!   [`authz_initialize_context_from_token`], and [`ERROR_CALL_NOT_IMPLEMENTED`].
//!
//! Skipped:
//! - `FIXME` traces, and the `debugstr_w` and `%08lx:%08lx` LUID formatting: diagnostics only.
//! - Export arguments: the C stubs never read them, so the model takes only the last-error slot.
//! - Out-parameters (`handle`, `reply`, `access_check_result`, `client_context`): the stubs never write them.
//! - The 35 `@ stub` names in `authz.spec` that have no body in `authz.c`
//!   (for example `AuthzCachedAccessCheck` and `AuthziSourceAudit`). Wine resolves them to
//!   `__wine_spec_unimplemented_stub` (`libs/winecrt0/stub.c`), which raises `EXCEPTION_WINE_STUB`
//!   and never returns, so there is no return value to model.

mod stubs;

pub use stubs::{
    authz_access_check, authz_free_context, authz_free_resource_manager,
    authz_initialize_context_from_sid, authz_initialize_context_from_token,
    authz_initialize_resource_manager, authz_install_security_event_source,
    ERROR_CALL_NOT_IMPLEMENTED,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
