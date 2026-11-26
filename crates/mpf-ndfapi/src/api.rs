/// `E_NOTIMPL` from `include/winerror.h`.
pub const E_NOTIMPL: i32 = 0x8000_4001u32 as i32;

/// `NdfCloseIncident`: traces its handle and returns `E_NOTIMPL`.
pub fn ndf_close_incident() -> i32 {
    E_NOTIMPL
}

/// `NdfCreateConnectivityIncident`: returns `E_NOTIMPL` and does not write `*handle`.
pub fn ndf_create_connectivity_incident() -> i32 {
    E_NOTIMPL
}

/// `NdfCreateWebIncident`: returns `E_NOTIMPL` and does not write `*handle`.
pub fn ndf_create_web_incident() -> i32 {
    E_NOTIMPL
}

/// `NdfCreateWinSockIncident`: returns `E_NOTIMPL` and does not write `*handle`.
pub fn ndf_create_winsock_incident() -> i32 {
    E_NOTIMPL
}

/// `NdfDiagnoseIncident`: returns `E_NOTIMPL` and does not write `*count` or `*causes`.
pub fn ndf_diagnose_incident() -> i32 {
    E_NOTIMPL
}

/// `NdfExecuteDiagnosis`: traces its handle and window and returns `E_NOTIMPL`.
pub fn ndf_execute_diagnosis() -> i32 {
    E_NOTIMPL
}
