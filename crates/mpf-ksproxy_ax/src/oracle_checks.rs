//! Oracle-derived test vectors for the ported ksproxy.ax stub records.
//!
//! Vectors were captured by compiling the verbatim Wine C body of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/ksproxy.ax/stub_harness.c`, and running the harness. It printed 17 rows: rows
//! 01-07 are the `@ stub` exports of `ksproxy.ax.spec`, and rows 08-17 are other inputs to the same
//! body. ASan and UBSan reported nothing, so no vector is excluded.

use super::*;

#[test]
fn oracle_dll_get_class_object_row() {
    // row 01 DllGetClassObject -> code=0x80000100 flags=0x1 nargs=2 p0=ksproxy.ax p1=DllGetClassObject
    assert_eq!(
        dll_get_class_object(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "ksproxy.ax",
            function: "DllGetClassObject",
        }
    );
}

#[test]
fn oracle_ks_get_media_type_row() {
    // row 02 KsGetMediaType -> code=0x80000100 flags=0x1 nargs=2 p0=ksproxy.ax p1=KsGetMediaType
    assert_eq!(
        ks_get_media_type(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "ksproxy.ax",
            function: "KsGetMediaType",
        }
    );
}

#[test]
fn oracle_ks_get_media_type_count_row() {
    // row 03 KsGetMediaTypeCount -> code=0x80000100 flags=0x1 nargs=2 p0=ksproxy.ax p1=KsGetMediaTypeCount
    assert_eq!(
        ks_get_media_type_count(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "ksproxy.ax",
            function: "KsGetMediaTypeCount",
        }
    );
}

#[test]
fn oracle_ks_get_multiple_pin_factory_items_row() {
    // row 04 KsGetMultiplePinFactoryItems -> code=0x80000100 flags=0x1 nargs=2 p0=ksproxy.ax p1=KsGetMultiplePinFactoryItems
    assert_eq!(
        ks_get_multiple_pin_factory_items(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "ksproxy.ax",
            function: "KsGetMultiplePinFactoryItems",
        }
    );
}

#[test]
fn oracle_ks_open_default_device_row() {
    // row 05 KsOpenDefaultDevice -> code=0x80000100 flags=0x1 nargs=2 p0=ksproxy.ax p1=KsOpenDefaultDevice
    assert_eq!(
        ks_open_default_device(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "ksproxy.ax",
            function: "KsOpenDefaultDevice",
        }
    );
}

#[test]
fn oracle_ks_resolve_required_attributes_row() {
    // row 06 KsResolveRequiredAttributes -> code=0x80000100 flags=0x1 nargs=2 p0=ksproxy.ax p1=KsResolveRequiredAttributes
    assert_eq!(
        ks_resolve_required_attributes(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "ksproxy.ax",
            function: "KsResolveRequiredAttributes",
        }
    );
}

#[test]
fn oracle_ks_synchronous_device_control_row() {
    // row 07 KsSynchronousDeviceControl -> code=0x80000100 flags=0x1 nargs=2 p0=ksproxy.ax p1=KsSynchronousDeviceControl
    assert_eq!(
        ks_synchronous_device_control(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "ksproxy.ax",
            function: "KsSynchronousDeviceControl",
        }
    );
}

#[test]
fn oracle_generic_stub_rows() {
    // rows 08-17: other inputs to the verbatim body. Every row raises the same code, flags and
    // parameter count, and echoes both names back unchanged.
    let rows: [(&'static str, &'static str, u32, u32, u32); 10] = [
        // row 08
        ("", "", 0x8000_0100, 0x01, 2),
        // row 09
        ("ksproxy.ax", "", 0x8000_0100, 0x01, 2),
        // row 10
        ("", "KsGetMediaType", 0x8000_0100, 0x01, 2),
        // row 11
        ("ksproxy.ax", "DllCanUnloadNow", 0x8000_0100, 0x01, 2),
        // row 12
        ("ksproxy.ax", "DllRegisterServer", 0x8000_0100, 0x01, 2),
        // row 13
        ("ksproxy.ax", "DllUnregisterServer", 0x8000_0100, 0x01, 2),
        // row 14
        ("KSPROXY.AX", "KsGetMediaType", 0x8000_0100, 0x01, 2),
        // row 15
        ("ksproxy.ax", "ksgetmediatype", 0x8000_0100, 0x01, 2),
        // row 16
        ("amsi.dll", "AmsiInitialize", 0x8000_0100, 0x01, 2),
        // row 17
        (
            "ksproxy.ax",
            "KsResolveRequiredAttributesWithAVeryLongNameToCheckNothingIsTruncated",
            0x8000_0100,
            0x01,
            2,
        ),
    ];
    for (module, function, code, flags, number_parameters) in rows {
        assert_eq!(
            wine_spec_unimplemented_stub(module, function),
            StubException {
                code,
                flags,
                number_parameters,
                module,
                function,
            }
        );
    }
}
