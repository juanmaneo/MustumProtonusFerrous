//! Edge-case tests for the ported ktmw32 logic.
//!
//! These cover the constants, the timeout and unicode boundaries, the `set_ntstatus` edges, and the
//! stub record. Not every expectation is oracle-derived. The ones that aren't say so in a comment.

use super::*;
use crate::oracle_checks::FakeNt;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: header values from include/winbase.h, include/winternl.h,
    // include/winnt.h and include/winerror.h. The oracle's attr and ret columns agree on the
    // OBJ_ flags and on INVALID_HANDLE_VALUE.
    assert_eq!(INFINITE, 0xFFFF_FFFF);
    assert_eq!(OBJ_INHERIT, 0x0000_0002);
    assert_eq!(OBJ_CASE_INSENSITIVE, 0x0000_0040);
    assert_eq!(INVALID_HANDLE_VALUE, usize::MAX);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn timeout_boundaries() {
    // Not oracle-derived: the largest finite timeout, and the first value with bit 31 set.
    assert_eq!(get_nt_timeout(INFINITE - 1), Some(-42_949_672_940_000));
    assert_eq!(get_nt_timeout(0x8000_0000), Some(-21_474_836_480_000));
}

#[test]
fn init_unicode_string_boundaries() {
    // Not oracle-derived: 0x7ffd characters is the last count below the cap, so the length is
    // 0xfffa and the maximum is 0xfffc.
    let mut below_cap: Vec<u16> = vec![0x61; 0x7ffd];
    below_cap.push(0);
    assert_eq!(
        init_unicode_string(Some(below_cap.as_slice())),
        UnicodeString {
            length: 0xfffa,
            maximum_length: 0xfffc,
            buffer: Some(below_cap.as_slice()),
        }
    );

    // Not oracle-derived: an embedded NUL ends the length, as `wcslen` does.
    let embedded: [u16; 4] = [0x61, 0, 0x62, 0];
    assert_eq!(
        init_unicode_string(Some(&embedded)),
        UnicodeString {
            length: 2,
            maximum_length: 4,
            buffer: Some(&embedded),
        }
    );

    // Not oracle-derived: with no NUL, the length stops at the end of the slice. C would read
    // past it, so this is the port's defined behaviour.
    let unterminated: [u16; 2] = [0x61, 0x62];
    assert_eq!(
        init_unicode_string(Some(&unterminated)),
        UnicodeString {
            length: 4,
            maximum_length: 6,
            buffer: Some(&unterminated),
        }
    );

    // Not oracle-derived: an empty slice has length 0 and maximum length 2, which differs from
    // a NULL source (length 0, maximum length 0).
    let empty: [u16; 0] = [];
    assert_eq!(
        init_unicode_string(Some(&empty)),
        UnicodeString {
            length: 0,
            maximum_length: 2,
            buffer: Some(&empty),
        }
    );
}

#[test]
fn set_ntstatus_boundaries() {
    // Not oracle-derived: zero is success, and no last error is set.
    let mut nt = FakeNt::default();
    assert!(set_ntstatus(&mut nt, 0));
    assert_eq!(nt.last_error, None);

    // Not oracle-derived: the smallest nonzero status maps to the error 1.
    let mut nt = FakeNt::default();
    assert!(!set_ntstatus(&mut nt, 1));
    assert_eq!(nt.last_error, Some(1));

    // Not oracle-derived: the all-ones status keeps only the low 16 bits in the stand-in.
    let mut nt = FakeNt::default();
    assert!(!set_ntstatus(&mut nt, -1));
    assert_eq!(nt.last_error, Some(0xffff));
}

#[test]
fn stub_record_passes_names_through() {
    // Not oracle-derived: the oracle only uses ktmw32.dll and the export names. The C body
    // copies both arguments into the record, and this checks that the port does too.
    assert_eq!(
        wine_spec_unimplemented_stub("other.dll", "Foo"),
        StubException {
            code: EXCEPTION_WINE_STUB,
            flags: EXCEPTION_NONCONTINUABLE,
            number_parameters: 2,
            module: "other.dll",
            function: "Foo",
        }
    );
    let empty = wine_spec_unimplemented_stub("", "");
    assert_eq!(empty.module, "");
    assert_eq!(empty.function, "");
}
