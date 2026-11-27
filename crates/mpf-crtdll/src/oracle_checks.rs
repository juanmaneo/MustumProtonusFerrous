//! Oracle-derived test vectors for the ported crtdll logic.
//!
//! Vectors were captured by compiling the verbatim msvcrt bodies of the ported exports and the
//! verbatim `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`) with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/crtdll/`, and running the harness.
//! The harness printed 25 rows. Two are excluded because UBSan flags them: row 24 (`abs(INT_MIN)`,
//! signed overflow) and row 25 (`_rotl(0x80000001, 0)`, shift by 32). The 23 remaining rows are the
//! nine stub rows (01 to 09) and the function rows (10 to 23).

use super::*;

#[test]
fn oracle_stub_rows() {
    // row 01 _getsystime -> raised=1 code=0x80000100 flags=0x1 params=2 module=crtdll.dll function=_getsystime
    assert_eq!(
        getsystime(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "crtdll.dll",
            function: "_getsystime",
        }
    );

    // row 02 _setsystime -> raised=1 code=0x80000100 flags=0x1 params=2 module=crtdll.dll function=_setsystime
    assert_eq!(
        setsystime(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "crtdll.dll",
            function: "_setsystime",
        }
    );

    // row 03 _ismbbalnum -> raised=1 code=0x80000100 flags=0x1 params=2 module=crtdll.dll function=_ismbbalnum
    assert_eq!(
        ismbbalnum(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "crtdll.dll",
            function: "_ismbbalnum",
        }
    );

    // row 04 _ismbbalpha -> raised=1 code=0x80000100 flags=0x1 params=2 module=crtdll.dll function=_ismbbalpha
    assert_eq!(
        ismbbalpha(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "crtdll.dll",
            function: "_ismbbalpha",
        }
    );

    // row 05 _ismbbgraph -> raised=1 code=0x80000100 flags=0x1 params=2 module=crtdll.dll function=_ismbbgraph
    assert_eq!(
        ismbbgraph(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "crtdll.dll",
            function: "_ismbbgraph",
        }
    );

    // row 06 _ismbbkalnum -> raised=1 code=0x80000100 flags=0x1 params=2 module=crtdll.dll function=_ismbbkalnum
    assert_eq!(
        ismbbkalnum(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "crtdll.dll",
            function: "_ismbbkalnum",
        }
    );

    // row 07 _ismbbkpunct -> raised=1 code=0x80000100 flags=0x1 params=2 module=crtdll.dll function=_ismbbkpunct
    assert_eq!(
        ismbbkpunct(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "crtdll.dll",
            function: "_ismbbkpunct",
        }
    );

    // row 08 _ismbbprint -> raised=1 code=0x80000100 flags=0x1 params=2 module=crtdll.dll function=_ismbbprint
    assert_eq!(
        ismbbprint(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "crtdll.dll",
            function: "_ismbbprint",
        }
    );

    // row 09 _ismbbpunct -> raised=1 code=0x80000100 flags=0x1 params=2 module=crtdll.dll function=_ismbbpunct
    assert_eq!(
        ismbbpunct(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "crtdll.dll",
            function: "_ismbbpunct",
        }
    );
}

#[test]
fn oracle_function_rows() {
    // row 10 abs(-5) -> 5
    assert_eq!(abs(-5), 5);

    // row 11 labs(-7) -> 7
    assert_eq!(labs(-7), 7);

    // row 12 div(-7,2) -> quot=-3 rem=-1
    assert_eq!(div(-7, 2), DivResult { quot: -3, rem: -1 });

    // row 13 ldiv(7,-2) -> quot=-3 rem=1
    assert_eq!(ldiv(7, -2), DivResult { quot: -3, rem: 1 });

    // row 14 _rotl(0x80000001,1) -> 0x00000003
    assert_eq!(rotl(0x8000_0001, 1), 0x0000_0003);

    // row 15 _lrotl(0x80000000,33) -> 0x00000001
    assert_eq!(lrotl(0x8000_0000, 33), 0x0000_0001);

    // row 16 _rotr(3,1) -> 0x80000001
    assert_eq!(rotr(3, 1), 0x8000_0001);

    // row 17 _lrotr(1,4) -> 0x10000000
    assert_eq!(lrotr(1, 4), 0x1000_0000);

    // row 18 _chgsign(1.5) -> bits=0xbff8000000000000
    assert_eq!(chgsign(1.5).to_bits(), 0xbff8_0000_0000_0000);

    // row 19 _finite(1.0) -> 1
    assert_eq!(finite(1.0), 1);

    // row 20 _isnan(NAN) -> 1
    assert_eq!(isnan(f64::NAN), 1);

    // row 21 _toupper(97) -> 65
    assert_eq!(toupper(97), 65);

    // row 22 _tolower(90) -> 122
    assert_eq!(tolower(90), 122);

    // row 23 _swab("ABCDE",5) -> dst=BADC tail=? (the byte after the last pair is untouched)
    let mut dst = [b'?'; 8];
    swab(b"ABCDE", &mut dst, 5);
    assert_eq!(&dst[..4], b"BADC");
    assert_eq!(dst[4], b'?');
}
