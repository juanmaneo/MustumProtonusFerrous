use crate::{
    apply_prosody,
    ProsodyAttribute::{Pitch, Rate, Volume},
    UnsupportedFormat,
};

fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

/// Edge rows, with expected values from the same Wine oracle run as
/// `oracle_checks.rs`; `None` means `SPERR_UNSUPPORTED_FORMAT`.
#[test]
fn rate_edges_match_wine() {
    let rows = [
        ("x-medium", 0, None),
        ("", 0, None),
        (" 5%", 0, Some(0)),
        ("\t5%", 0, Some(0)),
        ("5.", 0, Some(15)),
        ("+.5", 0, Some(-6)),
        (".", 0, None),
        ("-", 0, None),
        (" ", 0, None),
        ("1e1", 0, Some(21)),
        ("1E1", 0, Some(21)),
        ("1e+", 0, None),
        ("1e", 0, None),
        ("1x", 0, None),
        ("1xy", 0, None),
        ("5 ", 0, None),
        ("5%%", 0, None),
        ("-150%", 0, Some(0)),
        ("150%", 0, Some(8)),
        ("-100%", 0, Some(-10)),
        ("-200%", 0, Some(0)),
        ("1e-2", 0, Some(-10)),
        ("3", 0, Some(10)),
        ("0.005%", 0, Some(0)),
        ("1e2%", 0, Some(6)),
        ("1.5%x", 0, None),
    ];
    for (text, current, expected) in rows {
        assert_eq!(
            apply_prosody(Rate, &wide(text), current),
            expected.ok_or(UnsupportedFormat),
            "{text}"
        );
    }
}

#[test]
fn volume_edges_match_wine() {
    let rows = [
        ("", 50, None),
        (" 50", 50, Some(50)),
        ("+0", 50, Some(50)),
        ("-", 50, None),
        ("+%", 50, None),
        ("+5%", 40, Some(42)),
        ("-100%", 50, Some(0)),
        ("+200", 50, Some(100)),
        ("-200", 50, Some(0)),
        ("2.5", 50, Some(3)),
        ("0.5", 50, Some(1)),
        ("-0.5", 50, Some(50)),
        ("1e2", 0, Some(100)),
        ("1e2x", 0, None),
        ("50x", 0, None),
        ("x-soft", 0, Some(20)),
        ("loud", 0, Some(80)),
        ("x-loud", 0, Some(100)),
        ("medium", 0, Some(60)),
        ("soft", 0, Some(40)),
        ("silent", 0, Some(0)),
        ("x-silent", 0, None),
        ("+1e1", 50, Some(60)),
    ];
    for (text, current, expected) in rows {
        assert_eq!(
            apply_prosody(Volume, &wide(text), current),
            expected.ok_or(UnsupportedFormat),
            "{text}"
        );
    }
}

#[test]
fn pitch_edges_match_wine() {
    let rows = [
        ("1e2%", 0, Some(24)),
        ("-99.5%", 0, Some(-10)),
        ("-98.5%", 0, Some(-145)),
        ("+0%", 0, Some(0)),
        ("5%x", 0, None),
        ("5Hz", 7, Some(7)),
        ("5Hx", 7, None),
        ("Hz", 7, None),
        ("5hz", 7, None),
        ("1x%", 7, None),
        ("-", 7, None),
        (".", 7, None),
        ("50", 7, None),
        ("1%Hz", 7, None),
        ("-100%", 7, Some(-3)),
        ("-99%", 7, Some(-3)),
        ("x-high", 0, Some(9)),
        ("", 0, None),
        ("-5%", 7, Some(5)),
        ("+41.4%", 12, Some(24)),
        ("high", 0, Some(4)),
    ];
    for (text, current, expected) in rows {
        assert_eq!(
            apply_prosody(Pitch, &wide(text), current),
            expected.ok_or(UnsupportedFormat),
            "{text}"
        );
    }
}
