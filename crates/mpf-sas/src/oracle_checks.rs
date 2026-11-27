//! Oracle-derived test vectors for the ported sas stub record.
//!
//! Vectors were captured by compiling the verbatim Wine C body of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`, lines 504-516) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/sas/`. The harness replaces `RtlRaiseException` with a recorder that saves the
//! record and longjmps out, because the C body loops forever. All 18 printed rows were raised
//! exactly once (`raise=1`) and none is flagged by ASan or UBSan, so 0 rows are excluded. Row 01 is
//! the call winebuild emits for `@ stub SendSAS` in `dlls/sas/sas.spec`.

use super::*;

/// The `(module, function)` the C caller passed, then the code, flags, parameter count, module
/// and function the harness read back from the raised record.
const ROWS: [(&str, &str, u32, u32, u32, &str, &str); 18] = [
    // row 01
    (
        "sas.dll",
        "SendSAS",
        0x8000_0100,
        0x01,
        2,
        "sas.dll",
        "SendSAS",
    ),
    // row 02
    (
        "sas.dll",
        "SendSAS",
        0x8000_0100,
        0x01,
        2,
        "sas.dll",
        "SendSAS",
    ),
    // row 03
    (
        "sas.dll",
        "sendsas",
        0x8000_0100,
        0x01,
        2,
        "sas.dll",
        "sendsas",
    ),
    // row 04
    (
        "SAS.DLL",
        "SendSAS",
        0x8000_0100,
        0x01,
        2,
        "SAS.DLL",
        "SendSAS",
    ),
    // row 05
    (
        "sas.dll",
        "SENDSAS",
        0x8000_0100,
        0x01,
        2,
        "sas.dll",
        "SENDSAS",
    ),
    // row 06
    (
        "sas.dll",
        "SendSA",
        0x8000_0100,
        0x01,
        2,
        "sas.dll",
        "SendSA",
    ),
    // row 07
    (
        "sas.dll",
        "SendSAS2",
        0x8000_0100,
        0x01,
        2,
        "sas.dll",
        "SendSAS2",
    ),
    // row 08
    (
        "sas.dll",
        " SendSAS",
        0x8000_0100,
        0x01,
        2,
        "sas.dll",
        " SendSAS",
    ),
    // row 09
    (
        "sas.dll",
        "SendSAS ",
        0x8000_0100,
        0x01,
        2,
        "sas.dll",
        "SendSAS ",
    ),
    // row 10
    ("sas.dll", "", 0x8000_0100, 0x01, 2, "sas.dll", ""),
    // row 11
    ("", "SendSAS", 0x8000_0100, 0x01, 2, "", "SendSAS"),
    // row 12
    ("", "", 0x8000_0100, 0x01, 2, "", ""),
    // row 13
    (
        "fontsub.dll",
        "MergeFontPackage",
        0x8000_0100,
        0x01,
        2,
        "fontsub.dll",
        "MergeFontPackage",
    ),
    // row 14
    ("sas.dll", "1", 0x8000_0100, 0x01, 2, "sas.dll", "1"),
    // row 15
    ("sas.dll", "0", 0x8000_0100, 0x01, 2, "sas.dll", "0"),
    // row 16
    (
        "ntdll.dll",
        "__wine_spec_unimplemented_stub",
        0x8000_0100,
        0x01,
        2,
        "ntdll.dll",
        "__wine_spec_unimplemented_stub",
    ),
    // row 17
    (
        "sas.dll",
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
        0x8000_0100,
        0x01,
        2,
        "sas.dll",
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
    ),
    // row 18
    (
        "a_very_long_module_name_for_the_stub_record_check.dll",
        "SendSAS",
        0x8000_0100,
        0x01,
        2,
        "a_very_long_module_name_for_the_stub_record_check.dll",
        "SendSAS",
    ),
];

#[test]
fn oracle_send_sas_row() {
    // row 01: the export as winebuild emits it, with the names from MODULE and sas.spec.
    assert_eq!(
        send_sas(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "sas.dll",
            function: "SendSAS",
        }
    );
}

#[test]
fn oracle_wine_spec_unimplemented_stub_rows() {
    for (module, function, code, flags, number_parameters, out_module, out_function) in ROWS {
        assert_eq!(
            wine_spec_unimplemented_stub(module, function),
            StubException {
                code,
                flags,
                number_parameters,
                module: out_module,
                function: out_function,
            }
        );
    }
}
