//! Oracle-derived test vectors for the ported wmilib.sys logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `DriverEntry`
//! (`dlls/wmilib.sys/main.c`) and `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`)
//! with `gcc -fsanitize=address,undefined -g` under `target/canon/wmilib.sys/`, and running the
//! harness. The harness printed 14 rows: 9 `DriverEntry` rows (3 driver values by 3 path values)
//! and one row for each of the 5 `@ stub` exports. The recorder longjmps out of the stub's loop
//! instead of returning, which is the only change from the C source. ASan and UBSan reported
//! nothing for any row, so none is excluded.

use super::*;

type StubFn = fn() -> StubException;

/// One row per `@ stub` export, in spec order (rows 10 to 14 of the harness):
/// `(helper, export, code, flags, number of parameters, module)`.
const STUB_ROWS: [(StubFn, &str, u32, u32, u32, &str); 5] = [
    (
        dll_initialize,
        "DllInitialize",
        0x8000_0100,
        0x01,
        2,
        "wmilib.sys",
    ),
    (dll_unload, "DllUnload", 0x8000_0100, 0x01, 2, "wmilib.sys"),
    (
        wmi_complete_request,
        "WmiCompleteRequest",
        0x8000_0100,
        0x01,
        2,
        "wmilib.sys",
    ),
    (
        wmi_fire_event,
        "WmiFireEvent",
        0x8000_0100,
        0x01,
        2,
        "wmilib.sys",
    ),
    (
        wmi_system_control,
        "WmiSystemControl",
        0x8000_0100,
        0x01,
        2,
        "wmilib.sys",
    ),
];

#[test]
fn oracle_driver_entry_rows() {
    // row 01 DriverEntry driver=NULL path=NULL -> status=0x00000000
    assert_eq!(driver_entry(), 0x0000_0000);
    // row 02 DriverEntry driver=NULL path=buffer -> status=0x00000000
    assert_eq!(driver_entry(), 0x0000_0000);
    // row 03 DriverEntry driver=NULL path=junk -> status=0x00000000
    assert_eq!(driver_entry(), 0x0000_0000);
    // row 04 DriverEntry driver=obj path=NULL -> status=0x00000000
    assert_eq!(driver_entry(), 0x0000_0000);
    // row 05 DriverEntry driver=obj path=buffer -> status=0x00000000
    assert_eq!(driver_entry(), 0x0000_0000);
    // row 06 DriverEntry driver=obj path=junk -> status=0x00000000
    assert_eq!(driver_entry(), 0x0000_0000);
    // row 07 DriverEntry driver=junk path=NULL -> status=0x00000000
    assert_eq!(driver_entry(), 0x0000_0000);
    // row 08 DriverEntry driver=junk path=buffer -> status=0x00000000
    assert_eq!(driver_entry(), 0x0000_0000);
    // row 09 DriverEntry driver=junk path=junk -> status=0x00000000
    assert_eq!(driver_entry(), 0x0000_0000);
}

#[test]
fn oracle_stub_rows() {
    for (row, &(helper, function, code, flags, number_parameters, module)) in
        STUB_ROWS.iter().enumerate()
    {
        assert_eq!(
            helper(),
            StubException {
                code,
                flags,
                number_parameters,
                module,
                function,
            },
            "row {:02}",
            row + 10
        );
    }
}
