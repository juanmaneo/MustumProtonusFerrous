//! Replays the oracle rows through the crate and compares each result with the verbatim Wine
//! output. The oracle is Wine's `geometry.c` and `bitmap.c` built with ASan and UBSan; no row was
//! flagged by the sanitizers.

use super::*;

/// Parses `N` decimal values, as the oracle reads them with `scanf("%f")`.
pub(super) fn floats<const N: usize>(row: &str) -> [f32; N] {
    let mut values = [0.0; N];
    let mut tokens = row.split_whitespace();
    for value in &mut values {
        *value = tokens.next().unwrap().parse().unwrap();
    }
    assert!(tokens.next().is_none(), "extra tokens in {row:?}");
    values
}

pub(super) fn point_at(v: &[f32], i: usize) -> Point {
    Point {
        x: v[i],
        y: v[i + 1],
    }
}

pub(super) fn matrix(v: &[f32]) -> Matrix3x2 {
    Matrix3x2 {
        m11: v[0],
        m12: v[1],
        m21: v[2],
        m22: v[3],
        m31: v[4],
        m32: v[5],
    }
}

#[test]
fn ccw_rows() {
    const ROWS: [(&str, u32); 7] = [
        ("0 0 1 0 0 1", 0x3f80_0000),
        ("0 0 0 1 1 0", 0xbf80_0000),
        ("0 0 1 1 2 2", 0x0000_0000),
        ("0.1 0.1 0.2 0.2 0.3 0.3", 0x0000_0000),
        ("0 0 1 0 2 0", 0x0000_0000),
        ("12345.678 -98.7 0.25 3.5 -7.0 10000", 0xcceb_62f4),
        ("0.5 0.5 12 12 24 24.000001", 0x37b8_0000),
    ];
    for (row, bits) in ROWS {
        let v = floats::<6>(row);
        let got = ccw(point_at(&v, 0), point_at(&v, 2), point_at(&v, 4));
        assert_eq!(got.to_bits(), bits, "{row}");
    }
}

#[test]
fn line_rows() {
    const ROWS: [(&str, bool); 4] = [
        ("0.5 0 0 0 1 0 1 0 0 1 0 0 1 0.1", true),
        ("0.5 2 0 0 1 0 1 0 0 1 0 0 1 0.1", false),
        ("0.5 0.2 0 0 1 0 1 0 0 1 0 0 1 0.1", true),
        ("0 0 0 0 0 0 1 0 0 1 0 0 1 0.1", false),
    ];
    for (row, expected) in ROWS {
        let v = floats::<14>(row);
        let hit = point_on_line_segment(
            point_at(&v, 0),
            point_at(&v, 2),
            point_at(&v, 4),
            &matrix(&v[6..12]),
            v[12],
            v[13],
        );
        assert_eq!(hit, expected, "{row}");
    }
}

#[test]
fn bezier_rows() {
    const ROWS: [(&str, bool); 4] = [
        ("1 1 0 0 0 2 2 2 2 0 1 0 0 1 0 0 0.5 0.1", false),
        ("0.5 0.5 0 0 1 1 2 0 0 0 1 0 0 1 0 0 1 0.1", true),
        ("0.1 0 0 0 0 0 0 0 0 0 1 0 0 1 0 0 1 0.5", false),
        ("1 1 0 0 0 2 2 2 2 0 2 0 0 2 1 0 1 0.1", true),
    ];
    for (row, expected) in ROWS {
        let v = floats::<18>(row);
        let curve = Bezier {
            point1: point_at(&v, 4),
            point2: point_at(&v, 6),
            point3: point_at(&v, 8),
        };
        let hit = point_on_bezier_segment(
            point_at(&v, 0),
            point_at(&v, 2),
            &curve,
            &matrix(&v[10..16]),
            v[16],
            v[17],
        );
        assert_eq!(hit, expected, "{row}");
    }
}

#[test]
fn radius_rows() {
    const ROWS: [(&str, Option<u32>); 3] = [
        ("1 0.000000000001 2", Some(0x4000_0000)),
        ("1 0.000000000001 0.0000001", None),
        ("1 0.000000000001 -3", Some(0x4040_0000)),
    ];
    for (row, expected) in ROWS {
        let v = floats::<3>(row);
        let got = check_radius(v[0], v[1], v[2]).map(f32::to_bits);
        assert_eq!(got, expected, "{row}");
    }
}

#[test]
fn bezier_distance_rows() {
    const ROWS: [(&str, u32); 2] = [("0.5 1", 0x3eb6_eb84), ("-1 0", 0xbfaa_aaab)];
    for (row, bits) in ROWS {
        let v = floats::<2>(row);
        let got = bezier_distance(v[0], v[1] != 0.0);
        assert_eq!(got.to_bits(), bits, "{row}");
    }
}

#[test]
fn format_rows() {
    const ROWS: [(&str, bool); 3] = [("87 3", true), ("65 2", true), ("87 2", false)];
    for (row, expected) in ROWS {
        let v: Vec<u32> = row.split_whitespace().map(|t| t.parse().unwrap()).collect();
        let format = PixelFormat {
            format: v[0],
            alpha_mode: v[1],
        };
        assert_eq!(format_supported(format), expected, "{row}");
    }
}

#[test]
fn option_rows() {
    const ROWS: [(u32, bool); 2] = [(4, false), (9, true)];
    for (options, expected) in ROWS {
        assert_eq!(check_bitmap_options(options), expected, "{options}");
    }
}
