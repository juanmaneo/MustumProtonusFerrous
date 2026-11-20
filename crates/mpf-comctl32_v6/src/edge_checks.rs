//! Direct checks for the branches the oracle rows do not reach: negative window styles, every
//! button type, every vertical alignment, unknown image list values and the high-bit inputs.

use crate::{bs_to_dt, button_centers_text, button_type, il_align_to_bs};

const DT_CENTER: u32 = 0x1;
const DT_RIGHT: u32 = 0x2;
const DT_VCENTER: u32 = 0x4;
const DT_BOTTOM: u32 = 0x8;
const DT_WORDBREAK: u32 = 0x10;
const DT_SINGLELINE: u32 = 0x20;
const DT_NOCLIP: u32 = 0x100;

const BS_LEFT_VCENTER: u32 = 0x0100 | 0x0c00;
const BS_RIGHT_VCENTER: u32 = 0x0200 | 0x0c00;
const BS_CENTER_TOP: u32 = 0x0300 | 0x0400;
const BS_CENTER_BOTTOM: u32 = 0x0300 | 0x0800;
const BS_CENTER_VCENTER: u32 = 0x0300 | 0x0c00;

#[test]
fn type_ignores_high_bits_and_reads_negative_styles() {
    assert_eq!(button_type(0x0001_0005), 5);
    assert_eq!(button_type(-1), 15);
    assert_eq!(button_type(i32::MIN), 0);
}

#[test]
fn centers_text_only_for_default_push_and_split_types() {
    for kind in 0..16 {
        let expected = matches!(kind, 0 | 1 | 12 | 13);
        assert_eq!(button_centers_text(kind), expected, "type {kind}");
        assert_eq!(button_centers_text(kind | 0x1000 | 0x2000), expected);
    }
    assert!(button_centers_text(-15));
}

#[test]
fn explicit_horizontal_bits_override_centering_default() {
    assert_eq!(
        bs_to_dt(0x0002 | 0x0300, 0),
        DT_NOCLIP | DT_SINGLELINE | DT_CENTER | DT_VCENTER
    );
    assert_eq!(
        bs_to_dt(0x0200, 0),
        DT_NOCLIP | DT_SINGLELINE | DT_RIGHT | DT_VCENTER
    );
    assert_eq!(
        bs_to_dt(0x0002 | 0x0100, 0),
        DT_NOCLIP | DT_SINGLELINE | DT_VCENTER
    );
}

#[test]
fn default_horizontal_alignment_follows_centering() {
    assert_eq!(
        bs_to_dt(0x0000, 0),
        DT_NOCLIP | DT_SINGLELINE | DT_CENTER | DT_VCENTER
    );
    assert_eq!(bs_to_dt(0x0002, 0), DT_NOCLIP | DT_SINGLELINE | DT_VCENTER);
    assert_eq!(
        bs_to_dt(0x000c, 0),
        DT_NOCLIP | DT_SINGLELINE | DT_CENTER | DT_VCENTER
    );
}

#[test]
fn ws_ex_right_replaces_horizontal_alignment_only() {
    assert_eq!(
        bs_to_dt(0x0000, 0x1000),
        DT_NOCLIP | DT_SINGLELINE | DT_RIGHT | DT_VCENTER
    );
    assert_eq!(
        bs_to_dt(0x0300, 0x1000),
        DT_NOCLIP | DT_SINGLELINE | DT_RIGHT | DT_VCENTER
    );
    assert_eq!(
        bs_to_dt(0x0100, 0x1000),
        DT_NOCLIP | DT_SINGLELINE | DT_RIGHT | DT_VCENTER
    );
    assert_eq!(
        bs_to_dt(0x0000, 0x2000),
        DT_NOCLIP | DT_SINGLELINE | DT_CENTER | DT_VCENTER
    );
}

#[test]
fn vertical_alignment_selects_top_bottom_or_center() {
    assert_eq!(bs_to_dt(0x0100 | 0x0400, 0), DT_NOCLIP | DT_SINGLELINE);
    assert_eq!(
        bs_to_dt(0x0100 | 0x0800, 0),
        DT_NOCLIP | DT_SINGLELINE | DT_BOTTOM
    );
    assert_eq!(
        bs_to_dt(0x0100 | 0x0c00, 0),
        DT_NOCLIP | DT_SINGLELINE | DT_VCENTER
    );
    assert_eq!(bs_to_dt(0x0100, 0), DT_NOCLIP | DT_SINGLELINE | DT_VCENTER);
}

#[test]
fn groupbox_gets_no_vertical_flag() {
    assert_eq!(bs_to_dt(0x0007 | 0x0c00, 0), DT_NOCLIP | DT_SINGLELINE);
    assert_eq!(bs_to_dt(0x0007 | 0x0800, 0), DT_NOCLIP | DT_SINGLELINE);
    assert_eq!(
        bs_to_dt(0x0007 | 0x0300, 0x1000),
        DT_NOCLIP | DT_SINGLELINE | DT_RIGHT
    );
}

#[test]
fn multiline_selects_word_break_instead_of_single_line() {
    assert_eq!(
        bs_to_dt(0x2000, 0),
        DT_NOCLIP | DT_WORDBREAK | DT_CENTER | DT_VCENTER
    );
    assert_eq!(
        bs_to_dt(0x2000 | 0x0800, 0),
        DT_NOCLIP | DT_WORDBREAK | DT_CENTER | DT_BOTTOM
    );
}

#[test]
fn pushlike_strips_type_before_centering() {
    // A push-like radio button (type 4) has its type cleared, so it centers.
    assert_eq!(
        bs_to_dt(0x1004, 0),
        DT_NOCLIP | DT_SINGLELINE | DT_CENTER | DT_VCENTER
    );
    // A push-like group box has type 7 cleared too, so it still gets a vertical flag.
    assert_eq!(
        bs_to_dt(0x1007 | 0x0c00, 0),
        DT_NOCLIP | DT_SINGLELINE | DT_CENTER | DT_VCENTER
    );
}

#[test]
fn ex_style_bits_other_than_ws_ex_right_are_ignored() {
    assert_eq!(bs_to_dt(0x0002, 0x2000), bs_to_dt(0x0002, 0));
    assert_eq!(bs_to_dt(0x0002, 0xffff_efff), bs_to_dt(0x0002, 0));
}

#[test]
fn image_list_alignment_maps_each_known_value() {
    assert_eq!(il_align_to_bs(0), BS_LEFT_VCENTER);
    assert_eq!(il_align_to_bs(1), BS_RIGHT_VCENTER);
    assert_eq!(il_align_to_bs(2), BS_CENTER_TOP);
    assert_eq!(il_align_to_bs(3), BS_CENTER_BOTTOM);
    assert_eq!(il_align_to_bs(4), BS_CENTER_VCENTER);
}

#[test]
fn unknown_image_list_alignment_falls_back_to_left() {
    assert_eq!(il_align_to_bs(5), BS_LEFT_VCENTER);
    assert_eq!(il_align_to_bs(u32::MAX), BS_LEFT_VCENTER);
}
