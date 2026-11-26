//! Port of the portable parts of `dlls/joy.cpl/xinput.c`: the thumbstick and trigger
//! drawings, and the layout of the gamepad panel (`paint_gamepad_view`).

use crate::{mul_div, Point, Rect, Shape};

const DPAD_UP: u16 = 0x0001;
const DPAD_DOWN: u16 = 0x0002;
const DPAD_LEFT: u16 = 0x0004;
const DPAD_RIGHT: u16 = 0x0008;
const START: u16 = 0x0010;
const BACK: u16 = 0x0020;
const LEFT_SHOULDER: u16 = 0x0100;
const RIGHT_SHOULDER: u16 = 0x0200;
const A: u16 = 0x1000;
const B: u16 = 0x2000;
const X: u16 = 0x4000;
const Y: u16 = 0x8000;

/// `draw_axis_view`: a thumbstick. `dx` and `dy` are the raw stick values, from -32768 to
/// 32767. The knob is drawn at the centre plus the scaled position.
pub fn draw_axis_view(rect: Rect, dx: i16, dy: i16) -> Vec<Shape> {
    let centre = Point::new(rect.centre_x(), rect.centre_y());
    let knob = Point::new(
        centre.x.wrapping_add(mul_div(
            i32::from(dx),
            rect.width().wrapping_sub(20),
            0xffff,
        )),
        centre.y.wrapping_sub(mul_div(
            i32::from(dy),
            rect.height().wrapping_sub(20),
            0xffff,
        )),
    );
    vec![
        Shape::Ellipse(rect),
        Shape::Line {
            from: centre.offset(0, -3),
            to: centre.offset(0, 4),
        },
        Shape::Line {
            from: centre.offset(-3, 0),
            to: centre.offset(4, 0),
        },
        Shape::Line {
            from: centre,
            to: knob,
        },
        Shape::Ellipse(knob_box(knob)),
    ]
}

/// The 8x8 box around a thumbstick knob.
fn knob_box(knob: Point) -> Rect {
    Rect::new(
        knob.x.wrapping_sub(4),
        knob.y.wrapping_sub(4),
        knob.x.wrapping_add(4),
        knob.y.wrapping_add(4),
    )
}

/// `draw_trigger_view`: a trigger bar. `dt` is the raw trigger value, from 0 to 255. The bar
/// rises from the bottom of the rect, and a tick marks the centre line.
pub fn draw_trigger_view(rect: Rect, dt: u8) -> Vec<Shape> {
    let centre = Point::new(rect.centre_x(), rect.centre_y());
    let w = rect.width().wrapping_add(1) / 3;
    let y = rect
        .bottom
        .wrapping_sub(w.wrapping_add(1) / 2)
        .wrapping_sub(mul_div(i32::from(dt), rect.height().wrapping_sub(w), 0xff));
    let mut shapes = vec![Shape::RoundRect(rect)];
    if y > centre.y {
        shapes.push(horizontal_tick(centre));
    }
    shapes.push(Shape::Rectangle(Rect::new(
        centre.x.wrapping_sub(w),
        y,
        centre.x.wrapping_add(w),
        rect.bottom,
    )));
    if y < centre.y {
        shapes.push(horizontal_tick(centre));
    }
    shapes
}

fn horizontal_tick(centre: Point) -> Shape {
    Shape::Line {
        from: centre.offset(-3, 0),
        to: centre.offset(3, 0),
    }
}

/// One button cell of the gamepad panel: its box, its label, and the `wButtons` bit that
/// lights it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GamepadButton {
    pub rect: Rect,
    pub label: &'static str,
    pub mask: u16,
}

/// The gamepad panel of `paint_gamepad_view`. `sticks[0]` is the left thumbstick, lit by
/// `XINPUT_GAMEPAD_LEFT_THUMB` (0x0040), and `sticks[1]` is the right one, lit by
/// `XINPUT_GAMEPAD_RIGHT_THUMB` (0x0080). `buttons` are in the order the C code draws them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GamepadLayout {
    pub sticks: [Rect; 2],
    pub triggers: [Rect; 2],
    pub buttons: [GamepadButton; 13],
}

/// `paint_gamepad_view`: the layout of the sticks, triggers and buttons inside `client`.
/// Sizes use the same unsigned arithmetic as the C code, so narrow clients wrap.
pub fn gamepad_view_layout(client: Rect) -> GamepadLayout {
    let axis = client.height() as u32;
    let button = axis.wrapping_sub(1) / 3;
    let trigger = axis / 4;
    let space = (client.width() as u32)
        .wrapping_sub(axis.wrapping_mul(2))
        .wrapping_sub(trigger.wrapping_mul(2))
        .wrapping_sub(button.wrapping_mul(5))
        / 10;

    let axis_i = axis as i32;
    let trigger_i = trigger as i32;
    let button_i = button as i32;
    let space_i = space as i32;
    let left_stick = client.left.wrapping_add(space_i);
    let right_stick = left_stick.wrapping_add(axis_i).wrapping_add(space_i);
    let left_trigger = right_stick.wrapping_add(axis_i).wrapping_add(space_i);
    let right_trigger = left_trigger.wrapping_add(trigger_i).wrapping_add(space_i);
    let base = right_trigger.wrapping_add(trigger_i).wrapping_add(space_i);
    let half = (button.wrapping_add(space) / 2) as i32;
    let step = button_i.wrapping_add(space_i);
    let upper = base.wrapping_add(half);
    let middle = client.top.wrapping_add(button_i);
    let bottom = middle.wrapping_add(button_i);

    let at = |x: i32, n: i32| x.wrapping_add(step.wrapping_mul(n));
    let column =
        |x: i32, width: i32| Rect::new(x, client.top, x.wrapping_add(width), client.bottom);
    let cell = |x: i32, y: i32, label: &'static str, mask: u16| GamepadButton {
        rect: Rect::new(x, y, x.wrapping_add(button_i), y.wrapping_add(button_i)),
        label,
        mask,
    };

    GamepadLayout {
        sticks: [column(left_stick, axis_i), column(right_stick, axis_i)],
        triggers: [
            column(left_trigger, trigger_i),
            column(right_trigger, trigger_i),
        ],
        buttons: [
            cell(at(upper, 0), client.top, "^", DPAD_UP),
            cell(at(upper, 1), client.top, "L", LEFT_SHOULDER),
            cell(at(upper, 2), client.top, "R", RIGHT_SHOULDER),
            cell(at(upper, 3), client.top, "Y", Y),
            cell(at(base, 0), middle, "<", DPAD_LEFT),
            cell(at(base, 1), middle, ">", DPAD_RIGHT),
            cell(at(base, 2), middle, "@", 0x0400),
            cell(at(base, 3), middle, "X", X),
            cell(at(base, 4), middle, "B", B),
            cell(at(upper, 0), bottom, "v", DPAD_DOWN),
            cell(at(upper, 1), bottom, "#", BACK),
            cell(at(upper, 2), bottom, "=", START),
            cell(at(upper, 3), bottom, "A", A),
        ],
    }
}
