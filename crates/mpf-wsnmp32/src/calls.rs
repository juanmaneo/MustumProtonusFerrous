/// `SNMPAPI_SUCCESS` from `include/winsnmp.h`.
pub const SNMPAPI_SUCCESS: u32 = 1;

/// `SNMPAPI_FAILURE` from `include/winsnmp.h`. `SnmpOpen` returns it as an `HSNMP_SESSION`.
pub const SNMPAPI_FAILURE: u32 = 0;

/// `SNMPAPI_V2_SUPPORT` from `include/winsnmp.h`: the SNMP level `SnmpStartup` reports.
pub const SNMPAPI_V2_SUPPORT: u32 = 2;

/// `SNMPAPI_UNTRANSLATED_V1` from `include/winsnmp.h`: the translate mode `SnmpStartup` reports.
pub const SNMPAPI_UNTRANSLATED_V1: u32 = 1;

/// `SNMPAPI_ON` from `include/winsnmp.h`: the retransmit mode `SnmpStartup` reports.
pub const SNMPAPI_ON: u32 = 1;

/// A handle or pointer argument. `HWND` and `HSNMP_SESSION` are pointers in C; the bodies never
/// dereference them, so the port treats them as opaque integers.
pub type Handle = usize;

/// The `HSNMP_SESSION` that `SnmpOpen` returns: `SNMPAPI_FAILURE` as a handle, i.e. NULL.
pub const NULL_SESSION: Handle = SNMPAPI_FAILURE as Handle;

/// `SnmpCleanup`: returns `SNMPAPI_SUCCESS`.
pub fn snmp_cleanup() -> u32 {
    SNMPAPI_SUCCESS
}

/// `SnmpOpen`: returns `NULL_SESSION`. The window and the message are never read.
pub fn snmp_open(_hwnd: Handle, _msg: u32) -> Handle {
    NULL_SESSION
}

/// `SnmpSetRetransmitMode`: returns `SNMPAPI_SUCCESS` and ignores the mode.
pub fn snmp_set_retransmit_mode(_retransmit_mode: u32) -> u32 {
    SNMPAPI_SUCCESS
}

/// `SnmpSetTranslateMode`: returns `SNMPAPI_SUCCESS` and ignores the mode.
pub fn snmp_set_translate_mode(_translate_mode: u32) -> u32 {
    SNMPAPI_SUCCESS
}

/// `SnmpStartup`: writes the version, level and modes to each out-parameter that is `Some`,
/// then returns `SNMPAPI_SUCCESS`. A `None` out-parameter is not written, like a NULL pointer.
pub fn snmp_startup(
    major: Option<&mut u32>,
    minor: Option<&mut u32>,
    level: Option<&mut u32>,
    translate_mode: Option<&mut u32>,
    retransmit_mode: Option<&mut u32>,
) -> u32 {
    if let Some(major) = major {
        *major = 2;
    }
    if let Some(minor) = minor {
        *minor = 0;
    }
    if let Some(level) = level {
        *level = SNMPAPI_V2_SUPPORT;
    }
    if let Some(translate_mode) = translate_mode {
        *translate_mode = SNMPAPI_UNTRANSLATED_V1;
    }
    if let Some(retransmit_mode) = retransmit_mode {
        *retransmit_mode = SNMPAPI_ON;
    }
    SNMPAPI_SUCCESS
}
