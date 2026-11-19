//! Edge checks for the cases the oracle rows do not reach: each clause of the rectangle test, the
//! saturating clamp of the background colour, the token mismatch, the surface count limits, the
//! unknown device GUIDs, and the handle table's index and reuse rules.

use super::*;
use crate::oracle_checks::{ayuv, numbered_surfaces, rect};

#[test]
fn background_color_saturates_channels_that_leave_the_byte_range() {
    assert_eq!(
        background_color(&ayuv(0xff00, 0, 0xeb00, 0xffff)),
        0xffff_c900
    );
    assert_eq!(background_color(&ayuv(0, 0, 0, 0xffff)), 0xff00_8700);
}

#[test]
fn intersect_rect_rejects_an_empty_first_rect() {
    assert_eq!(
        intersect_rect(&rect(5, 5, 5, 10), &rect(0, 0, 10, 10)),
        None
    );
}

#[test]
fn intersect_rect_rejects_an_empty_second_rect() {
    assert!(is_rect_empty(&rect(0, 5, 10, 5)));
    assert_eq!(
        intersect_rect(&rect(0, 0, 10, 10), &rect(0, 5, 10, 5)),
        None
    );
}

#[test]
fn intersect_rect_rejects_a_first_rect_right_of_the_second() {
    assert_eq!(
        intersect_rect(&rect(20, 0, 30, 10), &rect(0, 0, 10, 10)),
        None
    );
}

#[test]
fn intersect_rect_rejects_a_first_rect_below_the_second() {
    assert_eq!(
        intersect_rect(&rect(0, 20, 10, 30), &rect(0, 0, 10, 10)),
        None
    );
}

#[test]
fn intersect_rect_rejects_a_second_rect_below_the_first() {
    assert_eq!(
        intersect_rect(&rect(0, 0, 10, 10), &rect(0, 20, 10, 30)),
        None
    );
}

#[test]
fn intersect_rect_keeps_the_inner_rect_when_one_contains_the_other() {
    let outer = rect(0, 0, 10, 10);
    let inner = rect(2, 2, 8, 8);

    assert_eq!(intersect_rect(&outer, &inner), Some(inner));
    assert_eq!(intersect_rect(&inner, &outer), Some(inner));
}

#[test]
fn surface_count_adds_the_front_buffer_and_rejects_overflow() {
    assert_eq!(surface_count(0), Ok(1));
    assert_eq!(surface_count(u32::MAX - 1), Ok(u32::MAX));
    assert_eq!(surface_count(u32::MAX), Err(E_INVALIDARG));
}

#[test]
fn create_surfaces_makes_one_surface_per_back_buffer_and_the_front_buffer() {
    assert_eq!(numbered_surfaces(2), Ok(vec![1, 2, 3]));
}

#[test]
fn check_substreams_accepts_the_largest_allowed_count() {
    assert_eq!(check_substreams(15), Ok(()));
}

#[test]
fn render_targets_for_an_unknown_device_are_not_implemented() {
    assert_eq!(render_target_formats(None, A8R8G8B8), Err(E_NOTIMPL));
}

#[test]
fn render_targets_reject_a_format_that_the_progressive_device_does_not_accept() {
    assert_eq!(
        render_target_formats(Some(Device::Progressive), fourcc(*b"UYVY")),
        Err(E_FAIL)
    );
    assert_eq!(
        render_target_formats(Some(Device::Progressive), A8R8G8B8),
        Ok([X8R8G8B8, NV12])
    );
}

#[test]
fn device_guids_for_nv12_list_only_the_progressive_device() {
    assert_eq!(device_guids(NV12), Ok(vec![Device::Progressive]));
}

#[test]
fn caps_for_an_unknown_device_fail_with_the_error_each_caller_returns() {
    assert_eq!(service_caps(None), Err(E_NOTIMPL));
    assert_eq!(processor_caps(None), Err(E_FAIL));
}

#[test]
fn reset_with_another_token_changes_nothing() {
    let mut handles = DeviceHandles::new(42);

    assert_eq!(handles.reset_device(7), Err(E_INVALIDARG));
    assert_eq!(handles.open_device_handle(), Err(DXVA2_E_NOT_INITIALIZED));

    assert_eq!(handles.reset_device(42), Ok(()));
    assert_eq!(handles.open_device_handle(), Ok(1));
    assert_eq!(handles.reset_device(7), Err(E_INVALIDARG));
    assert_eq!(handles.check_handle(1), Ok(()));
}

#[test]
fn handle_zero_and_handles_past_the_table_are_invalid() {
    let mut handles = DeviceHandles::new(42);

    assert_eq!(handles.reset_device(42), Ok(()));
    assert_eq!(handles.open_device_handle(), Ok(1));
    assert_eq!(handles.close_device_handle(0), Err(E_HANDLE));
    assert_eq!(handles.check_handle(0), Err(E_HANDLE));
    assert_eq!(handles.close_device_handle(2), Err(E_HANDLE));
    assert_eq!(handles.check_handle(2), Err(E_HANDLE));
}

#[test]
fn closing_a_slot_that_is_already_closed_is_an_invalid_handle() {
    let mut handles = DeviceHandles::new(42);

    assert_eq!(handles.reset_device(42), Ok(()));
    assert_eq!(handles.open_device_handle(), Ok(1));
    assert_eq!(handles.open_device_handle(), Ok(2));
    assert_eq!(handles.close_device_handle(1), Ok(()));
    assert_eq!(handles.close_device_handle(1), Err(E_HANDLE));
}

#[test]
fn reset_invalidates_a_closed_slot_that_open_then_reuses() {
    let mut handles = DeviceHandles::new(42);

    assert_eq!(handles.reset_device(42), Ok(()));
    assert_eq!(handles.open_device_handle(), Ok(1));
    assert_eq!(handles.open_device_handle(), Ok(2));
    assert_eq!(handles.close_device_handle(1), Ok(()));
    assert_eq!(handles.reset_device(42), Ok(()));
    assert_eq!(handles.open_device_handle(), Ok(3));
    assert_eq!(handles.check_handle(1), Err(DXVA2_E_NEW_VIDEO_DEVICE));
}

#[test]
fn closing_the_last_slot_removes_only_that_slot() {
    let mut handles = DeviceHandles::new(42);

    assert_eq!(handles.reset_device(42), Ok(()));
    assert_eq!(handles.open_device_handle(), Ok(1));
    assert_eq!(handles.open_device_handle(), Ok(2));
    assert_eq!(handles.close_device_handle(1), Ok(()));
    assert_eq!(handles.close_device_handle(2), Ok(()));
    assert_eq!(handles.check_handle(1), Err(E_HANDLE));
    assert_eq!(handles.check_handle(2), Err(E_HANDLE));
}
