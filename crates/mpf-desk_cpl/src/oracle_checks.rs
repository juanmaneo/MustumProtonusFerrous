//! Oracle-derived test vectors for the ported desk.cpl logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies from
//! `dlls/desk.cpl/main.c` and `dlls/user32/uitools.c` with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/desk.cpl/`
//! (`oracle.c`, `verbatim.inc`, `shims.h`, `list.h`). The 25 printed rows are all
//! kept: ASan and UBSan reported nothing, so no vectors were excluded.

use super::*;

/// Position and size as `(x, y, width, height)`.
type Bx = (i32, i32, u32, u32);
type RectRow = ((u32, i32, i32, u32, u32), (i32, i32, i32, i32));
type MoveRow = (Vec<Bx>, usize, Bx, Bx, (i32, i32));
type SnapRow = (Vec<Bx>, usize, Bx, (i32, i32));

fn rc(left: i32, top: i32, right: i32, bottom: i32) -> Rect {
    Rect {
        left,
        top,
        right,
        bottom,
    }
}

/// A mode with position and size flagged, as the oracle harness builds them.
fn boxed((x, y, w, h): Bx) -> DevMode {
    DevMode {
        fields: DM_POSITION | DM_PELSWIDTH | DM_PELSHEIGHT,
        position: Point { x, y },
        pels_width: w,
        pels_height: h,
        ..DevMode::default()
    }
}

/// One entry per current rectangle, with the device's pending mode replaced.
fn layout(cur: &[Bx], dev: usize, pending: Bx) -> Vec<Entry> {
    let mut entries: Vec<Entry> = cur
        .iter()
        .map(|&b| {
            let current = boxed(b);
            Entry {
                current,
                pending: current,
            }
        })
        .collect();
    entries[dev].pending = boxed(pending);
    entries
}

#[test]
fn rect_from_devmode_matches_oracle() {
    // (fields, x, y, width, height) -> (left, top, right, bottom)
    let rows: [RectRow; 5] = [
        ((0, 5, 5, 100, 100), (0, 0, 0, 0)),
        ((DM_POSITION, -50, 30, 999, 888), (-50, 30, -50, 30)),
        ((DM_PELSWIDTH, 10, 10, 1920, 1080), (0, 0, 1920, 0)),
        ((DM_PELSHEIGHT, 10, 10, 1920, 1080), (0, 0, 0, 1080)),
        (
            (
                DM_POSITION | DM_PELSWIDTH | DM_PELSHEIGHT,
                -1920,
                -100,
                1920,
                1080,
            ),
            (-1920, -100, 0, 980),
        ),
    ];
    for ((fields, x, y, w, h), (left, top, right, bottom)) in rows {
        let mode = DevMode {
            fields,
            position: Point { x, y },
            pels_width: w,
            pels_height: h,
            ..DevMode::default()
        };
        assert_eq!(rect_from_devmode(&mode), rc(left, top, right, bottom));
    }
}

#[test]
fn is_same_devmode_matches_oracle() {
    let a = DevMode {
        fields: DM_POSITION | DM_PELSWIDTH | DM_PELSHEIGHT,
        pels_width: 1920,
        pels_height: 1080,
        bits_per_pel: 32,
        display_frequency: 60,
        ..DevMode::default()
    };
    // Row 06: only fields and position differ.
    let b = DevMode {
        fields: 0,
        position: Point { x: 500, y: 0 },
        ..a
    };
    assert!(is_same_devmode(&a, &b));
    // Row 07: frequency differs.
    let c = DevMode {
        fields: DM_POSITION,
        display_frequency: 75,
        ..a
    };
    assert!(!is_same_devmode(&a, &c));
}

#[test]
fn move_rect_matches_oracle() {
    // (current rectangles, device, device pending, requested mode) -> pending (x, y)
    let rows: Vec<MoveRow> = vec![
        (
            vec![(0, 0, 1920, 1080), (1920, 0, 1280, 1024)],
            1,
            (1920, 0, 1280, 1024),
            (1500, 100, 1280, 1024),
            (1920, 100),
        ),
        (
            vec![(0, 0, 800, 600), (1000, 0, 800, 600)],
            0,
            (0, 0, 800, 600),
            (900, 50, 800, 600),
            (200, 50),
        ),
        (
            vec![(0, 0, 800, 600), (0, 700, 800, 600)],
            1,
            (0, 700, 800, 600),
            (0, 550, 800, 600),
            (0, 600),
        ),
        (
            vec![(0, 0, 100, 100), (500, 500, 100, 100)],
            1,
            (500, 500, 100, 100),
            (300, 300, 100, 100),
            (300, 300),
        ),
        (
            vec![
                (1000, 0, 100, 100),
                (100, 0, 100, 100),
                (250, 0, 50, 100),
                (350, 0, 100, 100),
            ],
            0,
            (1000, 0, 100, 100),
            (150, 0, 100, 100),
            (450, 0),
        ),
        (
            vec![
                (1000, 0, 100, 100),
                (100, 0, 100, 100),
                (350, 0, 100, 100),
                (250, 0, 50, 100),
            ],
            0,
            (1000, 0, 100, 100),
            (150, 0, 100, 100),
            (1000, 0),
        ),
    ];
    for (cur, dev, pending, mode, expected) in rows {
        let mut entries = layout(&cur, dev, pending);
        move_rect(&mut entries, dev, boxed(mode));
        assert_eq!(
            (
                entries[dev].pending.position.x,
                entries[dev].pending.position.y
            ),
            expected
        );
    }
}

#[test]
fn snap_rect_matches_oracle() {
    // (current rectangles, device, device pending) -> snapped (x, y), for both current and pending
    let rows: Vec<SnapRow> = vec![
        (
            vec![(1920, 0, 1280, 1024), (0, 0, 1920, 1080)],
            0,
            (1930, 40, 1280, 1024),
            (1920, 40),
        ),
        (
            vec![(500, 500, 100, 100), (0, 0, 100, 100)],
            0,
            (500, 500, 100, 100),
            (100, 100),
        ),
        (
            vec![(300, 200, 100, 100)],
            0,
            (300, 200, 100, 100),
            (300, 200),
        ),
        (
            vec![(0, 0, 100, 100), (50, 50, 100, 100)],
            1,
            (50, 50, 100, 100),
            (50, 50),
        ),
        (
            vec![(0, 0, 100, 100), (0, 300, 100, 100)],
            1,
            (120, 350, 100, 100),
            (100, 100),
        ),
    ];
    for (cur, dev, pending, expected) in rows {
        let mut entries = layout(&cur, dev, pending);
        snap_rect(&mut entries, dev);
        let current = (
            entries[dev].current.position.x,
            entries[dev].current.position.y,
        );
        assert_eq!(current, expected);
        assert_eq!(entries[dev].pending, entries[dev].current);
    }
}

#[test]
fn map_virtual_client_rect_matches_oracle() {
    // (rect, client, virtual, scale) -> mapped rect
    let rows: [(Rect, Rect, Rect, f32, Rect); 3] = [
        (
            rc(0, 0, 100, 100),
            rc(0, 0, 200, 200),
            rc(0, 0, 100, 100),
            0.5,
            rc(75, 75, 125, 125),
        ),
        (
            rc(-7, 3, 11, 20),
            rc(10, 20, 110, 120),
            rc(-100, -50, 100, 50),
            0.75,
            rc(55, 72, 68, 85),
        ),
        (
            rc(1, 1, 2, 2),
            rc(0, 0, 1, 1),
            rc(0, 0, 1, 1),
            0.95,
            rc(0, 0, 1, 1),
        ),
    ];
    for (rect, client, virtual_rect, scale, expected) in rows {
        assert_eq!(
            map_virtual_client_rect(rect, client, virtual_rect, scale),
            expected
        );
    }
}

#[test]
fn option_value_matches_oracle() {
    // (value present, first character, default) -> option
    let rows: [(bool, u16, bool, bool); 4] = [
        (true, 0x79, false, true),
        (true, 0x179, true, false),
        (true, 0x6e, true, false),
        (false, 0x79, true, true),
    ];
    for (present, ch, default_value, expected) in rows {
        let value = [ch];
        assert_eq!(
            option_value(present.then_some(&value[..]), default_value),
            expected
        );
    }
}
