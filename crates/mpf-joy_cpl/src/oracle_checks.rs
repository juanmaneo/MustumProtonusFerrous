//! Rows printed by the C oracle in `target/canon/joy.cpl` (`gen.sh`), built with
//! `-fsanitize=address,undefined -fno-sanitize-recover=all`. Each expected text is that row's
//! stdout with the `row <label>` header line removed. Rows the sanitizers flagged
//! (`mul_div_flag_negate_min`, `pov_oob`, `gamepad_narrow`) are not here.

use crate::{dinput, mul_div, xinput, Rect, Shape};

fn rect_text(rect: Rect) -> String {
    format!("{} {} {} {}", rect.left, rect.top, rect.right, rect.bottom)
}

fn shape_text(shape: &Shape) -> String {
    match shape {
        Shape::Ellipse(rect) => format!("Ellipse {}", rect_text(*rect)),
        Shape::Rectangle(rect) => format!("Rectangle {}", rect_text(*rect)),
        Shape::RoundRect(rect) => format!("RoundRect {}", rect_text(*rect)),
        Shape::Line { from, to } => format!("Line {} {} {} {}", from.x, from.y, to.x, to.y),
        Shape::Polygon([a, b, c]) => {
            format!("Polygon {} {} {} {} {} {}", a.x, a.y, b.x, b.y, c.x, c.y)
        }
    }
}

fn trace(shapes: &[Shape]) -> String {
    shapes
        .iter()
        .map(|shape| shape_text(shape) + "\n")
        .collect()
}

fn cells_text(cells: &[dinput::ButtonCell], first: usize) -> String {
    cells
        .iter()
        .enumerate()
        .skip(first)
        .map(|(index, cell)| {
            format!(
                "cell {index} {} set={} label={}\n",
                rect_text(cell.rect),
                cell.set,
                cell.label
            )
        })
        .collect()
}

fn alternating(count: usize) -> Vec<u8> {
    (0..count).map(|index| (index & 1) as u8).collect()
}

fn gamepad_text(
    client: Rect,
    w_buttons: u16,
    sticks: [(i16, i16); 2],
    triggers: [u8; 2],
) -> String {
    let layout = xinput::gamepad_view_layout(client);
    let mut text = String::new();
    for ((rect, (dx, dy)), mask) in layout.sticks.into_iter().zip(sticks).zip([0x0040, 0x0080]) {
        text.push_str(&format!(
            "stick {dx} {dy} set={} {}\n",
            w_buttons & mask,
            rect_text(rect)
        ));
    }
    for (rect, dt) in layout.triggers.into_iter().zip(triggers) {
        text.push_str(&format!("trigger {dt} {}\n", rect_text(rect)));
    }
    for button in layout.buttons {
        text.push_str(&format!(
            "button set={} {} label={}\n",
            w_buttons & button.mask,
            rect_text(button.rect),
            button.label
        ));
    }
    text
}

#[test]
fn mul_div_pos() {
    assert_eq!(mul_div(3, 4, 5), 2);
}

#[test]
fn mul_div_neg_div() {
    assert_eq!(mul_div(7, 3, -2), -11);
}

#[test]
fn mul_div_overflow() {
    assert_eq!(mul_div(2_147_483_647, 2, 1), -1);
}

#[test]
fn pov_k0() {
    let text = trace(&dinput::draw_pov_view(Rect::new(0, 0, 100, 100), 0));
    assert_eq!(
        text,
        r"Line 39 25 29 0
Line 60 25 71 0
Line 75 39 100 29
Line 75 60 100 71
Line 60 75 71 100
Line 39 75 29 100
Line 25 60 0 71
Line 25 39 0 29
Line 39 25 29 0
Polygon 29 0 50 50 71 0
"
    );
}

#[test]
fn pov_k2() {
    let text = trace(&dinput::draw_pov_view(Rect::new(10, 20, 210, 120), 9000));
    assert_eq!(
        text,
        r"Line 89 45 68 20
Line 131 45 152 20
Line 160 59 210 49
Line 160 80 210 91
Line 131 95 152 120
Line 89 95 68 120
Line 60 80 10 91
Line 60 59 10 49
Line 89 45 68 20
Polygon 210 49 110 70 210 91
"
    );
}

#[test]
fn pov_none() {
    let text = trace(&dinput::draw_pov_view(Rect::new(0, 0, 100, 100), u32::MAX));
    assert_eq!(
        text,
        r"Line 39 25 29 0
Line 60 25 71 0
Line 75 39 100 29
Line 75 60 100 71
Line 60 75 71 100
Line 39 75 29 100
Line 25 60 0 71
Line 25 39 0 29
Line 39 25 29 0
"
    );
}

#[test]
fn pov_k8() {
    let text = trace(&dinput::draw_pov_view(Rect::new(0, 0, 100, 100), 36000));
    assert_eq!(
        text,
        r"Line 39 25 29 0
Line 60 25 71 0
Line 75 39 100 29
Line 75 60 100 71
Line 60 75 71 100
Line 39 75 29 100
Line 25 60 0 71
Line 25 39 0 29
Line 39 25 29 0
Polygon 29 0 50 50 29 0
"
    );
}

#[test]
fn axis_tick_before() {
    let text = trace(&dinput::draw_axis_view(Rect::new(0, 0, 200, 40), 0.25));
    assert_eq!(
        text,
        r"RoundRect 20 0 200 40
Line 110 17 110 23
Rectangle 20 13 68 27
"
    );
}

#[test]
fn axis_tick_after() {
    let text = trace(&dinput::draw_axis_view(Rect::new(0, 0, 200, 40), 0.75));
    assert_eq!(
        text,
        r"RoundRect 20 0 200 40
Rectangle 20 13 152 27
Line 110 17 110 23
"
    );
}

#[test]
fn button_narrow() {
    let text = trace(&[dinput::draw_button_view(Rect::new(0, 0, 15, 15))]);
    assert_eq!(text, "Rectangle 0 0 15 15\n");
}

#[test]
fn button_wide() {
    let text = trace(&[dinput::draw_button_view(Rect::new(0, 0, 16, 16))]);
    assert_eq!(text, "Ellipse 0 0 16 16\n");
}

#[test]
fn stick_a() {
    let text = trace(&xinput::draw_axis_view(
        Rect::new(0, 0, 100, 100),
        16384,
        -16384,
    ));
    assert_eq!(
        text,
        r"Ellipse 0 0 100 100
Line 50 47 50 54
Line 47 50 54 50
Line 50 50 70 70
Ellipse 66 66 74 74
"
    );
}

#[test]
fn stick_b() {
    let text = trace(&xinput::draw_axis_view(
        Rect::new(0, 0, 100, 100),
        -32768,
        32767,
    ));
    assert_eq!(
        text,
        r"Ellipse 0 0 100 100
Line 50 47 50 54
Line 47 50 54 50
Line 50 50 10 10
Ellipse 6 6 14 14
"
    );
}

#[test]
fn trigger_a() {
    let text = trace(&xinput::draw_trigger_view(Rect::new(0, 0, 30, 100), 0));
    assert_eq!(
        text,
        r"RoundRect 0 0 30 100
Line 12 50 18 50
Rectangle 5 95 25 100
"
    );
}

#[test]
fn trigger_b() {
    let text = trace(&xinput::draw_trigger_view(Rect::new(0, 0, 30, 100), 255));
    assert_eq!(
        text,
        r"RoundRect 0 0 30 100
Rectangle 5 5 25 100
Line 12 50 18 50
"
    );
}

#[test]
fn axes_300x200() {
    let rects = dinput::axes_view_rects(Rect::new(0, 0, 300, 200));
    let text: String = [0, 4, 1, 5, 2, 6, 3, 7]
        .into_iter()
        .map(|axis| format!("axis {} {}\n", axis as f64 * 0.125, rect_text(rects[axis])))
        .collect();
    assert_eq!(
        text,
        r"axis 0 5 2 145 49
axis 0.5 155 2 295 49
axis 0.125 5 51 145 98
axis 0.625 155 51 295 98
axis 0.25 5 100 145 147
axis 0.75 155 100 295 147
axis 0.375 5 149 145 196
axis 0.875 155 149 295 196
"
    );
}

#[test]
fn povs_200x150() {
    let rects = dinput::povs_view_rects(Rect::new(0, 0, 200, 150));
    let text: String = [0u32, 4500, 9000, 13500]
        .into_iter()
        .zip(rects)
        .map(|(value, rect)| format!("pov {value} {}\n", rect_text(rect)))
        .collect();
    assert_eq!(
        text,
        r"pov 0 5 5 72 72
pov 4500 77 5 144 72
pov 9000 5 77 72 144
pov 13500 77 77 144 144
"
    );
}

#[test]
fn buttons_12() {
    let cells = dinput::buttons_view_cells(Rect::new(0, 0, 200, 60), &alternating(12));
    assert_eq!(
        cells_text(&cells, 0),
        r"cell 0 3 3 13 13 set=0 label=0
cell 1 15 3 25 13 set=1 label=1
cell 2 27 3 37 13 set=0 label=2
cell 3 39 3 49 13 set=1 label=3
cell 4 51 3 61 13 set=0 label=4
cell 5 63 3 73 13 set=1 label=5
cell 6 75 3 85 13 set=0 label=6
cell 7 87 3 97 13 set=1 label=7
cell 8 99 3 109 13 set=0 label=8
cell 9 111 3 121 13 set=1 label=9
cell 10 123 3 133 13 set=0 label=10
cell 11 135 3 145 13 set=1 label=11
"
    );
}

#[test]
fn buttons_50() {
    let cells = dinput::buttons_view_cells(Rect::new(0, 0, 300, 100), &alternating(50));
    assert_eq!(
        cells_text(&cells, 0),
        r"cell 0 5 5 15 15 set=0 label=00
cell 1 17 5 27 15 set=1 label=01
cell 2 29 5 39 15 set=0 label=02
cell 3 41 5 51 15 set=1 label=03
cell 4 53 5 63 15 set=0 label=04
cell 5 65 5 75 15 set=1 label=05
cell 6 77 5 87 15 set=0 label=06
cell 7 89 5 99 15 set=1 label=07
cell 8 101 5 111 15 set=0 label=08
cell 9 113 5 123 15 set=1 label=09
cell 10 125 5 135 15 set=0 label=0a
cell 11 137 5 147 15 set=1 label=0b
cell 12 149 5 159 15 set=0 label=0c
cell 13 161 5 171 15 set=1 label=0d
cell 14 173 5 183 15 set=0 label=0e
cell 15 185 5 195 15 set=1 label=0f
cell 16 197 5 207 15 set=0 label=10
cell 17 209 5 219 15 set=1 label=11
cell 18 221 5 231 15 set=0 label=12
cell 19 233 5 243 15 set=1 label=13
cell 20 245 5 255 15 set=0 label=14
cell 21 257 5 267 15 set=1 label=15
cell 22 269 5 279 15 set=0 label=16
cell 23 281 5 291 15 set=1 label=17
cell 24 5 17 15 27 set=0 label=18
cell 25 17 17 27 27 set=1 label=19
cell 26 29 17 39 27 set=0 label=1a
cell 27 41 17 51 27 set=1 label=1b
cell 28 53 17 63 27 set=0 label=1c
cell 29 65 17 75 27 set=1 label=1d
cell 30 77 17 87 27 set=0 label=1e
cell 31 89 17 99 27 set=1 label=1f
cell 32 101 17 111 27 set=0 label=20
cell 33 113 17 123 27 set=1 label=21
cell 34 125 17 135 27 set=0 label=22
cell 35 137 17 147 27 set=1 label=23
cell 36 149 17 159 27 set=0 label=24
cell 37 161 17 171 27 set=1 label=25
cell 38 173 17 183 27 set=0 label=26
cell 39 185 17 195 27 set=1 label=27
cell 40 197 17 207 27 set=0 label=28
cell 41 209 17 219 27 set=1 label=29
cell 42 221 17 231 27 set=0 label=2a
cell 43 233 17 243 27 set=1 label=2b
cell 44 245 17 255 27 set=0 label=2c
cell 45 257 17 267 27 set=1 label=2d
cell 46 269 17 279 27 set=0 label=2e
cell 47 281 17 291 27 set=1 label=2f
cell 48 5 29 15 39 set=0 label=30
cell 49 17 29 27 39 set=1 label=31
"
    );
}

#[test]
fn buttons_258_tail() {
    let cells = dinput::buttons_view_cells(Rect::new(0, 0, 400, 100), &alternating(258));
    assert_eq!(
        cells_text(&cells, 254),
        r"cell 254 231 167 245 181 set=0 label=fe
cell 255 247 167 261 181 set=1 label=ff
cell 256 263 167 277 181 set=0 label=10
cell 257 279 167 293 181 set=1 label=10
"
    );
}

#[test]
fn gamepad_300x60() {
    let text = gamepad_text(
        Rect::new(0, 0, 300, 60),
        0xFFFF,
        [(1000, -1000), (-32768, 32767)],
        [255, 128],
    );
    assert_eq!(
        text,
        r"stick 1000 -1000 set=64 5 0 65 60
stick -32768 32767 set=128 70 0 130 60
trigger 255 135 0 150 60
trigger 128 155 0 170 60
button set=1 187 0 206 19 label=^
button set=256 211 0 230 19 label=L
button set=512 235 0 254 19 label=R
button set=32768 259 0 278 19 label=Y
button set=4 175 19 194 38 label=<
button set=8 199 19 218 38 label=>
button set=1024 223 19 242 38 label=@
button set=16384 247 19 266 38 label=X
button set=8192 271 19 290 38 label=B
button set=2 187 38 206 57 label=v
button set=32 211 38 230 57 label=#
button set=16 235 38 254 57 label==
button set=4096 259 38 278 57 label=A
"
    );
}

#[test]
fn gamepad_190x47() {
    let text = gamepad_text(
        Rect::new(4, 2, 190, 47),
        0x1000,
        [(1000, -1000), (-32768, 32767)],
        [255, 128],
    );
    assert_eq!(
        text,
        r"stick 1000 -1000 set=0 4 2 49 47
stick -32768 32767 set=0 49 2 94 47
trigger 255 94 2 105 47
trigger 128 105 2 116 47
button set=0 123 2 137 16 label=^
button set=0 137 2 151 16 label=L
button set=0 151 2 165 16 label=R
button set=0 165 2 179 16 label=Y
button set=0 116 16 130 30 label=<
button set=0 130 16 144 30 label=>
button set=0 144 16 158 30 label=@
button set=0 158 16 172 30 label=X
button set=0 172 16 186 30 label=B
button set=0 123 30 137 44 label=v
button set=0 137 30 151 44 label=#
button set=0 151 30 165 44 label==
button set=4096 165 30 179 44 label=A
"
    );
}
