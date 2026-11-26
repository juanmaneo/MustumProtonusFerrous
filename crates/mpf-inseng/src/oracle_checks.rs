//! Oracle-derived test vectors for the ported inseng logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `inseng_main.c`
//! (`DllGetClassObject`, the class factory, the `InstallEngine` object, `CheckTrustEx`,
//! `DllInstall` and the `E_NOTIMPL` methods), `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`) and `DllCanUnloadNow` (`libs/winecrt0/dll_canunload.c`) with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/inseng/`. The harness printed 48
//! rows and ASan and UBSan flagged none, so no vector is excluded.
//!
//! The C `ppv` out-pointer is not modelled, so the `ppv=` notes in the row comments are the
//! reference values only; the assertions check the returned `HRESULT` or `Result`.

use super::*;

const CLSID_UNKNOWN: Guid = Guid {
    data1: 0,
    data2: 0,
    data3: 0,
    data4: [0; 8],
};

/// The HRESULT of a `Result` from the ported functions: `Ok` is `S_OK`, and `Err` carries its code.
fn hr<T>(result: &Result<T, HResult>) -> HResult {
    match result {
        Ok(_) => S_OK,
        Err(hres) => *hres,
    }
}

#[test]
fn oracle_stub_rows() {
    // row 01 CheckForVersionConflict -> code=0x80000100 flags=0x1 nargs=2 module=inseng.dll function=CheckForVersionConflict
    assert_eq!(
        check_for_version_conflict(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "inseng.dll",
            function: "CheckForVersionConflict",
        }
    );
    // row 02 CheckTrust -> code=0x80000100 flags=0x1 nargs=2 module=inseng.dll function=CheckTrust
    assert_eq!(
        check_trust(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "inseng.dll",
            function: "CheckTrust",
        }
    );
    // row 03 DownloadFile -> code=0x80000100 flags=0x1 nargs=2 module=inseng.dll function=DownloadFile
    assert_eq!(
        download_file(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "inseng.dll",
            function: "DownloadFile",
        }
    );
    // row 04 GetICifFileFromFile -> code=0x80000100 flags=0x1 nargs=2 module=inseng.dll function=GetICifFileFromFile
    assert_eq!(
        get_icif_file_from_file(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "inseng.dll",
            function: "GetICifFileFromFile",
        }
    );
    // row 05 GetICifRWFileFromFile -> code=0x80000100 flags=0x1 nargs=2 module=inseng.dll function=GetICifRWFileFromFile
    assert_eq!(
        get_icif_rw_file_from_file(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "inseng.dll",
            function: "GetICifRWFileFromFile",
        }
    );
    // row 06 PurgeDownloadDirectory -> code=0x80000100 flags=0x1 nargs=2 module=inseng.dll function=PurgeDownloadDirectory
    assert_eq!(
        purge_download_directory(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "inseng.dll",
            function: "PurgeDownloadDirectory",
        }
    );
}

#[test]
fn oracle_export_rows() {
    // row 07 CheckTrustEx(NULL x5) -> 1
    assert_eq!(check_trust_ex(), 1);
    // row 08 DllInstall(TRUE, "cmd") -> 0x00000000
    assert_eq!(dll_install(), 0);
    // row 09 DllCanUnloadNow() -> 0x00000001
    assert_eq!(dll_can_unload_now(), 1);
}

#[test]
fn oracle_dll_get_class_object_rows() {
    // row 10 DllGetClassObject(InstallEngine, IUnknown) -> 0x00000000 ppv=factory
    assert_eq!(
        hr(&dll_get_class_object(&CLSID_INSTALL_ENGINE, &IID_UNKNOWN)),
        0
    );
    // row 11 DllGetClassObject(InstallEngine, IClassFactory) -> 0x00000000 ppv=factory
    assert_eq!(
        hr(&dll_get_class_object(
            &CLSID_INSTALL_ENGINE,
            &IID_CLASS_FACTORY
        )),
        0
    );
    // row 12 DllGetClassObject(InstallEngine, IInstallEngine) -> 0x80004002 ppv=NULL
    assert_eq!(
        hr(&dll_get_class_object(
            &CLSID_INSTALL_ENGINE,
            &IID_INSTALL_ENGINE
        )),
        0x8000_4002
    );
    // row 13 DllGetClassObject(unknown CLSID, IUnknown) -> 0x80040111 ppv=unchanged
    assert_eq!(
        hr(&dll_get_class_object(&CLSID_UNKNOWN, &IID_UNKNOWN)),
        0x8004_0111
    );
}

#[test]
fn oracle_class_factory_rows() {
    // row 14 ClassFactory AddRef -> 2
    assert_eq!(class_factory_add_ref(), 2);
    // row 15 ClassFactory Release -> 1
    assert_eq!(class_factory_release(), 1);
    // row 16 ClassFactory LockServer(TRUE) -> 0x00000000
    assert_eq!(class_factory_lock_server(true), 0);
    // row 17 ClassFactory LockServer(FALSE) -> 0x00000000
    assert_eq!(class_factory_lock_server(false), 0);
}

#[test]
fn oracle_create_instance_rows() {
    // row 18 CreateInstance(IInstallEngine2, alloc ok) -> 0x00000000 ppv=object ref=1
    let engine = class_factory_create_instance(&IID_INSTALL_ENGINE2, true);
    assert_eq!(hr(&engine), 0);
    assert_eq!(engine.map(|engine| engine.ref_count()), Ok(1));
    // row 19 CreateInstance(IClassFactory, alloc ok) -> 0x80004002 ppv=NULL
    assert_eq!(
        hr(&class_factory_create_instance(&IID_CLASS_FACTORY, true)),
        0x8000_4002
    );
    // row 20 CreateInstance(IUnknown, alloc fails) -> 0x8007000e ppv=unchanged
    assert_eq!(
        hr(&class_factory_create_instance(&IID_UNKNOWN, false)),
        0x8007_000E
    );
}

#[test]
fn oracle_install_engine_object_rows() {
    // The object that row 18 creates; rows 21-24 run on it in order.
    let mut engine = class_factory_create_instance(&IID_INSTALL_ENGINE2, true).unwrap();
    // row 21 engine QueryInterface(IInstallEngine) -> 0x00000000 ref=2
    assert_eq!(hr(&engine.query_interface(&IID_INSTALL_ENGINE)), 0);
    assert_eq!(engine.ref_count(), 2);
    // row 22 engine QueryInterface(IClassFactory) -> 0x80004002 ppv=NULL
    assert_eq!(hr(&engine.query_interface(&IID_CLASS_FACTORY)), 0x8000_4002);
    // row 23 engine AddRef -> 3
    assert_eq!(engine.add_ref(), 3);
    // row 24 engine Release -> 2
    assert_eq!(engine.release(), 2);
}

#[test]
fn oracle_not_implemented_rows() {
    // row 25 GetEngineStatus -> 0x80004001
    assert_eq!(install_engine_get_engine_status(), 0x8000_4001);
    // row 26 SetCifFile -> 0x80004001
    assert_eq!(install_engine_set_cif_file(), 0x8000_4001);
    // row 27 DownloadComponents -> 0x80004001
    assert_eq!(install_engine_download_components(), 0x8000_4001);
    // row 28 InstallComponents -> 0x80004001
    assert_eq!(install_engine_install_components(), 0x8000_4001);
    // row 29 EnumInstallIDs -> 0x80004001
    assert_eq!(install_engine_enum_install_ids(), 0x8000_4001);
    // row 30 EnumDownloadIDs -> 0x80004001
    assert_eq!(install_engine_enum_download_ids(), 0x8000_4001);
    // row 31 IsComponentInstalled -> 0x80004001
    assert_eq!(install_engine_is_component_installed(), 0x8000_4001);
    // row 32 RegisterInstallEngineCallback -> 0x80004001
    assert_eq!(
        install_engine_register_install_engine_callback(),
        0x8000_4001
    );
    // row 33 UnregisterInstallEngineCallback -> 0x80004001
    assert_eq!(
        install_engine_unregister_install_engine_callback(),
        0x8000_4001
    );
    // row 34 SetAction -> 0x80004001
    assert_eq!(install_engine_set_action(), 0x8000_4001);
    // row 35 GetSizes -> 0x80004001
    assert_eq!(install_engine_get_sizes(), 0x8000_4001);
    // row 36 LaunchExtraCommand -> 0x80004001
    assert_eq!(install_engine_launch_extra_command(), 0x8000_4001);
    // row 37 GetDisplayName -> 0x80004001
    assert_eq!(install_engine_get_display_name(), 0x8000_4001);
    // row 38 SetBaseUrl -> 0x80004001
    assert_eq!(install_engine_set_base_url(), 0x8000_4001);
    // row 39 SetDownloadDir -> 0x80004001
    assert_eq!(install_engine_set_download_dir(), 0x8000_4001);
    // row 40 SetInstallDrive -> 0x80004001
    assert_eq!(install_engine_set_install_drive(), 0x8000_4001);
    // row 41 SetInstallOptions -> 0x80004001
    assert_eq!(install_engine_set_install_options(), 0x8000_4001);
    // row 42 SetHWND -> 0x80004001
    assert_eq!(install_engine_set_hwnd(), 0x8000_4001);
    // row 43 SetIStream -> 0x80004001
    assert_eq!(install_engine_set_istream(), 0x8000_4001);
    // row 44 Abort -> 0x80004001
    assert_eq!(install_engine_abort(), 0x8000_4001);
    // row 45 Suspend -> 0x80004001
    assert_eq!(install_engine_suspend(), 0x8000_4001);
    // row 46 Resume -> 0x80004001
    assert_eq!(install_engine_resume(), 0x8000_4001);
    // row 47 InstallEngine2 SetLocalCif -> 0x80004001
    assert_eq!(install_engine2_set_local_cif(), 0x8000_4001);
    // row 48 InstallEngine2 GetICifFile -> 0x80004001
    assert_eq!(install_engine2_get_icif_file(), 0x8000_4001);
}
