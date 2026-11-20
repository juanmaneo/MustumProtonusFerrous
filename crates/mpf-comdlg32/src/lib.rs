//! Deterministic color conversion helpers from Wine's `dlls/comdlg32/colordlg.c`.
//!
//! This ports `hsl_to_x`, `CC_HSLtoRGB`, and `CC_RGBtoHSL`. The conversions retain Wine's
//! integer arithmetic, 0..240 HSL hue/saturation/lightness scale, and Windows `COLORREF` layout.

pub mod color;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
