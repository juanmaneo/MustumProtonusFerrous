/// `mm_per_inch` from `dlls/gdiplus/gdiplus.c`.
const MM_PER_INCH: f32 = 25.4_f64 as f32;
/// `inch_per_mm` from `dlls/gdiplus/gdiplus.c`.
const INCH_PER_MM: f32 = (1.0_f64 / 25.4) as f32;
/// `point_per_inch` from `dlls/gdiplus/gdiplus.c`.
const POINT_PER_INCH: f32 = 72.0;
/// `inch_per_point` from `dlls/gdiplus/gdiplus.c`.
const INCH_PER_POINT: f32 = (1.0_f64 / 72.0) as f32;

/// `GpUnit` from `include/gdiplus.h`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Unit {
    World,
    Display,
    Pixel,
    Point,
    Inch,
    Document,
    Millimeter,
}

impl Unit {
    /// `None` for a value outside the enum, which the C `default` branch rejects.
    fn from_raw(unit: i32) -> Option<Unit> {
        match unit {
            0 => Some(Unit::World),
            1 => Some(Unit::Display),
            2 => Some(Unit::Pixel),
            3 => Some(Unit::Point),
            4 => Some(Unit::Inch),
            5 => Some(Unit::Document),
            6 => Some(Unit::Millimeter),
            _ => None,
        }
    }
}

/// `units_to_pixels` from `dlls/gdiplus/gdiplus.c`. An unknown unit gives 0, as in the C
/// `default` branch.
pub fn units_to_pixels(units: f32, unit: i32, dpi: f32, printer_display: bool) -> f32 {
    match Unit::from_raw(unit) {
        Some(Unit::World | Unit::Pixel) => units,
        Some(Unit::Display) if printer_display => units * dpi * 0.01,
        Some(Unit::Display) => units,
        Some(Unit::Point) => units * dpi * INCH_PER_POINT,
        Some(Unit::Inch) => units * dpi,
        Some(Unit::Document) => units * dpi * (1.0_f32 / 300.0),
        Some(Unit::Millimeter) => units * dpi * INCH_PER_MM,
        None => 0.0,
    }
}

/// `pixels_to_units` from `dlls/gdiplus/gdiplus.c`. The `Display` and `Document` branches divide
/// in double precision, as the C expressions do, and round the result to `REAL`.
pub fn pixels_to_units(pixels: f32, unit: i32, dpi: f32, printer_display: bool) -> f32 {
    match Unit::from_raw(unit) {
        Some(Unit::World | Unit::Pixel) => pixels,
        Some(Unit::Display) if printer_display => {
            (f64::from(pixels) * 100.0 / f64::from(dpi)) as f32
        }
        Some(Unit::Display) => pixels,
        Some(Unit::Point) => pixels * POINT_PER_INCH / dpi,
        Some(Unit::Inch) => pixels / dpi,
        Some(Unit::Document) => (f64::from(pixels) * 300.0 / f64::from(dpi)) as f32,
        Some(Unit::Millimeter) => pixels * MM_PER_INCH / dpi,
        None => 0.0,
    }
}

/// `units_scale` from `dlls/gdiplus/gdiplus.c`: one `from` unit expressed in `to` units.
pub fn units_scale(from: i32, to: i32, dpi: f32, printer_display: bool) -> f32 {
    let pixels = units_to_pixels(1.0, from, dpi, printer_display);
    pixels_to_units(pixels, to, dpi, printer_display)
}
