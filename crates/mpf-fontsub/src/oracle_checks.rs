//! Oracle-derived test vectors for the ported fontsub logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `CreateFontPackage`
//! (`dlls/fontsub/main.c`) and `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`)
//! with `gcc -fsanitize=address,undefined -g` under `target/canon/fontsub/`, and running the
//! harness. The 17 printed rows include one row (row 14: `CreateFontPackage` with a NULL `src`
//! and `src_len` 0) that UBSan flags as a null pointer passed to `memcpy`. It is excluded here,
//! so 16 rows remain.

use super::*;

const ABCDEFGHIJKLMNOP: &[u8] = b"ABCDEFGHIJKLMNOP";

/// The out-parameters the C harness starts with: `dest` is a non-NULL sentinel and
/// `dest_len` and `written` are sentinels.
fn sentinel() -> FontPackageOut {
    FontPackageOut {
        dest: Some(vec![0xaa]),
        dest_len: 0xffff_ffff,
        written: 0xffff_fffe,
    }
}

/// Calls `create_font_package` with an allocator that behaves like `malloc` or always returns NULL.
/// Returns the return value, the out-parameters, and the sizes the allocator was asked for.
fn call(src: &[u8], format: u16, fails: bool) -> (u32, FontPackageOut, Vec<usize>) {
    let mut out = sentinel();
    let mut asked = Vec::new();
    let ret = create_font_package(src, format, &mut out, |size| {
        asked.push(size);
        if fails {
            None
        } else {
            Some(vec![0; size])
        }
    });
    (ret, out, asked)
}

#[test]
fn oracle_create_font_package_rows() {
    // row 01 fmt=0 src_len=16 alloc=ok -> ret=0 calls=1 asize=16 dest=4142..50 dest_len=16 written=16
    let (ret, out, asked) = call(ABCDEFGHIJKLMNOP, TTFCFP_SUBSET, false);
    assert_eq!(ret, NO_ERROR);
    assert_eq!(asked, [16]);
    assert_eq!(
        out,
        FontPackageOut {
            dest: Some(ABCDEFGHIJKLMNOP.to_vec()),
            dest_len: 16,
            written: 16,
        }
    );

    // row 02 fmt=0 src_len=1 alloc=ok -> ret=0 calls=1 asize=1 dest=41 dest_len=1 written=1
    let (ret, out, asked) = call(&ABCDEFGHIJKLMNOP[..1], TTFCFP_SUBSET, false);
    assert_eq!(ret, NO_ERROR);
    assert_eq!(asked, [1]);
    assert_eq!(
        out,
        FontPackageOut {
            dest: Some(vec![0x41]),
            dest_len: 1,
            written: 1,
        }
    );

    // row 03 fmt=0 src_len=0 alloc=ok -> ret=0 calls=1 asize=0 dest= dest_len=0 written=0
    let (ret, out, asked) = call(&[], TTFCFP_SUBSET, false);
    assert_eq!(ret, NO_ERROR);
    assert_eq!(asked, [0]);
    assert_eq!(
        out,
        FontPackageOut {
            dest: Some(vec![]),
            dest_len: 0,
            written: 0,
        }
    );

    // row 04 fmt=0 src_len=0 alloc=fail -> ret=1005 calls=1 asize=0 dest=NULL dest_len=4294967295 written=4294967294
    let (ret, out, asked) = call(&[], TTFCFP_SUBSET, true);
    assert_eq!(ret, ERR_MEM);
    assert_eq!(asked, [0]);
    assert_eq!(
        out,
        FontPackageOut {
            dest: None,
            ..sentinel()
        }
    );

    // row 05 fmt=0 src_len=16 alloc=fail -> ret=1005 calls=1 asize=16 dest=NULL dest_len=4294967295 written=4294967294
    let (ret, out, asked) = call(ABCDEFGHIJKLMNOP, TTFCFP_SUBSET, true);
    assert_eq!(ret, ERR_MEM);
    assert_eq!(asked, [16]);
    assert_eq!(
        out,
        FontPackageOut {
            dest: None,
            ..sentinel()
        }
    );

    // row 06 fmt=0 src_len=3 alloc=fail -> ret=1005 calls=1 asize=3 dest=NULL dest_len=4294967295 written=4294967294
    let (ret, out, asked) = call(&ABCDEFGHIJKLMNOP[..3], TTFCFP_SUBSET, true);
    assert_eq!(ret, ERR_MEM);
    assert_eq!(asked, [3]);
    assert_eq!(
        out,
        FontPackageOut {
            dest: None,
            ..sentinel()
        }
    );

    // row 07 fmt=1 src_len=16 alloc=ok -> ret=1000 calls=0 asize=- dest=sentinel dest_len=4294967295 written=4294967294
    let (ret, out, asked) = call(ABCDEFGHIJKLMNOP, TTFCFP_SUBSET1, false);
    assert_eq!(ret, ERR_GENERIC);
    assert!(asked.is_empty());
    assert_eq!(out, sentinel());

    // row 08 fmt=2 src_len=16 alloc=ok -> ret=1000 calls=0 asize=- dest=sentinel dest_len=4294967295 written=4294967294
    let (ret, out, asked) = call(ABCDEFGHIJKLMNOP, TTFCFP_DELTA, false);
    assert_eq!(ret, ERR_GENERIC);
    assert!(asked.is_empty());
    assert_eq!(out, sentinel());

    // row 09 fmt=65535 src_len=16 alloc=ok -> ret=1000 calls=0 asize=- dest=sentinel dest_len=4294967295 written=4294967294
    let (ret, out, asked) = call(ABCDEFGHIJKLMNOP, 0xffff, false);
    assert_eq!(ret, ERR_GENERIC);
    assert!(asked.is_empty());
    assert_eq!(out, sentinel());

    // row 10 fmt=1 src_len=0 alloc=ok -> ret=1000 calls=0 asize=- dest=sentinel dest_len=4294967295 written=4294967294
    let (ret, out, asked) = call(&[], TTFCFP_SUBSET1, false);
    assert_eq!(ret, ERR_GENERIC);
    assert!(asked.is_empty());
    assert_eq!(out, sentinel());

    // row 11 fmt=2 src_len=8 alloc=fail -> ret=1000 calls=0 asize=- dest=sentinel dest_len=4294967295 written=4294967294
    let (ret, out, asked) = call(&ABCDEFGHIJKLMNOP[..8], TTFCFP_DELTA, true);
    assert_eq!(ret, ERR_GENERIC);
    assert!(asked.is_empty());
    assert_eq!(out, sentinel());

    // row 12 fmt=0 src_len=8 alloc=ok -> ret=0 calls=1 asize=8 dest=4142434445464748 dest_len=8 written=8
    let (ret, out, asked) = call(&ABCDEFGHIJKLMNOP[..8], TTFCFP_SUBSET, false);
    assert_eq!(ret, NO_ERROR);
    assert_eq!(asked, [8]);
    assert_eq!(
        out,
        FontPackageOut {
            dest: Some(b"ABCDEFGH".to_vec()),
            dest_len: 8,
            written: 8,
        }
    );

    // row 13 fmt=0 src_len=2 alloc=ok -> ret=0 calls=1 asize=2 dest=00ff dest_len=2 written=2
    let (ret, out, asked) = call(&[0x00, 0xff], TTFCFP_SUBSET, false);
    assert_eq!(ret, NO_ERROR);
    assert_eq!(asked, [2]);
    assert_eq!(
        out,
        FontPackageOut {
            dest: Some(vec![0x00, 0xff]),
            dest_len: 2,
            written: 2,
        }
    );

    // row 15 fmt=0 src_len=2 alloc=fail -> ret=1005 calls=1 asize=2 dest=NULL dest_len=4294967295 written=4294967294
    let (ret, out, asked) = call(&[0x00, 0xff], TTFCFP_SUBSET, true);
    assert_eq!(ret, ERR_MEM);
    assert_eq!(asked, [2]);
    assert_eq!(
        out,
        FontPackageOut {
            dest: None,
            ..sentinel()
        }
    );

    // row 16 fmt=2 src_len=0 alloc=fail -> ret=1000 calls=0 asize=- dest=sentinel dest_len=4294967295 written=4294967294
    let (ret, out, asked) = call(&[], TTFCFP_DELTA, true);
    assert_eq!(ret, ERR_GENERIC);
    assert!(asked.is_empty());
    assert_eq!(out, sentinel());
}

#[test]
fn oracle_merge_font_package_row() {
    // row 17 MergeFontPackage -> raise code=0x80000100 flags=0x1 nargs=2 p0=fontsub.dll p1=MergeFontPackage
    assert_eq!(
        merge_font_package(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "fontsub.dll",
            function: "MergeFontPackage",
        }
    );
}
