//! Boundary checks for the ported code, beyond the recorded Wine rows in `oracle_checks.rs`.

use crate::icc::{ProfileHeader, TagEntry, TagError, PROFILE_HEADER_SIZE};
use crate::profile::{
    basename, color_directory, match_profile, standard_profile, EnumRecord, ENUMTYPEW_SIZE,
    ENUM_TYPE_VERSION, ET_ATTRIBUTES, ET_CLASS, ET_CMMTYPE, ET_CONNECTIONSPACE, ET_CREATOR,
    ET_DATACOLORSPACE, ET_MANUFACTURER, ET_MODEL, ET_PLATFORM, ET_PROFILEFLAGS, ET_RENDERINGINTENT,
    ET_SIGNATURE, LCS_SRGB, LCS_WINDOWS_COLOR_SPACE,
};
use crate::transform::{
    check_multi_profile, proofing_flags, rendering_intent, CMS_FLAGS_SOFTPROOFING,
    INTENT_PERCEPTUAL,
};
use crate::{McsError, Profile};

const WTPT: u32 = u32::from_be_bytes(*b"wtpt");
const RXYZ: u32 = u32::from_be_bytes(*b"rXYZ");
const OTHER: u32 = u32::from_be_bytes(*b"othr");

/// A zeroed-table profile of `len` bytes whose byte `i` is `i as u8`, listing `tags` as
/// (signature, offset, size).
fn profile_with(tags: &[(u32, u32, u32)], len: usize) -> Profile {
    let mut data: Vec<u8> = (0..len).map(|i| i as u8).collect();
    put_u32(&mut data, 128, tags.len() as u32);
    for (index, &(sig, offset, size)) in tags.iter().enumerate() {
        let at = 132 + 12 * index;
        put_u32(&mut data, at, sig);
        put_u32(&mut data, at + 4, offset);
        put_u32(&mut data, at + 8, size);
    }
    Profile::new(data).expect("at least 132 bytes")
}

fn put_u32(data: &mut [u8], at: usize, value: u32) {
    data[at..at + 4].copy_from_slice(&value.to_be_bytes());
}

fn utf16(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

fn system_dir() -> Vec<u16> {
    utf16(r"C:\Windows\system32")
}

fn zero_record(fields: u32) -> EnumRecord {
    EnumRecord {
        size: 0,
        version: 0,
        fields,
        cmm_type: 0,
        class: 0,
        data_color_space: 0,
        connection_space: 0,
        signature: 0,
        platform: 0,
        profile_flags: 0,
        manufacturer: 0,
        model: 0,
        attributes: [0; 2],
        rendering_intent: 0,
        creator: 0,
    }
}

#[test]
fn profile_needs_the_tag_count_in_the_image() {
    assert!(Profile::new(vec![0; 131]).is_none());
    assert_eq!(
        Profile::new(vec![0; 132])
            .expect("132 bytes")
            .as_bytes()
            .len(),
        132
    );
}

#[test]
fn header_is_stored_big_endian_with_reserved_words_swapped() {
    let mut profile = profile_with(&[], 200);
    let mut header = profile.header();
    header.size = 0x0102_0304;
    header.reserved[0] = 0xab;
    profile.set_header(&header);
    assert_eq!(profile.as_bytes()[..4], [1, 2, 3, 4]);
    // The ramp fills the reserved bytes, so the first reserved word is ramp bytes 84..87 reversed.
    assert_eq!(profile.as_bytes()[84..88], [84, 85, 86, 0xab]);
    assert_eq!(profile.header(), header);
}

#[test]
fn tag_count_is_zero_when_the_table_overruns_the_image() {
    let mut data = vec![0; 143];
    data[128..132].copy_from_slice(&1u32.to_be_bytes());
    assert_eq!(Profile::new(data).expect("143 bytes").tag_count(), 0);
    assert_eq!(profile_with(&[(WTPT, 144, 0)], 144).tag_count(), 1);
    let mut data = vec![0; 132];
    data[128..132].copy_from_slice(&u32::MAX.to_be_bytes());
    assert_eq!(Profile::new(data).expect("132 bytes").tag_count(), 0);
}

#[test]
fn tag_entry_is_one_based_and_must_lie_in_the_image() {
    let profile = profile_with(
        &[
            (WTPT, 140, 4),
            (RXYZ, 200, 0),
            (OTHER, 140, 61),
            (OTHER, 201, 0),
        ],
        200,
    );
    assert_eq!(profile.tag_entry(0), None);
    assert_eq!(
        profile.tag_entry(1),
        Some(TagEntry {
            sig: WTPT,
            offset: 140,
            size: 4
        })
    );
    assert_eq!(
        profile.tag_entry(2),
        Some(TagEntry {
            sig: RXYZ,
            offset: 200,
            size: 0
        })
    );
    assert_eq!(profile.tag_entry(3), None);
    assert_eq!(profile.tag_entry(4), None);
    assert_eq!(profile.tag_entry(5), None);
}

#[test]
fn adjusted_tag_considers_only_the_first_match() {
    let profile = profile_with(&[(WTPT, 300, 4), (WTPT, 140, 4), (RXYZ, 140, 4)], 200);
    assert_eq!(profile.adjusted_tag(WTPT), None);
    assert_eq!(
        profile.adjusted_tag(RXYZ),
        Some(TagEntry {
            sig: RXYZ,
            offset: 140,
            size: 4
        })
    );
    assert_eq!(profile.adjusted_tag(OTHER), None);
}

#[test]
fn tag_data_reports_each_failure() {
    let profile = profile_with(&[(WTPT, 144, 8), (OTHER, 300, 8)], 200);
    assert_eq!(
        profile.tag_data(RXYZ, 0, Some(8)).unwrap_err(),
        TagError::Missing
    );
    assert_eq!(
        profile.tag_data(OTHER, 0, Some(8)).unwrap_err(),
        TagError::Missing
    );
    assert_eq!(
        profile.tag_data(WTPT, 9, Some(100)).unwrap_err(),
        TagError::OutOfRange
    );
    assert_eq!(
        profile.tag_data(WTPT, 0, Some(7)).unwrap_err(),
        TagError::TooSmall { needed: 8 }
    );
    assert_eq!(
        profile.tag_data(WTPT, 0, None).unwrap_err(),
        TagError::TooSmall { needed: 8 }
    );
    assert_eq!(
        profile.tag_data(WTPT, 9, None).unwrap_err(),
        TagError::TooSmall { needed: 8 }
    );
}

#[test]
fn tag_data_returns_the_tail_and_whether_the_extent_is_shared() {
    let profile = profile_with(&[(WTPT, 176, 8), (RXYZ, 176, 8), (OTHER, 184, 8)], 200);
    let read = profile.tag_data(WTPT, 2, Some(6)).expect("fits");
    let expected: Vec<u8> = (178..184).map(|i| i as u8).collect();
    assert_eq!(read.data, &expected[..]);
    assert!(read.linked);
    assert!(!profile.tag_data(OTHER, 0, Some(8)).expect("fits").linked);
    assert!(profile
        .tag_data(WTPT, 8, Some(0))
        .expect("empty tail")
        .data
        .is_empty());

    let duplicate = profile_with(&[(WTPT, 144, 8), (WTPT, 144, 8)], 200);
    assert!(!duplicate.tag_data(WTPT, 0, Some(8)).expect("fits").linked);
}

#[test]
fn set_tag_data_clips_to_the_tag() {
    let mut profile = profile_with(&[(WTPT, 144, 8)], 200);
    assert_eq!(profile.set_tag_data(WTPT, 6, &[0xaa; 5]), Ok(2));
    assert_eq!(profile.as_bytes()[150..153], [0xaa, 0xaa, 152]);
    assert_eq!(profile.set_tag_data(WTPT, 0, &[1]), Ok(1));
    assert_eq!(profile.set_tag_data(WTPT, 8, &[1, 2]), Ok(0));
    assert_eq!(
        profile.set_tag_data(WTPT, 9, &[1]),
        Err(TagError::OutOfRange)
    );
    assert_eq!(profile.set_tag_data(RXYZ, 0, &[1]), Err(TagError::Missing));
    assert_eq!(profile.as_bytes()[144], 1);
}

#[test]
fn rendering_intent_falls_back_to_perceptual_above_three() {
    assert_eq!(rendering_intent(3), 3);
    assert_eq!(rendering_intent(4), INTENT_PERCEPTUAL);
    assert_eq!(rendering_intent(u32::MAX), INTENT_PERCEPTUAL);
}

#[test]
fn soft_proofing_is_set_only_with_a_target() {
    assert_eq!(proofing_flags(true), CMS_FLAGS_SOFTPROOFING);
    assert_eq!(proofing_flags(false), 0);
}

#[test]
fn multi_profile_checks_run_in_wine_order() {
    assert_eq!(
        check_multi_profile(false, false, 1, 0),
        Err(McsError::InvalidParameter)
    );
    assert_eq!(
        check_multi_profile(true, false, 2, 2),
        Err(McsError::InvalidParameter)
    );
    assert_eq!(
        check_multi_profile(true, true, 3, 2),
        Err(McsError::InvalidParameter)
    );
    assert_eq!(
        check_multi_profile(true, true, 0, 1),
        Err(McsError::InvalidTransform)
    );
    assert_eq!(
        check_multi_profile(true, true, 1, 1),
        Err(McsError::InvalidTransform)
    );
    assert_eq!(
        check_multi_profile(true, true, 3, 3),
        Err(McsError::MoreThanTwoProfiles)
    );
    assert_eq!(check_multi_profile(true, true, 2, 2), Ok(()));
    assert_eq!(check_multi_profile(true, true, 2, 1), Ok(()));
}

#[test]
fn basename_keeps_the_part_after_the_last_separator() {
    let cases = [
        ("", ""),
        ("\\", ""),
        ("/", ""),
        ("a/", ""),
        ("\\x", "x"),
        ("a\\b/c", "c"),
        ("plain", "plain"),
    ];
    for (path, name) in cases {
        assert_eq!(
            basename(&utf16(path)),
            utf16(name).as_slice(),
            "path {path:?}"
        );
    }
}

#[test]
fn color_directory_needs_room_for_the_terminator() {
    let system = system_dir();
    assert_eq!(color_directory(&system, None), Err(78));
    assert_eq!(color_directory(&system, Some(77)), Err(78));
    assert_eq!(color_directory(&system, Some(78)), Err(80));
    assert_eq!(color_directory(&system, Some(79)), Err(80));
    let dir = color_directory(&system, Some(80)).expect("fits with the terminator");
    assert_eq!(dir.len(), 39);
    assert!(dir.starts_with(&system));
}

#[test]
fn standard_profile_checks_run_in_wine_order() {
    let system = system_dir();
    assert_eq!(
        standard_profile(true, LCS_SRGB, &system, None, false),
        Err(McsError::NotSupported)
    );
    assert_eq!(
        standard_profile(false, LCS_SRGB, &system, None, true),
        Err(McsError::InvalidParameter)
    );
    assert_eq!(
        standard_profile(false, LCS_SRGB, &system, Some(300), false),
        Err(McsError::InsufficientBuffer)
    );
    assert_eq!(
        standard_profile(false, 1, &system, Some(300), true),
        Err(McsError::FileNotFound)
    );
    let name = standard_profile(false, LCS_WINDOWS_COLOR_SPACE, &system, Some(300), true)
        .expect("the Windows colour space names the sRGB profile");
    assert_eq!(name.len(), 68);
    assert_eq!(
        standard_profile(false, LCS_SRGB, &system, Some(138), true)
            .expect("fits")
            .len(),
        68
    );
    assert_eq!(
        standard_profile(false, LCS_SRGB, &system, Some(137), true),
        Err(McsError::MoreData(138))
    );
    assert_eq!(
        standard_profile(false, LCS_SRGB, &system, Some(136), true),
        Err(McsError::MoreData(138))
    );
    assert_eq!(
        standard_profile(false, LCS_SRGB, &system, Some(135), true),
        Err(McsError::MoreData(136))
    );
}

#[test]
fn match_profile_compares_only_the_selected_fields() {
    let header = ProfileHeader::from_native(&[0; PROFILE_HEADER_SIZE]);
    assert!(match_profile(&zero_record(0), &header));
    assert!(match_profile(&zero_record(u32::MAX), &header));

    type Mutation = (u32, fn(&mut EnumRecord));
    let mutations: [Mutation; 12] = [
        (ET_CMMTYPE, |r| r.cmm_type = 1),
        (ET_CLASS, |r| r.class = 1),
        (ET_DATACOLORSPACE, |r| r.data_color_space = 1),
        (ET_CONNECTIONSPACE, |r| r.connection_space = 1),
        (ET_SIGNATURE, |r| r.signature = 1),
        (ET_PLATFORM, |r| r.platform = 1),
        (ET_PROFILEFLAGS, |r| r.profile_flags = 1),
        (ET_MANUFACTURER, |r| r.manufacturer = 1),
        (ET_MODEL, |r| r.model = 1),
        (ET_ATTRIBUTES, |r| r.attributes[1] = 1),
        (ET_RENDERINGINTENT, |r| r.rendering_intent = 1),
        (ET_CREATOR, |r| r.creator = 1),
    ];
    for (flag, mutate) in mutations {
        let mut selected = zero_record(flag);
        mutate(&mut selected);
        assert!(!match_profile(&selected, &header), "flag {flag:#x}");

        let mut unselected = zero_record(0);
        mutate(&mut unselected);
        assert!(
            match_profile(&unselected, &header),
            "flag {flag:#x} unselected"
        );
    }
}

#[test]
fn enum_record_validity_needs_size_and_version() {
    let mut record = zero_record(0);
    record.size = ENUMTYPEW_SIZE;
    record.version = ENUM_TYPE_VERSION;
    assert!(record.is_valid());
    record.size = ENUMTYPEW_SIZE - 1;
    assert!(!record.is_valid());
    record.size = ENUMTYPEW_SIZE;
    record.version = 0x0200;
    assert!(!record.is_valid());
}

#[test]
fn errors_map_to_their_win32_codes() {
    assert_eq!(McsError::InvalidParameter.win32_code(), Some(87));
    assert_eq!(McsError::InvalidTransform.win32_code(), Some(2020));
    assert_eq!(McsError::NotSupported.win32_code(), Some(50));
    assert_eq!(McsError::FileNotFound.win32_code(), Some(2));
    assert_eq!(McsError::InsufficientBuffer.win32_code(), Some(122));
    assert_eq!(McsError::MoreData(7).win32_code(), Some(234));
    assert_eq!(McsError::MoreThanTwoProfiles.win32_code(), None);
}
