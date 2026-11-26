//! Oracle-derived test vectors for the ported msftedit logic.
//!
//! Vectors were captured under `target/canon/msftedit/` by compiling the verbatim Wine C bodies of
//! `DllMain` (`dlls/msftedit/msftedit_main.c` lines 38-54), `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c` lines 504-516) and `DllGetVersion` (`libs/winecrt0/dll_version.c`) with
//! `gcc -fsanitize=address,undefined -g`. The OS calls are shims that record their arguments.
//! `RtlRaiseException` records the exception and `longjmp`s out, because the C stub loops forever.
//! All 25 printed rows are kept: ASan and UBSan reported nothing, so no vectors are excluded.

use super::*;

/// One OS call `DllMain` made, as the C harness logged it.
#[derive(Debug, PartialEq, Eq)]
enum Call {
    LoadLibraryW(String, usize),
    DisableThreadLibraryCalls(usize),
    FreeLibrary(usize),
}

/// The host of the C harness: `LoadLibraryW` returns `load_result`, and every call is logged.
struct Host {
    load_result: usize,
    calls: Vec<Call>,
}

impl DllHost for Host {
    fn load_library_w(&mut self, name: &str) -> usize {
        self.calls
            .push(Call::LoadLibraryW(name.to_owned(), self.load_result));
        self.load_result
    }

    fn disable_thread_library_calls(&mut self, module: usize) {
        self.calls.push(Call::DisableThreadLibraryCalls(module));
    }

    fn free_library(&mut self, module: usize) {
        self.calls.push(Call::FreeLibrary(module));
    }
}

/// Runs one `DllMain` call and returns its result and the OS calls it made.
fn run_dll_main(
    state: &mut DllMainState,
    load_result: usize,
    inst: usize,
    reason: u32,
) -> (bool, Vec<Call>) {
    let mut host = Host {
        load_result,
        calls: Vec::new(),
    };
    let ret = state.dll_main(&mut host, inst, reason);
    (ret, host.calls)
}

/// The `DLLVERSIONINFO2` the C harness fills with `0xee` bytes, with `cbSize` set.
fn sentinel(cb_size: u32) -> DllVersionInfo {
    DllVersionInfo {
        cb_size,
        major: 0xeeee_eeee,
        minor: 0xeeee_eeee,
        build: 0xeeee_eeee,
        platform_id: 0xeeee_eeee,
        flags: 0xeeee_eeee,
        ull_version: 0xeeee_eeee_eeee_eeee,
    }
}

/// Calls `dll_get_version` with a sentinel buffer of `cb_size` and returns the result and the buffer.
fn call_with(cb_size: u32, resource: Option<&[u8]>) -> (u32, DllVersionInfo) {
    let mut info = sentinel(cb_size);
    let ret = dll_get_version(Some(&mut info), resource);
    (ret, info)
}

/// The `VS_VERSION_INFO` bytes the C harness's `build_resource` writes: the six-byte header, the
/// UTF-16 key and its NUL, padding to a 4-byte boundary, then `VS_FIXEDFILEINFO` (52 bytes).
fn version_resource(key: &str, val_len: u16, sig: u32, ms: u32, ls: u32) -> Vec<u8> {
    let mut out = vec![0; 6];
    for unit in key.encode_utf16().chain([0]) {
        out.extend_from_slice(&unit.to_le_bytes());
    }
    while !out.len().is_multiple_of(4) {
        out.push(0);
    }
    for word in [sig, 0x0001_0000, ms, ls] {
        out.extend_from_slice(&word.to_le_bytes());
    }
    out.resize(out.len() + 36, 0);
    let total = out.len() as u16;
    out[0..2].copy_from_slice(&total.to_le_bytes());
    out[2..4].copy_from_slice(&val_len.to_le_bytes());
    out
}

#[test]
fn oracle_dll_main_rows() {
    let mut state = DllMainState::default();

    // row 01 detach-before-attach reason=0 -> ret=1 calls=[FreeLibrary(0)]
    assert_eq!(
        run_dll_main(&mut state, 0, 0x40_0000, DLL_PROCESS_DETACH),
        (true, vec![Call::FreeLibrary(0)])
    );

    // row 02 thread-attach reason=2 -> ret=1 calls=[]
    assert_eq!(
        run_dll_main(&mut state, 0, 0x40_0000, DLL_THREAD_ATTACH),
        (true, vec![])
    );

    // row 03 attach-ok reason=1 -> ret=1 calls=[LoadLibraryW(riched20.dll)->0x7000; DisableThreadLibraryCalls(0x400000)]
    assert_eq!(
        run_dll_main(&mut state, 0x7000, 0x40_0000, DLL_PROCESS_ATTACH),
        (
            true,
            vec![
                Call::LoadLibraryW("riched20.dll".to_owned(), 0x7000),
                Call::DisableThreadLibraryCalls(0x40_0000),
            ]
        )
    );

    // row 04 detach reason=0 -> ret=1 calls=[FreeLibrary(0x7000)]
    assert_eq!(
        run_dll_main(&mut state, 0, 0x40_0000, DLL_PROCESS_DETACH),
        (true, vec![Call::FreeLibrary(0x7000)])
    );

    // row 05 attach-load-fails reason=1 -> ret=1 calls=[LoadLibraryW(riched20.dll)->0; DisableThreadLibraryCalls(0x500000)]
    assert_eq!(
        run_dll_main(&mut state, 0, 0x50_0000, DLL_PROCESS_ATTACH),
        (
            true,
            vec![
                Call::LoadLibraryW("riched20.dll".to_owned(), 0),
                Call::DisableThreadLibraryCalls(0x50_0000),
            ]
        )
    );

    // row 06 detach-after-failed-load reason=0 -> ret=1 calls=[FreeLibrary(0)]
    assert_eq!(
        run_dll_main(&mut state, 0, 0x50_0000, DLL_PROCESS_DETACH),
        (true, vec![Call::FreeLibrary(0)])
    );

    // row 07 attach-ok reason=1 -> ret=1 calls=[LoadLibraryW(riched20.dll)->0x9000; DisableThreadLibraryCalls(0x600000)]
    assert_eq!(
        run_dll_main(&mut state, 0x9000, 0x60_0000, DLL_PROCESS_ATTACH),
        (
            true,
            vec![
                Call::LoadLibraryW("riched20.dll".to_owned(), 0x9000),
                Call::DisableThreadLibraryCalls(0x60_0000),
            ]
        )
    );

    // row 08 thread-detach reason=3 -> ret=1 calls=[]
    assert_eq!(
        run_dll_main(&mut state, 0, 0x60_0000, DLL_THREAD_DETACH),
        (true, vec![])
    );

    // row 09 detach-keeps-handle reason=0 -> ret=1 calls=[FreeLibrary(0x9000)]
    assert_eq!(
        run_dll_main(&mut state, 0, 0x60_0000, DLL_PROCESS_DETACH),
        (true, vec![Call::FreeLibrary(0x9000)])
    );

    // row 10 unknown-reason reason=99 -> ret=1 calls=[]
    assert_eq!(run_dll_main(&mut state, 0, 0x60_0000, 99), (true, vec![]));
}

#[test]
fn oracle_stub_rows() {
    // row 11 SetCustomTextOutHandlerEx -> code=0x80000100 flags=0x1 nparams=2 p0=msftedit.dll p1=SetCustomTextOutHandlerEx
    assert_eq!(
        set_custom_text_out_handler_ex(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "msftedit.dll",
            function: "SetCustomTextOutHandlerEx",
        }
    );

    // row 12 RichEditWndProc -> code=0x80000100 flags=0x1 nparams=2 p0=msftedit.dll p1=RichEditWndProc
    assert_eq!(
        rich_edit_wnd_proc(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "msftedit.dll",
            function: "RichEditWndProc",
        }
    );

    // row 13 RichListBoxWndProc -> code=0x80000100 flags=0x1 nparams=2 p0=msftedit.dll p1=RichListBoxWndProc
    assert_eq!(
        rich_list_box_wnd_proc(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "msftedit.dll",
            function: "RichListBoxWndProc",
        }
    );

    // row 14 RichComboBoxWndProc -> code=0x80000100 flags=0x1 nparams=2 p0=msftedit.dll p1=RichComboBoxWndProc
    assert_eq!(
        rich_combo_box_wnd_proc(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "msftedit.dll",
            function: "RichComboBoxWndProc",
        }
    );
}

#[test]
fn oracle_dll_get_version_rows() {
    let good = version_resource("VS_VERSION_INFO", 52, 0xfeef_04bd, 0x0001_0002, 0x0003_0004);

    // row 15 info=NULL -> ret=0x80070057 info=NULL
    assert_eq!(dll_get_version(None, Some(&good)), E_INVALIDARG);

    // rows 16 and 17 (no-resource and load-fails, cb=32) -> ret=0x80070057, info unchanged.
    // The port takes the lookup result, so both rows are the same call with `None`.
    assert_eq!(call_with(32, None), (E_INVALIDARG, sentinel(32)));

    // row 18 val_len=51 cb=32 -> ret=0x80070057, info unchanged
    let short = version_resource("VS_VERSION_INFO", 51, 0xfeef_04bd, 0x0001_0002, 0x0003_0004);
    assert_eq!(call_with(32, Some(&short)), (E_INVALIDARG, sentinel(32)));

    // row 19 bad-signature cb=32 -> ret=0x80070057, info unchanged
    let bad_sig = version_resource("VS_VERSION_INFO", 52, 0xdead_beef, 0x0001_0002, 0x0003_0004);
    assert_eq!(call_with(32, Some(&bad_sig)), (E_INVALIDARG, sentinel(32)));

    // row 20 cb=0 -> ret=0x80070057, info unchanged
    assert_eq!(call_with(0, Some(&good)), (E_INVALIDARG, sentinel(0)));

    // row 21 cb=24 -> ret=0x80070057, info unchanged
    assert_eq!(call_with(24, Some(&good)), (E_INVALIDARG, sentinel(24)));

    // row 22 cb=20 valid -> ret=0 cb=20 major=0x1 minor=0x2 build=0x3 platform=2 flags=0xeeeeeeee ull=0xeeeeeeeeeeeeeeee
    assert_eq!(
        call_with(20, Some(&good)),
        (
            S_OK,
            DllVersionInfo {
                cb_size: 20,
                major: 1,
                minor: 2,
                build: 3,
                platform_id: 2,
                flags: 0xeeee_eeee,
                ull_version: 0xeeee_eeee_eeee_eeee,
            }
        )
    );

    // row 23 cb=32 valid -> ret=0 cb=32 major=0x1 minor=0x2 build=0x3 platform=2 flags=0 ull=0x1000200030004
    assert_eq!(
        call_with(32, Some(&good)),
        (
            S_OK,
            DllVersionInfo {
                cb_size: 32,
                major: 1,
                minor: 2,
                build: 3,
                platform_id: 2,
                flags: 0,
                ull_version: 0x0001_0002_0003_0004,
            }
        )
    );

    // row 24 key=A cb=32 max-versions -> ret=0 cb=32 major=0xffff minor=0xffff build=0xffff platform=2 flags=0 ull=0xffffffffffff0000
    let max = version_resource("A", 52, 0xfeef_04bd, 0xffff_ffff, 0xffff_0000);
    assert_eq!(
        call_with(32, Some(&max)),
        (
            S_OK,
            DllVersionInfo {
                cb_size: 32,
                major: 0xffff,
                minor: 0xffff,
                build: 0xffff,
                platform_id: 2,
                flags: 0,
                ull_version: 0xffff_ffff_ffff_0000,
            }
        )
    );

    // row 25 val_len=60 cb=20 -> ret=0 cb=20 major=0x1 minor=0x2 build=0x3 platform=2 flags=0xeeeeeeee ull=0xeeeeeeeeeeeeeeee
    let long_val = version_resource("VS_VERSION_INFO", 60, 0xfeef_04bd, 0x0001_0002, 0x0003_0004);
    assert_eq!(
        call_with(20, Some(&long_val)),
        (
            S_OK,
            DllVersionInfo {
                cb_size: 20,
                major: 1,
                minor: 2,
                build: 3,
                platform_id: 2,
                flags: 0xeeee_eeee,
                ull_version: 0xeeee_eeee_eeee_eeee,
            }
        )
    );
}
