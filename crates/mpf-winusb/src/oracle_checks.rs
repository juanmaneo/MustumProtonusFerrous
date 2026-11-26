//! Oracle-derived test vectors for the ported winusb logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `WinUsb_Free`
//! (`dlls/winusb/main.c`) and `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`)
//! with `gcc -fsanitize=address,undefined -g` under `target/canon/winusb/`, and running the
//! harness. It printed 24 rows: rows 01 to 03 are `WinUsb_Free` with three handle values, and
//! rows 04 to 24 are the 21 `@ stub` exports. ASan and UBSan reported nothing, so no row is
//! excluded.

use super::*;

/// Rows 04 to 24: each stub row is the same record apart from the export name.
type StubRow = (fn() -> StubException, &'static str);

const STUB_ROWS: [StubRow; 21] = [
    (winusb_abort_pipe, "WinUsb_AbortPipe"),
    (winusb_control_transfer, "WinUsb_ControlTransfer"),
    (winusb_flush_pipe, "WinUsb_FlushPipe"),
    (
        winusb_get_associated_interface,
        "WinUsb_GetAssociatedInterface",
    ),
    (
        winusb_get_current_alternate_setting,
        "WinUsb_GetCurrentAlternateSetting",
    ),
    (winusb_get_descriptor, "WinUsb_GetDescriptor"),
    (winusb_get_overlapped_result, "WinUsb_GetOverlappedResult"),
    (winusb_get_pipe_policy, "WinUsb_GetPipePolicy"),
    (winusb_get_power_policy, "WinUsb_GetPowerPolicy"),
    (winusb_initialize, "WinUsb_Initialize"),
    (
        winusb_parse_configuration_descriptor,
        "WinUsb_ParseConfigurationDescriptor",
    ),
    (winusb_parse_descriptors, "WinUsb_ParseDescriptors"),
    (
        winusb_query_device_information,
        "WinUsb_QueryDeviceInformation",
    ),
    (
        winusb_query_interface_settings,
        "WinUsb_QueryInterfaceSettings",
    ),
    (winusb_query_pipe, "WinUsb_QueryPipe"),
    (winusb_read_pipe, "WinUsb_ReadPipe"),
    (winusb_reset_pipe, "WinUsb_ResetPipe"),
    (
        winusb_set_current_alternate_setting,
        "WinUsb_SetCurrentAlternateSetting",
    ),
    (winusb_set_pipe_policy, "WinUsb_SetPipePolicy"),
    (winusb_set_power_policy, "WinUsb_SetPowerPolicy"),
    (winusb_write_pipe, "WinUsb_WritePipe"),
];

#[test]
fn oracle_winusb_free_rows() {
    // row 01 WinUsb_Free handle=0x0 -> ret=1
    assert_eq!(winusb_free(std::ptr::null::<u8>()), 1);
    // row 02 WinUsb_Free handle=0x1234 -> ret=1
    assert_eq!(winusb_free(0x1234usize), 1);
    // row 03 WinUsb_Free handle=0xffffffffffffffff -> ret=1
    assert_eq!(winusb_free(usize::MAX), 1);
}

#[test]
fn oracle_stub_rows() {
    // rows 04 to 24: each -> raise code=0x80000100 flags=0x1 nargs=2 p0=winusb.dll p1=<export>
    for (helper, function) in STUB_ROWS {
        assert_eq!(
            helper(),
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "winusb.dll",
                function,
            }
        );
    }
}
