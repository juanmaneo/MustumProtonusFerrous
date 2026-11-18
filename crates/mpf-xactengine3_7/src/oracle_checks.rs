//! Tests against Wine's oracle vectors, recorded by `target/canon/xactengine3_7/build.sh`.
//!
//! Each row is `(input, reply)`: the `xf`, `fx` or `nf` command with that input, and the decimal
//! value the verbatim C printed. The rows are a subset of a run over 0 to 19 and 255 for each
//! command. No vector was flagged by ASan or UBSan.

use super::*;

/// `xf N`: `xact_notification_type_from_fact(N)`.
const XF: &[(u8, u8)] = &[(0, 0), (1, 1), (12, 12), (18, 18), (19, 0), (255, 0)];

/// `fx N`: `fact_notification_type_from_xact(N)`.
const FX: &[(u8, u8)] = &[(0, 0), (1, 1), (16, 16), (18, 18), (19, 0), (255, 0)];

/// `nf N`: the `NOTIFY_*` flags the `unwrap_notificationdesc` chain selects for FACT type `N`.
const NF: &[(u8, u32)] = &[
    (0, 0),
    (1, 21),
    (6, 1),
    (7, 2),
    (9, 0),
    (12, 42),
    (13, 31),
    (16, 42),
    (17, 2),
    (19, 0),
];

#[test]
fn xact_notification_type_from_fact_matches_wine() {
    for &(input, reply) in XF {
        assert_eq!(xact_notification_type_from_fact(input), reply, "xf {input}");
    }
}

#[test]
fn fact_notification_type_from_xact_matches_wine() {
    for &(input, reply) in FX {
        assert_eq!(fact_notification_type_from_xact(input), reply, "fx {input}");
    }
}

#[test]
fn notify_flags_matches_wine() {
    for &(input, reply) in NF {
        assert_eq!(notify_flags(input), reply, "nf {input}");
    }
}
