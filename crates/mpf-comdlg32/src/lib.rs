//! Deterministic color conversion helpers from Wine's `dlls/comdlg32/colordlg.c`.
//!
//! This ports `hsl_to_x`, `CC_HSLtoRGB`, and `CC_RGBtoHSL`. The conversions retain Wine's
//! integer arithmetic, 0..240 HSL hue/saturation/lightness scale, and Windows `COLORREF` layout.

pub mod color;

/// Converts HSL values to a Windows `COLORREF` through the crate's primary API.
pub fn hsl_to_rgb(hue: i32, saturation: i32, luminosity: i32) -> u32 {
    color::hsl_to_rgb(hue, saturation, luminosity)
}

#[cfg(test)]
mod root_api_tests {
    #[test]
    fn hsl_to_rgb_matches_the_color_module() {
        assert_eq!(
            super::hsl_to_rgb(0, 240, 120),
            super::color::hsl_to_rgb(0, 240, 120)
        );
    }
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
