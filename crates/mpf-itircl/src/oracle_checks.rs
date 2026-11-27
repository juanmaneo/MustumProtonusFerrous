//! Oracle-derived test vectors for the ported itircl stub logic.
//!
//! Vectors were captured by compiling the verbatim Wine C body of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/itircl/`, and running the harness `stub_oracle.c`. The harness replaces
//! `RtlRaiseException` with a recorder that longjmps out. All 16 printed rows raised the same
//! record with no sanitizer report, so no vector is excluded. In every row the echoed module and
//! function equal the inputs, and both pointers passed through unchanged.

use super::*;

const ORACLE_CODE: u32 = 0x8000_0100;
const ORACLE_FLAGS: u32 = 0x1;
const ORACLE_NUMBER_PARAMETERS: u32 = 2;

/// The `(module, function)` inputs of the 16 harness rows, in harness order.
const ROW_INPUTS: [(&str, &str); 16] = [
    ("itircl.dll", "DllGetClassObject"),
    ("itircl.dll", "DllCanUnloadNow"),
    ("itircl.dll", "DllRegisterServer"),
    ("itircl.dll", "DllUnregisterServer"),
    ("itircl.dll", "dllgetclassobject"),
    ("itircl.dll", "DllGetClassObjectX"),
    ("itircl.dll", ""),
    ("", "DllGetClassObject"),
    ("", ""),
    ("itircl", "DllGetClassObject"),
    ("ITIRCL.DLL", "DllGetClassObject"),
    ("itircl.dll ", "DllGetClassObject"),
    ("fontsub.dll", "MergeFontPackage"),
    ("shell32.dll", "DllGetClassObject"),
    (
        "itircl.dll",
        "DllGetClassObject_long_name_0123456789_abcdefghijklmnopqrstuvwxyz",
    ),
    ("itircl.dll", "DllCanUnloadNow "),
];

#[test]
fn oracle_stub_rows() {
    for (module, function) in ROW_INPUTS {
        assert_eq!(
            wine_spec_unimplemented_stub(module, function),
            StubException {
                code: ORACLE_CODE,
                flags: ORACLE_FLAGS,
                number_parameters: ORACLE_NUMBER_PARAMETERS,
                module,
                function,
            }
        );
    }
}

#[test]
fn oracle_dll_get_class_object_row() {
    // row 01 in module="itircl.dll" function="DllGetClassObject" -> code=0x80000100 flags=0x1
    // nparams=2 out module="itircl.dll" function="DllGetClassObject"
    assert_eq!(
        dll_get_class_object(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "itircl.dll",
            function: "DllGetClassObject",
        }
    );
}
