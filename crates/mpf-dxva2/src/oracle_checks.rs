//! Replays of the oracle rows in `target/canon/dxva2`. The oracle compiles the verbatim `main.c`
//! helpers and the `winuser.h` rectangle inlines with `-fsanitize=address,undefined`, and mocks
//! the device calls. Each test names the row it reproduces, and its expected values are the
//! output of that row in `rows.json`. The vectors that trip the sanitizer are excluded, see the
//! crate docs.

use super::*;
use std::cell::Cell;
use std::rc::Rc;

pub(crate) fn rect(left: i32, top: i32, right: i32, bottom: i32) -> Rect {
    Rect {
        left,
        top,
        right,
        bottom,
    }
}

pub(crate) fn ayuv(cr: u16, cb: u16, y: u16, alpha: u16) -> Ayuv16 {
    Ayuv16 { cr, cb, y, alpha }
}

/// `create_surfaces` with a closure that numbers each surface as it makes it. Shared by the
/// tests, so that the closure is instantiated once, for `u32`, and its body runs.
pub(crate) fn numbered_surfaces(backbuffers: u32) -> Result<Vec<u32>, HRESULT> {
    let mut made = 0;
    create_surfaces(backbuffers, || {
        made += 1;
        Ok(made)
    })
}

/// A surface that counts itself in `live` for as long as it exists.
struct Surface(Rc<Cell<usize>>);

impl Surface {
    fn new(live: &Rc<Cell<usize>>) -> Self {
        live.set(live.get() + 1);
        Self(Rc::clone(live))
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        self.0.set(self.0.get() - 1);
    }
}

/// r01: `rect 0 0 10 10 5 5 15 15`.
#[test]
fn r01_rect_overlap_clips_to_the_common_area() {
    let a = rect(0, 0, 10, 10);
    let b = rect(5, 5, 15, 15);

    assert!(!is_rect_empty(&a));
    assert!(!is_rect_empty(&b));
    assert_eq!(intersect_rect(&a, &b), Some(rect(5, 5, 10, 10)));
}

/// r02: `rect 0 0 10 10 10 0 20 10`. Rectangles that only share an edge do not intersect.
#[test]
fn r02_rect_touching_on_an_edge_does_not_intersect() {
    assert_eq!(
        intersect_rect(&rect(0, 0, 10, 10), &rect(10, 0, 20, 10)),
        None
    );
}

/// r03: `rect 0 0 10 10 2 2 2 8`. The second rectangle has no width, so it is empty.
#[test]
fn r03_zero_width_rect_is_empty_and_does_not_intersect() {
    let b = rect(2, 2, 2, 8);

    assert!(!is_rect_empty(&rect(0, 0, 10, 10)));
    assert!(is_rect_empty(&b));
    assert_eq!(intersect_rect(&rect(0, 0, 10, 10), &b), None);
}

/// r04: `rect 0 0 10 10 20 20 30 30`.
#[test]
fn r04_disjoint_rects_do_not_intersect() {
    assert_eq!(
        intersect_rect(&rect(0, 0, 10, 10), &rect(20, 20, 30, 30)),
        None
    );
}

/// r05: `colour 32768 32768 60160`. Mid-grey chroma and full-range luma give white.
#[test]
fn r05_neutral_chroma_full_luma_is_white() {
    assert_eq!(
        background_color(&ayuv(32768, 32768, 60160, 0xffff)),
        0xffff_ffff
    );
}

/// r06: `colour 40960 24576 36864`.
#[test]
fn r06_chroma_offsets_give_the_bt601_colour() {
    assert_eq!(
        background_color(&ayuv(40960, 24576, 36864, 0xffff)),
        0xffc8_8754
    );
}

/// r07: `blt 65535 32768 32768 60160 0 0 100 100 0`. No samples: only the fill happens.
#[test]
fn r07_blt_without_samples_only_fills_the_target() {
    let plan = plan_blt(
        &rect(0, 0, 100, 100),
        &ayuv(32768, 32768, 60160, 0xffff),
        &[],
    )
    .unwrap();

    assert_eq!(plan.background, 0xffff_ffff);
    assert_eq!(plan.fill, rect(0, 0, 100, 100));
    assert!(plan.stretches.is_empty());
}

/// r08: `blt 255 32768 32768 4096 0 0 100 100 0`. A background alpha other than `0xffff` is
/// rejected before any call is made.
#[test]
fn r08_blt_with_partial_background_alpha_is_rejected() {
    assert_eq!(
        plan_blt(&rect(0, 0, 100, 100), &ayuv(32768, 32768, 4096, 255), &[]),
        Err(E_INVALIDARG)
    );
}

/// r09: `blt 65535 32768 32768 4096 0 0 100 100 1 0 0 64 64 80 80 150 150`. The destination is
/// clipped to the target, and the source rectangle is passed through unchanged.
#[test]
fn r09_blt_clips_one_sample_to_the_target() {
    let samples = [Sample {
        src: rect(0, 0, 64, 64),
        dst: rect(80, 80, 150, 150),
    }];
    let plan = plan_blt(
        &rect(0, 0, 100, 100),
        &ayuv(32768, 32768, 4096, 0xffff),
        &samples,
    )
    .unwrap();

    assert_eq!(plan.background, 0xff00_0000);
    assert_eq!(plan.fill, rect(0, 0, 100, 100));
    assert_eq!(
        plan.stretches,
        [Stretch {
            sample: 0,
            src: rect(0, 0, 64, 64),
            dst: rect(80, 80, 100, 100),
        }]
    );
}

/// r10: `blt ... 2 0 0 64 64 80 80 150 150 0 0 64 64 200 200 300 300`. The second sample lies
/// outside the target, so only the first one is stretched.
#[test]
fn r10_blt_skips_a_sample_outside_the_target() {
    let samples = [
        Sample {
            src: rect(0, 0, 64, 64),
            dst: rect(80, 80, 150, 150),
        },
        Sample {
            src: rect(0, 0, 64, 64),
            dst: rect(200, 200, 300, 300),
        },
    ];
    let plan = plan_blt(
        &rect(0, 0, 100, 100),
        &ayuv(32768, 32768, 4096, 0xffff),
        &samples,
    )
    .unwrap();

    assert_eq!(
        plan.stretches,
        [Stretch {
            sample: 0,
            src: rect(0, 0, 64, 64),
            dst: rect(80, 80, 100, 100),
        }]
    );
}

/// r11: `guids 21`. `A8R8G8B8` is accepted by both devices, in table order.
#[test]
fn r11_guids_for_a8r8g8b8_lists_both_devices() {
    assert_eq!(
        device_guids(A8R8G8B8),
        Ok(vec![Device::Progressive, Device::Software])
    );
}

/// r12: `guids 0x59565955`, which is FOURCC `UYVY`. No device accepts it.
#[test]
fn r12_guids_for_uyvy_fails() {
    assert_eq!(device_guids(fourcc(*b"UYVY")), Err(E_FAIL));
}

/// r13: `rt S 21`. The software device renders to `X8R8G8B8` and `A8R8G8B8`.
#[test]
fn r13_render_targets_for_the_software_device() {
    assert_eq!(
        render_target_formats(Some(Device::Software), A8R8G8B8),
        Ok([X8R8G8B8, A8R8G8B8])
    );
}

/// r14: `rt P 0x3231564e`, which is FOURCC `NV12`. The progressive device renders to `NV12`.
#[test]
fn r14_render_targets_for_the_progressive_device_include_nv12() {
    assert_eq!(NV12, 0x3231_564e);
    assert_eq!(
        render_target_formats(Some(Device::Progressive), NV12),
        Ok([X8R8G8B8, NV12])
    );
}

/// r15: `rt S 0x3231564e`. The software device does not accept `NV12` input.
#[test]
fn r15_render_targets_reject_an_unsupported_input_format() {
    assert_eq!(
        render_target_formats(Some(Device::Software), NV12),
        Err(E_FAIL)
    );
}

/// r16: `svccaps S`.
#[test]
fn r16_service_caps_for_the_software_device() {
    assert_eq!(
        service_caps(Some(Device::Software)),
        Ok(ProcessorCaps {
            device_caps: 0x4,
            input_pool: 2,
            operations: 0x10f7,
        })
    );
}

/// r17: `objcaps P`.
#[test]
fn r17_processor_caps_for_the_progressive_device() {
    assert_eq!(
        processor_caps(Some(Device::Progressive)),
        Ok(ProcessorCaps {
            device_caps: 0x1,
            input_pool: 0,
            operations: 0x7,
        })
    );
}

/// r18: `surfaces 2 1`. The second of the three creations fails, so the one surface already
/// made is released, and nothing is left live.
#[test]
fn r18_failed_surface_creation_releases_the_earlier_surfaces() {
    let live = Rc::new(Cell::new(0));
    let mut calls = 0;

    let result = create_surfaces(2, || {
        calls += 1;
        if calls == 2 {
            return Err(E_FAIL);
        }
        Ok(Surface::new(&live))
    });

    assert_eq!(calls, 2);
    assert_eq!(result.err(), Some(E_FAIL));
    assert_eq!(live.get(), 0);
}

/// r19: `surfaces 4294967295 -1`. A back buffer count of `UINT_MAX` overflows the count, and
/// no surface is made.
#[test]
fn r19_back_buffer_count_of_uint_max_is_rejected() {
    assert_eq!(numbered_surfaces(u32::MAX), Err(E_INVALIDARG));
}

/// r20: `substreams P 16`. Sixteen substreams is one more than the limit.
#[test]
fn r20_sixteen_substreams_is_rejected() {
    assert_eq!(check_substreams(16), Err(E_INVALIDARG));
}

/// r21: `substreams U 3`. The device GUID is not checked, so the limit alone decides.
#[test]
fn r21_substream_check_does_not_look_at_the_device() {
    assert_eq!(check_substreams(3), Ok(()));
}

/// r22: `new 42;open`. Opening a handle before any reset returns `DXVA2_E_NOT_INITIALIZED`.
#[test]
fn r22_open_before_reset_is_not_initialized() {
    assert_eq!(
        DeviceHandles::new(42).open_device_handle(),
        Err(DXVA2_E_NOT_INITIALIZED)
    );
}

/// r23: `new 42;reset 42;open;open;close 1;open;test 1;close 1`. After handle 1 is closed, the
/// next open reuses its slot, and the new handle is 3.
#[test]
fn r23_open_after_close_reuses_the_slot_and_appends_one() {
    let mut handles = DeviceHandles::new(42);

    assert_eq!(handles.reset_device(42), Ok(()));
    assert_eq!(handles.open_device_handle(), Ok(1));
    assert_eq!(handles.open_device_handle(), Ok(2));
    assert_eq!(handles.close_device_handle(1), Ok(()));
    assert_eq!(handles.open_device_handle(), Ok(3));
    assert_eq!(handles.check_handle(1), Ok(()));
    assert_eq!(handles.close_device_handle(1), Ok(()));
}

/// r24: `new 42;reset 42;open;close 1;test 1`. A closed handle fails the test.
#[test]
fn r24_test_on_a_closed_handle_is_an_invalid_handle() {
    let mut handles = DeviceHandles::new(42);

    assert_eq!(handles.reset_device(42), Ok(()));
    assert_eq!(handles.open_device_handle(), Ok(1));
    assert_eq!(handles.close_device_handle(1), Ok(()));
    assert_eq!(handles.check_handle(1), Err(E_HANDLE));
}

/// r25: `new 42;reset 42;open;reset 42;test 1`. A second reset invalidates the open handle.
#[test]
fn r25_reset_invalidates_an_open_handle() {
    let mut handles = DeviceHandles::new(42);

    assert_eq!(handles.reset_device(42), Ok(()));
    assert_eq!(handles.open_device_handle(), Ok(1));
    assert_eq!(handles.reset_device(42), Ok(()));
    assert_eq!(handles.check_handle(1), Err(DXVA2_E_NEW_VIDEO_DEVICE));
}
