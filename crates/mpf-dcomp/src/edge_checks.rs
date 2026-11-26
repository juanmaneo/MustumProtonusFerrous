//! Edge-case tests for the ported dcomp logic.
//!
//! Expectations marked "not oracle-derived" come from reading `dcomp.spec` or
//! `dlls/ntdll/exception.c` rather than from the captured harness rows.

use super::*;

#[test]
fn create_device_ignores_every_argument_combination() {
    // Not oracle-derived: the C body never inspects its arguments, so every
    // combination of NULL and non-NULL should give E_NOTIMPL.
    let iid = [0xffu8; 16];
    for dev in [None, Some(0usize), Some(usize::MAX)] {
        for guid in [None, Some(&iid)] {
            let mut out = Some(0xbeefusize);
            assert_eq!(
                dcomposition_create_device(dev, guid, Some(&mut out)),
                E_NOTIMPL
            );
            assert_eq!(out, Some(0xbeef));
            assert_eq!(
                dcomposition_create_device2(dev, guid, Some(&mut out)),
                E_NOTIMPL
            );
            assert_eq!(out, Some(0xbeef));
            assert_eq!(
                dcomposition_create_device3(dev, guid, Some(&mut out)),
                E_NOTIMPL
            );
            assert_eq!(out, Some(0xbeef));
        }
    }
}

#[test]
fn e_notimpl_is_negative_hresult() {
    // Not oracle-derived: sign of the HRESULT reinterpretation of 0x80004001.
    assert_eq!(E_NOTIMPL, -2_147_467_263);
}

#[test]
fn spec_stub_exception_record() {
    // Not oracle-derived: fields come from `__wine_spec_unimplemented_stub` in
    // dlls/ntdll/exception.c (EXCEPTION_NONCONTINUABLE, 2 parameters).
    assert_eq!(
        unimplemented_stub_exception(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
        }
    );
}

#[test]
fn spec_stub_names_are_recognised() {
    // Not oracle-derived: names copied from `dcomp.spec` `stub` lines.
    for name in SPEC_STUBS {
        assert!(is_spec_stub(name), "{name}");
    }
    assert_eq!(SPEC_STUBS.len(), 12);
}

#[test]
fn non_stub_names_are_rejected() {
    // DCompositionCreateDevice is implemented in device.c, not a stub.
    assert!(!is_spec_stub("DCompositionCreateDevice"));
    // DllCanUnloadNow is -private with no source here.
    assert!(!is_spec_stub("DllCanUnloadNow"));
    assert!(!is_spec_stub(""));
    // Case differs from the spec entry, so no match.
    assert!(!is_spec_stub("dwmflush"));
}
