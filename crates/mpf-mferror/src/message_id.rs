/// `STATUS_SEVERITY_ERROR` from `include/ntstatus.h`, the `Severity=Error` entries of `mferror.mc`.
pub const STATUS_SEVERITY_ERROR: u32 = 0x3;

/// `STATUS_SEVERITY_SUCCESS` from `include/ntstatus.h`, the `Severity=Success` entries of `mferror.mc`.
pub const STATUS_SEVERITY_SUCCESS: u32 = 0x0;

/// `FACILITY_MF` from `FacilityNames=(MF=0xd:FACILITY_MF)` in `mferror.mc`.
pub const FACILITY_MF: u32 = 0xd;

/// The message ID that `complete_msg` in `tools/wmc/mcy.y` computes:
/// `id | (severity << 30) | (facility << 16)`. The C shifts a signed `int`, which is undefined
/// for severities 2 and 3 (see `oracle_checks.rs`). The port shifts a `u32`, which gives the same bits.
pub const fn make_message_id(severity: u32, facility: u32, id: u32) -> u32 {
    id | (severity << 30) | (facility << 16)
}
