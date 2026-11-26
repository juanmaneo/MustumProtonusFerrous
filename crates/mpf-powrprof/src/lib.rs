//! Port of the portable logic in Wine's `dlls/powrprof/powrprof.c`.
//!
//! Ported:
//! - `IsPwrHibernateAllowed`, `IsPwrShutdownAllowed`, `IsPwrSuspendAllowed` over
//!   the `SYSTEM_POWER_CAPABILITIES` flags: [`is_pwr_hibernate_allowed`],
//!   [`is_pwr_shutdown_allowed`], [`is_pwr_suspend_allowed`]
//! - `GetPwrDiskSpindownRange` after its registry reads: [`disk_spindown_range`]
//! - The `wcstol(value, NULL, 10)` parse of the registry values: [`wcstol_decimal`]
//!
//! Skipped:
//! - `CallNtPowerInformation`, `GetCurrentPowerPolicies`, `GetPwrCapabilities`:
//!   they call NtPowerInformation (OS).
//! - `CanUserWritePwrScheme`, `DllMain`, and the registry and semaphore calls in
//!   `GetPwrDiskSpindownRange`: registry and handles.
//! - The NULL check on the output pointers in `GetPwrDiskSpindownRange`: Rust
//!   references cannot be null.
//! - `DeletePwrScheme`, `EnumPwrSchemes`, `GetActivePwrScheme`, `ReadGlobalPwrPolicy`,
//!   `ReadProcessorPwrScheme`, `ReadPwrScheme`, `SetActivePwrScheme`, `WriteGlobalPwrPolicy`,
//!   `WriteProcessorPwrScheme`, `WritePwrScheme`, `IsAdminOverrideActive`, `SetSuspendState`,
//!   `PowerGetActiveScheme`, `PowerSetActiveScheme`, `PowerReadDCValue`, `PowerReadFriendlyName`,
//!   `PowerDeterminePlatformRole(Ex)`, `PowerEnumerate`, `PowerRegisterSuspendResumeNotification`,
//!   `PowerUnregisterSuspendResumeNotification`, `PowerSettingRegisterNotification`,
//!   `PowerSettingUnregisterNotification`, `PowerReadACValueIndex`, `PowerWriteACValueIndex`,
//!   `PowerRegisterForEffectivePowerModeNotifications`: FIXME stubs that return a
//!   fixed error or constant.

mod capabilities;
mod wcstol;

pub use capabilities::{
    is_pwr_hibernate_allowed, is_pwr_shutdown_allowed, is_pwr_suspend_allowed,
    SystemPowerCapabilities,
};
pub use wcstol::wcstol_decimal;

/// `GetPwrDiskSpindownRange` defaults, used when the registry value cannot be read.
const DEFAULT_SPINDOWN_MAX: u32 = 3600;
const DEFAULT_SPINDOWN_MIN: u32 = 3;

/// Size of the `BYTE lpValue[40]` buffer. Larger values fail with `ERROR_MORE_DATA`.
const SPINDOWN_VALUE_BUFFER_BYTES: usize = 40;

/// `GetPwrDiskSpindownRange` after its registry reads, returning `(RangeMax, RangeMin)`.
///
/// `max` and `min` hold the raw `DiskSpindownMax` and `DiskSpindownMin` data, or
/// `None` when that value cannot be read. A failed key open is `None` for both,
/// because the C code then takes the same defaults as two failed queries. The C
/// function always returns TRUE, so no flag is returned.
pub fn disk_spindown_range(max: Option<&[u8]>, min: Option<&[u8]>) -> (u32, u32) {
    (
        spindown_value(max, DEFAULT_SPINDOWN_MAX),
        spindown_value(min, DEFAULT_SPINDOWN_MIN),
    )
}

fn spindown_value(data: Option<&[u8]>, default: u32) -> u32 {
    match data {
        Some(bytes) if bytes.len() <= SPINDOWN_VALUE_BUFFER_BYTES => wcstol_decimal(bytes) as u32,
        _ => default,
    }
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
