//! Oracle-derived test vectors for the ported scarddlg stubs.
//!
//! Vectors were captured by compiling the verbatim Wine C body of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`, lines 504-516) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/scarddlg/`, and running the harness. The harness replaces `RtlRaiseException` with
//! a recorder. The C loop never returns, so the recorder longjmps back to the driver. All 10 printed
//! rows are kept: ASan and UBSan flagged none. Rows 01-05 are the five `@ stub` exports of
//! `scarddlg.spec`. Rows 06-10 are direct calls with other strings, which check that the record
//! passes them through unchanged.

use super::*;

#[test]
fn oracle_export_stub_rows() {
    // row 01 raise code=0x80000100 flags=0x1 rec_null=1 nargs=2 p0=[scarddlg.dll] p1=[GetOpenCardNameA] raises=1
    assert_eq!(
        get_open_card_name_a(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "scarddlg.dll",
            function: "GetOpenCardNameA",
        }
    );

    // row 02 raise code=0x80000100 flags=0x1 rec_null=1 nargs=2 p0=[scarddlg.dll] p1=[GetOpenCardNameW] raises=1
    assert_eq!(
        get_open_card_name_w(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "scarddlg.dll",
            function: "GetOpenCardNameW",
        }
    );

    // row 03 raise code=0x80000100 flags=0x1 rec_null=1 nargs=2 p0=[scarddlg.dll] p1=[SCardDlgExtendedError] raises=1
    assert_eq!(
        scard_dlg_extended_error(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "scarddlg.dll",
            function: "SCardDlgExtendedError",
        }
    );

    // row 04 raise code=0x80000100 flags=0x1 rec_null=1 nargs=2 p0=[scarddlg.dll] p1=[SCardUIDlgSelectCardA] raises=1
    assert_eq!(
        scard_ui_dlg_select_card_a(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "scarddlg.dll",
            function: "SCardUIDlgSelectCardA",
        }
    );

    // row 05 raise code=0x80000100 flags=0x1 rec_null=1 nargs=2 p0=[scarddlg.dll] p1=[SCardUIDlgSelectCardW] raises=1
    assert_eq!(
        scard_ui_dlg_select_card_w(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "scarddlg.dll",
            function: "SCardUIDlgSelectCardW",
        }
    );
}

#[test]
fn oracle_direct_stub_rows() {
    // row 06 raise code=0x80000100 flags=0x1 rec_null=1 nargs=2 p0=[] p1=[] raises=1
    assert_eq!(
        wine_spec_unimplemented_stub("", ""),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "",
            function: "",
        }
    );

    // row 07 raise code=0x80000100 flags=0x1 rec_null=1 nargs=2 p0=[scarddlg.dll] p1=[] raises=1
    assert_eq!(
        wine_spec_unimplemented_stub("scarddlg.dll", ""),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "scarddlg.dll",
            function: "",
        }
    );

    // row 08 raise code=0x80000100 flags=0x1 rec_null=1 nargs=2 p0=[] p1=[X] raises=1
    assert_eq!(
        wine_spec_unimplemented_stub("", "X"),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "",
            function: "X",
        }
    );

    // row 09 raise code=0x80000100 flags=0x1 rec_null=1 nargs=2 p0=[other.dll] p1=[Foo] raises=1
    assert_eq!(
        wine_spec_unimplemented_stub("other.dll", "Foo"),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "other.dll",
            function: "Foo",
        }
    );

    // row 10 raise code=0x80000100 flags=0x1 rec_null=1 nargs=2 p0=[scarddlg.dll] p1=[getopencardnamea] raises=1
    assert_eq!(
        wine_spec_unimplemented_stub("scarddlg.dll", "getopencardnamea"),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "scarddlg.dll",
            function: "getopencardnamea",
        }
    );
}
