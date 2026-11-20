use super::{accepts_media_type, FormatType, MediaMajorType};

#[test]
fn rejects_nonmatching_format_pairs() {
    assert!(!accepts_media_type(MediaMajorType::Audio, FormatType::None));
    assert!(!accepts_media_type(
        MediaMajorType::Interleaved,
        FormatType::VideoInfo
    ));
    assert!(!accepts_media_type(
        MediaMajorType::Other,
        FormatType::DvInfo
    ));
}
