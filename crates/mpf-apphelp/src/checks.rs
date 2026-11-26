use crate::StubResult;

/// `ApphelpCheckInstallShieldPackage`: always TRUE. Its `ptr` and `path` arguments only reach `FIXME`.
pub fn apphelp_check_install_shield_package() -> bool {
    true
}

/// `ApphelpCheckMsiPackage`: always TRUE. Its `ptr` and `path` arguments only reach `FIXME`.
pub fn apphelp_check_msi_package() -> bool {
    true
}

/// `ApphelpCheckShellObject`: always TRUE. `*flags` is cleared when the caller passes a pointer.
/// `clsid` and `shim` only reach `TRACE`.
pub fn apphelp_check_shell_object(flags: Option<&mut u64>) -> bool {
    if let Some(flags) = flags {
        *flags = 0;
    }
    true
}

/// `ShimFlushCache`: TRUE with `ERROR_CALL_NOT_IMPLEMENTED`. Its arguments only reach `FIXME`.
pub fn shim_flush_cache() -> StubResult<bool> {
    StubResult::not_implemented(true)
}
