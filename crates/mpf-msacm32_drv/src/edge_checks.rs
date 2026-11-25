//! Edge-case tests for the ported msacm32.drv logic.
//!
//! These cover sign and boundary cases of `MulDiv`, the `TIME_*` branches of the position
//! conversion, the `WAVE_MAPPED` device range, the probe order of the format search, and
//! the derived trait impls. Expectations that are not oracle-derived say so in a comment.

use super::oracle_checks::{run_open, Stub, ADPCM, NONE, PCM44};
use super::*;

fn rates(
    avg_speed_outer: u32,
    avg_speed_inner: u32,
    samples_per_sec_outer: u32,
    samples_per_sec_inner: u32,
) -> Rates {
    Rates {
        avg_speed_outer,
        avg_speed_inner,
        samples_per_sec_outer,
        samples_per_sec_inner,
    }
}

fn time(wtype: u32, u: [u8; 8]) -> MmTime {
    MmTime { wtype, u }
}

#[test]
fn mul_div_sign_combinations() {
    // Not oracle-derived: sign cases of the rounding rule, worked by hand from the C.
    assert_eq!(mul_div(-7, -3, 2), 11);
    assert_eq!(mul_div(-7, 3, 2), -11);
    assert_eq!(mul_div(7, -3, 2), -11);
    assert_eq!(mul_div(0, -3, 2), 0);
    assert_eq!(mul_div(1, 1, 2), 1);
    assert_eq!(mul_div(-1, 1, 2), -1);
    assert_eq!(mul_div(-7, -3, -2), -11);
    assert_eq!(mul_div(-7, 3, -2), 11);
}

#[test]
fn mul_div_wraps_int_min_divisor() {
    // Not oracle-derived. C negates INT_MIN, which is undefined. The port wraps, so the
    // divisor stays INT_MIN, the division still runs, and the quotient truncates to 0.
    assert_eq!(mul_div(5, 1, i32::MIN), 0);
}

#[test]
fn mul_div_range_boundaries() {
    // Not oracle-derived: the edges of the -i32::MAX..=i32::MAX result range.
    assert_eq!(mul_div(i32::MAX, 1, 1), i32::MAX);
    assert_eq!(mul_div(-i32::MAX, 1, 1), -i32::MAX);
    assert_eq!(mul_div(i32::MIN, 1, 1), -1);
    assert_eq!(mul_div(i32::MAX, -1, 1), -i32::MAX);
    assert_eq!(mul_div(i32::MAX, 2, 2), i32::MAX);
    assert_eq!(mul_div(i32::MAX, -2, 1), -1);
    assert_eq!(mul_div(-i32::MAX, 2, 1), -1);
}

#[test]
fn mul_div_zero_divisor() {
    // Not oracle-derived.
    assert_eq!(mul_div(0, 0, 0), -1);
    assert_eq!(mul_div(-5, 3, 0), -1);
}

#[test]
fn position_query_maps_ms_to_bytes_only() {
    // Not oracle-derived: both branches of the request mapping.
    let ms = time(TIME_MS, [1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(
        position_query(&ms),
        time(TIME_BYTES, [1, 2, 3, 4, 5, 6, 7, 8])
    );
    let samples = time(TIME_SAMPLES, [9; 8]);
    assert_eq!(position_query(&samples), samples);
}

#[test]
fn get_position_rounds_partial_outer_sample_up() {
    // Not oracle-derived: one stray byte rounds up to a whole outer sample. 1 byte becomes
    // 24 inner bytes, which is 2 outer bytes after MulDiv.
    let r = rates(16000, 192000, 8000, 48000);
    let req = time(TIME_BYTES, [0; 8]);
    let reply = time(TIME_BYTES, [1, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(
        get_position(&r, &req, &reply),
        time(TIME_BYTES, [2, 0, 0, 0, 0, 0, 0, 0])
    );
}

#[test]
fn get_position_smpte_request_with_bytes_reply_becomes_bytes() {
    // Not oracle-derived: a TIME_SMPTE request whose device answers in bytes is converted
    // to bytes, and its wType becomes TIME_BYTES.
    let r = rates(16000, 16000, 8000, 8000);
    let req = time(TIME_SMPTE, [0; 8]);
    let reply = time(TIME_BYTES, [0xe8, 0x03, 0, 0, 0, 0, 0, 0]);
    assert_eq!(
        get_position(&r, &req, &reply),
        time(TIME_BYTES, [0xe8, 0x03, 0, 0, 0, 0, 0, 0])
    );
}

#[test]
fn get_position_samples_request_copies_non_sample_reply() {
    // Not oracle-derived: a SAMPLES request with a SMPTE reply copies the reply's union and
    // keeps the request's wType (2).
    let r = rates(16000, 96000, 8000, 48000);
    let req = time(TIME_SAMPLES, [0; 8]);
    let reply = time(TIME_SMPTE, [1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(
        get_position(&r, &req, &reply),
        time(TIME_SAMPLES, [1, 2, 3, 4, 5, 6, 7, 8])
    );
}

#[test]
fn get_position_zero_inner_rate_keeps_request_tail() {
    // Not oracle-derived: MulDiv by a zero inner rate gives -1 (0xffffffff) in the low dword.
    // The bytes after it still come from the request.
    let r = rates(16000, 96000, 8000, 0);
    let req = time(TIME_SAMPLES, [0, 0, 0, 0, 0x11, 0x22, 0x33, 0x44]);
    let reply = time(TIME_SAMPLES, [5, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(
        get_position(&r, &req, &reply),
        time(
            TIME_SAMPLES,
            [0xff, 0xff, 0xff, 0xff, 0x11, 0x22, 0x33, 0x44]
        )
    );
}

#[test]
#[should_panic(expected = "attempt to divide by zero")]
fn get_position_zero_outer_rate_panics() {
    // Not oracle-derived: C divides by the zero outer sample rate (SIGFPE). The port panics.
    let r = rates(16000, 192000, 0, 48000);
    get_position(&r, &time(TIME_BYTES, [0; 8]), &time(TIME_BYTES, [0; 8]));
}

#[test]
#[should_panic(expected = "remainder with a divisor of zero")]
fn get_position_zero_inner_avg_panics() {
    // Not oracle-derived: a zero inner average rate makes the aligned block size 0, and the
    // remainder divides by it (SIGFPE in C, panic here).
    let r = rates(16000, 0, 8000, 8000);
    get_position(
        &r,
        &time(TIME_BYTES, [0; 8]),
        &time(TIME_BYTES, [1, 0, 0, 0, 0, 0, 0, 0]),
    );
}

#[test]
fn device_range_selects_devices() {
    // Not oracle-derived: range selection from the C ndlo / ndhi logic. The oracle rows for
    // WAVE_MAPPED are in oracle_open_rows.
    assert_eq!(device_range(0, 9, 3), Ok(0..3));
    assert_eq!(device_range(0, 0, 0), Ok(0..0));
    assert_eq!(device_range(WAVE_MAPPED, 0, 1), Ok(0..1));
    assert_eq!(device_range(WAVE_MAPPED, 2, 3), Ok(2..3));
    assert_eq!(
        device_range(WAVE_MAPPED | WAVE_FORMAT_DIRECT, 1, 2),
        Ok(1..2)
    );
    assert_eq!(device_range(WAVE_MAPPED, 3, 3), Err(MMSYSERR_INVALPARAM));
    assert_eq!(
        device_range(WAVE_MAPPED, u32::MAX, 1),
        Err(MMSYSERR_INVALPARAM)
    );
    assert_eq!(device_range(WAVE_MAPPED, 0, 0), Err(MMSYSERR_INVALPARAM));
}

#[test]
fn open_empty_device_range_fails_without_probing() {
    // Not oracle-derived: with no devices there is no direct open and no probe.
    assert_eq!(
        run_open(false, &PCM44, 0..0, 0, NONE),
        (Err(WAVERR_BADFORMAT), 0)
    );
    assert_eq!(
        run_open(true, &PCM44, 0..0, 0, NONE),
        (Err(WAVERR_BADFORMAT), 0)
    );
}

#[test]
fn open_direct_on_second_device() {
    // Not oracle-derived: the direct stage tries devices in order, and finds device 1.
    let opened = Opened {
        device: 1,
        converted: false,
        avg_speed_inner: 176400,
        samples_per_sec_inner: 44100,
    };
    let stub = Stub {
        direct_dev: Some(1),
        ..NONE
    };
    assert_eq!(run_open(false, &PCM44, 0..2, 0, stub), (Ok(opened), 0));
    let stub = Stub {
        direct_dev: Some(1),
        ..NONE
    };
    assert_eq!(run_open(true, &PCM44, 0..2, 0, stub), (Ok(opened), 0));
}

#[test]
fn wod_probe_order_pcm_two_devices() {
    // Not oracle-derived: the order of the PCM ladder for two devices. Each device runs the
    // channel and bit-depth stage, then all devices run the rate stage.
    let mut seen = Vec::new();
    let mut direct = |_: u32| WAVERR_BADFORMAT;
    let mut convert = |device: u32, format: &PcmFormat| {
        seen.push((
            device,
            format.samples_per_sec,
            format.channels,
            format.bits_per_sample,
        ));
        WAVERR_BADFORMAT
    };
    assert_eq!(
        wod_open(&PCM44, 0..2, 0, &mut direct, &mut convert),
        Err(WAVERR_BADFORMAT)
    );
    assert_eq!(seen.len(), 48);
    assert_eq!(
        seen[..8],
        [
            (0, 44100, 2, 16),
            (0, 44100, 2, 8),
            (0, 44100, 1, 16),
            (0, 44100, 1, 8),
            (1, 44100, 2, 16),
            (1, 44100, 2, 8),
            (1, 44100, 1, 16),
            (1, 44100, 1, 8),
        ]
    );
    assert_eq!(seen[8], (0, 96000, 2, 16));
    assert_eq!(seen[47], (1, 11025, 1, 8));
}

#[test]
fn wod_probe_order_non_pcm_two_devices() {
    // Not oracle-derived: a non-PCM source keeps rate and channels, and tries only bit depths.
    let mut seen = Vec::new();
    let mut direct = |_: u32| WAVERR_BADFORMAT;
    let mut convert = |device: u32, format: &PcmFormat| {
        seen.push((
            device,
            format.samples_per_sec,
            format.channels,
            format.bits_per_sample,
        ));
        WAVERR_BADFORMAT
    };
    assert_eq!(
        wod_open(&ADPCM, 0..2, 0, &mut direct, &mut convert),
        Err(WAVERR_BADFORMAT)
    );
    assert_eq!(
        seen,
        [
            (0, 22050, 1, 16),
            (0, 22050, 1, 8),
            (1, 22050, 1, 16),
            (1, 22050, 1, 8),
        ]
    );
}

#[test]
fn open_other_error_code_is_returned() {
    // Not oracle-derived: a code other than 0, WAVERR_BADFORMAT or ACMERR_NOTPOSSIBLE ends
    // the search and is returned unchanged.
    let mut direct = |_: u32| WAVERR_BADFORMAT;
    let mut convert = |_: u32, _: &PcmFormat| 5u32;
    assert_eq!(wod_open(&PCM44, 0..1, 0, &mut direct, &mut convert), Err(5));
}

#[test]
fn pcm_format_block_align_and_average() {
    // Not oracle-derived: nBlockAlign and nAvgBytesPerSec from wodOpenHelper.
    assert_eq!(
        PcmFormat::new(44100, 2, 16),
        PcmFormat {
            samples_per_sec: 44100,
            channels: 2,
            bits_per_sample: 16,
            block_align: 4,
            avg_bytes_per_sec: 176400,
        }
    );
    assert_eq!(PcmFormat::new(44100, 0, 16).block_align, 0);
    assert_eq!(PcmFormat::new(44100, 0, 16).avg_bytes_per_sec, 0);
    assert_eq!(PcmFormat::new(8000, 1, 12).block_align, 1);
    assert_eq!(
        PcmFormat::new(u32::MAX, 2, 16).avg_bytes_per_sec,
        u32::MAX.wrapping_mul(4)
    );
}

#[test]
fn pcm_format_block_align_overflow_wraps() {
    // Not oracle-derived. channels * bits overflows C int, which is undefined and would be
    // flagged by UBSan. The port wraps as gcc does: 0xffff * 0xffff wraps to -131071, which
    // divides to -16383, and that is stored as 49153.
    assert_eq!(PcmFormat::new(1, 0xffff, 0xffff).block_align, 49153);
    assert_eq!(PcmFormat::new(1, 0xffff, 0xffff).avg_bytes_per_sec, 49153);
}

#[test]
fn derived_clone_and_debug_impls() {
    // Calls the derived impls directly. assert_eq! only formats on failure.
    let time_value = time(TIME_MS, [1; 8]);
    assert_eq!(Clone::clone(&time_value), time_value);
    assert!(format!("{time_value:?}").contains("wtype: 1"));

    let rates_value = rates(1, 2, 3, 4);
    assert_eq!(Clone::clone(&rates_value), rates_value);
    assert!(format!("{rates_value:?}").contains("avg_speed_outer: 1"));

    let pcm = PcmFormat::new(8000, 1, 8);
    assert_eq!(Clone::clone(&pcm), pcm);
    assert!(format!("{pcm:?}").contains("block_align: 1"));

    let src = PCM44;
    assert_eq!(Clone::clone(&src), src);
    assert!(format!("{src:?}").contains("format_tag: 1"));

    let opened = Opened {
        device: 0,
        converted: true,
        avg_speed_inner: 1,
        samples_per_sec_inner: 2,
    };
    assert_eq!(Clone::clone(&opened), opened);
    assert!(format!("{opened:?}").contains("converted: true"));
}
