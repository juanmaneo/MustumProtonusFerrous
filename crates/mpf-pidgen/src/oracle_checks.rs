//! Oracle-derived test vectors for the ported pidgen logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `PIDGenSimpA` and
//! `PIDGenSimpW` (`dlls/pidgen/main.c`) and `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/pidgen/`, and running the harness. All 21 printed rows ran clean under both
//! sanitizers, so none is excluded. The stub rows come from an `RtlRaiseException` recorder that
//! longjmps back to the harness, because the C stub loops around the call and never returns.

use super::*;

/// One `PIDGenSimpA` row: the string argument, the eight integer arguments and the return value.
type AsciiRow = (Option<&'static [u8]>, [i32; 8], i32);

/// One `PIDGenSimpW` row, with the string held as NUL-terminated UTF-16 units.
type WideRow = (Option<Vec<u16>>, [i32; 8], i32);

/// The UTF-16 units the harness passes: the code units of `text` followed by the NUL terminator.
fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().chain(std::iter::once(0)).collect()
}

#[test]
fn oracle_pid_gen_simp_a_rows() {
    let rows: [AsciiRow; 8] = [
        // row 01 str=NULL p=0,0,0,0,0,0,0,0 -> ret=0
        (None, [0, 0, 0, 0, 0, 0, 0, 0], 0),
        // row 02 str=abc p=1,2,3,4,5,6,7,8 -> ret=0
        (Some(b"abc"), [1, 2, 3, 4, 5, 6, 7, 8], 0),
        // row 03 str=empty p=INT_MIN x8 -> ret=0
        (Some(b""), [i32::MIN; 8], 0),
        // row 04 str=empty p=INT_MAX x8 -> ret=0
        (Some(b""), [i32::MAX; 8], 0),
        // row 05 str=ff fe p=-1,-2,-3,-4,-5,-6,-7,-8 -> ret=0
        (Some(&[0xff, 0xfe]), [-1, -2, -3, -4, -5, -6, -7, -8], 0),
        // row 06 str=NULL p=INT_MIN,INT_MAX,0,-1,1,7,8,9 -> ret=0
        (None, [i32::MIN, i32::MAX, 0, -1, 1, 7, 8, 9], 0),
        // row 07 str="PID 123" p=12,0,0,0,0,0,0,0 -> ret=0
        (Some(b"PID 123"), [12, 0, 0, 0, 0, 0, 0, 0], 0),
        // row 08 str=alnum36 p=INT_MIN,INT_MAX,0,-1,1,7,8,9 -> ret=0
        (
            Some(b"0123456789abcdefghijklmnopqrstuvwxyz"),
            [i32::MIN, i32::MAX, 0, -1, 1, 7, 8, 9],
            0,
        ),
    ];
    for (text, p, ret) in rows {
        assert_eq!(
            pid_gen_simp_a(text, p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7]),
            ret
        );
    }
}

#[test]
fn oracle_pid_gen_simp_w_rows() {
    let rows: [WideRow; 8] = [
        // row 09 str=NULL p=0,0,0,0,0,0,0,0 -> ret=0
        (None, [0, 0, 0, 0, 0, 0, 0, 0], 0),
        // row 10 str=abc p=1,2,3,4,5,6,7,8 -> ret=0
        (Some(wide("abc")), [1, 2, 3, 4, 5, 6, 7, 8], 0),
        // row 11 str=empty p=INT_MIN x8 -> ret=0
        (Some(wide("")), [i32::MIN; 8], 0),
        // row 12 str=empty p=INT_MAX x8 -> ret=0
        (Some(wide("")), [i32::MAX; 8], 0),
        // row 13 str=d800 dc00 p=-1,-2,-3,-4,-5,-6,-7,-8 -> ret=0
        (Some(wide("\u{10000}")), [-1, -2, -3, -4, -5, -6, -7, -8], 0),
        // row 14 str=NULL p=INT_MIN,INT_MAX,0,-1,1,7,8,9 -> ret=0
        (None, [i32::MIN, i32::MAX, 0, -1, 1, 7, 8, 9], 0),
        // row 15 str="PID 123" p=12,0,0,0,0,0,0,0 -> ret=0
        (Some(wide("PID 123")), [12, 0, 0, 0, 0, 0, 0, 0], 0),
        // row 16 str=alnum13 p=1,2,3,4,5,6,7,8 -> ret=0
        (Some(wide("0123456789abc")), [1, 2, 3, 4, 5, 6, 7, 8], 0),
    ];
    for (text, p, ret) in rows {
        assert_eq!(
            pid_gen_simp_w(
                text.as_deref(),
                p[0],
                p[1],
                p[2],
                p[3],
                p[4],
                p[5],
                p[6],
                p[7]
            ),
            ret
        );
    }
}

#[test]
fn oracle_stub_rows() {
    // row 17 PIDGenA -> raise code=0x80000100 flags=0x1 nargs=2 module=pidgen.dll function=PIDGenA
    assert_eq!(
        pid_gen_a(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "pidgen.dll",
            function: "PIDGenA",
        }
    );

    // row 18 PIDGenW -> raise code=0x80000100 flags=0x1 nargs=2 module=pidgen.dll function=PIDGenW
    assert_eq!(
        pid_gen_w(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "pidgen.dll",
            function: "PIDGenW",
        }
    );

    // row 19 SetupPIDGenA -> raise code=0x80000100 flags=0x1 nargs=2 module=pidgen.dll function=SetupPIDGenA
    assert_eq!(
        setup_pid_gen_a(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "pidgen.dll",
            function: "SetupPIDGenA",
        }
    );

    // row 20 SetupPIDGenW -> raise code=0x80000100 flags=0x1 nargs=2 module=pidgen.dll function=SetupPIDGenW
    assert_eq!(
        setup_pid_gen_w(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "pidgen.dll",
            function: "SetupPIDGenW",
        }
    );

    // row 21 VerifyPIDSequenceW -> raise code=0x80000100 flags=0x1 nargs=2 module=pidgen.dll function=VerifyPIDSequenceW
    assert_eq!(
        verify_pid_sequence_w(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "pidgen.dll",
            function: "VerifyPIDSequenceW",
        }
    );
}
