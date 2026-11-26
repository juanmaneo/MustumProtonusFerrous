/// `NTSTATUS` from `include/ntstatus.h`, as `LONG` in the C headers.
pub type NtStatus = i32;

/// `STATUS_SUCCESS` from `include/ntstatus.h`.
pub const STATUS_SUCCESS: NtStatus = 0;

/// `DriverEntry` from `dlls/scsiport.sys/scsiport.c`. The C body only traces its arguments,
/// so the port takes none and returns the status unconditionally.
pub fn driver_entry() -> NtStatus {
    STATUS_SUCCESS
}
