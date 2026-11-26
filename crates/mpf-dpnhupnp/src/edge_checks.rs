//! Edge-case tests for the `dpnhupnp` port.
//!
//! Not every expectation here is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn hresult_constants_match_winerror() {
    // Not oracle-derived: values from winerror.h (E_NOTIMPL, E_FAIL).
    assert_eq!(E_NOTIMPL, -2147467263);
    assert_eq!(E_FAIL, -2147467259);
}

#[test]
fn stubs_ignore_all_arguments() {
    // Not oracle-derived: the oracle only covered NULL and non-NULL pointers with a
    // fixed GUID. These cases check that the result does not depend on the GUID bytes.
    let a = Guid {
        data1: 0,
        data2: 0,
        data3: 0,
        data4: [0; 8],
    };
    let b = Guid {
        data1: u32::MAX,
        data2: u16::MAX,
        data3: u16::MAX,
        data4: [0xff; 8],
    };
    let mut slot = 7usize;
    assert_eq!(
        direct_play_nat_help_create(Some(&a), Some(&mut slot)),
        E_NOTIMPL
    );
    assert_eq!(direct_play_nat_help_create(Some(&b), None), E_NOTIMPL);
    assert_eq!(direct_play_nat_help_create(None, None), E_NOTIMPL);
    assert_eq!(slot, 7);
    assert_eq!(
        dll_get_class_object(Some(&a), Some(&b), Some(&mut slot)),
        E_FAIL
    );
    assert_eq!(dll_get_class_object(None, None, None), E_FAIL);
    assert_eq!(slot, 7);
}

#[test]
fn stdcall_arg_bytes_for_each_export() {
    // Not oracle-derived: byte counts come from dpnhupnp.spec.
    assert_eq!(stdcall_arg_bytes("DirectPlayNATHelpCreate"), Some(8));
    assert_eq!(stdcall_arg_bytes("DllGetClassObject"), Some(12));
    assert_eq!(stdcall_arg_bytes("DllCanUnloadNow"), Some(0));
    assert_eq!(stdcall_arg_bytes("DllRegisterServer"), Some(0));
    assert_eq!(stdcall_arg_bytes("DllUnregisterServer"), Some(0));
}

#[test]
fn stdcall_arg_bytes_rejects_unknown_and_case_mismatch() {
    // Not oracle-derived: lookup is exact and case-sensitive.
    assert_eq!(stdcall_arg_bytes("dllgetclassobject"), None);
    assert_eq!(stdcall_arg_bytes(""), None);
    assert_eq!(stdcall_arg_bytes("DllNotExported"), None);
    assert_eq!(EXPORTS.len(), 5);
}
