//! Edge-case tests for the ported msvcrt20 logic.
//!
//! These cover the argument boundaries, the local copy of `new_mode`, the `_matherr` argument,
//! and the stub names. Not every expectation is oracle-derived; the ones that aren't say so in a
//! comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn argument_boundaries_pass_through_unchanged() {
    // Not oracle-derived: the C wrappers copy their arguments without checks, so the integer
    // extremes and the zero and maximum pointer tags must come out as they went in. The oracle
    // covered only some of these values.
    let mut seen = Vec::new();
    getmainargs(usize::MAX, 0, 1, i32::MIN, i32::MAX, |a, v, e, x, m| {
        seen.push((a, v, e, x, *m))
    });
    getmainargs(0, usize::MAX, 0, -1, 0, |a, v, e, x, m| {
        seen.push((a, v, e, x, *m))
    });
    wgetmainargs(1, 0, usize::MAX, 0, i32::MIN, |a, v, e, x, m| {
        seen.push((a, v, e, x, *m))
    });
    assert_eq!(
        seen,
        [
            (usize::MAX, 0, 1, i32::MIN, i32::MAX),
            (0, usize::MAX, 0, -1, 0),
            (1, 0, usize::MAX, 0, i32::MIN),
        ]
    );
}

#[test]
fn crt_writes_to_new_mode_stay_local() {
    // Not oracle-derived: the wrapper passes the address of its own copy of new_mode. A write
    // the CRT makes through it is dropped when the wrapper returns, so the next call starts
    // from its own input.
    let mut modes = Vec::new();
    getmainargs(0usize, 0usize, 0usize, 0, 5, |_, _, _, _, m| {
        modes.push(*m);
        *m = 99;
    });
    wgetmainargs(0usize, 0usize, 0usize, 0, 6, |_, _, _, _, m| modes.push(*m));
    assert_eq!(modes, [5, 6]);
}

#[test]
fn matherr_ignores_its_argument() {
    // Not oracle-derived: the oracle used NULL, one local and one fake address. The C body never
    // reads e, so any element type gives 0 as well.
    assert_eq!(matherr::<()>(std::ptr::null()), 0);
    let value = 7u64;
    assert_eq!(matherr(&value as *const u64), 0);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness passed only the two fixed names of msvcrt20.spec. The code
    // and flags are the same for any names.
    let other = wine_spec_unimplemented_stub("other.dll", "Foo");
    assert_eq!(other.module, "other.dll");
    assert_eq!(other.function, "Foo");
    assert_eq!(other.code, EXCEPTION_WINE_STUB);
    assert_eq!(other.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(other.number_parameters, 2);

    let empty = wine_spec_unimplemented_stub("", "");
    assert_eq!(empty.module, "");
    assert_eq!(empty.function, "");
}
