//! Edge-case tests for the ported netcfgx logic.
//!
//! These cover the GUID and HRESULT values, the interface branches, the reference-count wrap,
//! the fixed return values of the methods the oracle does not call, and the export and stub
//! tables. Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;
use crate::com::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/winternl.h and include/winnt.h.
    assert_eq!(S_OK, 0);
    assert_eq!(E_NOTIMPL as u32, 0x8000_4001);
    assert_eq!(E_NOINTERFACE as u32, 0x8000_4002);
    assert_eq!(E_OUTOFMEMORY as u32, 0x8007_000e);
    assert_eq!(CLASS_E_NOAGGREGATION as u32, 0x8004_0110);
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE as u32, 0x8004_0111);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn guids_are_pinned() {
    // Not oracle-derived: INetCfg, INetCfgLock and CNetCfg from include/netcfgx.idl, and the
    // standard IUnknown and IClassFactory IIDs.
    assert_eq!(
        IID_INETCFG,
        Guid {
            data1: 0xc0e8_ae93,
            data2: 0x306e,
            data3: 0x11d1,
            data4: [0xaa, 0xcf, 0x00, 0x80, 0x5f, 0xc1, 0x27, 0x0e],
        }
    );
    assert_eq!(
        IID_INETCFGLOCK,
        Guid {
            data1: 0xc0e8_ae9f,
            data2: 0x306e,
            data3: 0x11d1,
            data4: [0xaa, 0xcf, 0x00, 0x80, 0x5f, 0xc1, 0x27, 0x0e],
        }
    );
    assert_eq!(
        CLSID_CNETCFG,
        Guid {
            data1: 0x5b03_5261,
            data2: 0x40f9,
            data3: 0x11d1,
            data4: [0xaa, 0xec, 0x00, 0x80, 0x5f, 0xc1, 0x27, 0x0e],
        }
    );
    assert_eq!(
        IID_IUNKNOWN,
        Guid {
            data1: 0,
            data2: 0,
            data3: 0,
            data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46],
        }
    );
    assert_eq!(
        IID_ICLASSFACTORY,
        Guid {
            data1: 1,
            data2: 0,
            data3: 0,
            data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46],
        }
    );
}

#[test]
fn is_equal_guid_compares_every_field() {
    // Not oracle-derived: IsEqualGUID is true only when all four fields match.
    assert!(is_equal_guid(&IID_INETCFG, &IID_INETCFG));

    let mut near = IID_INETCFG;
    near.data1 ^= 1;
    assert!(!is_equal_guid(&IID_INETCFG, &near));

    let mut near = IID_INETCFG;
    near.data2 ^= 1;
    assert!(!is_equal_guid(&IID_INETCFG, &near));

    let mut near = IID_INETCFG;
    near.data3 ^= 1;
    assert!(!is_equal_guid(&IID_INETCFG, &near));

    let mut near = IID_INETCFG;
    near.data4[7] ^= 1;
    assert!(!is_equal_guid(&IID_INETCFG, &near));
}

#[test]
fn dll_get_class_object_checks_clsid_before_riid() {
    // Not oracle-derived: a matching CLSID with an interface the class factory does not serve
    // reaches netcfgcf_QueryInterface and fails with E_NOINTERFACE, leaving ppv unchanged.
    let mut out = Some(Iface::INetCfgLock);
    assert_eq!(
        dll_get_class_object(&CLSID_CNETCFG, &IID_INETCFG, &mut out),
        E_NOINTERFACE
    );
    assert_eq!(out, Some(Iface::INetCfgLock));
}

#[test]
fn class_factory_query_interface_accepts_iunknown_and_iclassfactory() {
    // Not oracle-derived: the IUnknown and IClassFactory branches of netcfgcf_QueryInterface
    // (netcfg.c lines 18-23). The oracle only drives the INetCfg rejection.
    let mut out = None;
    assert_eq!(ClassFactory.query_interface(&IID_IUNKNOWN, &mut out), S_OK);
    assert_eq!(out, Some(Iface::IClassFactory));

    let mut out = None;
    assert_eq!(
        ClassFactory.query_interface(&IID_ICLASSFACTORY, &mut out),
        S_OK
    );
    assert_eq!(out, Some(Iface::IClassFactory));
}

#[test]
fn class_factory_counts_and_lock_are_fixed() {
    // Not oracle-derived: netcfgcf_AddRef returns 2, netcfgcf_Release returns 1 and
    // netcfgcf_LockServer returns S_OK, whatever the arguments (netcfg.c lines 30-42 and 72-76).
    assert_eq!(ClassFactory.add_ref(), 2);
    assert_eq!(ClassFactory.release(), 1);
    assert_eq!(ClassFactory.lock_server(true), S_OK);
    assert_eq!(ClassFactory.lock_server(false), S_OK);
}

#[test]
fn class_factory_checks_p_outer_before_allocating() {
    // Not oracle-derived: netcfgcf_CreateInstance returns CLASS_E_NOAGGREGATION before it calls
    // INetCfg_CreateInstance, so a failing allocation does not change the result.
    let mut out = Some(Iface::INetCfg);
    assert_eq!(
        ClassFactory.create_instance(true, &IID_INETCFG, &mut out, false),
        CLASS_E_NOAGGREGATION
    );
    assert_eq!(out, None);
}

#[test]
fn net_cfg_fixed_methods_return_their_c_values() {
    // Not oracle-derived: the fixed return values of netcfg.c. The oracle does not call these
    // methods. None of them changes the reference count.
    let obj = inet_cfg_create_instance(true).unwrap();
    assert_eq!(obj.initialize(), S_OK);
    assert_eq!(obj.uninitialize(), S_OK);
    assert_eq!(obj.apply(), E_NOTIMPL);
    assert_eq!(obj.cancel(), E_NOTIMPL);
    assert_eq!(obj.enum_components(), E_NOTIMPL);
    assert_eq!(obj.find_component(), E_NOTIMPL);
    assert_eq!(obj.query_net_cfg_class(), E_NOTIMPL);
    assert_eq!(obj.acquire_write_lock(), S_OK);
    assert_eq!(obj.release_write_lock(), S_OK);
    assert_eq!(obj.is_write_locked(), E_NOTIMPL);
    assert_eq!(obj.ref_count, 1);
}

#[test]
fn ref_count_wraps_like_interlocked_counters() {
    // Not oracle-derived: InterlockedIncrement and InterlockedDecrement wrap the signed LONG, and
    // netcfg_AddRef and netcfg_Release return it as a ULONG.
    let mut obj = NetConfiguration {
        ref_count: i32::MAX,
    };
    assert_eq!(obj.add_ref(), 0x8000_0000);
    assert_eq!(obj.ref_count, i32::MIN);

    let mut obj = NetConfiguration { ref_count: 0 };
    assert_eq!(obj.release(), 0xffff_ffff);
    assert_eq!(obj.ref_count, -1);
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the sixteen lines of dlls/netcfgx/netcfgx.spec, in order.
    assert_eq!(
        EXPORTS,
        [
            ("DllCanUnloadNow", ExportKind::WinecrtDefault),
            ("DllGetClassObject", ExportKind::Ported),
            ("DllRegisterServer", ExportKind::WinecrtDefault),
            ("DllUnregisterServer", ExportKind::WinecrtDefault),
            ("HrDiAddComponentToINetCfg", ExportKind::Stub),
            ("LanaCfgFromCommandArgs", ExportKind::Stub),
            ("ModemClassCoInstaller", ExportKind::Stub),
            ("NetCfgDiagFromCommandArgs", ExportKind::Stub),
            ("NetCfgDiagRepairRegistryBindings", ExportKind::Stub),
            ("NetClassInstaller", ExportKind::Stub),
            ("NetPropPageProvider", ExportKind::Stub),
            ("RasAddBindings", ExportKind::Stub),
            ("RasCountBindings", ExportKind::Stub),
            ("RasRemoveBindings", ExportKind::Stub),
            ("SvchostChangeSvchostGroup", ExportKind::Stub),
            ("UpdateLanaConfigUsingAnswerfile", ExportKind::Stub),
        ]
    );
    assert_eq!(export_kind("DllGetClassObject"), Some(ExportKind::Ported));
    assert_eq!(
        export_kind("DllCanUnloadNow"),
        Some(ExportKind::WinecrtDefault)
    );
    assert_eq!(export_kind("NetClassInstaller"), Some(ExportKind::Stub));
    assert_eq!(export_kind("netclassinstaller"), None);
    assert_eq!(export_kind(""), None);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the fixed names of netcfgx.spec. The code and
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
