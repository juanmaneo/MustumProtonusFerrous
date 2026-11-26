/// `HRESULT` from `include/winnt.h`: a signed 32-bit value, as in the C ABI.
pub type Hresult = i32;

/// `S_OK` from `include/winerror.h`.
pub const S_OK: Hresult = 0;

/// `FACILITY_NT_BIT` from `include/winerror.h`.
pub const FACILITY_NT_BIT: u32 = 0x1000_0000;

/// `BOOL` from `include/windef.h`, an `int` in C.
pub type Bool = i32;

/// `FALSE` from `include/windef.h`.
pub const FALSE: Bool = 0;

/// `WLDP_LOCKDOWN_DEFINED_FLAG` from `include/wldp.h`.
pub const WLDP_LOCKDOWN_DEFINED_FLAG: u32 = 0x8000_0000;

/// `WLDP_WINDOWS_LOCKDOWN_MODE_*` from `include/wldp.h`, in declaration order.
pub const WLDP_WINDOWS_LOCKDOWN_MODE_UNLOCKED: u32 = 0;
pub const WLDP_WINDOWS_LOCKDOWN_MODE_TRIAL: u32 = 1;
pub const WLDP_WINDOWS_LOCKDOWN_MODE_LOCKED: u32 = 2;
pub const WLDP_WINDOWS_LOCKDOWN_MODE_MAX: u32 = 3;

/// `WLDP_HOST_INFORMATION` from `include/wldp.h`. `WldpGetLockdownPolicy` only checks the pointer
/// for NULL, so the port keeps no fields.
pub struct WldpHostInformation;

/// `HRESULT_FROM_NT` from `include/winerror.h`: sets the NT facility bit on an `NTSTATUS`.
pub fn hresult_from_nt(status: u32) -> Hresult {
    (status | FACILITY_NT_BIT) as Hresult
}

/// `WldpIsDynamicCodePolicyEnabled`: always reports the policy as disabled.
pub fn is_dynamic_code_policy_enabled(is_enabled: &mut Bool) -> Hresult {
    *is_enabled = FALSE;
    S_OK
}

/// `WldpGetLockdownPolicy`: always writes `WLDP_LOCKDOWN_DEFINED_FLAG` to `state`. A NULL
/// `info` returns `HRESULT_FROM_NT(0)`, and any other `info` returns `S_OK`. `_flags` is not read.
pub fn get_lockdown_policy(
    info: Option<&WldpHostInformation>,
    state: &mut u32,
    _flags: u32,
) -> Hresult {
    *state = WLDP_LOCKDOWN_DEFINED_FLAG;
    if info.is_none() {
        return hresult_from_nt(0);
    }
    S_OK
}

/// `WldpQueryWindowsLockdownMode`: always reports `WLDP_WINDOWS_LOCKDOWN_MODE_UNLOCKED`.
pub fn query_windows_lockdown_mode(lockdown_mode: &mut u32) -> Hresult {
    *lockdown_mode = WLDP_WINDOWS_LOCKDOWN_MODE_UNLOCKED;
    S_OK
}
