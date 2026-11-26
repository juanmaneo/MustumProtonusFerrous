/// How `dlls/winusb/winusb.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: a C function in `main.c`, ported as [`crate::winusb_free`].
    Stdcall,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

use ExportKind::{Stdcall, Stub};

/// The `winusb.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 22] = [
    ("WinUsb_AbortPipe", Stub),
    ("WinUsb_ControlTransfer", Stub),
    ("WinUsb_FlushPipe", Stub),
    ("WinUsb_Free", Stdcall),
    ("WinUsb_GetAssociatedInterface", Stub),
    ("WinUsb_GetCurrentAlternateSetting", Stub),
    ("WinUsb_GetDescriptor", Stub),
    ("WinUsb_GetOverlappedResult", Stub),
    ("WinUsb_GetPipePolicy", Stub),
    ("WinUsb_GetPowerPolicy", Stub),
    ("WinUsb_Initialize", Stub),
    ("WinUsb_ParseConfigurationDescriptor", Stub),
    ("WinUsb_ParseDescriptors", Stub),
    ("WinUsb_QueryDeviceInformation", Stub),
    ("WinUsb_QueryInterfaceSettings", Stub),
    ("WinUsb_QueryPipe", Stub),
    ("WinUsb_ReadPipe", Stub),
    ("WinUsb_ResetPipe", Stub),
    ("WinUsb_SetCurrentAlternateSetting", Stub),
    ("WinUsb_SetPipePolicy", Stub),
    ("WinUsb_SetPowerPolicy", Stub),
    ("WinUsb_WritePipe", Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
