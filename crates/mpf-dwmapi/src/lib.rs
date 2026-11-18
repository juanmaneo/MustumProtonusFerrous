//! dwmapi subset for MPF: the deterministic logic of two `dlls/dwmapi` exports.
//!
//! Reference Wine's `dlls/dwmapi/dwmapi_main.c` has bodies for 25 of the 85
//! entries in `dwmapi.spec`; the other 60 are spec-only stubs. Two of the 25
//! compute a result from their arguments alone. They are ported here with the
//! host state passed in as parameters: the `RtlGetVersion` result, the display
//! mode query, and the performance counter values. No window handle, compositor,
//! or live clock is read.
//!
//! Ported (table tests against a verbatim Wine C oracle, see the tests):
//! - `DwmIsCompositionEnabled` as `is_composition_enabled`: the null check, and
//!   composition is enabled from Windows 6.3 on. The C body calls `RtlGetVersion`
//!   after its null check; the port takes that result as `version`, and `None`
//!   stands for a failed call.
//! - `DwmGetCompositionTimingInfo` as `composition_timing_info`: the argument checks,
//!   the 60 Hz fallback, and the refresh period and vblank arithmetic. The `HWND`
//!   argument is unused in C, so the port does not take it. The display mode query
//!   and the performance counters are parameters.
//!
//! Divergences from the C bodies:
//! - Where C divides by zero, or evaluates `i64::MIN / -1`, the C process traps.
//!   The port returns `E_FAIL` and leaves the struct untouched.
//! - The `FIXME`, `WARN` and `TRACE` logging, and the `static` once-only flags that
//!   gate it, are not modelled.
//!
//! Skipped (83 of 85 spec entries):
//! - 60 spec-only entries with no body in `dwmapi_main.c`: nothing to port.
//! - 21 entries whose body only logs `FIXME` and returns a constant (`S_OK`,
//!   `E_NOTIMPL`, `FALSE`, or `DWM_E_COMPOSITIONDISABLED`): `DwmEnableComposition`,
//!   `DwmExtendFrameIntoClientArea`, `DwmGetColorizationColor`,
//!   `DwmInvalidateIconicBitmaps`, `DwmSetWindowAttribute`,
//!   `DwmGetGraphicsStreamClient`, `DwmGetTransportAttributes`,
//!   `DwmUnregisterThumbnail`, `DwmEnableMMCSS`,
//!   `DwmGetGraphicsStreamTransformHint`, `DwmEnableBlurBehindWindow`,
//!   `DwmDefWindowProc`, `DwmRegisterThumbnail`, `DwmAttachMilContent`,
//!   `DwmDetachMilContent`, `DwmUpdateThumbnailProperties`,
//!   `DwmSetPresentParameters`, `DwmSetIconicLivePreviewBitmap`,
//!   `DwmSetIconicThumbnail`, `DwmpGetColorizationParameters`, `DwmShowContact`.
//!   There is no logic to port; a port would invent behaviour.
//! - `DwmGetWindowAttribute`: checks the window with `IsWindow`, and reads window
//!   rectangles and styles. It needs live windows.
//! - `DwmFlush`: sleeps until the next refresh through `NtDelayExecution`, and its
//!   only result is `S_OK`. The delay is a live timing wait, not a value the caller
//!   sees.

use std::mem::size_of;

/// `S_OK`.
pub const S_OK: i32 = 0;
/// `E_FAIL`: returned where the C body would trap on a zero divisor or overflow.
pub const E_FAIL: i32 = 0x8000_4005u32 as i32;
/// `E_INVALIDARG`: returned for a null out-pointer.
pub const E_INVALIDARG: i32 = 0x8007_0057u32 as i32;
/// `MILERR_MISMATCHED_SIZE`: returned when `cbSize` is not `sizeof(DWM_TIMING_INFO)`.
pub const MILERR_MISMATCHED_SIZE: i32 = 0x8898_0090u32 as i32;

/// `sizeof(DWM_TIMING_INFO)` on x86-64, the value that `cbSize` must hold.
const DWM_TIMING_INFO_SIZE: u32 = 320;
/// `DM_DISPLAYFREQUENCY`: the `dmFields` bit that says `dmDisplayFrequency` is set.
const DM_DISPLAYFREQUENCY: u32 = 0x0040_0000;
/// The refresh rate in hertz, used when the display mode does not provide one.
const FALLBACK_REFRESH_RATE: i32 = 60;

/// The version numbers from a successful `RtlGetVersion` call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OsVersion {
    pub major: u32,
    pub minor: u32,
}

/// The `dmFields` and `dmDisplayFrequency` members of the `DEVMODEW` that
/// `EnumDisplaySettingsExW` fills in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DisplayMode {
    pub fields: u32,
    pub display_frequency: u32,
}

/// `UNSIGNED_RATIO`: a rate as a numerator over a denominator.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UnsignedRatio {
    pub numerator: u32,
    pub denominator: u32,
}

/// `DWM_TIMING_INFO`, field for field in `dwmapi.h` order, so the layout matches
/// the C struct.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DwmTimingInfo {
    pub cb_size: u32,
    pub rate_refresh: UnsignedRatio,
    pub qpc_refresh_period: u64,
    pub rate_compose: UnsignedRatio,
    pub qpc_vblank: u64,
    pub c_refresh: u64,
    pub c_dx_refresh: u32,
    pub qpc_compose: u64,
    pub c_frame: u64,
    pub c_dx_present: u32,
    pub c_refresh_frame: u64,
    pub c_frame_submitted: u64,
    pub c_dx_present_submitted: u32,
    pub c_frame_confirmed: u64,
    pub c_dx_present_confirmed: u32,
    pub c_refresh_confirmed: u64,
    pub c_dx_refresh_confirmed: u32,
    pub c_frames_late: u64,
    pub c_frames_outstanding: u32,
    pub c_frame_displayed: u64,
    pub qpc_frame_displayed: u64,
    pub c_refresh_frame_displayed: u64,
    pub c_frame_complete: u64,
    pub qpc_frame_complete: u64,
    pub c_frame_pending: u64,
    pub qpc_frame_pending: u64,
    pub c_frames_displayed: u64,
    pub c_frames_complete: u64,
    pub c_frames_pending: u64,
    pub c_frames_available: u64,
    pub c_frames_dropped: u64,
    pub c_frames_missed: u64,
    pub c_refresh_next_displayed: u64,
    pub c_refresh_next_presented: u64,
    pub c_refreshes_displayed: u64,
    pub c_refreshes_presented: u64,
    pub c_refresh_started: u64,
    pub c_pixels_received: u64,
    pub c_pixels_drawn: u64,
    pub c_buffers_empty: u64,
}

const _: () = assert!(size_of::<DwmTimingInfo>() == DWM_TIMING_INFO_SIZE as usize);

/// `DwmIsCompositionEnabled`: stores whether composition is on and returns `S_OK`.
///
/// `version` is the `RtlGetVersion` result, with `None` when that call failed.
/// A failed call leaves composition off, as the C body does. A `None` `enabled`
/// returns `E_INVALIDARG` and nothing is written.
pub fn is_composition_enabled(enabled: Option<&mut bool>, version: Option<OsVersion>) -> i32 {
    let Some(enabled) = enabled else {
        return E_INVALIDARG;
    };
    *enabled = version
        .is_some_and(|version| version.major > 6 || (version.major == 6 && version.minor >= 3));
    S_OK
}

/// `get_display_frequency`: the refresh rate in hertz, or the 60 Hz fallback.
///
/// `mode` is `None` when `EnumDisplaySettingsExW` failed, and its fields are then
/// ignored. The C body stores `dmDisplayFrequency` in an `int`, so a value above
/// `i32::MAX` becomes negative; the port keeps that view.
fn refresh_rate(mode: Option<DisplayMode>) -> i32 {
    match mode {
        Some(mode) if mode.fields & DM_DISPLAYFREQUENCY != 0 && mode.display_frequency != 0 => {
            mode.display_frequency as i32
        }
        _ => FALLBACK_REFRESH_RATE,
    }
}

/// `DwmGetCompositionTimingInfo`, without the unused `HWND` argument.
///
/// `display_mode` is `None` when the display query failed. `performance_frequency`
/// and `performance_counter` are the `QueryPerformanceFrequency` and
/// `QueryPerformanceCounter` results. Returns `E_INVALIDARG` for a `None` `info`,
/// and `MILERR_MISMATCHED_SIZE` when `cb_size` is not `DWM_TIMING_INFO_SIZE`. Returns
/// `E_FAIL` where C would trap. Each of these error returns leaves `info` untouched.
/// On success the whole struct is replaced, as C's `memset` followed by the stores does.
pub fn composition_timing_info(
    info: Option<&mut DwmTimingInfo>,
    display_mode: Option<DisplayMode>,
    performance_frequency: i64,
    performance_counter: i64,
) -> i32 {
    let Some(info) = info else {
        return E_INVALIDARG;
    };
    if info.cb_size != DWM_TIMING_INFO_SIZE {
        return MILERR_MISMATCHED_SIZE;
    }

    let rate = refresh_rate(display_mode);
    let Some(period) = performance_frequency.checked_div(i64::from(rate)) else {
        return E_FAIL;
    };
    // C stores the signed quotient in a ULONGLONG, then divides the unsigned counter by it.
    let period = period as u64;
    let Some(quotient) = (performance_counter as u64).checked_div(period) else {
        return E_FAIL;
    };

    *info = DwmTimingInfo {
        cb_size: DWM_TIMING_INFO_SIZE,
        rate_refresh: UnsignedRatio {
            numerator: rate as u32,
            denominator: 1,
        },
        qpc_refresh_period: period,
        rate_compose: UnsignedRatio {
            numerator: rate as u32,
            denominator: 1,
        },
        qpc_vblank: quotient * period,
        ..Default::default()
    };
    S_OK
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::offset_of;

    type CompositionCase = (&'static str, Option<(u32, u32)>, bool);
    type TimingCase = (
        &'static str,
        u32,
        Option<DisplayMode>,
        i64,
        i64,
        i32,
        Option<(u32, u64, u64)>,
    );

    fn os_version((major, minor): (u32, u32)) -> OsVersion {
        OsVersion { major, minor }
    }

    fn mode(fields: u32, display_frequency: u32) -> Option<DisplayMode> {
        Some(DisplayMode {
            fields,
            display_frequency,
        })
    }

    /// A struct whose named fields hold non-zero sentinels, so that a store is visible.
    fn sentinel(cb_size: u32) -> DwmTimingInfo {
        DwmTimingInfo {
            cb_size,
            rate_refresh: UnsignedRatio {
                numerator: 0xA5A5_A5A5,
                denominator: 0xA5A5_A5A5,
            },
            qpc_refresh_period: 0xA5A5_A5A5_A5A5_A5A5,
            rate_compose: UnsignedRatio {
                numerator: 0xA5A5_A5A5,
                denominator: 0xA5A5_A5A5,
            },
            qpc_vblank: 0xA5A5_A5A5_A5A5_A5A5,
            c_buffers_empty: 0xA5A5_A5A5_A5A5_A5A5,
            ..Default::default()
        }
    }

    /// The struct that a successful call writes: both rates are `rate/1`, and every
    /// other field is zero.
    fn written(rate: u32, period: u64, vblank: u64) -> DwmTimingInfo {
        DwmTimingInfo {
            cb_size: DWM_TIMING_INFO_SIZE,
            rate_refresh: UnsignedRatio {
                numerator: rate,
                denominator: 1,
            },
            qpc_refresh_period: period,
            rate_compose: UnsignedRatio {
                numerator: rate,
                denominator: 1,
            },
            qpc_vblank: vblank,
            ..Default::default()
        }
    }

    /// Calls the function on a sentinel struct with `cb_size` and returns both.
    fn call(
        cb_size: u32,
        display_mode: Option<DisplayMode>,
        qpf: i64,
        qpc: i64,
    ) -> (i32, DwmTimingInfo) {
        let mut info = sentinel(cb_size);
        let hr = composition_timing_info(Some(&mut info), display_mode, qpf, qpc);
        (hr, info)
    }

    #[test]
    fn composition_enabled_matches_wine_oracle() {
        // Rows IE03 to IE20 of target/canon/dwmapi/out_is_composition_enabled.txt. The
        // version is the RtlGetVersion result, with None for a failed call. The slot
        // starts as TRUE, so the stored FALSE is visible.
        let cases: [CompositionCase; 18] = [
            ("IE03", Some((5, 1)), false),
            ("IE04", Some((6, 0)), false),
            ("IE05", Some((6, 1)), false),
            ("IE06", Some((6, 2)), false),
            ("IE07", Some((6, 3)), true),
            ("IE08", Some((6, 4)), true),
            ("IE09", Some((6, 255)), true),
            ("IE10", Some((7, 0)), true),
            ("IE11", Some((10, 0)), true),
            ("IE12", Some((5, 9)), false),
            ("IE13", Some((4, 10)), false),
            ("IE14", Some((0, 0)), false),
            ("IE15", Some((u32::MAX, 0)), true),
            ("IE16", Some((6, u32::MAX)), true),
            ("IE17", None, false),
            ("IE18", None, false),
            ("IE19", None, false),
            ("IE20", None, false),
        ];
        for (id, version, expected) in cases {
            let mut enabled = true;
            let hr = is_composition_enabled(Some(&mut enabled), version.map(os_version));
            assert_eq!(hr, S_OK, "{id}");
            assert_eq!(enabled, expected, "{id}");
        }
    }

    #[test]
    fn null_enabled_slot_matches_wine_oracle() {
        // Rows IE01 and IE02: a null slot is E_INVALIDARG, whatever the version query says.
        assert_eq!(
            is_composition_enabled(None, Some(os_version((0, 0)))),
            E_INVALIDARG
        );
        assert_eq!(is_composition_enabled(None, None), E_INVALIDARG);
    }

    #[test]
    fn timing_info_matches_wine_oracle() {
        // Rows TC02 to TC21 of target/canon/dwmapi/out_composition_timing_info.txt, with
        // each row's cbSize, scripted display mode (None when the query fails),
        // performance frequency and counter. The expected struct is the oracle's
        // printed fields; the rest of the struct is zero in every successful row.
        let cases: [TimingCase; 20] = [
            (
                "TC02",
                0,
                mode(DM_DISPLAYFREQUENCY, 60),
                10_000_000,
                0,
                MILERR_MISMATCHED_SIZE,
                None,
            ),
            (
                "TC03",
                319,
                mode(DM_DISPLAYFREQUENCY, 60),
                10_000_000,
                0,
                MILERR_MISMATCHED_SIZE,
                None,
            ),
            (
                "TC04",
                321,
                mode(DM_DISPLAYFREQUENCY, 60),
                10_000_000,
                0,
                MILERR_MISMATCHED_SIZE,
                None,
            ),
            (
                "TC05",
                320,
                mode(DM_DISPLAYFREQUENCY, 144),
                10_000_000,
                0,
                S_OK,
                Some((144, 69_444, 0)),
            ),
            (
                "TC06",
                320,
                mode(DM_DISPLAYFREQUENCY, 60),
                10_000_000,
                123_456_789,
                S_OK,
                Some((60, 166_666, 123_332_840)),
            ),
            (
                "TC07",
                320,
                mode(DM_DISPLAYFREQUENCY, 240),
                3_000_000_000,
                7_000_000_007,
                S_OK,
                Some((240, 12_500_000, 7_000_000_000)),
            ),
            (
                "TC08",
                320,
                mode(DM_DISPLAYFREQUENCY, 75),
                1_000_000,
                1,
                S_OK,
                Some((75, 13_333, 0)),
            ),
            (
                "TC09",
                320,
                mode(DM_DISPLAYFREQUENCY, 1),
                5,
                9,
                S_OK,
                Some((1, 5, 5)),
            ),
            (
                "TC10",
                320,
                mode(DM_DISPLAYFREQUENCY, 1000),
                1_000_000,
                1_000_000,
                S_OK,
                Some((1000, 1000, 1_000_000)),
            ),
            (
                "TC11",
                320,
                mode(DM_DISPLAYFREQUENCY, 0),
                10_000_000,
                123_456_789,
                S_OK,
                Some((60, 166_666, 123_332_840)),
            ),
            (
                "TC12",
                320,
                mode(0x003F_FFFF, 144),
                10_000_000,
                123_456_789,
                S_OK,
                Some((60, 166_666, 123_332_840)),
            ),
            (
                "TC13",
                320,
                None,
                10_000_000,
                123_456_789,
                S_OK,
                Some((60, 166_666, 123_332_840)),
            ),
            (
                "TC14",
                320,
                mode(DM_DISPLAYFREQUENCY, 0xFFFF_FFFE),
                10,
                17,
                S_OK,
                Some((0xFFFF_FFFE, 18_446_744_073_709_551_611, 0)),
            ),
            (
                "TC15",
                320,
                mode(DM_DISPLAYFREQUENCY, 0x8000_0000),
                1_099_511_627_776,
                1000,
                S_OK,
                Some((0x8000_0000, 18_446_744_073_709_551_104, 0)),
            ),
            (
                "TC16",
                320,
                mode(DM_DISPLAYFREQUENCY, 60),
                -10_000_000,
                5000,
                S_OK,
                Some((60, 18_446_744_073_709_384_950, 0)),
            ),
            (
                "TC17",
                320,
                mode(DM_DISPLAYFREQUENCY, 60),
                -600,
                -5,
                S_OK,
                Some((60, 18_446_744_073_709_551_606, 18_446_744_073_709_551_606)),
            ),
            (
                "TC18",
                320,
                mode(DM_DISPLAYFREQUENCY, 60),
                10_000_000,
                -1,
                S_OK,
                Some((60, 166_666, 18_446_744_073_709_389_410)),
            ),
            (
                "TC19",
                320,
                mode(DM_DISPLAYFREQUENCY, 60),
                i64::MAX,
                i64::MAX,
                S_OK,
                Some((60, 153_722_867_280_912_930, 9_223_372_036_854_775_800)),
            ),
            (
                "TC20",
                320,
                mode(DM_DISPLAYFREQUENCY, 0xFFFF_FFFF),
                10,
                17,
                S_OK,
                Some((0xFFFF_FFFF, 18_446_744_073_709_551_606, 0)),
            ),
            (
                "TC21",
                320,
                mode(DM_DISPLAYFREQUENCY, 1),
                -5,
                -3,
                S_OK,
                Some((1, 18_446_744_073_709_551_611, 18_446_744_073_709_551_611)),
            ),
        ];
        for (id, cb_size, display_mode, qpf, qpc, hr, expected) in cases {
            let (got, info) = call(cb_size, display_mode, qpf, qpc);
            assert_eq!(got, hr, "{id}");
            match expected {
                Some((rate, period, vblank)) => {
                    assert_eq!(info, written(rate, period, vblank), "{id}");
                }
                None => assert_eq!(info, sentinel(cb_size), "{id}"),
            }
        }
    }

    #[test]
    fn null_info_is_rejected_before_anything_else() {
        // Row TC01: NULL is E_INVALIDARG. The zero period below would trap in C if
        // the pointer were checked later.
        assert_eq!(
            composition_timing_info(None, mode(DM_DISPLAYFREQUENCY, 60), 1, 1),
            E_INVALIDARG
        );
        assert_eq!(
            composition_timing_info(None, mode(DM_DISPLAYFREQUENCY, 60), 59, 0),
            E_INVALIDARG
        );
    }

    #[test]
    fn size_is_checked_before_the_arithmetic() {
        for cb_size in [0, 1, 319, 321, u32::MAX] {
            // The zero period below would fail with E_FAIL if the arithmetic ran first.
            let (hr, info) = call(cb_size, mode(DM_DISPLAYFREQUENCY, 60), 59, 0);
            assert_eq!(hr, MILERR_MISMATCHED_SIZE, "cb_size {cb_size}");
            assert_eq!(info, sentinel(cb_size), "cb_size {cb_size}");
        }
    }

    #[test]
    fn zero_period_fails_and_leaves_the_struct_untouched() {
        // The C body divides the counter by a period of zero and traps.
        let (hr, info) = call(320, mode(DM_DISPLAYFREQUENCY, 60), 59, 0);
        assert_eq!(hr, E_FAIL);
        assert_eq!(info, sentinel(320));

        // A negative frequency view gives a period of zero: 10000000 / i32::MIN is 0.
        let (hr, info) = call(320, mode(DM_DISPLAYFREQUENCY, 0x8000_0000), 10_000_000, 1);
        assert_eq!(hr, E_FAIL);
        assert_eq!(info, sentinel(320));
    }

    #[test]
    fn minimum_counter_over_minus_one_fails_and_leaves_the_struct_untouched() {
        // The C body evaluates i64::MIN / -1, which overflows and traps.
        let (hr, info) = call(320, mode(DM_DISPLAYFREQUENCY, 0xFFFF_FFFF), i64::MIN, 0);
        assert_eq!(hr, E_FAIL);
        assert_eq!(info, sentinel(320));
    }

    #[test]
    fn failed_display_query_uses_the_60_hz_fallback() {
        // 6000 / 60 is a period of 100, and 60000 is 600 whole periods.
        let (hr, info) = call(320, None, 6000, 60_000);
        assert_eq!(hr, S_OK);
        assert_eq!(info, written(60, 100, 60_000));
    }

    #[test]
    fn refresh_rate_uses_the_frequency_only_when_flagged_and_nonzero() {
        assert_eq!(refresh_rate(None), FALLBACK_REFRESH_RATE);
        assert_eq!(refresh_rate(mode(u32::MAX, 144)), 144);
        assert_eq!(refresh_rate(mode(0x003F_FFFF, 144)), FALLBACK_REFRESH_RATE);
        assert_eq!(
            refresh_rate(mode(DM_DISPLAYFREQUENCY, 0)),
            FALLBACK_REFRESH_RATE
        );
        assert_eq!(refresh_rate(mode(0, 0)), FALLBACK_REFRESH_RATE);
        assert_eq!(
            refresh_rate(mode(DM_DISPLAYFREQUENCY, 0x8000_0000)),
            i32::MIN
        );
    }

    #[test]
    fn struct_layout_matches_the_wine_oracle() {
        // The offsets printed by the oracle for DWM_TIMING_INFO on x86-64.
        assert_eq!(size_of::<DwmTimingInfo>(), 320);
        assert_eq!(offset_of!(DwmTimingInfo, cb_size), 0);
        assert_eq!(offset_of!(DwmTimingInfo, rate_refresh), 4);
        assert_eq!(offset_of!(DwmTimingInfo, qpc_refresh_period), 16);
        assert_eq!(offset_of!(DwmTimingInfo, rate_compose), 24);
        assert_eq!(offset_of!(DwmTimingInfo, qpc_vblank), 32);
        assert_eq!(offset_of!(DwmTimingInfo, c_buffers_empty), 312);
    }

    #[test]
    fn debug_output_names_the_fields() {
        assert_eq!(
            format!("{:?}", OsVersion { major: 6, minor: 3 }),
            "OsVersion { major: 6, minor: 3 }"
        );
        assert_eq!(
            format!(
                "{:?}",
                DisplayMode {
                    fields: DM_DISPLAYFREQUENCY,
                    display_frequency: 60,
                }
            ),
            "DisplayMode { fields: 4194304, display_frequency: 60 }"
        );
        assert_eq!(
            format!(
                "{:?}",
                UnsignedRatio {
                    numerator: 60,
                    denominator: 1,
                }
            ),
            "UnsignedRatio { numerator: 60, denominator: 1 }"
        );
        assert!(format!("{:?}", written(60, 166_666, 0))
            .starts_with("DwmTimingInfo { cb_size: 320, rate_refresh: UnsignedRatio"));
    }
}
