//! Oracle-derived test vectors for the ported loadperf logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of the `InstallPerfDll`,
//! `LoadPerfCounterTextStrings` and `UnloadPerfCounterTextStrings` A/W pairs
//! (`dlls/loadperf/loadperf_main.c`) and of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/loadperf/`, and running the harness. The 25 printed rows were all clean under
//! ASan and UBSan, so none is excluded.

use super::*;
use std::cell::Cell;

const COMP_A: &[u8] = b"comp";
const INI_A: &[u8] = b"ini";
const APP_A: &[u8] = b"app";

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

/// A converter that stands in for `MultiByteToWideChar` plus `HeapAlloc`. It widens each byte
/// and counts its calls in `calls`. Call number `fail_at` (1-based) returns `None`, as a failed
/// `HeapAlloc` does. A `fail_at` of 0 never fails.
fn converter(fail_at: usize, calls: &Cell<usize>) -> impl FnMut(&[u8]) -> Option<Vec<u16>> + '_ {
    move |s| {
        calls.set(calls.get() + 1);
        if calls.get() == fail_at {
            None
        } else {
            Some(s.iter().map(|&b| u16::from(b)).collect())
        }
    }
}

#[test]
fn oracle_install_perf_dll_rows() {
    let (comp, ini) = (wide("comp"), wide("ini"));

    // row 01 InstallPerfDllW computer=NULL ini=NULL flags=0 -> ret=0 allocs=0
    assert_eq!(install_perf_dll_w(None, None, 0), ERROR_SUCCESS);

    // row 02 InstallPerfDllW computer=comp ini=ini flags=7 -> ret=0 allocs=0
    assert_eq!(
        install_perf_dll_w(Some(comp.as_slice()), Some(ini.as_slice()), 7),
        ERROR_SUCCESS
    );

    // row 03 InstallPerfDllA computer=NULL ini=NULL flags=0 fail_at=0 -> ret=0 allocs=0
    let calls = Cell::new(0);
    assert_eq!(
        install_perf_dll_a(None, None, 0, converter(0, &calls)),
        ERROR_SUCCESS
    );
    assert_eq!(calls.get(), 0);

    // row 04 InstallPerfDllA computer=comp ini=ini flags=1 fail_at=0 -> ret=0 allocs=2
    let calls = Cell::new(0);
    assert_eq!(
        install_perf_dll_a(Some(COMP_A), Some(INI_A), 1, converter(0, &calls)),
        ERROR_SUCCESS
    );
    assert_eq!(calls.get(), 2);

    // row 05 InstallPerfDllA computer=comp ini=NULL flags=1 fail_at=1 -> ret=14 allocs=1
    let calls = Cell::new(0);
    assert_eq!(
        install_perf_dll_a(Some(COMP_A), None, 1, converter(1, &calls)),
        ERROR_OUTOFMEMORY
    );
    assert_eq!(calls.get(), 1);

    // row 06 InstallPerfDllA computer=comp ini=ini flags=1 fail_at=2 -> ret=14 allocs=2
    let calls = Cell::new(0);
    assert_eq!(
        install_perf_dll_a(Some(COMP_A), Some(INI_A), 1, converter(2, &calls)),
        ERROR_OUTOFMEMORY
    );
    assert_eq!(calls.get(), 2);

    // row 07 InstallPerfDllA computer=NULL ini=ini flags=1 fail_at=1 -> ret=14 allocs=1
    let calls = Cell::new(0);
    assert_eq!(
        install_perf_dll_a(None, Some(INI_A), 1, converter(1, &calls)),
        ERROR_OUTOFMEMORY
    );
    assert_eq!(calls.get(), 1);
}

#[test]
fn oracle_load_perf_counter_text_strings_rows() {
    let ini = wide("ini");

    // row 08 LoadPerfCounterTextStringsW cmdline=NULL quiet=0 -> ret=0 allocs=0
    assert_eq!(load_perf_counter_text_strings_w(None, false), ERROR_SUCCESS);

    // row 09 LoadPerfCounterTextStringsW cmdline=ini quiet=1 -> ret=0 allocs=0
    assert_eq!(
        load_perf_counter_text_strings_w(Some(ini.as_slice()), true),
        ERROR_SUCCESS
    );

    // row 10 LoadPerfCounterTextStringsA cmdline=NULL quiet=0 fail_at=0 -> ret=0 allocs=0
    let calls = Cell::new(0);
    assert_eq!(
        load_perf_counter_text_strings_a(None, false, converter(0, &calls)),
        ERROR_SUCCESS
    );
    assert_eq!(calls.get(), 0);

    // row 11 LoadPerfCounterTextStringsA cmdline=ini quiet=1 fail_at=0 -> ret=0 allocs=1
    let calls = Cell::new(0);
    assert_eq!(
        load_perf_counter_text_strings_a(Some(INI_A), true, converter(0, &calls)),
        ERROR_SUCCESS
    );
    assert_eq!(calls.get(), 1);

    // row 12 LoadPerfCounterTextStringsA cmdline=ini quiet=0 fail_at=1 -> ret=14 allocs=1
    let calls = Cell::new(0);
    assert_eq!(
        load_perf_counter_text_strings_a(Some(INI_A), false, converter(1, &calls)),
        ERROR_OUTOFMEMORY
    );
    assert_eq!(calls.get(), 1);
}

#[test]
fn oracle_unload_perf_counter_text_strings_rows() {
    let app = wide("app");

    // row 13 UnloadPerfCounterTextStringsW cmdline=NULL quiet=0 -> ret=0 allocs=0
    assert_eq!(
        unload_perf_counter_text_strings_w(None, false),
        ERROR_SUCCESS
    );

    // row 14 UnloadPerfCounterTextStringsW cmdline=app quiet=1 -> ret=0 allocs=0
    assert_eq!(
        unload_perf_counter_text_strings_w(Some(app.as_slice()), true),
        ERROR_SUCCESS
    );

    // row 15 UnloadPerfCounterTextStringsA cmdline=NULL quiet=1 fail_at=0 -> ret=0 allocs=0
    let calls = Cell::new(0);
    assert_eq!(
        unload_perf_counter_text_strings_a(None, true, converter(0, &calls)),
        ERROR_SUCCESS
    );
    assert_eq!(calls.get(), 0);

    // row 16 UnloadPerfCounterTextStringsA cmdline=app quiet=0 fail_at=0 -> ret=0 allocs=1
    let calls = Cell::new(0);
    assert_eq!(
        unload_perf_counter_text_strings_a(Some(APP_A), false, converter(0, &calls)),
        ERROR_SUCCESS
    );
    assert_eq!(calls.get(), 1);

    // row 17 UnloadPerfCounterTextStringsA cmdline=app quiet=1 fail_at=1 -> ret=14 allocs=1
    let calls = Cell::new(0);
    assert_eq!(
        unload_perf_counter_text_strings_a(Some(APP_A), true, converter(1, &calls)),
        ERROR_OUTOFMEMORY
    );
    assert_eq!(calls.get(), 1);
}

#[test]
fn oracle_stub_rows() {
    // row 18 stub BackupPerfRegistryToFileW -> code=0x80000100 flags=0x1 nparams=2 module=loadperf.dll function=BackupPerfRegistryToFileW
    assert_eq!(
        backup_perf_registry_to_file_w(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "loadperf.dll",
            function: "BackupPerfRegistryToFileW",
        }
    );

    // row 19 stub LoadMofFromInstalledServiceA -> code=0x80000100 flags=0x1 nparams=2 module=loadperf.dll function=LoadMofFromInstalledServiceA
    assert_eq!(
        load_mof_from_installed_service_a(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "loadperf.dll",
            function: "LoadMofFromInstalledServiceA",
        }
    );

    // row 20 stub LoadMofFromInstalledServiceW -> code=0x80000100 flags=0x1 nparams=2 module=loadperf.dll function=LoadMofFromInstalledServiceW
    assert_eq!(
        load_mof_from_installed_service_w(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "loadperf.dll",
            function: "LoadMofFromInstalledServiceW",
        }
    );

    // row 21 stub RestorePerfRegistryFromFileW -> code=0x80000100 flags=0x1 nparams=2 module=loadperf.dll function=RestorePerfRegistryFromFileW
    assert_eq!(
        restore_perf_registry_from_file_w(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "loadperf.dll",
            function: "RestorePerfRegistryFromFileW",
        }
    );

    // row 22 stub SetServiceAsTrustedA -> code=0x80000100 flags=0x1 nparams=2 module=loadperf.dll function=SetServiceAsTrustedA
    assert_eq!(
        set_service_as_trusted_a(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "loadperf.dll",
            function: "SetServiceAsTrustedA",
        }
    );

    // row 23 stub SetServiceAsTrustedW -> code=0x80000100 flags=0x1 nparams=2 module=loadperf.dll function=SetServiceAsTrustedW
    assert_eq!(
        set_service_as_trusted_w(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "loadperf.dll",
            function: "SetServiceAsTrustedW",
        }
    );

    // row 24 stub UpdatePerfNameFilesA -> code=0x80000100 flags=0x1 nparams=2 module=loadperf.dll function=UpdatePerfNameFilesA
    assert_eq!(
        update_perf_name_files_a(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "loadperf.dll",
            function: "UpdatePerfNameFilesA",
        }
    );

    // row 25 stub UpdatePerfNameFilesW -> code=0x80000100 flags=0x1 nparams=2 module=loadperf.dll function=UpdatePerfNameFilesW
    assert_eq!(
        update_perf_name_files_w(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "loadperf.dll",
            function: "UpdatePerfNameFilesW",
        }
    );
}
