//! Oracle-derived test vectors for the ported dbgeng logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of
//! `debug_target_return_string` and the `debug_target_get_module_info_by_base`
//! loop from `dlls/dbgeng/dbgeng.c` (with minimal Windows type shims) under
//! `target/canon/dbgeng/` using `gcc -fsanitize=address,undefined -g`. The
//! harness printed 22 rows (16 string rows, 6 module rows). ASan and UBSan
//! reported nothing, so no vector was excluded.

use super::*;

/// Runs `return_string` on a 16-byte buffer pre-filled with `#`, as the oracle does.
/// Returns `(hr, size, written)` where `written` is the bytes up to the first NUL
/// within `min(bsize, 16)`, or `None` when no buffer was passed.
fn run(s: &str, bsize: usize, use_buf: bool, use_size: bool) -> (i32, Option<u32>, Option<String>) {
    let mut buf = [b'#'; 16];
    let mut size = 0xdead_u32;
    let hr = return_string(
        s.as_bytes(),
        if use_buf { Some(&mut buf) } else { None },
        bsize,
        if use_size { Some(&mut size) } else { None },
    );
    let written = use_buf.then(|| {
        buf[..bsize.min(16)]
            .iter()
            .take_while(|&&b| b != 0)
            .map(|&b| b as char)
            .collect()
    });
    (hr, use_size.then_some(size), written)
}

#[test]
fn oracle_return_string_rows() {
    // r01 str="hello" bsize=16 buf=1 size=1
    assert_eq!(
        run("hello", 16, true, true),
        (S_OK, Some(6), Some("hello".into()))
    );
    // r02 str="hello" bsize=6 buf=1 size=1
    assert_eq!(
        run("hello", 6, true, true),
        (S_OK, Some(6), Some("hello".into()))
    );
    // r03 str="hello" bsize=5 buf=1 size=1
    assert_eq!(
        run("hello", 5, true, true),
        (S_FALSE, Some(6), Some("hell".into()))
    );
    // r04 str="" bsize=1 buf=1 size=1
    assert_eq!(run("", 1, true, true), (S_OK, Some(1), Some(String::new())));
    // r05 str="" bsize=0 buf=1 size=1
    assert_eq!(
        run("", 0, true, true),
        (S_FALSE, Some(1), Some(String::new()))
    );
    // r06 str="abc" bsize=0 buf=1 size=1
    assert_eq!(
        run("abc", 0, true, true),
        (S_FALSE, Some(4), Some(String::new()))
    );
    // r07 str="abc" bsize=10 buf=0 size=1
    assert_eq!(run("abc", 10, false, true), (S_OK, Some(4), None));
    // r08 str="abc" bsize=3 buf=0 size=1
    assert_eq!(run("abc", 3, false, true), (S_FALSE, Some(4), None));
    // r09 str="abc" bsize=4 buf=1 size=0
    assert_eq!(run("abc", 4, true, false), (S_OK, None, Some("abc".into())));
    // r10 str="abc" bsize=1 buf=1 size=1
    assert_eq!(
        run("abc", 1, true, true),
        (S_FALSE, Some(4), Some(String::new()))
    );
    // r11 str="" bsize=0 buf=0 size=0
    assert_eq!(run("", 0, false, false), (S_FALSE, None, None));
    // r12 str="abcdefghij" bsize=11 buf=1 size=1
    assert_eq!(
        run("abcdefghij", 11, true, true),
        (S_OK, Some(11), Some("abcdefghij".into()))
    );
    // r13 str="abcdefghij" bsize=10 buf=1 size=1
    assert_eq!(
        run("abcdefghij", 10, true, true),
        (S_FALSE, Some(11), Some("abcdefghi".into()))
    );
    // r14 str="a" bsize=2 buf=1 size=1
    assert_eq!(run("a", 2, true, true), (S_OK, Some(2), Some("a".into())));
    // r15 str="a" bsize=1 buf=1 size=1
    assert_eq!(
        run("a", 1, true, true),
        (S_FALSE, Some(2), Some(String::new()))
    );
    // r16 str="xyz" bsize=2 buf=0 size=1
    assert_eq!(run("xyz", 2, false, true), (S_FALSE, Some(4), None));
}

/// Bases as set up in the oracle harness: `0x1000, 0x2000, 0x2000, 0xffff...ffff`.
const ORACLE_BASES: [u64; 4] = [0x1000, 0x2000, 0x2000, 0xffff_ffff_ffff_ffff];

#[test]
fn oracle_module_index_by_base_rows() {
    // m01 base=0x1000 -> found index=0
    assert_eq!(module_index_by_base(&ORACLE_BASES, 0x1000), Some(0));
    // m02 base=0x2000 -> found index=1 (first of two equal bases)
    assert_eq!(module_index_by_base(&ORACLE_BASES, 0x2000), Some(1));
    // m03 base=0xffffffffffffffff -> found index=3
    assert_eq!(
        module_index_by_base(&ORACLE_BASES, 0xffff_ffff_ffff_ffff),
        Some(3)
    );
    // m04 base=0x3000 -> null
    assert_eq!(module_index_by_base(&ORACLE_BASES, 0x3000), None);
    // m05 base=0 -> null
    assert_eq!(module_index_by_base(&ORACLE_BASES, 0), None);
    // m06 loaded=0 with base=0x1000 -> null
    assert_eq!(module_index_by_base(&[], 0x1000), None);
}
