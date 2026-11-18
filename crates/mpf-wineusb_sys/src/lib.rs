//! Headless subset of Wine's `dlls/wineusb.sys` (`wineusb.c` and `unixlib.c`).
//!
//! The ported functions compute from their arguments what the C bodies compute.
//! The ID strings are returned as UTF-16 buffers, and nothing is allocated from a pool.
//!
//! Ported:
//! - `query_id`: the device, hardware, compatible and instance ID strings of a
//!   USB device, and the `BUS_QUERY_ID_TYPE` dispatch.
//! - `reserve_capacity`: the growth rule of `array_reserve`, which sizes the
//!   USB event queue.
//! - `usbd_status_from_libusb`: the libusb transfer status to USBD status map.
//!
//! Skipped:
//! - The IRP and PnP paths (`fdo_pnp`, `pdo_pnp`, `driver_*`,
//!   `remove_pending_irps`) and `DriverEntry`.
//! - The libusb side: device enumeration, hotplug, transfers, `transfer_cb`,
//!   the event thread and the main loop.
//! - `make_pipe_handle` and `get_pipe`: they store the endpoint and type in the
//!   low bytes of a `HANDLE` and leave the other bytes unset, so the handle has
//!   no deterministic value.
//! - `ExAllocatePool`, `ExFreePool`, `realloc`, `free`, and the TRACE, FIXME,
//!   ERR and WARN output.
//!
//! Divergences from the C:
//! - `reserve_capacity` returns `None` only for a count over the limit. It does
//!   not model a failed `realloc`, and the caller multiplies the capacity by the
//!   element size. `size` must be non-zero, as the C code divides by it.
//! - A negative `interface_index` with `interface` set prints its 32-bit two's
//!   complement, as the C runtime does in practice. C leaves `%X` of a negative
//!   `int` undefined.
//! - `query_id` returns `None` for an unhandled type, where C returns the IRP's
//!   status. It does not model the `STATUS_NO_MEMORY` case of a failed allocation.

mod array;
mod device_ids;
mod status;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use device_ids::UsbDevice;

/// `query_id`: the ID strings of `device` for `query_type`, or `None` when the
/// type is not handled. Each string is NUL-terminated. The hardware and
/// compatible lists end with an empty string.
pub fn query_id(device: &UsbDevice, query_type: u32) -> Option<Vec<u16>> {
    device_ids::query_id(device, query_type)
}

/// `array_reserve`: the capacity once room is reserved for `count` elements, or
/// `None` when `count` is over the limit.
pub fn reserve_capacity(capacity: usize, count: usize, size: usize) -> Option<usize> {
    array::reserve_capacity(capacity, count, size)
}

/// `usbd_status_from_libusb`: the USBD status for a libusb transfer status.
pub fn usbd_status_from_libusb(status: i32) -> i32 {
    status::usbd_status_from_libusb(status)
}
