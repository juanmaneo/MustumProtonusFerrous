//! Oracle-derived test vectors for the ported wbemdisp stub exports.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`) with `gcc -fsanitize=address,undefined -g` under `target/canon/wbemdisp/`,
//! and running the harness. The 4 printed rows are the StubException fields
//! (code, flags, number of parameters, module, function) for each `@ stub` export.

use super::*;

#[test]
fn oracle_dll_can_unload_now_row() {
    // row 01 DllCanUnloadNow -> raised=1 code=0x80000100 flags=0x01 nparams=2 module=wbemdisp.dll function=DllCanUnloadNow
    assert_eq!(
        dll_can_unload_now(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "wbemdisp.dll",
            function: "DllCanUnloadNow",
        }
    );
}

#[test]
fn oracle_dll_get_class_object_row() {
    // row 02 DllGetClassObject -> raised=1 code=0x80000100 flags=0x01 nparams=2 module=wbemdisp.dll function=DllGetClassObject
    assert_eq!(
        dll_get_class_object(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "wbemdisp.dll",
            function: "DllGetClassObject",
        }
    );
}

#[test]
fn oracle_dll_register_server_row() {
    // row 03 DllRegisterServer -> raised=1 code=0x80000100 flags=0x01 nparams=2 module=wbemdisp.dll function=DllRegisterServer
    assert_eq!(
        dll_register_server(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "wbemdisp.dll",
            function: "DllRegisterServer",
        }
    );
}

#[test]
fn oracle_dll_unregister_server_row() {
    // row 04 DllUnregisterServer -> raised=1 code=0x80000100 flags=0x01 nparams=2 module=wbemdisp.dll function=DllUnregisterServer
    assert_eq!(
        dll_unregister_server(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "wbemdisp.dll",
            function: "DllUnregisterServer",
        }
    );
}
