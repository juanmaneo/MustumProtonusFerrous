//! Oracle rows for `ComDBOpen` and `ComDBClose` (`dlls/msports/msports.c`) and for the nine
//! `@ stub` exports of `dlls/msports/msports.spec`. The rows are the output of
//! `target/canon/msports/oracle.c`, which compiles the verbatim C bodies with
//! `gcc -fsanitize=address,undefined -g` and runs them. All 16 rows ran clean under ASan and
//! UBSan, so none is excluded.

use super::*;

#[test]
fn oracle_com_db_close_rows() {
    // row 01 ComDBClose(h=(nil)) -> ret=87
    assert_eq!(com_db_close(), 87);
    // row 02 ComDBClose(h=0x1234) -> ret=87
    assert_eq!(com_db_close(), 87);
    // row 03 ComDBClose(h=0xffffffffffffffff) -> ret=87
    assert_eq!(com_db_close(), 87);
    // row 04 ComDBClose(h=0x1) -> ret=87
    assert_eq!(com_db_close(), 87);
}

#[test]
fn oracle_com_db_open_rows() {
    // row 05 ComDBOpen(*h=(nil)) -> ret=5 *h=0xffffffffffffffff
    let mut h: HCOMDB = 0;
    assert_eq!(com_db_open(&mut h), 5);
    assert_eq!(h, -1);

    // row 06 ComDBOpen(*h=0x1234) -> ret=5 *h=0xffffffffffffffff
    let mut h: HCOMDB = 0x1234;
    assert_eq!(com_db_open(&mut h), 5);
    assert_eq!(h, -1);

    // row 07 ComDBOpen(*h=0xffffffffffffffff) -> ret=5 *h=0xffffffffffffffff
    let mut h: HCOMDB = -1;
    assert_eq!(com_db_open(&mut h), 5);
    assert_eq!(h, -1);
}

#[test]
fn oracle_stub_rows() {
    // row 08 ComDBClaimNextFreePort -> raise code=0x80000100 flags=0x1 nargs=2 p0=msports.dll p1=ComDBClaimNextFreePort
    assert_eq!(
        com_db_claim_next_free_port(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "msports.dll",
            function: "ComDBClaimNextFreePort",
        }
    );

    // row 09 ComDBClaimPort -> raise code=0x80000100 flags=0x1 nargs=2 p0=msports.dll p1=ComDBClaimPort
    assert_eq!(
        com_db_claim_port(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "msports.dll",
            function: "ComDBClaimPort",
        }
    );

    // row 10 ComDBGetCurrentPortUsage -> raise code=0x80000100 flags=0x1 nargs=2 p0=msports.dll p1=ComDBGetCurrentPortUsage
    assert_eq!(
        com_db_get_current_port_usage(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "msports.dll",
            function: "ComDBGetCurrentPortUsage",
        }
    );

    // row 11 ComDBReleasePort -> raise code=0x80000100 flags=0x1 nargs=2 p0=msports.dll p1=ComDBReleasePort
    assert_eq!(
        com_db_release_port(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "msports.dll",
            function: "ComDBReleasePort",
        }
    );

    // row 12 ComDBResizeDatabase -> raise code=0x80000100 flags=0x1 nargs=2 p0=msports.dll p1=ComDBResizeDatabase
    assert_eq!(
        com_db_resize_database(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "msports.dll",
            function: "ComDBResizeDatabase",
        }
    );

    // row 13 ParallelPortPropPageProvider -> raise code=0x80000100 flags=0x1 nargs=2 p0=msports.dll p1=ParallelPortPropPageProvider
    assert_eq!(
        parallel_port_prop_page_provider(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "msports.dll",
            function: "ParallelPortPropPageProvider",
        }
    );

    // row 14 PortsClassInstaller -> raise code=0x80000100 flags=0x1 nargs=2 p0=msports.dll p1=PortsClassInstaller
    assert_eq!(
        ports_class_installer(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "msports.dll",
            function: "PortsClassInstaller",
        }
    );

    // row 15 SerialDisplayAdvancedSettings -> raise code=0x80000100 flags=0x1 nargs=2 p0=msports.dll p1=SerialDisplayAdvancedSettings
    assert_eq!(
        serial_display_advanced_settings(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "msports.dll",
            function: "SerialDisplayAdvancedSettings",
        }
    );

    // row 16 SerialPortPropPageProvider -> raise code=0x80000100 flags=0x1 nargs=2 p0=msports.dll p1=SerialPortPropPageProvider
    assert_eq!(
        serial_port_prop_page_provider(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "msports.dll",
            function: "SerialPortPropPageProvider",
        }
    );
}
