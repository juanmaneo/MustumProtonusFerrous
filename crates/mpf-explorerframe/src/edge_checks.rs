//! Edge-case tests for the ported explorerframe logic.
//!
//! These cover the default tree-view style, zero masks, inverted flag
//! mappings in both directions, and the `Err` and wrap-around paths. Expectations
//! that are not oracle-derived are marked `non-oracle` in a comment.

use super::*;
use crate::item_state::{NSTCIS_BOLD, NSTCIS_DISABLED, NSTCIS_EXPANDED, NSTCIS_SELECTED};
use crate::style::*;

/// Default tree-view style, from oracle row A01.
const DEFAULT_TV: u32 = 0x5601_ca08;

#[test]
fn treeview_zero_mask_keeps_style() {
    // non-oracle: a zero mask changes nothing, whatever the flags say.
    assert_eq!(
        treeview_style_from_nstcs(Some(0x1234), 0xffff_ffff, 0),
        (0x1234, 0)
    );
    assert_eq!(
        treeview_style_from_nstcs(None, 0xffff_ffff, 0),
        (DEFAULT_TV, 0)
    );
}

#[test]
fn treeview_plain_flag_both_ways() {
    // non-oracle: NSTCS_HASLINES (TVS_HASLINES) set, then cleared.
    assert_eq!(
        treeview_style_from_nstcs(Some(0), NSTCS_HASLINES, NSTCS_HASLINES),
        (0x2, 0x2)
    );
    assert_eq!(
        treeview_style_from_nstcs(Some(0x2), 0, NSTCS_HASLINES),
        (0, 0x2)
    );
}

#[test]
fn treeview_inverted_flags_both_ways() {
    // non-oracle: NSTCS_NOINFOTIP is inverted onto TVS_INFOTIP (0x800).
    assert_eq!(
        treeview_style_from_nstcs(Some(0), 0, NSTCS_NOINFOTIP),
        (0x800, 0x800)
    );
    assert_eq!(
        treeview_style_from_nstcs(Some(0x800), NSTCS_NOINFOTIP, NSTCS_NOINFOTIP),
        (0, 0x800)
    );
    // non-oracle: NSTCS_HORIZONTALSCROLL is inverted onto TVS_NOHSCROLL (0x8000).
    assert_eq!(
        treeview_style_from_nstcs(None, NSTCS_HORIZONTALSCROLL, NSTCS_HORIZONTALSCROLL),
        (DEFAULT_TV & !0x8000, 0x8000)
    );
    assert_eq!(
        treeview_style_from_nstcs(None, 0, NSTCS_HORIZONTALSCROLL),
        (DEFAULT_TV, 0)
    );
}

#[test]
fn set_control_style_zero_mask_is_noop() {
    // non-oracle: mask 0 is valid and changes no state, and no tree-view style.
    let u = set_control_style(0x55, 0, 0xffff_ffff, Some(7)).unwrap();
    assert_eq!(
        u,
        ControlStyleUpdate {
            style: 0x55,
            unsupported: 0,
            tv_style: None,
        }
    );
}

#[test]
fn set_control_style_rejects_unknown_bits() {
    // non-oracle: 0x100 and 0x4000 are not in any mask table; the E_FAIL path.
    assert_eq!(set_control_style(0, 0x100, 0x100, None), Err(E_FAIL));
    assert_eq!(set_control_style(0, 0x4000, 0x4000, Some(0)), Err(E_FAIL));
}

#[test]
fn set_control_style_tv_style_needs_both_window_and_mask() {
    // non-oracle: no tree-view window, so no tv_style even with a TV mask.
    let u = set_control_style(0, NSTCS_HASLINES, NSTCS_HASLINES, None).unwrap();
    assert_eq!(u.style, NSTCS_HASLINES);
    assert_eq!(u.tv_style, None);
    // non-oracle: a window exists but the mask is host-only, so no tv_style.
    let u = set_control_style(0, NSTCS_BORDER, NSTCS_BORDER, Some(5)).unwrap();
    assert_eq!(u.tv_style, None);
}

#[test]
fn apply_host_style_mask_without_host_bits_is_noop() {
    // non-oracle: a mask with neither BORDER nor TABSTOP leaves both windows alone.
    assert_eq!(
        apply_host_style(0x1234, 0x5678, NSTCS_HASLINES, 0xffff_ffff),
        (0x1234, 0x5678)
    );
}

#[test]
fn apply_host_style_clears_and_sets_bits() {
    // non-oracle: clearing both host bits removes WS_BORDER and WS_EX_CONTROLPARENT.
    assert_eq!(
        apply_host_style(
            WS_BORDER,
            WS_EX_CONTROLPARENT,
            NSTCS_BORDER | NSTCS_TABSTOP,
            0
        ),
        (0, 0)
    );
    // non-oracle: setting both host bits from zero adds them.
    assert_eq!(
        apply_host_style(
            0,
            0,
            NSTCS_BORDER | NSTCS_TABSTOP,
            NSTCS_BORDER | NSTCS_TABSTOP
        ),
        (WS_BORDER, WS_EX_CONTROLPARENT)
    );
}

#[test]
fn set_control_style2_ignores_bits_outside_mask() {
    // non-oracle: style bits outside the mask are not applied, and an
    // unsupported bit outside the mask is not reported.
    assert_eq!(set_control_style2(0xf0, 0x0c, 0xff), (0xfc, 0x0c));
    assert_eq!(set_control_style2(0xf0, 0, 0xffff_ffff), (0xf0, 0));
}

#[test]
fn item_state_without_disabled_mask_ignores_ex() {
    // non-oracle: DISABLED is masked out, so TVIS_EX_DISABLED does not matter.
    // 0x2 is TVIS_EX_DISABLED, kept literal because it is private to item_state.
    assert_eq!(get_item_state(true, 0, 0x2, NSTCIS_SELECTED), Ok(0));
}

#[test]
fn item_state_zero_mask_and_all_bits() {
    // non-oracle: a zero mask reports no flags even when the item is found.
    assert_eq!(get_item_state(true, 0x0032, 0x2, 0), Ok(0));
    // non-oracle: all four TVIS flags set, full mask -> all four NSTCIS flags.
    assert_eq!(
        get_item_state(true, 0x0002 | 0x0010 | 0x0020, 0x2, 0xffff),
        Ok(NSTCIS_SELECTED | NSTCIS_EXPANDED | NSTCIS_BOLD | NSTCIS_DISABLED)
    );
}

#[test]
fn query_interface_refcount_wraps() {
    // non-oracle: a successful query at u32::MAX wraps the refcount to 0.
    assert_eq!(
        query_interface(TaskbarIid::ITaskbarList, u32::MAX),
        (S_OK, 0)
    );
    // non-oracle: a failed query leaves the refcount at u32::MAX.
    assert_eq!(
        query_interface(TaskbarIid::Other, u32::MAX),
        (E_NOINTERFACE, u32::MAX)
    );
}

#[test]
fn can_unload_now_negative_count_is_busy() {
    // non-oracle: any nonzero count, including negative, keeps the DLL loaded.
    assert_eq!(can_unload_now(-1), S_FALSE);
    assert_eq!(can_unload_now(i32::MIN), S_FALSE);
}

#[test]
fn taskbar_list_constructor_edges() {
    // non-oracle: an aggregated call fails regardless of the lock count.
    assert_eq!(
        taskbar_list_constructor(true, i32::MAX),
        Err(CLASS_E_NOAGGREGATION)
    );
    // non-oracle: the lock count wraps at i32::MAX instead of panicking.
    assert_eq!(taskbar_list_constructor(false, i32::MAX), Ok(i32::MIN));
}

#[test]
fn succeeded_checks_severity_bit() {
    // non-oracle: SUCCEEDED is true for severity 0 (S_OK, S_FALSE) and false
    // for severity 1 (E_FAIL, E_NOTIMPL, E_INVALIDARG).
    assert!(succeeded(S_OK));
    assert!(succeeded(S_FALSE));
    assert!(!succeeded(E_FAIL));
    assert!(!succeeded(E_NOTIMPL));
    assert!(!succeeded(E_INVALIDARG));
}
