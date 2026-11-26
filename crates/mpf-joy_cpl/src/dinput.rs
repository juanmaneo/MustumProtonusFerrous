//! Port of the portable parts of `dlls/joy.cpl/dinput.c`: the DirectInput axis, POV and
//! button drawings, and the layouts of the axes, POV and button panels.

use crate::{Point, Rect, Shape};

/// `draw_axis_view`: one axis as a slider. `value` is the position from `0.0` to `1.0`.
pub fn draw_axis_view(rect: Rect, value: f64) -> Vec<Shape> {
    let centre = Point::new(rect.centre_x().wrapping_add(10), rect.centre_y());
    let w = rect.height().wrapping_add(1) / 3;
    let travel = rect.width().wrapping_sub(20).wrapping_sub(w);
    // C truncates the double to LONG; out-of-range values are undefined there.
    let x = (f64::from(
        rect.left
            .wrapping_add(20)
            .wrapping_add(w.wrapping_add(1) / 2),
    ) + value * f64::from(travel)) as i32;
    let track = Rect::new(
        rect.left.wrapping_add(20),
        rect.top,
        rect.right,
        rect.bottom,
    );

    let mut shapes = vec![Shape::RoundRect(track)];
    if x < centre.x {
        shapes.push(vertical_tick(centre));
    }
    shapes.push(Shape::Rectangle(Rect::new(
        track.left,
        rect.top.wrapping_add(w),
        x,
        rect.bottom.wrapping_sub(w),
    )));
    if x > centre.x {
        shapes.push(vertical_tick(centre));
    }
    shapes
}

fn vertical_tick(centre: Point) -> Shape {
    Shape::Line {
        from: centre.offset(0, -3),
        to: centre.offset(0, 3),
    }
}

/// `draw_pov_view`: an 8-way POV hat. `value` is in hundredths of a degree, or `u32::MAX`
/// when centred. The result is eight spokes from the centre to the rim, then the wedge
/// for `value`.
pub fn draw_pov_view(rect: Rect, value: u32) -> Vec<Shape> {
    let points = pov_points(rect);
    let mut shapes: Vec<Shape> = points
        .as_chunks::<2>()
        .0
        .iter()
        .map(|&[a, b]| Shape::Line {
            from: midpoint(a, b),
            to: a,
        })
        .collect();
    if value != POV_CENTRED {
        let start = (value / 4500 * 2) as usize;
        // The C code reads `points[start..start + 3]` without a bounds check, which is
        // undefined for `value >= 40500`. Here that wedge is omitted.
        if let Some(&[a, b, c]) = points.get(start..start + 3) {
            shapes.push(Shape::Polygon([a, b, c]));
        }
    }
    shapes
}

/// `DWORD` `-1` in `draw_pov_view`: the hat is centred and no wedge is drawn.
const POV_CENTRED: u32 = u32::MAX;

/// The `points[]` table of `draw_pov_view`: for each 45° sector, its rim point and the
/// centre. The final rim point repeats the 0° one, as in the C table.
fn pov_points(rect: Rect) -> [Point; 19] {
    let at = |x: (f64, f64), y: (f64, f64)| {
        Point::new(
            blend(rect.left, rect.right, x),
            blend(rect.top, rect.bottom, y),
        )
    };
    let centre = at((0.5, 0.5), (0.5, 0.5));
    [
        at((0.71, 0.29), (1.00, 0.00)),
        centre,
        at((0.29, 0.71), (1.00, 0.00)),
        centre,
        at((0.00, 1.00), (0.71, 0.29)),
        centre,
        at((0.00, 1.00), (0.29, 0.71)),
        centre,
        at((0.29, 0.71), (0.00, 1.00)),
        centre,
        at((0.71, 0.29), (0.00, 1.00)),
        centre,
        at((1.00, 0.00), (0.29, 0.71)),
        centre,
        at((1.00, 0.00), (0.71, 0.29)),
        centre,
        at((0.71, 0.29), (1.00, 0.00)),
        centre,
        at((0.71, 0.29), (1.00, 0.00)),
    ]
}

/// `round(a * wa + b * wb)`: the expression the C table uses for each coordinate.
fn blend(a: i32, b: i32, (wa, wb): (f64, f64)) -> i32 {
    (f64::from(a) * wa + f64::from(b) * wb).round() as i32
}

fn midpoint(a: Point, b: Point) -> Point {
    Point::new(a.x.wrapping_add(b.x) / 2, a.y.wrapping_add(b.y) / 2)
}

/// `draw_button_view`: a circle, or a square when the cell is narrower than 16 pixels.
/// The button label is drawn as text in C and is not part of the shape.
pub fn draw_button_view(rect: Rect) -> Shape {
    if rect.width() < 16 {
        Shape::Rectangle(rect)
    } else {
        Shape::Ellipse(rect)
    }
}

/// The rectangle of each axis in `paint_axes_view`, indexed by axis number. The C code draws
/// them in the order 0, 4, 1, 5, 2, 6, 3, 7.
pub fn axes_view_rects(client: Rect) -> [Rect; 8] {
    let width = client.width() / 2 - 10;
    let height = client.height().wrapping_sub(2) / 4 - 2;
    std::array::from_fn(|axis| {
        let column = (axis / 4) as i32;
        let row = (axis % 4) as i32;
        let left = client
            .left
            .wrapping_add(5)
            .wrapping_add(column.wrapping_mul(width.wrapping_add(10)));
        let top = client
            .top
            .wrapping_add(2)
            .wrapping_add(row.wrapping_mul(height.wrapping_add(2)));
        Rect::new(
            left,
            top,
            left.wrapping_add(width),
            top.wrapping_add(height),
        )
    })
}

/// The square of each POV hat in `paint_povs_view`, indexed by hat number, in a 2x2 grid.
pub fn povs_view_rects(client: Rect) -> [Rect; 4] {
    let size = client.height().wrapping_sub(5) / 2 - 5;
    let step = size + 5;
    std::array::from_fn(|hat| {
        let column = (hat % 2) as i32;
        let row = (hat / 2) as i32;
        let left = client
            .left
            .wrapping_add(5)
            .wrapping_add(step.wrapping_mul(column));
        let top = client
            .top
            .wrapping_add(5)
            .wrapping_add(step.wrapping_mul(row));
        Rect::new(left, top, left.wrapping_add(size), top.wrapping_add(size))
    })
}

/// One cell of `paint_buttons_view`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ButtonCell {
    pub rect: Rect,
    /// At most two characters: the C code formats into a three-`WCHAR` buffer.
    pub label: String,
    /// The button state the C code passes as `set`. It only chooses colours.
    pub set: u8,
}

/// `paint_buttons_view`: one cell per entry of `buttons`, filled row by row. Up to 48 buttons
/// use 16-pixel cells labelled in decimal. More use 24-pixel cells labelled in hex.
pub fn buttons_view_cells(client: Rect, buttons: &[u8]) -> Vec<ButtonCell> {
    const SPACE: u32 = 2;
    let count = buttons.len() as u32;
    let step: u32 = if count <= 48 { 16 } else { 24 };
    let client_width = client.width() as u32;
    let size = client_width.wrapping_sub(SPACE) / step;
    let offset = client_width
        .wrapping_sub(step.wrapping_mul(size))
        .wrapping_sub(SPACE)
        / 2;
    let origin = client.offset(offset as i32, offset as i32);
    let first = Rect::new(
        origin.left,
        origin.top,
        origin
            .left
            .wrapping_add(size as i32)
            .wrapping_sub(SPACE as i32),
        origin
            .top
            .wrapping_add(size as i32)
            .wrapping_sub(SPACE as i32),
    );
    buttons
        .iter()
        .enumerate()
        .map(|(index, &set)| {
            let index = index as u32;
            let column = index % step;
            let row = index / step;
            ButtonCell {
                rect: first.offset(
                    column.wrapping_mul(size) as i32,
                    row.wrapping_mul(size) as i32,
                ),
                label: button_label(step, index),
                set,
            }
        })
        .collect()
}

/// `swprintf(buffer, 3, L"%02x")` or `L"%d"`: the buffer holds two characters and the
/// terminator, so longer text is cut to its first two characters.
fn button_label(step: u32, index: u32) -> String {
    let text = if step == 24 {
        format!("{index:02x}")
    } else {
        format!("{index}")
    };
    text.chars().take(2).collect()
}
