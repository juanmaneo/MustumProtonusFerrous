//! Oracle-derived test vectors for the ported ctapi32 logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies from
//! `dlls/ctapi32/ctapi32.c`, `dlls/ctapi32/unixlib.c` and `dlls/ctapi32/unixlib.h`
//! with `gcc -fsanitize=address,undefined -g` under `target/canon/ctapi32/` and
//! running the harness against a fake `libctapi.so`. All 25 printed rows are checked
//! here; ASan and UBSan flagged none, so none are excluded.

use super::*;
use std::cell::Cell;

const DLL_THREAD_ATTACH: u32 = 2;

/// Stand-in for `libctapi.so` with the semantics of the oracle's `libctapi.c`. It
/// records the pointer arguments of `ct_data` without dereferencing them.
#[derive(Default)]
pub(crate) struct FakeLib {
    pub(crate) seen: Cell<[u64; 5]>,
}

impl CtLibrary for FakeLib {
    type Ptr = u64;

    fn ct_init(&self, params: CtInitParams) -> i8 {
        (i32::from(params.ctn) + i32::from(params.pn)) as i8
    }

    fn ct_data(&self, params: CtDataParams<u64>) -> i8 {
        self.seen.set([
            params.dad,
            params.sad,
            params.command,
            params.lenr,
            params.response,
        ]);
        (i32::from(params.ctn) - i32::from(params.lenc)) as i8
    }

    fn ct_close(&self, params: CtCloseParams) -> i8 {
        params.ctn as i8
    }
}

fn all_symbols(_: &str) -> bool {
    true
}

fn without_ct_close(name: &str) -> bool {
    name != "CT_close"
}

fn no_symbols(_: &str) -> bool {
    false
}

/// The `dlopen` result for each soname the oracle uses; `None` means the library is missing.
pub(crate) fn open_lib(name: &[u8]) -> Option<fn(&str) -> bool> {
    match name {
        b"./libctapi.so" | b"libctapi.so" => Some(all_symbols),
        b"./libctapi_partial.so" => Some(without_ct_close),
        b"libm.so.6" => Some(no_symbols),
        _ => None,
    }
}

/// Little-endian UTF-16 bytes of `text`, followed by a NUL unit when `nul` is set.
pub(crate) fn reg_bytes(text: &str, nul: bool) -> Vec<u8> {
    let mut units: Vec<u16> = text.encode_utf16().collect();
    if nul {
        units.push(0);
    }
    units.iter().flat_map(|unit| unit.to_le_bytes()).collect()
}

/// Runs `dll_main` with the real library load and counting stand-ins for the unix
/// calls. Returns `(result, init_calls, load_calls, detach_calls)`.
pub(crate) fn run_dll_main(
    reason: u32,
    reserved: bool,
    init: NtStatus,
    value: Option<(u32, &[u8])>,
) -> (bool, u32, u32, u32) {
    let init_calls = Cell::new(0);
    let load_calls = Cell::new(0);
    let detach_calls = Cell::new(0);
    let result = dll_main(
        reason,
        reserved,
        || {
            init_calls.set(init_calls.get() + 1);
            init
        },
        || {
            load_calls.set(load_calls.get() + 1);
            load_functions(value, open_lib)
        },
        || {
            detach_calls.set(detach_calls.get() + 1);
            detach()
        },
    );
    (
        result,
        init_calls.get(),
        load_calls.get(),
        detach_calls.get(),
    )
}

#[test]
fn oracle_attach_rows() {
    // row 01 attach(./no_such_libctapi.so) -> c0000135
    assert_eq!(
        attach(open_lib(b"./no_such_libctapi.so")) as u32,
        0xc000_0135
    );
    // row 02 attach(libm.so.6) -> c0000139
    assert_eq!(attach(open_lib(b"libm.so.6")) as u32, 0xc000_0139);
    // row 03 attach(./libctapi_partial.so) -> c0000139
    assert_eq!(
        attach(open_lib(b"./libctapi_partial.so")) as u32,
        0xc000_0139
    );
    // row 04 attach(./libctapi.so) -> 00000000
    assert_eq!(attach(open_lib(b"./libctapi.so")) as u32, 0);
}

#[test]
fn oracle_ct_init_rows() {
    let lib = FakeLib::default();
    // row 05 CT_init(3,4) -> 7
    assert_eq!(ct_init(&lib, 3, 4), 7);
    // row 06 CT_init(200,100) -> 44
    assert_eq!(ct_init(&lib, 200, 100), 44);
    // row 07 CT_init(65535,65535) -> -2
    assert_eq!(ct_init(&lib, 65535, 65535), -2);
}

#[test]
fn oracle_ct_close_rows() {
    let lib = FakeLib::default();
    // row 08 CT_close(200) -> -56
    assert_eq!(ct_close(&lib, 200), -56);
    // row 09 CT_close(127) -> 127
    assert_eq!(ct_close(&lib, 127), 127);
    // row 10 CT_close(128) -> -128
    assert_eq!(ct_close(&lib, 128), -128);
}

#[test]
fn oracle_ct_data_rows() {
    let lib = FakeLib::default();
    // row 11 CT_data(1, lenc=5) -> -4 forwarded=1
    let params = CtDataParams {
        ctn: 1,
        dad: 0x1000,
        sad: 0x2000,
        lenc: 5,
        command: 0x3000,
        lenr: 0x4000,
        response: 0x5000,
    };
    assert_eq!(ct_data(&lib, params), -4);
    assert_eq!(lib.seen.get(), [0x1000, 0x2000, 0x3000, 0x4000, 0x5000]);

    let lib = FakeLib::default();
    // row 12 wow64_ct_data(ctn=2) -> 2 dad=deadbeef sad=1 command=80000000 lenr=0 response=ffffffff
    let params = Wow64CtDataParams {
        ctn: 2,
        dad: 0xdead_beef,
        sad: 1,
        lenc: 0,
        command: 0x8000_0000,
        lenr: 0,
        response: 0xffff_ffff,
    };
    assert_eq!(wow64_ct_data(&lib, params), 2);
    assert_eq!(
        lib.seen.get(),
        [0xdead_beef, 1, 0x8000_0000, 0, 0xffff_ffff]
    );
}

#[test]
fn oracle_slot_table_rows() {
    // row 13 tables native[ct_data]=ct_data 1 wow64[ct_data]=wow64_ct_data 1 count=5
    assert_eq!(entry(false, UnixFunc::CtData), Entry::CtData);
    assert_eq!(entry(true, UnixFunc::CtData), Entry::Wow64CtData);
    assert_eq!(UNIX_FUNCS_COUNT, 5);
}

#[test]
fn oracle_dll_main_rows() {
    // row 14 DllMain(THREAD_ATTACH) -> 1
    assert_eq!(
        run_dll_main(DLL_THREAD_ATTACH, false, STATUS_SUCCESS, None),
        (true, 0, 0, 0)
    );
    // row 15 DllMain(PROCESS_DETACH, reserved) -> 1
    assert_eq!(
        run_dll_main(DLL_PROCESS_DETACH, true, STATUS_SUCCESS, None),
        (true, 0, 0, 0)
    );
    // row 16 DllMain(ATTACH, init fails) -> 0 key_queries=0, so no load (load_calls == 0)
    assert_eq!(
        run_dll_main(DLL_PROCESS_ATTACH, false, 0xc000_0001_u32 as NtStatus, None),
        (false, 1, 0, 0)
    );
    // row 17 DllMain(ATTACH, key missing) -> 1 with the default libctapi.so
    assert_eq!(
        run_dll_main(DLL_PROCESS_ATTACH, false, STATUS_SUCCESS, None),
        (true, 1, 1, 0)
    );
    // row 25 DllMain(PROCESS_DETACH, NULL) -> 1
    assert_eq!(
        run_dll_main(DLL_PROCESS_DETACH, false, STATUS_SUCCESS, None),
        (true, 0, 0, 1)
    );
}

#[test]
fn oracle_dll_main_library_rows() {
    // row 18 DllMain(ATTACH, REG_SZ ./no_such_libctapi.so) -> 0
    let value = reg_bytes("./no_such_libctapi.so", true);
    assert_eq!(
        run_dll_main(
            DLL_PROCESS_ATTACH,
            false,
            STATUS_SUCCESS,
            Some((REG_SZ, value.as_slice()))
        ),
        (false, 1, 1, 0)
    );

    // row 19 DllMain(ATTACH, REG_SZ ./libctapi_partial.so) -> 0
    let value = reg_bytes("./libctapi_partial.so", true);
    assert_eq!(
        run_dll_main(
            DLL_PROCESS_ATTACH,
            false,
            STATUS_SUCCESS,
            Some((REG_SZ, value.as_slice()))
        ),
        (false, 1, 1, 0)
    );

    // row 20 DllMain(ATTACH, REG_DWORD ./no_such_libctapi.so) -> 1: type ignored, libctapi.so used
    let value = reg_bytes("./no_such_libctapi.so", true);
    assert_eq!(
        run_dll_main(
            DLL_PROCESS_ATTACH,
            false,
            STATUS_SUCCESS,
            Some((4, value.as_slice()))
        ),
        (true, 1, 1, 0)
    );

    // row 21 DllMain(ATTACH, REG_SZ 642 bytes) -> 1: ERROR_MORE_DATA, libctapi.so used
    let value = reg_bytes(&format!("./no_such_libctapi.so{}", "a".repeat(299)), true);
    assert_eq!(value.len(), 642);
    assert_eq!(
        run_dll_main(
            DLL_PROCESS_ATTACH,
            false,
            STATUS_SUCCESS,
            Some((REG_SZ, value.as_slice()))
        ),
        (true, 1, 1, 0)
    );

    // row 22 DllMain(ATTACH, REG_SZ unterminated + odd byte 0x41) -> 0
    let mut value = reg_bytes("./libctapi_partial.so", false);
    value.push(0x41);
    assert_eq!(
        run_dll_main(
            DLL_PROCESS_ATTACH,
            false,
            STATUS_SUCCESS,
            Some((REG_SZ, value.as_slice()))
        ),
        (false, 1, 1, 0)
    );

    // row 23 DllMain(ATTACH, REG_SZ ./libctapi_partial.so NUL junk NUL, 27 units) -> 0
    let mut value = reg_bytes("./libctapi_partial.so", true);
    value.extend(reg_bytes("junk", true));
    assert_eq!(value.len(), 27 * 2);
    assert_eq!(
        run_dll_main(
            DLL_PROCESS_ATTACH,
            false,
            STATUS_SUCCESS,
            Some((REG_SZ, value.as_slice()))
        ),
        (false, 1, 1, 0)
    );

    // row 24 DllMain(ATTACH, value missing) -> 1: the same None input as row 17
    assert_eq!(
        run_dll_main(DLL_PROCESS_ATTACH, false, STATUS_SUCCESS, None),
        (true, 1, 1, 0)
    );
}
