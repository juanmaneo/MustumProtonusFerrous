//! Boundary and branch checks. None of the expectations here is an oracle row. Each one is
//! derived by reading the Wine C source, or is this port's defined behaviour where the C code
//! has undefined behaviour. Those cases are marked "non-oracle" in their comments.

use crate::{dinput, mul_div, xinput, Point, Rect, Shape};

#[test]
fn mul_div_zero_divisor_is_minus_one() {
    // Source-derived: MulDiv returns -1 when nDenominator is 0.
    assert_eq!(mul_div(5, 3, 0), -1);
    assert_eq!(mul_div(0, 0, 0), -1);
}

#[test]
fn mul_div_negative_divisor_flips_the_number() {
    // Non-oracle, source-derived: a negative divisor negates the multiplicand first.
    assert_eq!(mul_div(6, 2, -3), -4);
    assert_eq!(mul_div(-6, 2, -3), 4);
}

#[test]
fn mul_div_rounds_halves_away_from_zero() {
    // Non-oracle, source-derived: half the divisor is added or subtracted before truncation.
    assert_eq!(mul_div(1, 1, 2), 1);
    assert_eq!(mul_div(-1, 1, 2), -1);
    assert_eq!(mul_div(1, -1, 2), -1);
    assert_eq!(mul_div(-1, -1, 2), 1);
    assert_eq!(mul_div(1, 1, 3), 0);
    assert_eq!(mul_div(2, 1, 3), 1);
}

#[test]
fn mul_div_result_limits() {
    // Non-oracle, source-derived: results in -i32::MAX..=i32::MAX are returned, anything else
    // gives -1.
    assert_eq!(mul_div(i32::MAX, 1, 1), i32::MAX);
    assert_eq!(mul_div(i32::MAX, -1, 1), -i32::MAX);
    assert_eq!(mul_div(i32::MAX, i32::MAX, i32::MAX), i32::MAX);
    assert_eq!(mul_div(i32::MIN, 1, 1), -1);
    assert_eq!(mul_div(i32::MAX, 2, 1), -1);
}

#[test]
fn mul_div_min_divisor_is_negated_with_wrapping() {
    // Non-oracle: the C code negates a negative divisor, which is undefined for i32::MIN
    // (the sanitizers flag the oracle row). This port wraps the negation of the multiplicand:
    // i32::MIN stays i32::MIN, and the quotient -2^31 is out of range, so -1.
    assert_eq!(mul_div(i32::MIN, 1, -1), -1);
}

#[test]
fn pov_wedge_at_sector_boundaries() {
    // Non-oracle, derived from the points table: the wedge starts at points[value / 4500 * 2].
    let rect = Rect::new(0, 0, 100, 100);
    let rim_0 = Point::new(29, 0);
    let centre = Point::new(50, 50);
    let rim_90 = Point::new(71, 0);
    assert_eq!(
        dinput::draw_pov_view(rect, 4499).last(),
        Some(&Shape::Polygon([rim_0, centre, rim_90]))
    );
    assert_eq!(
        dinput::draw_pov_view(rect, 4500).last(),
        Some(&Shape::Polygon([rim_90, centre, Point::new(100, 29)]))
    );
    assert_eq!(
        dinput::draw_pov_view(rect, 40499).last(),
        Some(&Shape::Polygon([rim_0, centre, rim_0]))
    );
}

#[test]
fn pov_out_of_table_has_no_wedge() {
    // Non-oracle: for value >= 40500 the C code reads past points[] (the sanitizers flag the
    // oracle row). This port omits the wedge, so only the nine spokes are returned.
    let rect = Rect::new(0, 0, 100, 100);
    assert_eq!(dinput::draw_pov_view(rect, 40500).len(), 9);
    assert_eq!(dinput::draw_pov_view(rect, u32::MAX - 1).len(), 9);
}

#[test]
fn axis_view_centre_draws_no_tick() {
    // Non-oracle: x = 27 + 0.5 * 167 = 110.5 truncates to 110, the centre x, so neither tick
    // is drawn.
    assert_eq!(
        dinput::draw_axis_view(Rect::new(0, 0, 200, 40), 0.5),
        [
            Shape::RoundRect(Rect::new(20, 0, 200, 40)),
            Shape::Rectangle(Rect::new(20, 13, 110, 27)),
        ]
    );
}

#[test]
fn axis_view_ends_of_travel() {
    // Non-oracle: 0.0 puts the bar end at x = 27, left of the centre; 1.0 puts it at x = 194,
    // right of the centre. The tick goes before the bar on the left and after it on the right.
    assert_eq!(
        dinput::draw_axis_view(Rect::new(0, 0, 200, 40), 0.0),
        [
            Shape::RoundRect(Rect::new(20, 0, 200, 40)),
            Shape::Line {
                from: Point::new(110, 17),
                to: Point::new(110, 23),
            },
            Shape::Rectangle(Rect::new(20, 13, 27, 27)),
        ]
    );
    assert_eq!(
        dinput::draw_axis_view(Rect::new(0, 0, 200, 40), 1.0),
        [
            Shape::RoundRect(Rect::new(20, 0, 200, 40)),
            Shape::Rectangle(Rect::new(20, 13, 194, 27)),
            Shape::Line {
                from: Point::new(110, 17),
                to: Point::new(110, 23),
            },
        ]
    );
}

#[test]
fn axis_view_zero_size_rect() {
    // Non-oracle: w = 0 and the travel is -20, so the bar end x = 20 is right of the centre
    // x = 10. The rect is inverted, and the shapes still follow the same order.
    assert_eq!(
        dinput::draw_axis_view(Rect::new(0, 0, 0, 0), 0.0),
        [
            Shape::RoundRect(Rect::new(20, 0, 0, 0)),
            Shape::Rectangle(Rect::new(20, 0, 20, 0)),
            Shape::Line {
                from: Point::new(10, -3),
                to: Point::new(10, 3),
            },
        ]
    );
}

#[test]
fn button_view_width_threshold() {
    // Non-oracle: an inverted rect has a negative width, which is below 16, so it is a square.
    assert_eq!(
        dinput::draw_button_view(Rect::new(0, 0, -5, 10)),
        Shape::Rectangle(Rect::new(0, 0, -5, 10))
    );
}

#[test]
fn axes_view_rects_narrow_client() {
    // Non-oracle: a 10x10 client makes the axis width -5 and the height 0. The rects are
    // inverted, not clamped.
    let rects = dinput::axes_view_rects(Rect::new(0, 0, 10, 10));
    assert_eq!(rects[0], Rect::new(5, 2, 0, 2));
    assert_eq!(rects[1], Rect::new(5, 4, 0, 4));
    assert_eq!(rects[4], Rect::new(10, 2, 5, 2));
}

#[test]
fn povs_view_rects_tiny_client() {
    // Non-oracle: size = (10 - 5) / 2 - 5 = -3 and the step is 2, so the squares are inverted.
    assert_eq!(
        dinput::povs_view_rects(Rect::new(0, 0, 10, 10)),
        [
            Rect::new(5, 5, 2, 2),
            Rect::new(7, 5, 4, 2),
            Rect::new(5, 7, 2, 4),
            Rect::new(7, 7, 4, 4),
        ]
    );
}

#[test]
fn buttons_view_cells_empty() {
    // Non-oracle: no buttons, no cells.
    assert!(dinput::buttons_view_cells(Rect::new(0, 0, 300, 100), &[]).is_empty());
}

#[test]
fn buttons_view_cells_step_boundary() {
    // Non-oracle, derived from the C layout: 48 buttons use 16-pixel decimal cells, and 49
    // switch to 24-pixel hex cells.
    let client = Rect::new(0, 0, 300, 100);
    let decimal = dinput::buttons_view_cells(client, &[0; 48]);
    assert_eq!(decimal[47].rect, Rect::new(275, 41, 291, 57));
    assert_eq!(decimal[47].label, "47");
    let hex = dinput::buttons_view_cells(client, &[0; 49]);
    assert_eq!(hex[48].rect, Rect::new(5, 29, 15, 39));
    assert_eq!(hex[48].label, "30");
}

#[test]
fn buttons_view_cells_zero_client_wraps() {
    // Non-oracle: a zero-width client makes the unsigned cell size wrap to 268435455 (the
    // decimal step of 16), so the cells sit far to the right.
    let cells = dinput::buttons_view_cells(Rect::new(0, 0, 0, 0), &[0, 1]);
    assert_eq!(cells[0].rect, Rect::new(7, 7, 268435460, 268435460));
    assert_eq!(cells[1].rect, Rect::new(268435462, 7, 536870915, 268435460));
    assert_eq!(cells[1].label, "1");
}

#[test]
fn gamepad_view_layout_400x60() {
    // Non-oracle, derived from the C sizes: axis 60, button 19, trigger 15, and the spacing
    // (400 - 120 - 30 - 95) / 10 = 15.
    let layout = xinput::gamepad_view_layout(Rect::new(0, 0, 400, 60));
    assert_eq!(
        layout.sticks,
        [Rect::new(15, 0, 75, 60), Rect::new(90, 0, 150, 60)]
    );
    assert_eq!(
        layout.triggers,
        [Rect::new(165, 0, 180, 60), Rect::new(195, 0, 210, 60)]
    );
    let buttons: Vec<(&str, Rect)> = layout
        .buttons
        .iter()
        .map(|button| (button.label, button.rect))
        .collect();
    assert_eq!(
        buttons,
        [
            ("^", Rect::new(242, 0, 261, 19)),
            ("L", Rect::new(276, 0, 295, 19)),
            ("R", Rect::new(310, 0, 329, 19)),
            ("Y", Rect::new(344, 0, 363, 19)),
            ("<", Rect::new(225, 19, 244, 38)),
            (">", Rect::new(259, 19, 278, 38)),
            ("@", Rect::new(293, 19, 312, 38)),
            ("X", Rect::new(327, 19, 346, 38)),
            ("B", Rect::new(361, 19, 380, 38)),
            ("v", Rect::new(242, 38, 261, 57)),
            ("#", Rect::new(276, 38, 295, 57)),
            ("=", Rect::new(310, 38, 329, 57)),
            ("A", Rect::new(344, 38, 363, 57)),
        ]
    );
}

#[test]
fn stick_at_rest_puts_knob_on_centre() {
    // Non-oracle, derived from the C code: dx = dy = 0 puts the knob at the centre, so its
    // line has zero length.
    assert_eq!(
        xinput::draw_axis_view(Rect::new(0, 0, 100, 100), 0, 0),
        [
            Shape::Ellipse(Rect::new(0, 0, 100, 100)),
            Shape::Line {
                from: Point::new(50, 47),
                to: Point::new(50, 54),
            },
            Shape::Line {
                from: Point::new(47, 50),
                to: Point::new(54, 50),
            },
            Shape::Line {
                from: Point::new(50, 50),
                to: Point::new(50, 50),
            },
            Shape::Ellipse(Rect::new(46, 46, 54, 54)),
        ]
    );
}

#[test]
fn trigger_at_centre_draws_no_tick() {
    // Non-oracle, derived from the C code: dt = 127 puts the bar top at y = 50, the centre, so
    // neither tick is drawn.
    assert_eq!(
        xinput::draw_trigger_view(Rect::new(0, 0, 30, 100), 127),
        [
            Shape::RoundRect(Rect::new(0, 0, 30, 100)),
            Shape::Rectangle(Rect::new(5, 50, 25, 100)),
        ]
    );
}

#[test]
fn point_and_rect_arithmetic_wraps() {
    // Non-oracle: the port wraps explicitly where the C code uses LONG arithmetic.
    assert_eq!(
        Point::new(i32::MAX, 0).offset(1, -1),
        Point::new(i32::MIN, -1)
    );
    let rect = Rect::new(i32::MIN, i32::MIN, i32::MAX, i32::MAX);
    assert_eq!(rect.width(), -1);
    assert_eq!(rect.height(), -1);
    assert_eq!(rect.centre_x(), 0);
    assert_eq!(rect.centre_y(), 0);
    assert_eq!(
        rect.offset(1, 1),
        Rect::new(i32::MIN + 1, i32::MIN + 1, i32::MIN, i32::MIN)
    );
}

#[test]
fn extreme_rects_do_not_panic() {
    // Non-oracle: the C arithmetic overflows for these rects. The port wraps and must not panic.
    let extreme = Rect::new(i32::MIN, i32::MIN, i32::MAX, i32::MAX);
    assert!(!dinput::draw_axis_view(extreme, 1.0).is_empty());
    assert!(!xinput::draw_axis_view(extreme, i16::MAX, i16::MIN).is_empty());
    assert!(!xinput::draw_trigger_view(extreme, u8::MAX).is_empty());
    assert!(!dinput::draw_pov_view(extreme, 0).is_empty());
    assert_eq!(dinput::draw_button_view(extreme), Shape::Rectangle(extreme));
    assert_eq!(dinput::axes_view_rects(extreme).len(), 8);
    assert_eq!(dinput::povs_view_rects(extreme).len(), 4);
    assert_eq!(dinput::buttons_view_cells(extreme, &[0, 1]).len(), 2);
    let layout = xinput::gamepad_view_layout(extreme);
    assert_eq!(layout.sticks[0].top, i32::MIN);
    assert_eq!(layout.sticks[0].bottom, i32::MAX);
}
