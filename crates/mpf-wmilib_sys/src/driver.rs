/// `NTSTATUS`.
pub type NtStatus = u32;

/// `STATUS_SUCCESS` from `include/ntstatus.h`.
pub const STATUS_SUCCESS: NtStatus = 0x0000_0000;

/// `DriverEntry` from `dlls/wmilib.sys/main.c`. The C body returns `STATUS_SUCCESS` and does not
/// use its arguments.
pub fn driver_entry() -> NtStatus {
    STATUS_SUCCESS
}
