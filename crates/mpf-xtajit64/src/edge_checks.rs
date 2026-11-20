//! Edge case tests for the xtajit64 port.

use crate::{is_processor_feature_present, ProcessorFeature};

#[test]
fn test_all_known_features_present() {
    // All features in the X86_FEATURES bitmask should return true
    assert!(is_processor_feature_present(
        ProcessorFeature::CompareExchangeDouble.index()
    ));
    assert!(is_processor_feature_present(
        ProcessorFeature::MmxInstructionsAvailable.index()
    ));
    assert!(is_processor_feature_present(
        ProcessorFeature::XmmiInstructionsAvailable.index()
    ));
    assert!(is_processor_feature_present(
        ProcessorFeature::RdtscInstructionAvailable.index()
    ));
    assert!(is_processor_feature_present(
        ProcessorFeature::Xmmi64InstructionsAvailable.index()
    ));
    assert!(is_processor_feature_present(
        ProcessorFeature::NxEnabled.index()
    ));
    assert!(is_processor_feature_present(
        ProcessorFeature::Sse3InstructionsAvailable.index()
    ));
    assert!(is_processor_feature_present(
        ProcessorFeature::CompareExchange128.index()
    ));
    assert!(is_processor_feature_present(
        ProcessorFeature::FastFailAvailable.index()
    ));
    assert!(is_processor_feature_present(
        ProcessorFeature::RdtscpInstructionAvailable.index()
    ));
    assert!(is_processor_feature_present(
        ProcessorFeature::Ssse3InstructionsAvailable.index()
    ));
    assert!(is_processor_feature_present(
        ProcessorFeature::Sse41InstructionsAvailable.index()
    ));
    assert!(is_processor_feature_present(
        ProcessorFeature::Sse42InstructionsAvailable.index()
    ));
}

#[test]
fn test_unknown_features_absent() {
    // Features not in the bitmask should return false
    assert!(!is_processor_feature_present(13));
    assert!(!is_processor_feature_present(14));
    assert!(!is_processor_feature_present(15));
    assert!(!is_processor_feature_present(31));
    assert!(!is_processor_feature_present(63));
}

#[test]
fn test_feature_64_and_above_returns_false() {
    // feature >= 64 should return false (bounds check)
    assert!(!is_processor_feature_present(64));
    assert!(!is_processor_feature_present(65));
    assert!(!is_processor_feature_present(100));
    assert!(!is_processor_feature_present(u32::MAX));
}

#[test]
fn test_feature_zero_returns_true() {
    // Feature 0 (CompareExchangeDouble) is in the bitmask
    assert!(is_processor_feature_present(0));
}

#[test]
fn test_feature_twelve_returns_true() {
    // Feature 12 (SSE4.2) is the highest in the bitmask
    assert!(is_processor_feature_present(12));
}
