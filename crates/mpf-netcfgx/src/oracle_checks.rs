//! Oracle-derived test vectors for the ported netcfgx logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `DllGetClassObject`, the
//! `netcfgcf` class factory and the `CNetCfg` object (`dlls/netcfgx/netcfg.c`), and of
//! `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`), with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/netcfgx/`, and running the harness.
//! The harness printed 25 rows. ASan and UBSan flagged none of them, so none are excluded.
//!
//! The C harness starts each out-parameter at a non-NULL sentinel. These tests start at
//! `sentinel()`, `Some(Iface::INetCfgLock)`, which no failing path in these rows writes.

use super::*;
use crate::com::*;

fn sentinel() -> Option<Iface> {
    Some(Iface::INetCfgLock)
}

#[test]
fn oracle_dll_get_class_object_rows() {
    // row 01 DllGetClassObject(CLSID_CNetCfg, IID_IClassFactory) hr=0x00000000 out=IClassFactory
    let mut out = sentinel();
    assert_eq!(
        dll_get_class_object(&CLSID_CNETCFG, &IID_ICLASSFACTORY, &mut out),
        S_OK
    );
    assert_eq!(out, Some(Iface::IClassFactory));

    // row 02 DllGetClassObject(other CLSID, IID_IClassFactory) hr=0x80040111 out=SENT
    let mut out = sentinel();
    assert_eq!(
        dll_get_class_object(&IID_IUNKNOWN, &IID_ICLASSFACTORY, &mut out),
        CLASS_E_CLASSNOTAVAILABLE
    );
    assert_eq!(out, sentinel());
}

#[test]
fn oracle_stub_rows() {
    // row 03 HrDiAddComponentToINetCfg: code=0x80000100 flags=0x1 params=2
    assert_eq!(
        hr_di_add_component_to_inet_cfg(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "netcfgx.dll",
            function: "HrDiAddComponentToINetCfg",
        }
    );

    // row 04 LanaCfgFromCommandArgs: code=0x80000100 flags=0x1 params=2
    assert_eq!(
        lana_cfg_from_command_args(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "netcfgx.dll",
            function: "LanaCfgFromCommandArgs",
        }
    );

    // row 05 ModemClassCoInstaller: code=0x80000100 flags=0x1 params=2
    assert_eq!(
        modem_class_co_installer(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "netcfgx.dll",
            function: "ModemClassCoInstaller",
        }
    );

    // row 06 NetCfgDiagFromCommandArgs: code=0x80000100 flags=0x1 params=2
    assert_eq!(
        net_cfg_diag_from_command_args(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "netcfgx.dll",
            function: "NetCfgDiagFromCommandArgs",
        }
    );

    // row 07 NetCfgDiagRepairRegistryBindings: code=0x80000100 flags=0x1 params=2
    assert_eq!(
        net_cfg_diag_repair_registry_bindings(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "netcfgx.dll",
            function: "NetCfgDiagRepairRegistryBindings",
        }
    );

    // row 08 NetClassInstaller: code=0x80000100 flags=0x1 params=2
    assert_eq!(
        net_class_installer(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "netcfgx.dll",
            function: "NetClassInstaller",
        }
    );

    // row 09 NetPropPageProvider: code=0x80000100 flags=0x1 params=2
    assert_eq!(
        net_prop_page_provider(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "netcfgx.dll",
            function: "NetPropPageProvider",
        }
    );

    // row 10 RasAddBindings: code=0x80000100 flags=0x1 params=2
    assert_eq!(
        ras_add_bindings(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "netcfgx.dll",
            function: "RasAddBindings",
        }
    );

    // row 11 RasCountBindings: code=0x80000100 flags=0x1 params=2
    assert_eq!(
        ras_count_bindings(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "netcfgx.dll",
            function: "RasCountBindings",
        }
    );

    // row 12 RasRemoveBindings: code=0x80000100 flags=0x1 params=2
    assert_eq!(
        ras_remove_bindings(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "netcfgx.dll",
            function: "RasRemoveBindings",
        }
    );

    // row 13 SvchostChangeSvchostGroup: code=0x80000100 flags=0x1 params=2
    assert_eq!(
        svchost_change_svchost_group(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "netcfgx.dll",
            function: "SvchostChangeSvchostGroup",
        }
    );

    // row 14 UpdateLanaConfigUsingAnswerfile: code=0x80000100 flags=0x1 params=2
    assert_eq!(
        update_lana_config_using_answerfile(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "netcfgx.dll",
            function: "UpdateLanaConfigUsingAnswerfile",
        }
    );
}

#[test]
fn oracle_class_factory_query_interface_row() {
    // row 15 netcfgcf_QueryInterface(IID_INetCfg) hr=0x80004002 out=SENT
    let mut out = sentinel();
    assert_eq!(
        ClassFactory.query_interface(&IID_INETCFG, &mut out),
        E_NOINTERFACE
    );
    assert_eq!(out, sentinel());
}

#[test]
fn oracle_class_factory_create_instance_rows() {
    // row 16 netcfgcf_CreateInstance(pOuter, IID_INetCfg) hr=0x80040110 out=NULL allocs=0 frees=0
    let mut out = sentinel();
    assert_eq!(
        ClassFactory.create_instance(true, &IID_INETCFG, &mut out, true),
        CLASS_E_NOAGGREGATION
    );
    assert_eq!(out, None);

    // row 17 netcfgcf_CreateInstance(IID_INetCfg) hr=0x00000000 out=INetCfg ref=1 allocs=1 frees=0
    // The C harness reads ref=1 from the object it gets back. The Rust object is dropped inside
    // create_instance, so its count is checked by row 25 instead.
    let mut out = sentinel();
    assert_eq!(
        ClassFactory.create_instance(false, &IID_INETCFG, &mut out, true),
        S_OK
    );
    assert_eq!(out, Some(Iface::INetCfg));

    // row 18 netcfgcf_CreateInstance(IID_IClassFactory) hr=0x80004002 out=NULL allocs=1 frees=1
    let mut out = sentinel();
    assert_eq!(
        ClassFactory.create_instance(false, &IID_ICLASSFACTORY, &mut out, true),
        E_NOINTERFACE
    );
    assert_eq!(out, None);

    // row 19 netcfgcf_CreateInstance(IID_INetCfg, HeapAlloc fails) hr=0x8007000e out=NULL
    let mut out = sentinel();
    assert_eq!(
        ClassFactory.create_instance(false, &IID_INETCFG, &mut out, false),
        E_OUTOFMEMORY
    );
    assert_eq!(out, None);
}

#[test]
fn oracle_net_cfg_rows() {
    // row 20 netcfg QueryInterface(IID_IUnknown) hr=0x00000000 out=INetCfg ref=2
    let mut obj = inet_cfg_create_instance(true).unwrap();
    let mut out = sentinel();
    assert_eq!(obj.query_interface(&IID_IUNKNOWN, &mut out), S_OK);
    assert_eq!(out, Some(Iface::INetCfg));
    assert_eq!(obj.ref_count, 2);
    assert_eq!(obj.release(), 1);
    assert_eq!(obj.release(), 0);

    // row 21 netcfg QueryInterface(IID_INetCfgLock) hr=0x00000000 out=INetCfgLock ref=2
    let mut obj = inet_cfg_create_instance(true).unwrap();
    let mut out = sentinel();
    assert_eq!(obj.query_interface(&IID_INETCFGLOCK, &mut out), S_OK);
    assert_eq!(out, Some(Iface::INetCfgLock));
    assert_eq!(obj.ref_count, 2);
    assert_eq!(obj.release(), 1);
    assert_eq!(obj.release(), 0);

    // row 22 netcfg QueryInterface(IID_IClassFactory) hr=0x80004002 out=SENT ref=1
    let mut obj = inet_cfg_create_instance(true).unwrap();
    let mut out = sentinel();
    assert_eq!(
        obj.query_interface(&IID_ICLASSFACTORY, &mut out),
        E_NOINTERFACE
    );
    assert_eq!(out, sentinel());
    assert_eq!(obj.ref_count, 1);

    // row 23 netcfg Release(ref=1) ret=0 frees=1. The C code frees the object here; Rust drops it.
    assert_eq!(obj.release(), 0);
}

#[test]
fn oracle_inet_cfg_create_instance_rows() {
    // row 24 INetCfg_CreateInstance(HeapAlloc fails) hr=0x8007000e punk=SENT. The Result form has
    // no out-pointer to write on failure, so the SENT value of the C harness has no counterpart.
    assert_eq!(inet_cfg_create_instance(false).err(), Some(E_OUTOFMEMORY));

    // row 25 INetCfg_CreateInstance hr=0x00000000 punk=INetCfg ref=1
    let obj = inet_cfg_create_instance(true).unwrap();
    assert_eq!(obj.ref_count, 1);
}
