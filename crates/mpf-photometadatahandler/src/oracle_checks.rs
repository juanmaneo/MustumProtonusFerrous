//! Oracle-derived test vectors for the ported photometadatahandler stub model.
//!
//! Vectors were captured by compiling the verbatim Wine C body of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`, lines 504-516) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/photometadatahandler/`, and running the harness. The harness replaces
//! `RtlRaiseException` with a recorder that longjmps out, because the C loop calls it again forever.
//! All 15 printed rows ran clean under ASan and UBSan, so none were excluded.

use super::*;

#[test]
fn oracle_wine_spec_unimplemented_stub_rows() {
    let rows: [(&'static str, &'static str, StubException); 15] = [
        // row 01 in: module="photometadatahandler.dll" function="DllGetClassObject"
        (
            "photometadatahandler.dll",
            "DllGetClassObject",
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "photometadatahandler.dll",
                function: "DllGetClassObject",
            },
        ),
        // row 02 in: module="" function=""
        (
            "",
            "",
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "",
                function: "",
            },
        ),
        // row 03 in: module="a.dll" function="F"
        (
            "a.dll",
            "F",
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "a.dll",
                function: "F",
            },
        ),
        // row 04 in: module="fontsub.dll" function="MergeFontPackage"
        (
            "fontsub.dll",
            "MergeFontPackage",
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "fontsub.dll",
                function: "MergeFontPackage",
            },
        ),
        // row 05 in: module="photometadatahandler.dll" function=""
        (
            "photometadatahandler.dll",
            "",
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "photometadatahandler.dll",
                function: "",
            },
        ),
        // row 06 in: module="" function="DllGetClassObject"
        (
            "",
            "DllGetClassObject",
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "",
                function: "DllGetClassObject",
            },
        ),
        // row 07 in: module="photometadatahandler.dll" function="Dll Get Class Object"
        (
            "photometadatahandler.dll",
            "Dll Get Class Object",
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "photometadatahandler.dll",
                function: "Dll Get Class Object",
            },
        ),
        // row 08 in: module="PHOTOMETADATAHANDLER.DLL" function="DllGetClassObject"
        (
            "PHOTOMETADATAHANDLER.DLL",
            "DllGetClassObject",
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "PHOTOMETADATAHANDLER.DLL",
                function: "DllGetClassObject",
            },
        ),
        // row 09 in: module="photometadatahandler.dll" function="dllgetclassobject"
        (
            "photometadatahandler.dll",
            "dllgetclassobject",
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "photometadatahandler.dll",
                function: "dllgetclassobject",
            },
        ),
        // row 10 in: module="photometadatahandler.dll.so" function="DllGetClassObject"
        (
            "photometadatahandler.dll.so",
            "DllGetClassObject",
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "photometadatahandler.dll.so",
                function: "DllGetClassObject",
            },
        ),
        // row 11 in: module="module with<TAB>tab" function="fn\"quoted\""
        (
            "module with\ttab",
            "fn\"quoted\"",
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "module with\ttab",
                function: "fn\"quoted\"",
            },
        ),
        // row 12 in: module="é.dll" function="€" (UTF-8 bytes)
        (
            "é.dll",
            "€",
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "é.dll",
                function: "€",
            },
        ),
        // row 13 in: module="photometadatahandler.dll" function="ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef"
        (
            "photometadatahandler.dll",
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef",
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "photometadatahandler.dll",
                function: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef",
            },
        ),
        // row 14 in: module="ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_-." function="x"
        (
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_-.",
            "x",
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_-.",
                function: "x",
            },
        ),
        // row 15 in: module="x" function="y"
        (
            "x",
            "y",
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "x",
                function: "y",
            },
        ),
    ];
    for (module, function, expected) in rows {
        assert_eq!(wine_spec_unimplemented_stub(module, function), expected);
    }
}

#[test]
fn oracle_dll_get_class_object_row() {
    // row 01 of oracle_wine_spec_unimplemented_stub_rows: the only @ stub export in the spec.
    assert_eq!(
        dll_get_class_object(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "photometadatahandler.dll",
            function: "DllGetClassObject",
        }
    );
}
