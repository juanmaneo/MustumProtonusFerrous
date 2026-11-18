use super::*;
use crate::analytics_info::device_family_version;

fn utf16(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

#[test]
fn zero_version_is_zero() {
    assert_eq!(device_family_version(0, 0, 0, 0), "0");
}

#[test]
fn major_minor_and_build_are_masked_to_16_bits() {
    assert_eq!(device_family_version(0x1_0000, 0, 0, 0), "0");
    assert_eq!(device_family_version(0, 0x1_0000, 0, 0), "0");
    assert_eq!(device_family_version(0, 0, 0x1_0000, 0), "0");
    assert_eq!(
        device_family_version(0x1_0000 | 10, 0x1_0000 | 3, 0x1_0000 | 2, 0),
        device_family_version(10, 3, 2, 0)
    );
}

#[test]
fn revision_is_not_masked_and_collides_with_build() {
    assert_eq!(device_family_version(0, 0, 0, 0x1_0000), "65536");
    assert_eq!(
        device_family_version(0, 0, 1, 0),
        device_family_version(0, 0, 0, 0x1_0000)
    );
}

#[test]
fn revision_below_bit_16_does_not_touch_build() {
    assert_eq!(device_family_version(0, 0, 0, 0xffff), "65535");
    assert_eq!(device_family_version(0, 0, 1, 0), "65536");
    assert_eq!(device_family_version(0, 0, 1, 0xffff), "131071");
}

#[test]
fn all_ones_revision_alone_is_its_decimal_value() {
    assert_eq!(device_family_version(0, 0, 0, u32::MAX), "4294967295");
}

#[test]
fn empty_name_has_no_factory() {
    assert_eq!(activation_factory_for_class(&[]), None);
}

#[test]
fn leading_nul_has_no_factory() {
    assert_eq!(
        activation_factory_for_class(&utf16("\0Windows.System.Profile.AnalyticsInfo")),
        None
    );
}

#[test]
fn trailing_text_without_nul_has_no_factory() {
    assert_eq!(
        activation_factory_for_class(&utf16("Windows.System.Profile.AnalyticsInfo!")),
        None
    );
    assert_eq!(
        activation_factory_for_class(&utf16("Windows.System.Profile.AnalyticsInfo\u{e9}")),
        None
    );
}

#[test]
fn proper_prefixes_have_no_factory() {
    let units = utf16("Windows.System.Profile.AnalyticsInfo");
    for len in 0..units.len() {
        assert_eq!(
            activation_factory_for_class(&units[..len]),
            None,
            "len {len}"
        );
    }
    assert_eq!(
        activation_factory_for_class(&units),
        Some(ActivationFactory::AnalyticsInfo)
    );
}

#[test]
fn lone_surrogate_has_no_factory() {
    assert_eq!(activation_factory_for_class(&[0xd800]), None);
    assert_eq!(activation_factory_for_class(&[0xdc00, 0]), None);
}
