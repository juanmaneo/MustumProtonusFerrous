//! Edge cases of the public API that the oracle rows do not reach directly.

use crate::rate::{
    fastest_rate, is_rate_supported, set_rate, slowest_rate, RateDirection, MAX_RATE,
};
use crate::thin::{StreamEvent, StreamThinning};
use crate::wave::{WaveFormatEx, WaveSink};
use crate::{
    E_FAIL, E_INVALIDARG, MF_E_REVERSE_UNSUPPORTED, MF_E_SHUTDOWN, MF_E_STREAMSINK_REMOVED,
    MF_E_UNSUPPORTED_RATE, S_OK,
};

fn format(
    format_tag: u16,
    channels: u16,
    samples_per_sec: u32,
    bits_per_sample: u16,
) -> WaveFormatEx {
    WaveFormatEx {
        format_tag,
        channels,
        samples_per_sec,
        avg_bytes_per_sec: 0,
        block_align: 0,
        bits_per_sample,
        cb_size: 0,
    }
}

fn pcm_sink() -> WaveSink {
    WaveSink::new(format(1, 1, 11025, 8).with_derived_fields())
}

#[test]
fn derived_fields_of_mono_8_bit() {
    let fmt = format(1, 1, 11025, 8).with_derived_fields();
    assert_eq!(fmt.avg_bytes_per_sec, 11025);
    assert_eq!(fmt.block_align, 1);
}

#[test]
fn derived_block_align_wraps_like_gcc() {
    // 65535 * 65535 overflows int, which is undefined behaviour in the C.
    // gcc wraps it to 0xFFFE0001 (-131071), then /8 gives -16383, which is 0xC001 as a WORD.
    let fmt = format(1, 65535, 0, 65535).with_derived_fields();
    assert_eq!(fmt.block_align, 0xC001);
    assert_eq!(fmt.avg_bytes_per_sec, 0);
}

#[test]
fn format_bytes_are_packed_little_endian() {
    let fmt = WaveFormatEx {
        format_tag: 3,
        channels: 2,
        samples_per_sec: 44100,
        avg_bytes_per_sec: 176400,
        block_align: 4,
        bits_per_sample: 16,
        cb_size: 22,
    };
    assert_eq!(
        fmt.to_bytes(),
        [
            0x03, 0x00, 0x02, 0x00, 0x44, 0xAC, 0x00, 0x00, 0x10, 0xB1, 0x02, 0x00, 0x04, 0x00,
            0x10, 0x00, 0x16, 0x00
        ]
    );
}

#[test]
fn sink_rejects_work_after_shutdown() {
    let mut sink = pcm_sink();
    assert_eq!(sink.shutdown(), Ok(()));
    assert_eq!(sink.shutdown(), Err(MF_E_SHUTDOWN));
    assert_eq!(sink.process_sample(&[1, 2]), Err(MF_E_STREAMSINK_REMOVED));
    assert!(sink.bytes().is_empty());
    assert_eq!(sink.full_length(), 0);
}

#[test]
fn sink_finalize_twice_is_invalid() {
    let mut sink = pcm_sink();
    sink.process_sample(&[1, 2]).unwrap();
    assert_eq!(sink.finalize(), Ok(()));
    assert_eq!(sink.finalize(), Err(E_INVALIDARG));
}

#[test]
fn sink_finalize_empty_wraps_the_riff_size() {
    let mut sink = pcm_sink();
    assert_eq!(sink.finalize(), Ok(()));
    assert_eq!(sink.full_length(), 8);
    assert_eq!(sink.bytes(), &[0, 0, 0, 0]);
}

#[test]
fn sink_empty_sample_writes_only_the_headers() {
    let mut sink = pcm_sink();
    assert_eq!(sink.process_sample(&[]), Ok(()));
    assert_eq!(sink.bytes().len(), 82);
    assert_eq!(sink.full_length(), 82);
    assert_eq!(sink.data_length(), 0);
}

#[test]
fn sink_counts_data_and_full_length() {
    let mut sink = pcm_sink();
    sink.process_sample(&[1, 2, 3]).unwrap();
    sink.process_sample(&[4]).unwrap();
    assert_eq!(sink.data_length(), 4);
    assert_eq!(sink.full_length(), 86);
    assert_eq!(&sink.bytes()[82..], &[1, 2, 3, 4]);
}

#[test]
fn accepts_sample_follows_thinning() {
    let open = StreamThinning {
        thin: false,
        pending_thin: false,
    };
    assert!(open.accepts_sample(false, Some(0)));
    assert!(open.accepts_sample(false, None));
    assert!(open.accepts_sample(true, Some(7)));
    assert!(!open.accepts_sample(true, None));

    let thinned = StreamThinning {
        thin: true,
        pending_thin: false,
    };
    assert!(thinned.accepts_sample(false, Some(1)));
    assert!(!thinned.accepts_sample(false, Some(0)));
    assert!(!thinned.accepts_sample(false, None));
}

#[test]
fn set_thin_reports_only_changes() {
    let mut stream = StreamThinning {
        thin: false,
        pending_thin: false,
    };
    assert_eq!(stream.set_thin(false), None);
    assert_eq!(stream.set_thin(true), Some(StreamEvent::ThinMode(1)));
    assert!(stream.thin);
    assert_eq!(stream.set_thin(true), None);
    assert_eq!(stream.set_thin(false), Some(StreamEvent::ThinMode(0)));
}

#[test]
fn send_sample_ends_thinning_unless_pending() {
    let mut stream = StreamThinning {
        thin: true,
        pending_thin: false,
    };
    assert_eq!(
        stream.send_sample(None),
        Ok(vec![StreamEvent::ThinMode(0), StreamEvent::Sample])
    );
    assert!(!stream.thin);

    let mut stream = StreamThinning {
        thin: true,
        pending_thin: true,
    };
    assert_eq!(
        stream.send_sample(Some(Ok(()))),
        Ok(vec![StreamEvent::Sample])
    );
    assert!(stream.thin);
}

#[test]
fn send_sample_token_failure_leaves_stream_unchanged() {
    let mut stream = StreamThinning {
        thin: true,
        pending_thin: false,
    };
    assert_eq!(stream.send_sample(Some(Err(E_FAIL))), Err(E_FAIL));
    assert!(stream.thin);
}

#[test]
fn slowest_and_fastest_rates() {
    assert_eq!(slowest_rate(), 0.0);
    assert_eq!(fastest_rate(RateDirection::Forward), MAX_RATE);
    assert_eq!(fastest_rate(RateDirection::Reverse), -MAX_RATE);
}

#[test]
fn supported_rates_span_the_range() {
    assert_eq!(is_rate_supported(MAX_RATE), (S_OK, MAX_RATE));
    assert_eq!(is_rate_supported(-MAX_RATE), (S_OK, -MAX_RATE));
    assert_eq!(is_rate_supported(0.5), (S_OK, 0.5));

    let above = f32::from_bits(MAX_RATE.to_bits() + 1);
    assert_eq!(is_rate_supported(above), (MF_E_UNSUPPORTED_RATE, above));
    assert_eq!(is_rate_supported(-above), (MF_E_UNSUPPORTED_RATE, -above));

    let (hr, nearest) = is_rate_supported(f32::NAN);
    assert_eq!(hr, MF_E_UNSUPPORTED_RATE);
    assert!(nearest.is_nan());
}

#[test]
fn set_rate_checks_sign_then_support() {
    assert_eq!(set_rate(-1.0), Err(MF_E_REVERSE_UNSUPPORTED));
    assert_eq!(set_rate(-0.0), Ok(()));
    assert_eq!(set_rate(1.0), Ok(()));
    assert_eq!(set_rate(2e6), Err(MF_E_UNSUPPORTED_RATE));
}
