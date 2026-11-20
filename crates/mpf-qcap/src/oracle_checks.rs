use super::{accepts_media_type, FormatType as F, MediaMajorType as M};

const ORACLE_ROWS: &[(M, F, bool)] = &[
    (M::Audio, F::WaveFormatEx, true),
    (M::Audio, F::DvInfo, false),
    (M::Audio, F::VideoInfo, false),
    (M::Audio, F::None, false),
    (M::Audio, F::Other, false),
    (M::Interleaved, F::WaveFormatEx, false),
    (M::Interleaved, F::DvInfo, true),
    (M::Interleaved, F::VideoInfo, false),
    (M::Interleaved, F::None, false),
    (M::Interleaved, F::Other, false),
    (M::Video, F::WaveFormatEx, false),
    (M::Video, F::DvInfo, true),
    (M::Video, F::VideoInfo, true),
    (M::Video, F::None, false),
    (M::Video, F::Other, false),
    (M::Other, F::WaveFormatEx, false),
    (M::Other, F::DvInfo, false),
    (M::Other, F::VideoInfo, false),
    (M::Other, F::None, false),
    (M::Other, F::Other, false),
];

#[test]
fn matches_wine_c_oracle_rows() {
    for &(major, format, expected) in ORACLE_ROWS {
        assert_eq!(
            accepts_media_type(major, format),
            expected,
            "{major:?}/{format:?}"
        );
    }
}
