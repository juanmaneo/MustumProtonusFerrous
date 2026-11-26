//! Oracle-derived test vectors for the ported sensapi logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `IsDestinationReachableA`,
//! `IsDestinationReachableW` and `IsNetworkAlive` (`dlls/sensapi/sensapi.c`) with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/sensapi/`, and running the harness.
//! The 19 printed rows produced no ASan or UBSan report, so none are excluded.

use super::*;

type Reachable = fn() -> bool;

/// The `IsDestinationReachable` rows. The port takes no arguments, so each row only checks the
/// return value. The inputs appear in the row labels.
#[test]
fn oracle_is_destination_reachable_rows() {
    let rows: [(&str, Reachable); 12] = [
        (
            "row 01 fn=A dest=wine.com qoc=NULL",
            is_destination_reachable_a,
        ),
        (
            "row 02 fn=A dest=empty qoc=NULL",
            is_destination_reachable_a,
        ),
        ("row 03 fn=A dest=NULL qoc=NULL", is_destination_reachable_a),
        (
            "row 04 fn=A dest=192.168.0.1 qoc=ptr",
            is_destination_reachable_a,
        ),
        ("row 05 fn=A dest=x qoc=ptr", is_destination_reachable_a),
        (
            "row 06 fn=A dest=wine.com+qoc qoc=ptr",
            is_destination_reachable_a,
        ),
        (
            "row 07 fn=W dest=microsoft.com qoc=NULL",
            is_destination_reachable_w,
        ),
        ("row 08 fn=W dest=NULL qoc=NULL", is_destination_reachable_w),
        (
            "row 09 fn=W dest=empty+qoc qoc=ptr",
            is_destination_reachable_w,
        ),
        (
            "row 10 fn=W dest=latin1-eacute qoc=NULL",
            is_destination_reachable_w,
        ),
        (
            "row 11 fn=W dest=example.org+qoc qoc=ptr",
            is_destination_reachable_w,
        ),
        (
            "row 12 fn=W dest=NULL+qoc qoc=ptr",
            is_destination_reachable_w,
        ),
    ];
    for (row, reachable) in rows {
        assert!(reachable(), "{row}");
    }
}

/// The `IsNetworkAlive` rows as `(row, input, out)`. An input of `None` is a NULL pointer, and
/// `out` is then the untouched sentinel `0xa5a5_a5a5`.
#[test]
fn oracle_is_network_alive_rows() {
    const SENTINEL: u32 = 0xa5a5_a5a5;
    let rows: [(&str, Option<u32>, u32); 7] = [
        ("row 13 in=NULL", None, SENTINEL),
        ("row 14 in=0", Some(0), 1),
        ("row 15 in=0xffffffff", Some(0xffff_ffff), 1),
        ("row 16 in=4(AOL)", Some(4), 1),
        ("row 17 in=2(WAN)", Some(2), 1),
        ("row 18 in=1(LAN)", Some(1), 1),
        ("row 19 in=0xdeadbeef", Some(0xdead_beef), 1),
    ];
    for (row, input, expected_out) in rows {
        let mut value = input.unwrap_or(SENTINEL);
        let alive = is_network_alive(input.map(|_| &mut value));
        assert!(alive, "{row}");
        assert_eq!(value, expected_out, "{row}");
    }
}
