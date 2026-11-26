/// The `SYSTEM_POWER_CAPABILITIES` flags read by the `Is*Allowed` predicates.
/// Each field is a `BOOLEAN` (`UCHAR`), and the C code does not normalize them.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SystemPowerCapabilities {
    pub system_s1: u8,
    pub system_s2: u8,
    pub system_s3: u8,
    pub system_s4: u8,
    pub system_s5: u8,
    pub hiber_file_present: u8,
}

/// `IsPwrHibernateAllowed`: `SystemS4 && HiberFilePresent`, returned as 0 or 1.
pub fn is_pwr_hibernate_allowed(caps: &SystemPowerCapabilities) -> u8 {
    u8::from(caps.system_s4 != 0 && caps.hiber_file_present != 0)
}

/// `IsPwrShutdownAllowed`: returns `SystemS5` unchanged, without normalizing it to 0 or 1.
pub fn is_pwr_shutdown_allowed(caps: &SystemPowerCapabilities) -> u8 {
    caps.system_s5
}

/// `IsPwrSuspendAllowed`: `SystemS1 && SystemS2 && SystemS3`, returned as 0 or 1.
pub fn is_pwr_suspend_allowed(caps: &SystemPowerCapabilities) -> u8 {
    u8::from(caps.system_s1 != 0 && caps.system_s2 != 0 && caps.system_s3 != 0)
}
