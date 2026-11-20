//! Edge rows at the limits of the scanner, the clamps and the padding, replayed
//! against the same oracle harness. Also checks the `ios` state queries directly, and
//! the harness dispatch for an unknown operation.

use crate::oracle_checks::run_line;
use crate::{bad, eof, fail, good, IOSTATE_BADBIT, IOSTATE_EOFBIT, IOSTATE_FAILBIT};

const EDGE_ROWS: &[&str] = &[
    "scan 0 - => base=0 state=2 consumed=0 token=-",
    "scan 0 2d => base=0 state=2 consumed=0 token=-",
    "scan 0 2b2d31 => base=0 state=2 consumed=0 token=-",
    "scan 0 352d => base=0 state=0 consumed=1 token=35",
    "scan 64 3078 => base=16 state=2 consumed=0 token=-",
    "scan 16 3078 => base=10 state=0 consumed=1 token=30",
    "scan 32 30373839 => base=8 state=0 consumed=2 token=3037",
    "scan 0 30 => base=0 state=1 consumed=1 token=30",
    "scan 0 2d30 => base=0 state=1 consumed=2 token=2d30",
    "scan 0 3061 => base=8 state=0 consumed=1 token=30",
    "scan 0 307831 => base=16 state=1 consumed=3 token=307831",
    "scan 64 3167 => base=16 state=0 consumed=1 token=31",
    "scan 0 3132333435363738393031323334 => base=0 state=1 consumed=14 token=3132333435363738393031323334",
    "readint 0 -32768 32767 1 3430303030 => value=32767 state=3 consumed=5",
    "readint 0 -2147483648 2147483647 0 2d3939393939393939393939 => value=-2147483648 state=1 consumed=12",
    "readint 0 -2147483648 2147483647 1 2d3939393939393939393939 => value=-2147483648 state=3 consumed=12",
    "readint 0 -2147483648 2147483647 1 3939393939393939393939 => value=2147483647 state=3 consumed=11",
    "readint 0 -5 5 0 2d => value=0 state=2 consumed=0",
    "readint 0 -5 5 0 2d39 => value=-5 state=3 consumed=2",
    "readuint 0 -2147483648 4294967295 34323934393637323935 => value=4294967295 state=1 consumed=10",
    "readuint 0 -32768 65535 2d => value=0 state=2 consumed=0",
    "readuint 0 -2147483648 4294967295 34323934393637323935 => value=4294967295 state=1 consumed=10",
    "readuint 0 -32768 65535 34323934393334353238 => value=4294934528 state=1 consumed=10",
    "readuint 0 -32768 65535 2d31 => value=4294967295 state=1 consumed=2",
    "readuint 64 0 65535 6666666666666666 => value=4294967295 state=1 consumed=8",
    "readuint 64 0 65535 666666666666666666 => value=65535 state=3 consumed=9",
    "readint 0 -2147483648 2147483647 0 2b35 => value=5 state=1 consumed=2",
    "readint 0 -2147483648 2147483647 0 30783166 => value=31 state=1 consumed=4",
    "readuint 64 0 65535 30783166 => value=31 state=1 consumed=4",
    "print 704 255 0 0 0 32 => out=30584646",
    "print 0 0 0 0 0 32 => out=30",
    "print 16 0 0 0 0 32 => out=30",
    "print 1024 0 1 0 0 32 => out=30",
    "print 1024 -3 0 1 0 32 => out=2d33",
    "print 1664 0 0 0 0 32 => out=30",
    "print 192 0 0 0 0 32 => out=307830",
    "print 160 0 0 0 0 32 => out=3030",
    "print 32 8 0 1 0 32 => out=3130",
    "print 576 48879 0 0 0 32 => out=42454546",
    "print 16 2147483647 0 0 3 32 => out=32313437343833363437",
    "print 16 -2147483648 0 0 0 32 => out=2d32313437343833363438",
    "print 16 70000 0 1 0 32 => out=34343634",
    "print 16 70000 1 1 0 32 => out=34343634",
    "print 64 -2147483648 0 0 0 32 => out=3830303030303030",
    "print 2 42 0 0 4 42 => out=34322a2a",
    "print 8 42 0 0 4 42 => out=2a2a3432",
    "print 10 42 0 0 6 42 => out=2a2a2a2a3432",
    "print 0 42 0 0 0 42 => out=3432",
    "pad 0 4 46 6162 6364 => out=61626364",
    "pad 8 6 46 6162 6364 => out=61622e2e6364",
    "pad 2 2 46 6162 6364 => out=61626364",
    "pad 2 6 46 6162 - => out=61622e2e2e2e",
    "pad 0 -1 46 6162 6364 => out=61626364",
    "pad 10 6 46 6162 6364 => out=61622e2e6364",
    "pad 0 6 46 - - => out=2e2e2e2e2e2e",
];

#[test]
fn edge_rows_match_wine() {
    for row in EDGE_ROWS {
        let (input, expected) = row.split_once(" => ").expect("row separator");
        assert_eq!(run_line(input), expected, "row: {row}");
    }
}

#[test]
fn ios_state_queries() {
    assert!(good(0));
    assert!(!good(IOSTATE_EOFBIT));
    assert!(!good(IOSTATE_FAILBIT));
    assert!(!good(IOSTATE_BADBIT));
    assert!(fail(IOSTATE_FAILBIT));
    assert!(fail(IOSTATE_BADBIT));
    assert!(!fail(IOSTATE_EOFBIT));
    assert!(eof(IOSTATE_EOFBIT));
    assert!(!eof(IOSTATE_FAILBIT));
    assert!(bad(IOSTATE_BADBIT));
    assert!(!bad(IOSTATE_FAILBIT));
}

#[test]
#[should_panic(expected = "unknown harness operation")]
fn unknown_operation_panics() {
    run_line("seek 0");
}
