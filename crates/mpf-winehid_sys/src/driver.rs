use crate::ioctl::PASS_THROUGH_IOCTLS;

/// `NTSTATUS` from `include/ntstatus.h`, as the bit pattern the C code compares.
pub type NtStatus = u32;

/// `STATUS_SUCCESS` from `include/ntstatus.h`.
pub const STATUS_SUCCESS: NtStatus = 0;

/// `IRP_MJ_INTERNAL_DEVICE_CONTROL` from `include/ddk/wdm.h`.
pub const IRP_MJ_INTERNAL_DEVICE_CONTROL: usize = 0x0f;

/// `IRP_MJ_PNP` from `include/ddk/wdm.h`.
pub const IRP_MJ_PNP: usize = 0x1b;

/// What the driver does with an IRP. The kernel calls behind each outcome are not modelled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrpOutcome {
    /// `IoSkipCurrentIrpStackLocation` then `IoCallDriver` on `NextDeviceObject`. The status
    /// comes from the lower driver and is not modelled.
    ForwardToNextDevice,
    /// `IoCompleteRequest(irp, IO_NO_INCREMENT)`, then the function returns the IRP status it
    /// read on entry, which is `status`.
    CompleteUnsupported { status: NtStatus },
}

/// `internal_ioctl` from `dlls/winehid.sys/main.c`. `irp_status` is `irp->IoStatus.Status` on entry.
pub fn internal_ioctl(code: u32, irp_status: NtStatus) -> IrpOutcome {
    if PASS_THROUGH_IOCTLS.contains(&code) {
        IrpOutcome::ForwardToNextDevice
    } else {
        IrpOutcome::CompleteUnsupported { status: irp_status }
    }
}

/// `driver_pnp` from `dlls/winehid.sys/main.c`, which forwards every IRP.
pub fn driver_pnp() -> IrpOutcome {
    IrpOutcome::ForwardToNextDevice
}

/// `add_device` from `dlls/winehid.sys/main.c`.
pub fn add_device() -> NtStatus {
    STATUS_SUCCESS
}

/// The functions `DriverEntry` installs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Handler {
    InternalIoctl,
    Pnp,
    AddDevice,
}

/// The slots `DriverEntry` fills: `MajorFunction` entries and `DriverExtension->AddDevice`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DispatchSetup {
    pub major_functions: [(usize, Handler); 2],
    pub add_device: Handler,
}

/// What `DriverEntry` installs, in its order.
pub const DISPATCH_SETUP: DispatchSetup = DispatchSetup {
    major_functions: [
        (IRP_MJ_INTERNAL_DEVICE_CONTROL, Handler::InternalIoctl),
        (IRP_MJ_PNP, Handler::Pnp),
    ],
    add_device: Handler::AddDevice,
};

/// The `HID_MINIDRIVER_REGISTRATION` fields `DriverEntry` sets after `memset(0)`. The driver
/// object and registry path are the entry arguments and are not repeated here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MinidriverRegistration {
    pub revision: u32,
    pub device_extension_size: u32,
    pub devices_are_polled: bool,
    pub reserved: [u8; 3],
}

/// `sizeof(HID_DEVICE_EXTENSION)`: three pointers, each `pointer_size` bytes.
pub const fn hid_device_extension_size(pointer_size: u32) -> u32 {
    3 * pointer_size
}

/// The registration `DriverEntry` builds for a target with `pointer_size`-byte pointers.
pub const fn minidriver_registration(pointer_size: u32) -> MinidriverRegistration {
    MinidriverRegistration {
        revision: 0,
        device_extension_size: hid_device_extension_size(pointer_size),
        devices_are_polled: false,
        reserved: [0; 3],
    }
}
