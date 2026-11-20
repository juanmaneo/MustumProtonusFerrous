//! Domain boundaries and component selectors not fully represented by the oracle rows.

use crate::color::{hsl_to_rgb, rgb_to_hsl};

#[test]
fn hsl_conversion_covers_hue_boundaries_and_primary_colors() {
    for hue in [0, 79, 80, 81, 119, 120, 159, 160, 199, 200, 239, 240] {
        assert!(hsl_to_rgb(hue, 240, 120) <= 0x00ff_ffff);
    }

    assert_eq!(hsl_to_rgb(0, 240, 120), 0x0000_00ff);
    assert_eq!(hsl_to_rgb(80, 240, 120), 0x0000_ff00);
    assert_eq!(hsl_to_rgb(160, 240, 120), 0x00ff_0000);
    assert_eq!(hsl_to_rgb(120, 0, 0), 0);
    assert_eq!(hsl_to_rgb(120, 0, 120), 0x0080_8080);
    assert_eq!(hsl_to_rgb(120, 0, 240), 0x00ff_ffff);
}

#[test]
fn rgb_conversion_handles_equal_channels_and_unrecognized_selectors() {
    for color in [0, 0x0080_8080, 0x00ff_ffff] {
        assert_eq!(rgb_to_hsl(b'H', color), 160);
    }
    assert_eq!(rgb_to_hsl(b'?', 0x0012_3456), 0);
}

#[test]
fn rgb_to_hsl_uses_windows_colorref_channel_order() {
    assert_eq!(rgb_to_hsl(b'H', 0x0000_00ff), 0);
    assert_eq!(rgb_to_hsl(b'H', 0x0000_ff00), 80);
    assert_eq!(rgb_to_hsl(b'H', 0x00ff_0000), 160);
}
