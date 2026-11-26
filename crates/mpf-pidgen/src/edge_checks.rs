//! Edge-case tests for the ported pidgen logic.
//!
//! These cover the argument boundaries of the two `PIDGenSimp` functions and the fields of the
//! stub records. Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

fn simp_a(text: Option<&[u8]>, p: [i32; 8]) -> i32 {
    pid_gen_simp_a(text, p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7])
}

fn simp_w(text: Option<&[u16]>, p: [i32; 8]) -> i32 {
    pid_gen_simp_w(text, p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7])
}

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn pid_gen_simp_a_ignores_every_argument() {
    // Not oracle-derived: the oracle rows cover eight argument sets. This walks each of the eight
    // integer parameters through the signed boundaries on its own, with the others at 0, and
    // with each pointer state. The C body never reads an argument, so every call returns 0.
    let boundaries = [i32::MIN, -1, 0, 1, i32::MAX];
    for text in [None, Some(&b""[..]), Some(&b"abc"[..])] {
        assert_eq!(simp_a(text, [0; 8]), 0);
        for position in 0..8 {
            for value in boundaries {
                let mut p = [0; 8];
                p[position] = value;
                assert_eq!(simp_a(text, p), 0);
            }
        }
    }
}

#[test]
fn pid_gen_simp_w_ignores_every_argument() {
    // Not oracle-derived: the same per-parameter boundary walk as the ASCII variant, for the
    // wide-string signature. The NUL-only and the surrogate-pair strings are the edge pointer
    // states here.
    let boundaries = [i32::MIN, -1, 0, 1, i32::MAX];
    for text in [None, Some(&[0u16][..]), Some(&[0xd800, 0xdc00, 0][..])] {
        assert_eq!(simp_w(text, [0; 8]), 0);
        for position in 0..8 {
            for value in boundaries {
                let mut p = [0; 8];
                p[position] = value;
                assert_eq!(simp_w(text, p), 0);
            }
        }
    }
}

#[test]
fn wine_spec_unimplemented_stub_passes_names_through() {
    // Not oracle-derived: the C body copies module and function into ExceptionInformation[0] and
    // [1] unchanged. The oracle rows only use "pidgen.dll" with the five export names, so this
    // checks that the port passes arbitrary names through without transforming them.
    assert_eq!(
        wine_spec_unimplemented_stub("other.dll", ""),
        StubException {
            code: EXCEPTION_WINE_STUB,
            flags: EXCEPTION_NONCONTINUABLE,
            number_parameters: 2,
            module: "other.dll",
            function: "",
        }
    );
}

#[test]
fn every_stub_helper_shares_the_stub_record_fields() {
    // Not oracle-derived: the fields every helper shares, and that the five export names are
    // distinct. The oracle rows already check each full record once.
    let records = [
        pid_gen_a(),
        pid_gen_w(),
        setup_pid_gen_a(),
        setup_pid_gen_w(),
        verify_pid_sequence_w(),
    ];
    for record in records {
        assert_eq!(record.code, EXCEPTION_WINE_STUB);
        assert_eq!(record.flags, EXCEPTION_NONCONTINUABLE);
        assert_eq!(record.number_parameters, 2);
        assert_eq!(record.module, "pidgen.dll");
    }
    for (i, first) in records.iter().enumerate() {
        for second in &records[i + 1..] {
            assert_ne!(first.function, second.function);
        }
    }
}
