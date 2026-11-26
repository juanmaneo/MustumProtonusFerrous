//! Port of the portable logic in Wine's `dlls/winehid.sys/main.c`.
//!
//! Ported:
//! - `CTL_CODE` and the `HID_*_CTL_CODE` macros, and the IOCTLs that `internal_ioctl` passes
//!   down: [`ctl_code`], [`hid_ctl_code`], [`hid_buffer_ctl_code`], [`hid_in_ctl_code`],
//!   [`hid_out_ctl_code`], [`PASS_THROUGH_IOCTLS`]
//! - The bit decode of an IOCTL code in the `FIXME` of `internal_ioctl`, and its text:
//!   [`ioctl_fields`], [`IoctlFields`], [`unsupported_ioctl_message`]
//! - `internal_ioctl`, `driver_pnp` and `add_device`, with the kernel calls replaced by the
//!   [`IrpOutcome`] and [`NtStatus`] they produce: [`internal_ioctl`], [`driver_pnp`], [`add_device`]
//! - The setup `DriverEntry` does: the `MajorFunction` and `AddDevice` slots ([`DISPATCH_SETUP`])
//!   and the registration fields ([`minidriver_registration`], [`hid_device_extension_size`])
//!
//! Skipped:
//! - The `TRACE` calls and `debugstr_w` of the driver path, which only log.
//! - `HidRegisterMinidriver`, `IoCallDriver`, `IoSkipCurrentIrpStackLocation` and
//!   `IoCompleteRequest`: the lower driver's status and the IRP are not modelled.
//! - The return value of `DriverEntry`, which is `HidRegisterMinidriver`'s.
//! - The `DriverObject` and `RegistryPath` fields of the registration, which are the entry arguments.
//! - `winehid.inf` (install data) and `winehid.rc` (version resource).
//! - `@ stub` exports: winehid.sys has no `.spec` file and exports none, so no
//!   `__wine_spec_unimplemented_stub` model is needed.
//!
//! Bug-compatible behaviour: an unsupported IOCTL completes with the IRP's status from entry,
//! not `STATUS_NOT_SUPPORTED`, and the `FIXME` prints code 0 as `0` (C `%#lx`).

mod driver;
mod ioctl;

pub use driver::{
    add_device, driver_pnp, hid_device_extension_size, internal_ioctl, minidriver_registration,
    DispatchSetup, Handler, IrpOutcome, MinidriverRegistration, NtStatus, DISPATCH_SETUP,
    IRP_MJ_INTERNAL_DEVICE_CONTROL, IRP_MJ_PNP, STATUS_SUCCESS,
};
pub use ioctl::{
    ctl_code, hid_buffer_ctl_code, hid_ctl_code, hid_in_ctl_code, hid_out_ctl_code, ioctl_fields,
    unsupported_ioctl_message, IoctlFields, FILE_ANY_ACCESS, FILE_DEVICE_KEYBOARD, METHOD_BUFFERED,
    METHOD_IN_DIRECT, METHOD_NEITHER, METHOD_OUT_DIRECT, PASS_THROUGH_IOCTLS,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
