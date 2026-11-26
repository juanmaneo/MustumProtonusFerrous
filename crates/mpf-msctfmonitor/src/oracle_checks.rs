//! Oracle-derived test vectors for the ported msctfmonitor logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `InitLocalMsCtfMonitor`
//! (`dlls/msctfmonitor/main.c`), `DllCanUnloadNow`, `DllRegisterServer` and `DllUnregisterServer`
//! (`libs/winecrt0/dll_canunload.c`, `libs/winecrt0/dll_register.c`, `libs/winecrt0/register.c`),
//! and `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`) with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/msctfmonitor/`. The harness replaces
//! `RtlRaiseException` with a recorder that longjmps out, because the stub loops on it. It shims
//! `EnumResourceNamesW` to enumerate no resources, which matches this module. The 16 printed rows
//! ran clean under ASan and UBSan, so no vector is excluded.

use super::*;

/// Rows 01-08: `InitLocalMsCtfMonitor(flags)` returns `E_FAIL` for every flags value.
#[test]
fn oracle_init_local_ms_ctf_monitor_rows() {
    // row 01 InitLocalMsCtfMonitor flags=0x00000000 -> hr=0x80004005
    // row 02 InitLocalMsCtfMonitor flags=0x00000001 -> hr=0x80004005
    // row 03 InitLocalMsCtfMonitor flags=0x00000002 -> hr=0x80004005
    // row 04 InitLocalMsCtfMonitor flags=0x00000010 -> hr=0x80004005
    // row 05 InitLocalMsCtfMonitor flags=0x00010000 -> hr=0x80004005
    // row 06 InitLocalMsCtfMonitor flags=0x7fffffff -> hr=0x80004005
    // row 07 InitLocalMsCtfMonitor flags=0x80000000 -> hr=0x80004005
    // row 08 InitLocalMsCtfMonitor flags=0xffffffff -> hr=0x80004005
    const ROWS: [(u32, u32); 8] = [
        (0x0000_0000, 0x8000_4005),
        (0x0000_0001, 0x8000_4005),
        (0x0000_0002, 0x8000_4005),
        (0x0000_0010, 0x8000_4005),
        (0x0001_0000, 0x8000_4005),
        (0x7fff_ffff, 0x8000_4005),
        (0x8000_0000, 0x8000_4005),
        (0xffff_ffff, 0x8000_4005),
    ];
    for (flags, hr) in ROWS {
        assert_eq!(
            init_local_ms_ctf_monitor(flags) as u32,
            hr,
            "flags={flags:#010x}"
        );
    }
}

/// Row 09: `DllCanUnloadNow` returns `S_FALSE`.
#[test]
fn oracle_dll_can_unload_now_row() {
    // row 09 DllCanUnloadNow -> hr=0x00000001
    assert_eq!(dll_can_unload_now() as u32, 0x0000_0001);
}

/// Rows 10-13: `DllRegisterServer` and `DllUnregisterServer` return `S_OK` whether the
/// enumerator returns TRUE or FALSE. The harness also observed one `EnumResourceNamesW` call
/// with the `WINE_REGISTRY` type in each case; the port has no enumerator to check that against.
#[test]
fn oracle_register_rows() {
    // row 10 DllRegisterServer enum=TRUE -> hr=0x00000000 enum_calls=1 type_ok=1
    // row 11 DllRegisterServer enum=FALSE -> hr=0x00000000 enum_calls=1 type_ok=1
    // row 12 DllUnregisterServer enum=TRUE -> hr=0x00000000 enum_calls=1 type_ok=1
    // row 13 DllUnregisterServer enum=FALSE -> hr=0x00000000 enum_calls=1 type_ok=1
    assert_eq!(dll_register_server() as u32, 0x0000_0000);
    assert_eq!(dll_unregister_server() as u32, 0x0000_0000);
}

/// Rows 14-16: each `@ stub` export raises `EXCEPTION_WINE_STUB` with the module and export name.
#[test]
fn oracle_stub_rows() {
    // row 14 DllGetClassObject -> code=0x80000100 flags=0x1 nargs=2 p0=msctfmonitor.dll p1=DllGetClassObject
    assert_eq!(
        dll_get_class_object(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "msctfmonitor.dll",
            function: "DllGetClassObject",
        }
    );
    // row 15 DoMsCtfMonitor -> code=0x80000100 flags=0x1 nargs=2 p0=msctfmonitor.dll p1=DoMsCtfMonitor
    assert_eq!(
        do_ms_ctf_monitor(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "msctfmonitor.dll",
            function: "DoMsCtfMonitor",
        }
    );
    // row 16 UnInitLocalMsCtfMonitor -> code=0x80000100 flags=0x1 nargs=2 p0=msctfmonitor.dll p1=UnInitLocalMsCtfMonitor
    assert_eq!(
        un_init_local_ms_ctf_monitor(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "msctfmonitor.dll",
            function: "UnInitLocalMsCtfMonitor",
        }
    );
}
