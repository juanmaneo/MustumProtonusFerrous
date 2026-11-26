//! The `authz.c` stub exports. Each one traces with `FIXME`, sets the last error to
//! `ERROR_CALL_NOT_IMPLEMENTED` and returns `FALSE`. The arguments are never read and no
//! out-parameter is written, so the model takes only the thread's last-error slot.

/// `ERROR_CALL_NOT_IMPLEMENTED` from `include/winerror.h`.
pub const ERROR_CALL_NOT_IMPLEMENTED: u32 = 120;

fn not_implemented(last_error: &mut u32) -> bool {
    *last_error = ERROR_CALL_NOT_IMPLEMENTED;
    false
}

/// `AuthzInitializeResourceManager`.
pub fn authz_initialize_resource_manager(last_error: &mut u32) -> bool {
    not_implemented(last_error)
}

/// `AuthzFreeResourceManager`.
pub fn authz_free_resource_manager(last_error: &mut u32) -> bool {
    not_implemented(last_error)
}

/// `AuthzInstallSecurityEventSource`.
pub fn authz_install_security_event_source(last_error: &mut u32) -> bool {
    not_implemented(last_error)
}

/// `AuthzAccessCheck`.
pub fn authz_access_check(last_error: &mut u32) -> bool {
    not_implemented(last_error)
}

/// `AuthzFreeContext`.
pub fn authz_free_context(last_error: &mut u32) -> bool {
    not_implemented(last_error)
}

/// `AuthzInitializeContextFromSid`.
pub fn authz_initialize_context_from_sid(last_error: &mut u32) -> bool {
    not_implemented(last_error)
}

/// `AuthzInitializeContextFromToken`.
pub fn authz_initialize_context_from_token(last_error: &mut u32) -> bool {
    not_implemented(last_error)
}
