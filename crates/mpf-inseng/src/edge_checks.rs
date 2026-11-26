//! Edge-case tests for the ported inseng logic.
//!
//! These cover the GUID comparison, the interface and reference-count branches of the
//! `InstallEngine` object, the order of checks in the factory and `DllGetClassObject`, the export
//! table, and the stub pass-through. Not every expectation is oracle-derived; the ones that
//! aren't say so in a comment.

use super::*;

const CLSID_UNKNOWN: Guid = Guid {
    data1: 0,
    data2: 0,
    data3: 0,
    data4: [0; 8],
};

/// An object made the way `InstallEngineCF_CreateInstance` makes it, with a count of 1.
fn new_engine() -> InstallEngine {
    class_factory_create_instance(&IID_INSTALL_ENGINE2, true).unwrap()
}

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/windef.h, include/winternl.h
    // and include/winnt.h.
    assert_eq!(S_OK, 0);
    assert_eq!(S_FALSE, 1);
    assert_eq!(E_NOINTERFACE, 0x8000_4002);
    assert_eq!(E_NOTIMPL, 0x8000_4001);
    assert_eq!(E_OUTOFMEMORY, 0x8007_000E);
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE, 0x8004_0111);
    assert_eq!(TRUE, 1);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn guid_constants_match_headers() {
    // Not oracle-derived: values from include/unknwn.h and include/inseng.idl.
    assert!(
        IID_UNKNOWN
            == Guid {
                data1: 0,
                data2: 0,
                data3: 0,
                data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46],
            }
    );
    assert!(
        IID_CLASS_FACTORY
            == Guid {
                data1: 1,
                data2: 0,
                data3: 0,
                data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46],
            }
    );
    assert!(
        IID_INSTALL_ENGINE
            == Guid {
                data1: 0x6e44_9684,
                data2: 0xc509,
                data3: 0x11cf,
                data4: [0xaa, 0xfa, 0x00, 0xaa, 0x00, 0xb6, 0x01, 0x5c],
            }
    );
    assert!(
        IID_INSTALL_ENGINE2
            == Guid {
                data1: 0x6e44_9689,
                data2: 0xc509,
                data3: 0x11cf,
                data4: [0xaa, 0xfa, 0x00, 0xaa, 0x00, 0xb6, 0x01, 0x5c],
            }
    );
    assert!(
        CLSID_INSTALL_ENGINE
            == Guid {
                data1: 0x6e44_9686,
                data2: 0xc509,
                data3: 0x11cf,
                data4: [0xaa, 0xfa, 0x00, 0xaa, 0x00, 0xb6, 0x01, 0x5c],
            }
    );
}

#[test]
fn is_equal_guid_compares_every_field() {
    // Not oracle-derived: IsEqualGUID compares all 16 bytes, so each field must matter on its own.
    let base = Guid {
        data1: 1,
        data2: 2,
        data3: 3,
        data4: [4; 8],
    };
    let mut last_byte = [4u8; 8];
    last_byte[7] = 5;
    assert!(is_equal_guid(
        &base,
        &Guid {
            data1: 1,
            data2: 2,
            data3: 3,
            data4: [4; 8],
        }
    ));
    assert!(!is_equal_guid(
        &base,
        &Guid {
            data1: 9,
            data2: 2,
            data3: 3,
            data4: [4; 8],
        }
    ));
    assert!(!is_equal_guid(
        &base,
        &Guid {
            data1: 1,
            data2: 9,
            data3: 3,
            data4: [4; 8],
        }
    ));
    assert!(!is_equal_guid(
        &base,
        &Guid {
            data1: 1,
            data2: 2,
            data3: 9,
            data4: [4; 8],
        }
    ));
    assert!(!is_equal_guid(
        &base,
        &Guid {
            data1: 1,
            data2: 2,
            data3: 3,
            data4: last_byte,
        }
    ));
}

#[test]
fn query_interface_accepts_the_engine_iids_and_adds_a_reference() {
    // Not oracle-derived: row 21 covers IInstallEngine. IUnknown and IInstallEngine2 are the
    // other accepted branches in the C body, and the count is read from that body.
    for riid in [&IID_UNKNOWN, &IID_INSTALL_ENGINE, &IID_INSTALL_ENGINE2] {
        let mut engine = new_engine();
        assert!(engine.query_interface(riid).is_ok());
        assert_eq!(engine.ref_count(), 2);
    }
}

#[test]
fn query_interface_refusal_leaves_the_count_alone() {
    // Not oracle-derived: row 22 prints only ppv=NULL. The count is unchanged because the C body
    // returns E_NOINTERFACE before it calls AddRef.
    let mut engine = new_engine();
    assert_eq!(
        engine.query_interface(&CLSID_UNKNOWN).err(),
        Some(E_NOINTERFACE)
    );
    assert_eq!(
        engine.query_interface(&IID_CLASS_FACTORY).err(),
        Some(E_NOINTERFACE)
    );
    assert_eq!(engine.ref_count(), 1);
}

#[test]
fn release_reaches_zero_and_wraps_below_zero() {
    // Not oracle-derived: the C object is freed at 0 and never released again, so the wrap below
    // 0 only pins the Rust arithmetic.
    let mut engine = new_engine();
    assert_eq!(engine.release(), 0);
    assert_eq!(engine.ref_count(), 0);
    assert_eq!(engine.release(), u32::MAX);
}

#[test]
fn create_instance_checks_allocation_before_the_interface() {
    // Not oracle-derived: the C body returns E_OUTOFMEMORY before it looks at riid. Row 20 covers
    // IUnknown; IClassFactory would be refused by the interface check, so this shows the order.
    assert_eq!(
        class_factory_create_instance(&IID_CLASS_FACTORY, false).err(),
        Some(E_OUTOFMEMORY)
    );
}

#[test]
fn dll_get_class_object_checks_the_clsid_before_the_iid() {
    // Not oracle-derived: row 13 covers an unknown CLSID with IUnknown. Here the IID is one the
    // factory would refuse, and the CLSID check still decides the result.
    assert_eq!(
        dll_get_class_object(&CLSID_UNKNOWN, &IID_INSTALL_ENGINE).err(),
        Some(CLASS_E_CLASSNOTAVAILABLE)
    );
    // Not oracle-derived: with the known CLSID, the factory's interface check decides the result.
    assert_eq!(
        dll_get_class_object(&CLSID_INSTALL_ENGINE, &CLSID_UNKNOWN).err(),
        Some(E_NOINTERFACE)
    );
}

#[test]
fn export_table_is_in_spec_order() {
    // Not oracle-derived: the order is the order of the lines in dlls/inseng/inseng.spec.
    assert_eq!(EXPORTS.len(), 12);
    for pair in EXPORTS.windows(2) {
        assert!(pair[0].0 < pair[1].0);
    }
}

#[test]
fn export_kind_finds_each_name_and_rejects_others() {
    // Not oracle-derived: the kinds are read from dlls/inseng/inseng.spec.
    for (name, kind) in EXPORTS {
        assert_eq!(export_kind(name), Some(kind));
    }
    assert_eq!(export_kind("CheckTrustEx"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("DllInstall"), Some(ExportKind::Stdcall));
    assert_eq!(
        export_kind("DllCanUnloadNow"),
        Some(ExportKind::StdcallPrivate)
    );
    assert_eq!(
        export_kind("DllRegisterServer"),
        Some(ExportKind::StdcallPrivate)
    );
    assert_eq!(
        export_kind("PurgeDownloadDirectory"),
        Some(ExportKind::Stub)
    );
    assert_eq!(export_kind("checktrust"), None);
    assert_eq!(export_kind("CheckTrust "), None);
    assert_eq!(export_kind(""), None);
}

#[test]
fn wine_spec_unimplemented_stub_passes_names_through() {
    // Not oracle-derived: the oracle runs only inseng's six names. The port copies whatever
    // module and function it is given, as the C body does.
    assert_eq!(
        wine_spec_unimplemented_stub("other.dll", "Ordinal42"),
        StubException {
            code: 0x8000_0100,
            flags: 1,
            number_parameters: 2,
            module: "other.dll",
            function: "Ordinal42",
        }
    );
}
