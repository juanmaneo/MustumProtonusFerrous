//! Oracle-derived test vectors for the ported wsnmp32 logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `SnmpCleanup`, `SnmpOpen`,
//! `SnmpSetRetransmitMode`, `SnmpSetTranslateMode` and `SnmpStartup` (`dlls/wsnmp32/wsnmp32.c`)
//! and `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`) with `gcc
//! -fsanitize=address,undefined -g` under `target/canon/wsnmp32/`, and running the harness. The
//! harness printed 60 rows: rows 01 to 17 for the explicit functions and rows 18 to 60 for the 43
//! `stub` exports. ASan and UBSan reported nothing, so no vector is excluded.

use super::*;

/// The value the C harness starts each out-parameter with. A row that prints `-` was a NULL
/// pointer, so the C body never wrote it; the port leaves the `None` out-parameter untouched.
const UNSET: u32 = 0xffff_ffff;

/// The `stub` export names in oracle row order, rows 18 to 60.
const STUB_ROWS: [&str; 43] = [
    "SnmpGetTranslateMode",
    "SnmpGetRetransmitMode",
    "SnmpGetTimeout",
    "SnmpSetTimeout",
    "SnmpSetRetry",
    "SnmpGetRetry",
    "_SnmpConveyAgentAddress@4",
    "_SnmpSetAgentAddress@4",
    "SnmpGetVendorInfo",
    "SnmpClose",
    "SnmpSendMsg",
    "SnmpRecvMsg",
    "SnmpRegister",
    "SnmpCreateSession",
    "SnmpListen",
    "SnmpCancelMsg",
    "SnmpStrToEntity",
    "SnmpEntityToStr",
    "SnmpFreeEntity",
    "SnmpSetPort",
    "SnmpStrToContext",
    "SnmpContextToStr",
    "SnmpFreeContext",
    "SnmpCreatePdu",
    "SnmpGetPduData",
    "SnmpSetPduData",
    "SnmpDuplicatePdu",
    "SnmpFreePdu",
    "SnmpCreateVbl",
    "SnmpDuplicateVbl",
    "SnmpFreeVbl",
    "SnmpCountVbl",
    "SnmpGetVb",
    "SnmpSetVb",
    "SnmpDeleteVb",
    "SnmpFreeDescriptor",
    "SnmpEncodeMsg",
    "SnmpDecodeMsg",
    "SnmpStrToOid",
    "SnmpOidToStr",
    "SnmpOidCopy",
    "SnmpOidCompare",
    "SnmpGetLastError",
];

/// Calls `snmp_startup`. Bit 0x01 gives `major` a pointer, 0x02 `minor`, 0x04 `level`, 0x08
/// `translate_mode` and 0x10 `retransmit_mode`. Returns the return value and the five values,
/// with `UNSET` for each out-parameter that was not passed.
fn startup(mask: u32) -> (u32, [u32; 5]) {
    let mut v = [UNSET; 5];
    let [major, minor, level, tmode, rmode] = &mut v;
    let ret = snmp_startup(
        (mask & 0x01 != 0).then_some(major),
        (mask & 0x02 != 0).then_some(minor),
        (mask & 0x04 != 0).then_some(level),
        (mask & 0x08 != 0).then_some(tmode),
        (mask & 0x10 != 0).then_some(rmode),
    );
    (ret, v)
}

#[test]
fn oracle_cleanup_row() {
    // row 01 SnmpCleanup() -> ret=1
    assert_eq!(snmp_cleanup(), 1);
}

#[test]
fn oracle_open_rows() {
    // row 02 SnmpOpen(hwnd=NULL,msg=0) -> handle=0
    assert_eq!(snmp_open(0, 0), 0);
    // row 03 SnmpOpen(hwnd=0x1234,msg=0x400) -> handle=0
    assert_eq!(snmp_open(0x1234, 0x400), 0);
}

#[test]
fn oracle_set_retransmit_mode_rows() {
    // rows 04 to 07 SnmpSetRetransmitMode(0, 1, 2, 4294967295) -> ret=1
    assert_eq!(snmp_set_retransmit_mode(0), 1);
    assert_eq!(snmp_set_retransmit_mode(1), 1);
    assert_eq!(snmp_set_retransmit_mode(2), 1);
    assert_eq!(snmp_set_retransmit_mode(0xffff_ffff), 1);
}

#[test]
fn oracle_set_translate_mode_rows() {
    // rows 08 to 11 SnmpSetTranslateMode(0, 1, 2, 4294967295) -> ret=1
    assert_eq!(snmp_set_translate_mode(0), 1);
    assert_eq!(snmp_set_translate_mode(1), 1);
    assert_eq!(snmp_set_translate_mode(2), 1);
    assert_eq!(snmp_set_translate_mode(0xffff_ffff), 1);
}

#[test]
fn oracle_startup_rows() {
    // row 12 SnmpStartup(mask=0x1f) -> ret=1 major=2 minor=0 level=2 translate=1 retransmit=1
    assert_eq!(startup(0x1f), (1, [2, 0, 2, 1, 1]));
    // row 13 SnmpStartup(mask=0x00) -> ret=1, no out-parameter written
    assert_eq!(startup(0x00), (1, [UNSET; 5]));
    // row 14 SnmpStartup(mask=0x01) -> ret=1 major=2
    assert_eq!(startup(0x01), (1, [2, UNSET, UNSET, UNSET, UNSET]));
    // row 15 SnmpStartup(mask=0x02) -> ret=1 minor=0
    assert_eq!(startup(0x02), (1, [UNSET, 0, UNSET, UNSET, UNSET]));
    // row 16 SnmpStartup(mask=0x10) -> ret=1 retransmit=1
    assert_eq!(startup(0x10), (1, [UNSET, UNSET, UNSET, UNSET, 1]));
    // row 17 SnmpStartup(mask=0x0a) -> ret=1 minor=0 translate=1
    assert_eq!(startup(0x0a), (1, [UNSET, 0, UNSET, 1, UNSET]));
}

#[test]
fn oracle_stub_rows() {
    // rows 18 to 60: each `stub` export raises code=0x80000100 flags=0x1 nparams=2
    // module=wsnmp32.dll function=<export name>.
    for (i, &name) in STUB_ROWS.iter().enumerate() {
        let helper = STUB_EXPORTS
            .iter()
            .find(|(n, _)| *n == name)
            .expect("every oracle stub row names a stub export")
            .1;
        assert_eq!(
            helper(),
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "wsnmp32.dll",
                function: name,
            },
            "row {}",
            i + 18
        );
    }
}
