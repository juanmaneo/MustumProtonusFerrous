/// `NTSTATUS`, as returned by `DriverEntry`.
pub type NtStatus = u32;

/// `STATUS_SUCCESS` from `ntstatus.h`.
pub const STATUS_SUCCESS: NtStatus = 0x0000_0000;

/// `DriverEntry` from `dlls/cng.sys/main.c`. Both of its arguments are unused and the body
/// always returns `STATUS_SUCCESS`, so the port takes no arguments.
pub fn driver_entry() -> NtStatus {
    STATUS_SUCCESS
}
