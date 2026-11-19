//! Edge cases that the committed oracle rows do not reach. Every expected value is replayed from
//! the verbatim Wine oracle (`geometry.c` and `bitmap.c` built with ASan and UBSan), so the rows are
//! oracle-derived. The trait checks at the end cover the derived impls that the rows do not call.

use super::oracle_checks::{floats, matrix, point_at};
use super::*;

/// Replays `row` through `fast_expansion_sum_zeroelim`, formatted like the oracle's `fes` output.
fn expansion_sum(row: &str) -> String {
    let t: Vec<&str> = row.split_whitespace().collect();
    let na: usize = t[0].parse().unwrap();
    let a: Vec<f32> = t[1..=na].iter().map(|x| x.parse().unwrap()).collect();
    let nb: usize = t[na + 1].parse().unwrap();
    let b: Vec<f32> = t[na + 2..].iter().map(|x| x.parse().unwrap()).collect();
    assert_eq!(b.len(), nb, "{row}");
    let mut out = [0.0_f32; 16];
    let n = expansion::fast_expansion_sum_zeroelim(&mut out, &a, &b);
    std::iter::once(n.to_string())
        .chain(out[..n].iter().map(|x| format!("{:08x}", x.to_bits())))
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn ccw_edge_rows() {
    const ROWS: &[(&str, u32)] = &[
        ("0 0 -1 0 0 1", 0xbf80_0000),
        ("0 0 1 1 2 1", 0xbf80_0000),
        ("0 0 1 1 1 2", 0x3f80_0000),
        ("0 0 1 1 -1 -1.5", 0xbf00_0000),
        ("0 0 0 -1 1 0", 0x3f80_0000),
        ("0 0 1 1 -1 -1", 0x0000_0000),
        ("0.1 0.1 0.3 0.3 0.5 0.5", 0x0000_0000),
        ("0.1 0.1 0.3 0.3 0.5 0.50000006", 0x324c_ccce),
        ("0.1 0.3 0.3 0.1 0.7 0.7", 0x3e4c_ccce),
        ("0.3 0.1 0.7 0.3 1.1 0.5", 0xb2b3_3333),
        ("0.1 0.2 0.7 1.4 0.3 0.6", 0x0000_0000),
        ("0.1 0.2 0.7 1.4 0.30000001 0.6", 0x0000_0000),
        ("1.1 2.2 3.3 6.6 5.5 11", 0x0000_0000),
        ("0.7 0.3 0.1 0.9 0.5 0.6", 0xbd75_c292),
        ("12.5 0.1 -33.3 7.7 2.2 0.05", 0x42a1_23d7),
        ("0 0 1 0 1 1", 0x3f80_0000),
        ("0 0 1 1 1 0", 0xbf80_0000),
        ("0 0 0 1 1 1", 0xbf80_0000),
        ("0 0 -1 -1 -2 -1", 0xbf80_0000),
        ("0 0 -1 -1 -1 -2", 0x3f80_0000),
        ("0 0 -1 0 0 -1", 0x3f80_0000),
        ("0 0 1.00000036 1 1 1", 0x34c0_0000),
    ];
    for (row, bits) in ROWS {
        let v = floats::<6>(row);
        let got = ccw(point_at(&v, 0), point_at(&v, 2), point_at(&v, 4));
        assert_eq!(got.to_bits(), *bits, "{row}");
    }
}

#[test]
fn line_edge_rows() {
    const ROWS: &[(&str, bool)] = &[
        ("0.5 0.5 0 0 1 0 1 0 0 1 0 0 1 0.1", true),
        ("0.5 0.5 0 0 1 0 1 0 0 -1 0 0 1 0.1", true),
        ("0.5 0.5 0 0 1 0 1 0 0 1 0 0 -1 0.1", true),
        ("0.5 0.5 0 0 1 0 0 0 0 0 0 0 1 0.1", true),
        ("0.5 0.5 0 0 1 0 0 0 0 0 0 0 1 0", false),
        ("0.5 -1.05 0 0 1 0 1 0 0 1 0 0 1 0.1", true),
        ("0.5 -1.5 0 0 1 0 1 0 0 1 0 0 1 0.1", false),
        ("0.5 1.05 0 0 1 0 1 0 0 1 0 0 1 0.1", true),
        ("0.5 1.5 0 0 1 0 1 0 0 1 0 0 1 0.1", false),
        ("-0.5 0.5 0 0 1 0 1 0 0 1 0 0 1 0.1", false),
        ("1.5 0.5 0 0 1 0 1 0 0 1 0 0 1 0.1", false),
        ("0.5 0.5 0 0 2 0 1 0 0 1 0 0 1 0.1", true),
        ("1.5 0.3 0 0 3 0 1 0 0 1 0 0 1 0.1", true),
        ("1.5 0.3 0 0 3 0 1 0 0 1 0 0 1 0.5", true),
        ("0.5 0.2 0 0 1 0 1 0 0 1 0 0 1 0.1", true),
        ("3 2.5 0 0 1 0 1 0 1 1 0 0 1 0.1", false),
        ("-1.2 -1.5 0 0 1 0 1 0 1 1 0 0 1 0.1", false),
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
        assert_eq!(hit, *expected, "{row}");
    }
}

#[test]
fn bezier_edge_rows() {
    const ROWS: &[(&str, bool)] = &[
        ("0 0 0 0 1 0 2 0 0 0 1 0 0 1 0 0 1 0.1", true),
        ("0.5 0 0 0 1 0 1 0 0 0 1 0 0 1 0 0 0.5 0.1", true),
        ("0.5 2 0 0 1 0 1 0 0 0 1 0 0 1 0 0 0.5 0.1", false),
        ("0 0 0 0 0 0 0 0 0 0 1 0 0 1 0 0 1 0.1", false),
        (
            "0.5 0 0 0 0.3333333 0 0.6666667 0 1 0 1 0 0 1 0 0 1 0.1",
            true,
        ),
        (
            "0.5 3 0 0 0.3333333 0 0.6666667 0 1 0 1 0 0 1 0 0 1 0.1",
            false,
        ),
        (
            "0.5 0.2 0 0 0.3333333 0 0.6666667 0 1 0 1 0 0 1 0 0 1 0.001",
            true,
        ),
        ("1 1.5 0 0 0 2 2 2 2 0 1 0 0 1 0 0 0.5 0.1", true),
        ("1 1.9 0 0 0 2 2 2 2 0 1 0 0 1 0 0 0.5 0.1", false),
        ("1 0.2 0 0 0 2 2 2 2 0 1 0 0 1 0 0 0.5 0.1", false),
        ("1 1.5 0 0 0 2 2 2 2 0 1 0 0 1 0 0 0.05 0.01", true),
        ("1 -1 0 0 0 2 2 2 2 0 1 0 0 1 0 0 0.5 0.1", false),
        ("2 3 0 0 0 2 2 2 2 0 2 1 0 0 1 1 0.5 0.1", false),
        ("1.5 1.5 0 0 0 2 2 2 2 0 2 1 0 0 1 1 0.5 0.1", false),
        ("1 0.5 0 0 0 2 2 2 2 0 1 0 0 1 0 0 -0.5 0.1", false),
        ("1 1.5 0 0 0 2 2 2 2 0 1 0 0 1 0 0 2 0.1", true),
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
        assert_eq!(hit, *expected, "{row}");
    }
}

#[test]
fn radius_edge_rows() {
    const ROWS: &[(&str, Option<u32>)] = &[
        ("0 0 0.5", Some(0x3f00_0000)),
        ("0 0 -0.5", Some(0x3f00_0000)),
        ("1 0 0", None),
        ("1 0 -0", None),
        ("2 0.5 -1", None),
        ("0 1 -3", Some(0x4040_0000)),
    ];
    for (row, expected) in ROWS {
        let v = floats::<3>(row);
        let got = check_radius(v[0], v[1], v[2]).map(f32::to_bits);
        assert_eq!(got, *expected, "{row}");
    }
}

#[test]
fn bezier_distance_edge_rows() {
    const ROWS: &[(&str, u32)] = &[
        ("-2 1", 0x0000_0000),
        ("1 1", 0x0000_0000),
        ("1 0", 0x0000_0000),
        ("0 1", 0x3f0d_6289),
        ("-0.9 0", 0xbf87_f25a),
    ];
    for (row, bits) in ROWS {
        let v = floats::<2>(row);
        let got = bezier_distance(v[0], v[1] != 0.0);
        assert_eq!(got.to_bits(), *bits, "{row}");
    }
}

#[test]
fn expansion_edge_rows() {
    const ROWS: &[(&str, &str)] = &[
        ("1 1 1 -1", "1 00000000"),
        ("1 0 1 0", "1 00000000"),
        ("1 1 1 2", "1 40400000"),
        ("1 1 1 0.5", "1 3fc00000"),
        ("2 0.25 2 2 -0.25 2", "1 40800000"),
        ("2 1 2 1 2", "1 40a00000"),
        ("2 1 2 2 -1 -2", "1 00000000"),
        ("2 0.5 1 1 -1", "1 3f000000"),
        ("2 1 2 1 3", "1 40c00000"),
        ("3 1 2 4 3 3 1 -1", "1 41200000"),
        ("1 1 3 -1 -2 -3", "1 c0a00000"),
        ("2 2 0.5 1 -0.25", "1 40100000"),
        ("3 0.25 1 2 2 0.5 1", "1 40980000"),
        ("2 0.0000001 1 1 -1.00000012", "1 b2a501ac"),
    ];
    for (row, expected) in ROWS {
        assert_eq!(expansion_sum(row), *expected, "{row}");
    }
}

#[test]
fn value_types_compare_clone_and_format() {
    let p = Point { x: 1.0, y: -2.0 };
    let m = Matrix3x2 {
        m11: 1.0,
        m12: 0.0,
        m21: 0.0,
        m22: 1.0,
        m31: 0.5,
        m32: -0.5,
    };
    let curve = Bezier {
        point1: p,
        point2: Point { x: 0.0, y: 0.0 },
        point3: p,
    };
    let pixel = PixelFormat {
        format: DXGI_FORMAT_B8G8R8A8_UNORM,
        alpha_mode: D2D1_ALPHA_MODE_IGNORE,
    };

    assert_eq!(p.clone(), p);
    assert_ne!(p, Point { x: 1.0, y: 2.0 });
    assert_eq!(m.clone(), m);
    assert_ne!(m, Matrix3x2 { m32: 0.5, ..m });
    assert_eq!(curve.clone(), curve);
    assert_ne!(
        curve,
        Bezier {
            point3: Point { x: 0.0, y: 0.0 },
            ..curve
        }
    );
    assert_eq!(pixel.clone(), pixel);
    assert_ne!(
        pixel,
        PixelFormat {
            alpha_mode: D2D1_ALPHA_MODE_STRAIGHT,
            ..pixel
        }
    );
    assert_eq!(format!("{p:?}"), "Point { x: 1.0, y: -2.0 }");
    assert!(format!("{curve:?}").starts_with("Bezier { point1: Point"));
    assert!(format!("{m:?}").starts_with("Matrix3x2 { m11: 1.0"));
    assert!(format!("{pixel:?}").starts_with("PixelFormat { format: 87"));
}
