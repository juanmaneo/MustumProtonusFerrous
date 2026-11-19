//! Edge cases the oracle rows do not reach: the store's walk over several records, the CLSID
//! grammar, and the stream conversions' zero, wrap and overflow cases. Expected values were checked
//! against the C harness where C defines them. The rest follow the C source, and the undefined
//! cases are marked as the port's own choice.

use super::*;

fn parse(s: &str) -> Result<Clsid, HResult> {
    clsid_from_str(Some(s.as_bytes()))
}

fn timing(start: u32, length: u32, rate: u32, scale: u32) -> StreamTiming {
    StreamTiming {
        start,
        length,
        rate,
        scale,
    }
}

#[test]
fn empty_store_reports_nodata_and_leaves_buffer() {
    let store = ExtraChunks::default();
    let mut buf = [0xEE; 4];
    assert_eq!(
        store.read_chunk(0x6162_6364, Some(&mut buf)),
        Err(AVIERR_NODATA)
    );
    assert_eq!(buf, [0xEE; 4]);
    assert_eq!(store.read_chunk(0x6162_6364, None), Err(AVIERR_NODATA));
}

#[test]
fn missing_chunk_after_other_records_reports_nodata() {
    let mut store = ExtraChunks::default();
    store.write_chunk(0x61, &[1, 2]);
    store.write_chunk(0x62, &[3]);
    assert_eq!(store.read_chunk(0x63, None), Err(AVIERR_NODATA));
}

#[test]
fn records_are_walked_by_their_own_sizes() {
    // An odd-sized record leaves the next header at an odd offset. C reads that header through a
    // misaligned pointer, which UBSan flags, so this case has no oracle row.
    let mut store = ExtraChunks::default();
    store.write_chunk(0x61, &[1, 2, 3, 4, 5]);
    store.write_chunk(0x62, &[6, 7, 8]);
    let mut buf = [0; 3];
    assert_eq!(store.read_chunk(0x62, Some(&mut buf)), Ok(3));
    assert_eq!(buf, [6, 7, 8]);
}

#[test]
fn zero_length_buffer_gets_the_size_without_a_copy() {
    let mut store = ExtraChunks::default();
    store.write_chunk(0x61, &[1, 2]);
    assert_eq!(store.read_chunk(0x61, Some(&mut [])), Ok(2));
}

#[test]
#[should_panic(expected = "size > 0")]
fn empty_chunk_panics_like_the_c_assert() {
    ExtraChunks::default().write_chunk(0x61, &[]);
}

#[test]
fn null_string_gives_zero_clsid() {
    assert_eq!(clsid_from_str(None), Ok([0; 16]));
}

#[test]
fn hex_digits_decode_in_either_case() {
    let upper = parse("{0123ABCD-EF01-2345-6789-ABCDEF012345}");
    let lower = parse("{0123abcd-ef01-2345-6789-abcdef012345}");
    let expected = [
        0xcd, 0xab, 0x23, 0x01, 0x01, 0xef, 0x45, 0x23, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23,
        0x45,
    ];
    assert_eq!(upper, Ok(expected));
    assert_eq!(lower, Ok(expected));
}

#[test]
fn string_ends_at_its_first_nul() {
    let good = "{00000001-0002-0003-0408-090a0b0c0d0e}";
    assert_eq!(parse(&format!("{good}\0junk")), parse(good));
    // The NUL cuts the string to 36 bytes.
    assert_eq!(
        parse("{00000001-0002-0003-0408-090a0b0c0d0\0e}"),
        Err(CO_E_CLASSSTRING)
    );
}

#[test]
fn malformed_strings_are_rejected() {
    let good = "{00000001-0002-0003-0408-090a0b0c0d0e}";
    let cases = [
        "",
        "{}",
        &good[1..],
        &good[..37],
        "(00000001-0002-0003-0408-090a0b0c0d0e}",
        "{00000001-0002-0003-0408-090a0b0c0d0)",
        "{00000001+0002-0003-0408-090a0b0c0d0e}",
        "{00000001-0002-0003-0408-090a0b0c0d0g}",
        "{00000001-0002-0003-0408-090a0b0c0d0e}x",
    ];
    for bad in cases {
        assert_eq!(parse(bad), Err(CO_E_CLASSSTRING), "{bad}");
    }
}

#[test]
fn non_ascii_byte_in_a_digit_position_is_rejected() {
    let mut bytes = b"{00000001-0002-0003-0408-090a0b0c0d0e}".to_vec();
    bytes[20] = 0xE9;
    assert_eq!(clsid_from_str(Some(&bytes)), Err(CO_E_CLASSSTRING));
}

#[test]
fn zero_rate_or_scale_is_unusable_for_sample_to_time() {
    assert_eq!(sample_to_time(timing(0, 100, 0, 1), 10), -1);
    // C divides by `dwScale` here and traps, so the port returns -1 instead.
    assert_eq!(sample_to_time(timing(0, 100, 30, 0), 10), -1);
}

#[test]
fn zero_scale_is_unusable_for_time_to_sample() {
    assert_eq!(time_to_sample(timing(0, 100, 30, 0), 10), -1);
}

#[test]
fn zero_rate_gives_the_stream_start_for_time_to_sample() {
    // C does not check `dwRate` here: 0 * time / 1000 is 0, which the clamp raises to `dwStart`.
    assert_eq!(time_to_sample(timing(5, 100, 0, 1), 10), 5);
}

#[test]
fn empty_stream_pins_every_sample_to_its_start() {
    assert_eq!(sample_to_time(timing(10, 0, 25, 1), 3), 400);
    assert_eq!(time_to_sample(timing(10, 0, 25, 1), 1000), 10);
}

#[test]
fn negative_sample_clamps_to_the_stream_end() {
    // A negative sample compares as a huge unsigned value, so it is clamped to the end.
    assert_eq!(sample_to_time(timing(0, 100, 30, 1), i32::MIN), 3333);
}

#[test]
fn start_plus_length_wraps_like_dword_arithmetic() {
    // `dwStart + dwLength` wraps to 0x10. The clamp raises the sample to `dwStart`, then lowers it
    // to the wrapped end, so both conversions land on 16.
    assert_eq!(time_to_sample(timing(0xFFFF_FFF0, 0x20, 1000, 1), 5000), 16);
    assert_eq!(sample_to_time(timing(0xFFFF_FFF0, 0x20, 1000, 1), 0), 16);
}

#[test]
fn infinite_quotient_saturates_then_clamps_to_the_end() {
    // `scale * 1000` wraps to 0, so the quotient is infinite. C's cast of infinity is undefined,
    // but the clamp gives 100 there too.
    assert_eq!(time_to_sample(timing(0, 100, 1000, 1 << 29), 1), 100);
}

#[test]
fn nan_quotient_converts_to_zero_then_clamps_to_the_start() {
    // 0 / 0 is NaN. The port converts NaN to 0 and clamps to 7. On x86-64, C's undefined cast
    // gives INT_MIN and clamps to 107 instead, so this is the port's own choice.
    assert_eq!(time_to_sample(timing(7, 100, 1000, 1 << 29), 0), 7);
}
