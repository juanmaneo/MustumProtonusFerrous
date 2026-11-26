//! Oracle-derived test vectors for the ported msvcrt20 logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `MSVCRT20__getmainargs`,
//! `MSVCRT20__wgetmainargs` and `MSVCRT20__matherr` (`dlls/msvcrt20/msvcrt20.c`) and of
//! `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`) with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/msvcrt20/`, and running the harness.
//! The 19 printed rows are all clean: ASan and UBSan reported nothing, so no vector is excluded.

use super::*;

/// The `(argc, argv, envp, expand_wildcards, new_mode)` tuple that the CRT closure receives.
type Forwarded = Vec<(usize, usize, usize, i32, i32)>;

/// Calls `getmainargs` with fake pointer tags. Returns what the CRT closure saw on each call.
fn call_getmainargs(argc: usize, argv: usize, envp: usize, expand: i32, mode: i32) -> Forwarded {
    let mut seen = Vec::new();
    getmainargs(argc, argv, envp, expand, mode, |a, v, e, x, m| {
        seen.push((a, v, e, x, *m))
    });
    seen
}

/// Calls `wgetmainargs` with fake pointer tags. Returns what the CRT closure saw on each call.
fn call_wgetmainargs(argc: usize, argv: usize, envp: usize, expand: i32, mode: i32) -> Forwarded {
    let mut seen = Vec::new();
    wgetmainargs(argc, argv, envp, expand, mode, |a, v, e, x, m| {
        seen.push((a, v, e, x, *m))
    });
    seen
}

#[test]
fn oracle_getmainargs_rows() {
    // row 01 __getmainargs argc=0x1000 argv=0x2000 envp=0x3000 expand=0 new_mode=0 -> calls=1 argc=0x1000 argv=0x2000 envp=0x3000 expand=0 mode=0
    assert_eq!(
        call_getmainargs(0x1000, 0x2000, 0x3000, 0, 0),
        [(0x1000, 0x2000, 0x3000, 0, 0)]
    );

    // row 02 __getmainargs argc=0x1000 argv=0x2000 envp=0x3000 expand=1 new_mode=1 -> calls=1 argc=0x1000 argv=0x2000 envp=0x3000 expand=1 mode=1
    assert_eq!(
        call_getmainargs(0x1000, 0x2000, 0x3000, 1, 1),
        [(0x1000, 0x2000, 0x3000, 1, 1)]
    );

    // row 03 __getmainargs argc=0x1000 argv=0x2000 envp=0x3000 expand=-1 new_mode=-1 -> calls=1 argc=0x1000 argv=0x2000 envp=0x3000 expand=-1 mode=-1
    assert_eq!(
        call_getmainargs(0x1000, 0x2000, 0x3000, -1, -1),
        [(0x1000, 0x2000, 0x3000, -1, -1)]
    );

    // row 04 __getmainargs argc=0x0 argv=0x0 envp=0x0 expand=0 new_mode=0 -> calls=1 argc=0x0 argv=0x0 envp=0x0 expand=0 mode=0
    assert_eq!(call_getmainargs(0, 0, 0, 0, 0), [(0, 0, 0, 0, 0)]);

    // row 05 __getmainargs argc=0x1000 argv=0x0 envp=0x3000 expand=0 new_mode=1 -> calls=1 argc=0x1000 argv=0x0 envp=0x3000 expand=0 mode=1
    assert_eq!(
        call_getmainargs(0x1000, 0, 0x3000, 0, 1),
        [(0x1000, 0, 0x3000, 0, 1)]
    );

    // row 06 __getmainargs argc=0x1000 argv=0x2000 envp=0x0 expand=2 new_mode=7 -> calls=1 argc=0x1000 argv=0x2000 envp=0x0 expand=2 mode=7
    assert_eq!(
        call_getmainargs(0x1000, 0x2000, 0, 2, 7),
        [(0x1000, 0x2000, 0, 2, 7)]
    );

    // row 07 __getmainargs argc=0x1000 argv=0x2000 envp=0x3000 expand=2147483647 new_mode=-2147483648 -> calls=1 argc=0x1000 argv=0x2000 envp=0x3000 expand=2147483647 mode=-2147483648
    assert_eq!(
        call_getmainargs(0x1000, 0x2000, 0x3000, i32::MAX, i32::MIN),
        [(0x1000, 0x2000, 0x3000, i32::MAX, i32::MIN)]
    );

    // row 08 __getmainargs argc=0x1000 argv=0x2000 envp=0x3000 expand=0 new_mode=2147483647 -> calls=1 argc=0x1000 argv=0x2000 envp=0x3000 expand=0 mode=2147483647
    assert_eq!(
        call_getmainargs(0x1000, 0x2000, 0x3000, 0, i32::MAX),
        [(0x1000, 0x2000, 0x3000, 0, i32::MAX)]
    );
}

#[test]
fn oracle_wgetmainargs_rows() {
    // row 09 __wgetmainargs argc=0x1000 argv=0x2000 envp=0x3000 expand=0 new_mode=0 -> calls=1 argc=0x1000 argv=0x2000 envp=0x3000 expand=0 mode=0
    assert_eq!(
        call_wgetmainargs(0x1000, 0x2000, 0x3000, 0, 0),
        [(0x1000, 0x2000, 0x3000, 0, 0)]
    );

    // row 10 __wgetmainargs argc=0x1000 argv=0x2000 envp=0x3000 expand=1 new_mode=1 -> calls=1 argc=0x1000 argv=0x2000 envp=0x3000 expand=1 mode=1
    assert_eq!(
        call_wgetmainargs(0x1000, 0x2000, 0x3000, 1, 1),
        [(0x1000, 0x2000, 0x3000, 1, 1)]
    );

    // row 11 __wgetmainargs argc=0x1000 argv=0x2000 envp=0x3000 expand=-1 new_mode=2 -> calls=1 argc=0x1000 argv=0x2000 envp=0x3000 expand=-1 mode=2
    assert_eq!(
        call_wgetmainargs(0x1000, 0x2000, 0x3000, -1, 2),
        [(0x1000, 0x2000, 0x3000, -1, 2)]
    );

    // row 12 __wgetmainargs argc=0x0 argv=0x0 envp=0x0 expand=0 new_mode=0 -> calls=1 argc=0x0 argv=0x0 envp=0x0 expand=0 mode=0
    assert_eq!(call_wgetmainargs(0, 0, 0, 0, 0), [(0, 0, 0, 0, 0)]);

    // row 13 __wgetmainargs argc=0x1000 argv=0x0 envp=0x3000 expand=0 new_mode=-1 -> calls=1 argc=0x1000 argv=0x0 envp=0x3000 expand=0 mode=-1
    assert_eq!(
        call_wgetmainargs(0x1000, 0, 0x3000, 0, -1),
        [(0x1000, 0, 0x3000, 0, -1)]
    );

    // row 14 __wgetmainargs argc=0x1000 argv=0x2000 envp=0x0 expand=2147483647 new_mode=2147483647 -> calls=1 argc=0x1000 argv=0x2000 envp=0x0 expand=2147483647 mode=2147483647
    assert_eq!(
        call_wgetmainargs(0x1000, 0x2000, 0, i32::MAX, i32::MAX),
        [(0x1000, 0x2000, 0, i32::MAX, i32::MAX)]
    );
}

#[test]
fn oracle_matherr_rows() {
    // row 15 _matherr e=NULL -> ret=0
    assert_eq!(matherr::<u8>(std::ptr::null()), 0);

    // row 16 _matherr e=local -> ret=0
    let local = (1i32, 2.0f64);
    assert_eq!(matherr(&local as *const (i32, f64)), 0);

    // row 17 _matherr e=0x1000 -> ret=0
    assert_eq!(matherr(0x1000 as *const u8), 0);
}

#[test]
fn oracle_stub_rows() {
    // row 18 ??_EIostream_init@@QAEPAXI@Z -> raise code=0x80000100 flags=0x1 nargs=2 p0=msvcrt20.dll p1=??_EIostream_init@@QAEPAXI@Z
    assert_eq!(
        vector_deleting_dtor_iostream_init(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "msvcrt20.dll",
            function: "??_EIostream_init@@QAEPAXI@Z",
        }
    );

    // row 19 ??_GIostream_init@@QAEPAXI@Z -> raise code=0x80000100 flags=0x1 nargs=2 p0=msvcrt20.dll p1=??_GIostream_init@@QAEPAXI@Z
    assert_eq!(
        scalar_deleting_dtor_iostream_init(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "msvcrt20.dll",
            function: "??_GIostream_init@@QAEPAXI@Z",
        }
    );
}
