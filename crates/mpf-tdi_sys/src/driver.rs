//! Export return value from Wine's `dlls/tdi.sys/main.c`.

use crate::{NtStatus, STATUS_SUCCESS};

/// `DriverEntry`. The C body only traces its arguments, so the port takes none.
pub fn driver_entry() -> NtStatus {
    STATUS_SUCCESS
}
