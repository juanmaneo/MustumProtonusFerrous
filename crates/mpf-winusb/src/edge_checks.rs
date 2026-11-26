//! Edge-case tests for the ported winusb logic.
//!
//! These cover the handle type, the constants, the export table and the stub table. Not every
//! expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/windef.h, include/winternl.h and include/winnt.h,
    // and the MODULE line of dlls/winusb/Makefile.in.
    assert_eq!(TRUE, 1);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    assert_eq!(WINUSB_DLL_NAME, "winusb.dll");
}

#[test]
fn free_ignores_the_handle_value() {
    // Not oracle-derived: the oracle used three handle values. The C body never reads the handle,
    // so every value and every pointer type gives TRUE.
    assert_eq!(winusb_free(()), TRUE);
    assert_eq!(winusb_free(&0u8 as *const u8), TRUE);
    assert_eq!(winusb_free(1u64), TRUE);
    assert_eq!(winusb_free(&mut 0u32 as *mut u32), TRUE);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the fixed names of winusb.spec. The code and
    // flags are the same for any names.
    let other = wine_spec_unimplemented_stub("other.dll", "Foo");
    assert_eq!(other.module, "other.dll");
    assert_eq!(other.function, "Foo");
    assert_eq!(other.code, EXCEPTION_WINE_STUB);
    assert_eq!(other.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(other.number_parameters, 2);

    let empty = wine_spec_unimplemented_stub("", "");
    assert_eq!(empty.module, "");
    assert_eq!(empty.function, "");
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the 22 lines of dlls/winusb/winusb.spec, in order.
    assert_eq!(EXPORTS.len(), 22);
    assert_eq!(EXPORTS[0], ("WinUsb_AbortPipe", ExportKind::Stub));
    assert_eq!(EXPORTS[3], ("WinUsb_Free", ExportKind::Stdcall));
    assert_eq!(EXPORTS[21], ("WinUsb_WritePipe", ExportKind::Stub));
    assert_eq!(export_kind("WinUsb_Free"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("WinUsb_ReadPipe"), Some(ExportKind::Stub));
    assert_eq!(export_kind("winusb_free"), None);
    assert_eq!(export_kind("WinUsb_Nope"), None);
    assert_eq!(export_kind(""), None);
}

#[test]
fn stub_table_is_the_stub_exports_in_spec_order() {
    // Not oracle-derived: STUB_EXPORTS must list exactly the @ stub lines of EXPORTS, in order,
    // and each helper must raise the record for its own export name.
    let stub_names: Vec<&str> = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .map(|(name, _)| *name)
        .collect();
    assert_eq!(stub_names.len(), STUB_EXPORTS.len());
    for ((name, helper), expected) in STUB_EXPORTS.iter().zip(stub_names) {
        assert_eq!(*name, expected);
        assert_eq!(helper().function, expected);
        assert_eq!(helper().module, WINUSB_DLL_NAME);
    }
}
