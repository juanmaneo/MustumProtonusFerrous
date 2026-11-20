//! Edge-case tests for the `mpf-cards` port.
//!
//! These tests verify boundary conditions and corner cases that are not
//! covered by the oracle vectors, ensuring the port handles all integer
//! regimes correctly.

use super::*;

#[test]
fn card_zero_is_valid() {
    let result = resolve_draw_args(0, 0, 71, 96, 71, 96);
    assert!(result.is_ok(), "card=0 should be valid");
}

#[test]
fn card_one_is_valid() {
    let result = resolve_draw_args(1, 0, 71, 96, 71, 96);
    assert!(result.is_ok(), "card=1 should be valid");
}

#[test]
fn card_51_is_valid() {
    let result = resolve_draw_args(51, 0, 71, 96, 71, 96);
    assert!(result.is_ok(), "card=51 should be valid");
}

#[test]
fn card_52_is_valid() {
    let result = resolve_draw_args(52, 0, 71, 96, 71, 96);
    assert!(result.is_ok(), "card=52 should be valid");
}

#[test]
fn card_68_is_valid() {
    let result = resolve_draw_args(68, 0, 71, 96, 71, 96);
    assert!(result.is_ok(), "card=68 should be valid");
}

#[test]
fn card_69_is_invalid() {
    let result = resolve_draw_args(69, 0, 71, 96, 71, 96);
    assert!(matches!(result, Err(DrawError::InvalidCard)));
}

#[test]
fn card_neg_one_is_invalid() {
    let result = resolve_draw_args(-1, 0, 71, 96, 71, 96);
    assert!(matches!(result, Err(DrawError::InvalidCard)));
}

#[test]
fn mode_7_is_valid() {
    let result = resolve_draw_args(0, 7, 71, 96, 71, 96);
    assert!(result.is_ok(), "mode=7 should be valid");
}

#[test]
fn mode_neg_one_is_invalid() {
    let result = resolve_draw_args(0, -1, 71, 96, 71, 96);
    assert!(matches!(result, Err(DrawError::InvalidMode)));
}

#[test]
fn mode_8_is_invalid() {
    let result = resolve_draw_args(0, 8, 71, 96, 71, 96);
    assert!(matches!(result, Err(DrawError::InvalidMode)));
}

#[test]
fn resolve_draw_delegates_with_default_size() {
    let result = resolve_draw(5, 0, 71, 96);
    let args = result.expect("card=5 mode=0 should be valid");
    assert_eq!(args.card, 5);
    assert!(args.round_corners, "dx=dy=card size should round corners");
}

#[test]
fn resolve_draw_propagates_card_error() {
    assert!(matches!(
        resolve_draw(69, 0, 71, 96),
        Err(DrawError::InvalidCard)
    ));
}

#[test]
fn resolve_draw_propagates_mode_error() {
    assert!(matches!(
        resolve_draw(0, 8, 71, 96),
        Err(DrawError::InvalidMode)
    ));
}

#[test]
fn dx_equals_card_width_round_corners_true() {
    let result = resolve_draw_args(0, 0, 71, 96, 71, 96);
    assert!(result.is_ok());
    let args = result.unwrap();
    assert!(
        args.round_corners,
        "dx==card_width should make round_corners true"
    );
}

#[test]
fn dy_equals_card_height_round_corners_true() {
    let result = resolve_draw_args(0, 0, 71, 96, 71, 96);
    assert!(result.is_ok());
    let args = result.unwrap();
    assert!(
        args.round_corners,
        "dy==card_height should make round_corners true"
    );
}

#[test]
fn dx_not_equal_card_width_round_corners_false() {
    let result = resolve_draw_args(0, 0, 70, 96, 71, 96);
    assert!(result.is_ok());
    let args = result.unwrap();
    assert!(
        !args.round_corners,
        "dx!=card_width should make round_corners false"
    );
}

#[test]
fn dy_not_equal_card_height_round_corners_false() {
    let result = resolve_draw_args(0, 0, 71, 95, 71, 96);
    assert!(result.is_ok());
    let args = result.unwrap();
    assert!(
        !args.round_corners,
        "dy!=card_height should make round_corners false"
    );
}

#[test]
fn faceup_no_change_card() {
    let result = resolve_draw_args(5, 0, 71, 96, 71, 96);
    assert!(result.is_ok());
    let args = result.unwrap();
    assert_eq!(args.card, 5, "FACEUP should not change card index");
}

#[test]
fn ghost_card_changes_to_free_mask() {
    let result = resolve_draw_args(5, 3, 71, 96, 71, 96);
    assert!(result.is_ok());
    let args = result.unwrap();
    assert_eq!(
        args.card, CARD_FREE_MASK,
        "GHOST should change card to CARD_FREE_MASK"
    );
}

#[test]
fn invisibleghost_card_changes_to_free_mask() {
    let result = resolve_draw_args(5, 5, 71, 96, 71, 96);
    assert!(result.is_ok());
    let args = result.unwrap();
    assert_eq!(
        args.card, CARD_FREE_MASK,
        "INVISIBLEGHOST should change card to CARD_FREE_MASK"
    );
}

#[test]
fn deckx_card_changes_to_back_x() {
    let result = resolve_draw_args(5, 6, 71, 96, 71, 96);
    assert!(result.is_ok());
    let args = result.unwrap();
    assert_eq!(
        args.card, CARD_BACK_THE_X,
        "DECKX should change card to CARD_BACK_THE_X"
    );
}

#[test]
fn decko_card_changes_to_back_o() {
    let result = resolve_draw_args(5, 7, 71, 96, 71, 96);
    assert!(result.is_ok());
    let args = result.unwrap();
    assert_eq!(
        args.card, CARD_BACK_THE_O,
        "DECKO should change card to CARD_BACK_THE_O"
    );
}

#[test]
fn remove_erase_true_draw_false() {
    let result = resolve_draw_args(5, 4, 71, 96, 71, 96);
    assert!(result.is_ok());
    let args = result.unwrap();
    assert!(args.erase_flag, "REMOVE should set erase_flag=true");
    assert!(!args.draw_flag, "REMOVE should set draw_flag=false");
}

#[test]
fn hilite_raster_notsrc_copy() {
    let result = resolve_draw_args(5, 2, 71, 96, 71, 96);
    assert!(result.is_ok());
    let args = result.unwrap();
    assert_eq!(
        args.raster_op, NOTSRCCOPY,
        "HILITE should set raster_op=NOTSRCCOPY"
    );
}

#[test]
fn ghost_raster_src_and() {
    let result = resolve_draw_args(5, 3, 71, 96, 71, 96);
    assert!(result.is_ok());
    let args = result.unwrap();
    assert_eq!(args.raster_op, SRCAND, "GHOST should set raster_op=SRCAND");
}

#[test]
fn remove_raster_srccopy() {
    let result = resolve_draw_args(5, 4, 71, 96, 71, 96);
    assert!(result.is_ok());
    let args = result.unwrap();
    assert_eq!(
        args.raster_op, SRCCOPY,
        "REMOVE should set raster_op=SRCCOPY"
    );
}

#[test]
fn mode_flag_clearing() {
    // After mode &= ~MODEFLAG_DONT_ROUND_CORNERS, the high bit should be cleared.
    // Mode 0x80000001 should become 1, mode 0x80000002 should become 2, etc.
    let result = resolve_draw_args(0, -2147483647, 71, 96, 71, 96); // 0x80000001 → mode=1
    assert!(result.is_ok());
    let args = result.unwrap();
    assert!(
        !args.round_corners,
        "high bit cleared, dx/dy match but flag was set"
    );
    // mode 1 = FACEDOWN, so raster_op = SRCCOPY
    assert_eq!(
        args.raster_op, SRCCOPY,
        "mode 1 after masking should be SRCCOPY"
    );
}

#[test]
fn mode_flag_clearing_highest_bit() {
    // Mode 0x80000000 should become 0 after masking
    let result = resolve_draw_args(0, -2147483648, 71, 96, 71, 96);
    assert!(result.is_ok());
    let args = result.unwrap();
    assert!(!args.round_corners, "high bit set → round_corners false");
    assert_eq!(
        args.raster_op, SRCCOPY,
        "mode 0 after masking should be SRCCOPY"
    );
}

#[test]
fn round_corners_with_hilite_mode_flag() {
    // MODE_HILITE (2) with MODEFLAG_DONT_ROUND_CORNERS set → mode becomes 2 after masking,
    // round_corners = false (high bit was set), raster_op = NOTSRCCOPY (HILITE)
    let result = resolve_draw_args(0, -2147483646, 71, 96, 71, 96); // 0x80000002 → mode=2
    assert!(result.is_ok());
    let args = result.unwrap();
    assert_eq!(
        args.raster_op, NOTSRCCOPY,
        "HILITE should set raster_op=NOTSRCCOPY regardless of flag"
    );
    assert!(
        !args.round_corners,
        "high bit set should disable round corners"
    );
}

#[test]
fn round_corners_with_ghost_mode_flag() {
    // MODE_GHOST (3) with MODEFLAG_DONT_ROUND_CORNERS set → mode becomes 3 after masking,
    // round_corners = false, card = CARD_FREE_MASK, raster = SRCAND
    let result = resolve_draw_args(5, -2147483645, 71, 96, 71, 96); // 0x80000003 → mode=3
    assert!(result.is_ok());
    let args = result.unwrap();
    assert_eq!(
        args.card, CARD_FREE_MASK,
        "GHOST should change card to CARD_FREE_MASK"
    );
    assert_eq!(args.raster_op, SRCAND, "GHOST should set raster_op=SRCAND");
    assert!(
        !args.round_corners,
        "high bit set should disable round corners"
    );
}

#[test]
fn round_corners_with_deckx_mode_flag() {
    // MODE_DECKX (6) with MODEFLAG_DONT_ROUND_CORNERS set → mode becomes 6 after masking,
    // round_corners = false, card = CARD_BACK_THE_X, raster = SRCCOPY
    let result = resolve_draw_args(5, -2147483642, 71, 96, 71, 96); // 0x80000006 → mode=6
    assert!(result.is_ok());
    let args = result.unwrap();
    assert_eq!(
        args.card, CARD_BACK_THE_X,
        "DECKX should change card to CARD_BACK_THE_X"
    );
    assert_eq!(
        args.raster_op, SRCCOPY,
        "DECKX should set raster_op=SRCCOPY"
    );
    assert!(
        !args.round_corners,
        "high bit set should disable round corners"
    );
}

#[test]
fn round_corners_with_decko_mode_flag() {
    // MODE_DECKO (7) with MODEFLAG_DONT_ROUND_CORNERS set → mode becomes 7 after masking,
    // round_corners = false, card = CARD_BACK_THE_O, raster = SRCCOPY
    let result = resolve_draw_args(5, -2147483641, 71, 96, 71, 96); // 0x80000007 → mode=7
    assert!(result.is_ok());
    let args = result.unwrap();
    assert_eq!(
        args.card, CARD_BACK_THE_O,
        "DECKO should change card to CARD_BACK_THE_O"
    );
    assert_eq!(
        args.raster_op, SRCCOPY,
        "DECKO should set raster_op=SRCCOPY"
    );
    assert!(
        !args.round_corners,
        "high bit set should disable round corners"
    );
}
