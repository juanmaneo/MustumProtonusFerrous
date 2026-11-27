//! Oracle-derived test vectors for the ported profapi stubs.
//!
//! Vectors were captured by compiling the verbatim Wine C body of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/profapi/`, and running the harness once per ordinal in `profapi.spec`. The
//! harness recorder longjmps out of the C loop instead of returning. ASan and UBSan reported
//! nothing for the 17 printed rows, so none are excluded.

use super::*;

#[test]
fn oracle_profapi_stub_rows() {
    // row 01 ordinal=101 -> code=0x80000100 flags=0x1 nargs=2 p0=profapi.dll p1=101
    assert_eq!(
        ordinal_101(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "profapi.dll",
            function: "101",
        }
    );

    // row 02 ordinal=102 -> code=0x80000100 flags=0x1 nargs=2 p0=profapi.dll p1=102
    assert_eq!(
        ordinal_102(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "profapi.dll",
            function: "102",
        }
    );

    // row 03 ordinal=103 -> code=0x80000100 flags=0x1 nargs=2 p0=profapi.dll p1=103
    assert_eq!(
        ordinal_103(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "profapi.dll",
            function: "103",
        }
    );

    // row 04 ordinal=104 -> code=0x80000100 flags=0x1 nargs=2 p0=profapi.dll p1=104
    assert_eq!(
        ordinal_104(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "profapi.dll",
            function: "104",
        }
    );

    // row 05 ordinal=105 -> code=0x80000100 flags=0x1 nargs=2 p0=profapi.dll p1=105
    assert_eq!(
        ordinal_105(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "profapi.dll",
            function: "105",
        }
    );

    // row 06 ordinal=106 -> code=0x80000100 flags=0x1 nargs=2 p0=profapi.dll p1=106
    assert_eq!(
        ordinal_106(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "profapi.dll",
            function: "106",
        }
    );

    // row 07 ordinal=107 -> code=0x80000100 flags=0x1 nargs=2 p0=profapi.dll p1=107
    assert_eq!(
        ordinal_107(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "profapi.dll",
            function: "107",
        }
    );

    // row 08 ordinal=108 -> code=0x80000100 flags=0x1 nargs=2 p0=profapi.dll p1=108
    assert_eq!(
        ordinal_108(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "profapi.dll",
            function: "108",
        }
    );

    // row 09 ordinal=109 -> code=0x80000100 flags=0x1 nargs=2 p0=profapi.dll p1=109
    assert_eq!(
        ordinal_109(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "profapi.dll",
            function: "109",
        }
    );

    // row 10 ordinal=110 -> code=0x80000100 flags=0x1 nargs=2 p0=profapi.dll p1=110
    assert_eq!(
        ordinal_110(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "profapi.dll",
            function: "110",
        }
    );

    // row 11 ordinal=111 -> code=0x80000100 flags=0x1 nargs=2 p0=profapi.dll p1=111
    assert_eq!(
        ordinal_111(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "profapi.dll",
            function: "111",
        }
    );

    // row 12 ordinal=112 -> code=0x80000100 flags=0x1 nargs=2 p0=profapi.dll p1=112
    assert_eq!(
        ordinal_112(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "profapi.dll",
            function: "112",
        }
    );

    // row 13 ordinal=113 -> code=0x80000100 flags=0x1 nargs=2 p0=profapi.dll p1=113
    assert_eq!(
        ordinal_113(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "profapi.dll",
            function: "113",
        }
    );

    // row 14 ordinal=114 -> code=0x80000100 flags=0x1 nargs=2 p0=profapi.dll p1=114
    assert_eq!(
        ordinal_114(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "profapi.dll",
            function: "114",
        }
    );

    // row 15 ordinal=115 -> code=0x80000100 flags=0x1 nargs=2 p0=profapi.dll p1=115
    assert_eq!(
        ordinal_115(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "profapi.dll",
            function: "115",
        }
    );

    // row 16 ordinal=116 -> code=0x80000100 flags=0x1 nargs=2 p0=profapi.dll p1=116
    assert_eq!(
        ordinal_116(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "profapi.dll",
            function: "116",
        }
    );

    // row 17 ordinal=117 -> code=0x80000100 flags=0x1 nargs=2 p0=profapi.dll p1=117
    assert_eq!(
        ordinal_117(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "profapi.dll",
            function: "117",
        }
    );
}
