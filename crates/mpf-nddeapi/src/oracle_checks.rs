//! Oracle-derived test vectors for the ported nddeapi logic.
//!
//! Vectors were captured by compiling the verbatim `dlls/nddeapi/nddeapi.c` and the verbatim
//! `__wine_spec_unimplemented_stub` body from `dlls/ntdll/exception.c` with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/nddeapi/`, and running the harness
//! `oracle.c`. The 28 printed rows are one per export: the 26 explicit `stdcall` functions and the
//! two `@ stub` exports. ASan and UBSan flagged no row, so none is excluded.

use super::*;

/// One printed row: the call text and the export it names.
type Row<T> = (&'static str, fn() -> T);

/// Rows 01-06, 09-19 and 22-26: the printed return value is `NDDE_NOT_IMPLEMENTED` (14).
const NOT_IMPLEMENTED_ROWS: [Row<u32>; 22] = [
    (
        r#"01 NDdeShareAddA("SRV", 2, NULL, (LPBYTE)abuf, 64)"#,
        ndde_share_add_a,
    ),
    (r#"02 NDdeShareDelA("SRV", "SHARE", 0)"#, ndde_share_del_a),
    (
        r#"03 NDdeShareEnumA("SRV", 1, (LPBYTE)abuf, 64, &n_read, &n_avail)"#,
        ndde_share_enum_a,
    ),
    (
        r#"04 NDdeShareGetInfoA("SRV", "SHARE", 2, (LPBYTE)abuf, 64, &n_avail, &n_items)"#,
        ndde_share_get_info_a,
    ),
    (
        r#"05 NDdeShareSetInfoA("SRV", "SHARE", 2, (LPBYTE)abuf, 64, 1)"#,
        ndde_share_set_info_a,
    ),
    (
        r#"06 NDdeGetErrorStringA(14, abuf, 64)"#,
        ndde_get_error_string_a,
    ),
    (
        r#"09 NDdeGetShareSecurityA("SRV", "SHARE", 4, NULL, 0, &sd_len)"#,
        ndde_get_share_security_a,
    ),
    (
        r#"10 NDdeSetShareSecurityA("SRV", "SHARE", 4, NULL)"#,
        ndde_set_share_security_a,
    ),
    (
        r#"11 NDdeGetTrustedShareA("SRV", "SHARE", &opts, &id0, &id1)"#,
        ndde_get_trusted_share_a,
    ),
    (
        r#"12 NDdeSetTrustedShareA("SRV", "SHARE", 1)"#,
        ndde_set_trusted_share_a,
    ),
    (
        r#"13 NDdeTrustedShareEnumA("SRV", 1, (LPBYTE)abuf, 64, &n_read, &n_avail)"#,
        ndde_trusted_share_enum_a,
    ),
    (
        r#"14 NDdeShareAddW(wsrv, 2, NULL, (LPBYTE)wbuf, 128)"#,
        ndde_share_add_w,
    ),
    (r#"15 NDdeShareDelW(wsrv, wshare, 0)"#, ndde_share_del_w),
    (
        r#"16 NDdeShareEnumW(wsrv, 1, (LPBYTE)wbuf, 128, &n_read, &n_avail)"#,
        ndde_share_enum_w,
    ),
    (
        r#"17 NDdeShareGetInfoW(wsrv, wshare, 2, (LPBYTE)wbuf, 128, &n_avail, &n_items)"#,
        ndde_share_get_info_w,
    ),
    (
        r#"18 NDdeShareSetInfoW(wsrv, wshare, 2, (LPBYTE)wbuf, 128, 1)"#,
        ndde_share_set_info_w,
    ),
    (
        r#"19 NDdeGetErrorStringW(14, wbuf, 64)"#,
        ndde_get_error_string_w,
    ),
    (
        r#"22 NDdeGetShareSecurityW(wsrv, wshare, 4, NULL, 0, &sd_len)"#,
        ndde_get_share_security_w,
    ),
    (
        r#"23 NDdeSetShareSecurityW(wsrv, wshare, 4, NULL)"#,
        ndde_set_share_security_w,
    ),
    (
        r#"24 NDdeGetTrustedShareW(wsrv, wshare, &opts, &id0, &id1)"#,
        ndde_get_trusted_share_w,
    ),
    (
        r#"25 NDdeSetTrustedShareW(wsrv, wshare, 1)"#,
        ndde_set_trusted_share_w,
    ),
    (
        r#"26 NDdeTrustedShareEnumW(wsrv, 1, (LPBYTE)wbuf, 128, &n_read, &n_avail)"#,
        ndde_trusted_share_enum_w,
    ),
];

/// Rows 07, 08, 20 and 21: the printed return value is `FALSE`.
const IS_VALID_ROWS: [Row<bool>; 4] = [
    (
        r#"07 NDdeIsValidShareNameA("SHARE")"#,
        ndde_is_valid_share_name_a,
    ),
    (
        r#"08 NDdeIsValidAppTopicListA("TOPIC")"#,
        ndde_is_valid_app_topic_list_a,
    ),
    (
        r#"20 NDdeIsValidShareNameW(wshare)"#,
        ndde_is_valid_share_name_w,
    ),
    (
        r#"21 NDdeIsValidAppTopicListW(wtopic)"#,
        ndde_is_valid_app_topic_list_w,
    ),
];

#[test]
fn oracle_not_implemented_rows() {
    for (row, export) in NOT_IMPLEMENTED_ROWS {
        assert_eq!(export(), 14, "row {row}");
    }
}

#[test]
fn oracle_is_valid_rows() {
    for (row, export) in IS_VALID_ROWS {
        assert!(!export(), "row {row}");
    }
}

#[test]
fn oracle_stub_rows() {
    // row 27 __wine_spec_unimplemented_stub("nddeapi.dll", "NDdeSpecialCommandA") -> code=0x80000100 flags=0x1 nargs=2 p0=nddeapi.dll p1=NDdeSpecialCommandA raises=1
    assert_eq!(
        ndde_special_command_a(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "nddeapi.dll",
            function: "NDdeSpecialCommandA",
        }
    );

    // row 28 __wine_spec_unimplemented_stub("nddeapi.dll", "NDdeSpecialCommandW") -> code=0x80000100 flags=0x1 nargs=2 p0=nddeapi.dll p1=NDdeSpecialCommandW raises=1
    assert_eq!(
        ndde_special_command_w(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "nddeapi.dll",
            function: "NDdeSpecialCommandW",
        }
    );
}
