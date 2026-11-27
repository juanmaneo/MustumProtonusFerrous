//! Oracle-derived test vectors for the ported initpki logic.
//!
//! Vectors were captured by compiling the verbatim Wine C body of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`, lines 504-516) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/initpki/`, and running the harness. The harness replaces `RtlRaiseException` with
//! a recorder that jumps out of the stub's `for (;;)` loop, because the stub never returns. All 16
//! printed rows were accepted: ASan and UBSan reported nothing, so no vector is excluded.
//!
//! Rows 01 and 02 are the two `@ stub` exports of `initpki.spec`, called with the names winebuild
//! emits. `DllRegisterServer` and `DllUnregisterServer` have no reference body, so they have no
//! row. Rows 03 to 16 probe the same wrapper with other inputs, to check that the record depends
//! only on the two names.

use super::*;

#[test]
fn oracle_stub_export_rows() {
    // row 01 DllInstall -> raise code=0x80000100 flags=0x1 nargs=2 p0=initpki.dll p1=DllInstall
    assert_eq!(
        dll_install(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "initpki.dll",
            function: "DllInstall",
        }
    );

    // row 02 InitializePKI -> raise code=0x80000100 flags=0x1 nargs=2 p0=initpki.dll p1=InitializePKI
    assert_eq!(
        initialize_pki(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "initpki.dll",
            function: "InitializePKI",
        }
    );
}

#[test]
fn oracle_wrapper_rows() {
    // (module, function) as the harness passed them, then code, flags and nargs as printed.
    // Every row raised exactly once (raises=1).
    let rows: [(&str, &str, u32, u32, u32); 16] = [
        // row 01 DllInstall
        ("initpki.dll", "DllInstall", 0x8000_0100, 0x01, 2),
        // row 02 InitializePKI
        ("initpki.dll", "InitializePKI", 0x8000_0100, 0x01, 2),
        // row 03 DllRegisterServer (probe)
        ("initpki.dll", "DllRegisterServer", 0x8000_0100, 0x01, 2),
        // row 04 DllUnregisterServer (probe)
        ("initpki.dll", "DllUnregisterServer", 0x8000_0100, 0x01, 2),
        // row 05 empty module and function (probe)
        ("", "", 0x8000_0100, 0x01, 2),
        // row 06 empty function (probe)
        ("initpki.dll", "", 0x8000_0100, 0x01, 2),
        // row 07 empty module (probe)
        ("", "DllInstall", 0x8000_0100, 0x01, 2),
        // row 08 another DLL's stub (probe)
        ("fontsub.dll", "MergeFontPackage", 0x8000_0100, 0x01, 2),
        // row 09 module name in upper case (probe)
        ("INITPKI.DLL", "InitializePKI", 0x8000_0100, 0x01, 2),
        // row 10 function name in lower case (probe)
        ("initpki.dll", "dllinstall", 0x8000_0100, 0x01, 2),
        // row 11 module as a Windows path (probe)
        (
            "C:\\Windows\\System32\\initpki.dll",
            "DllInstall",
            0x8000_0100,
            0x01,
            2,
        ),
        // row 12 long function name (probe)
        (
            "initpki.dll",
            "a long function name that is well past sixteen bytes",
            0x8000_0100,
            0x01,
            2,
        ),
        // row 13 function name with a space (probe)
        ("initpki.dll", "Initialize PKI", 0x8000_0100, 0x01, 2),
        // row 14 single-character names (probe)
        ("x", "y", 0x8000_0100, 0x01, 2),
        // row 15 module with a trailing space (probe)
        ("initpki.dll ", "DllInstall", 0x8000_0100, 0x01, 2),
        // row 16 function name with a suffix (probe)
        ("initpki.dll", "DllInstallX", 0x8000_0100, 0x01, 2),
    ];
    for (i, (module, function, code, flags, number_parameters)) in rows.into_iter().enumerate() {
        assert_eq!(
            wine_spec_unimplemented_stub(module, function),
            StubException {
                code,
                flags,
                number_parameters,
                module,
                function,
            },
            "row {:02}",
            i + 1
        );
    }
}
