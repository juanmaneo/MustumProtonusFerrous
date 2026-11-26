/// `NTSTATUS`.
pub type NtStatus = u32;

/// `STATUS_SUCCESS` from `include/ntstatus.h`.
pub const STATUS_SUCCESS: NtStatus = 0x0000_0000;

/// `DriverEntry` from `dlls/ksecdd.sys/main.c`. The C body only traces its arguments and
/// returns `STATUS_SUCCESS`.
pub fn driver_entry() -> NtStatus {
    STATUS_SUCCESS
}
