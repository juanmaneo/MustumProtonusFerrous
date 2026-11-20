//! Port of the deterministic logic in Wine's `dlls/xtajit64/cpu.c`,
//! the x86-64 emulation on ARM64 support DLL.
//!
//! Ported:
//! - [`processor::is_processor_feature_present`]: the `BTCpu64IsProcessorFeaturePresent`
//!   function that checks if an x86 processor feature is present on ARM64EC.
//!
//! Skipped: all other exports are stubs that call `NtTerminateProcess` or only trace.

/// Processor feature constants matching Windows `PF_*` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ProcessorFeature {
    /// `PF_COMPARE_EXCHANGE_DOUBLE` (0)
    CompareExchangeDouble = 0,
    /// `PF_MMX_INSTRUCTIONS_AVAILABLE` (1)
    MmxInstructionsAvailable = 1,
    /// `PF_XMMI_INSTRUCTIONS_AVAILABLE` (2)
    XmmiInstructionsAvailable = 2,
    /// `PF_RDTSC_INSTRUCTION_AVAILABLE` (3)
    RdtscInstructionAvailable = 3,
    /// `PF_XMMI64_INSTRUCTIONS_AVAILABLE` (4)
    Xmmi64InstructionsAvailable = 4,
    /// `PF_NX_ENABLED` (5)
    NxEnabled = 5,
    /// `PF_SSE3_INSTRUCTIONS_AVAILABLE` (6)
    Sse3InstructionsAvailable = 6,
    /// `PF_COMPARE_EXCHANGE128` (7)
    CompareExchange128 = 7,
    /// `PF_FASTFAIL_AVAILABLE` (8)
    FastFailAvailable = 8,
    /// `PF_RDTSCP_INSTRUCTION_AVAILABLE` (9)
    RdtscpInstructionAvailable = 9,
    /// `PF_SSSE3_INSTRUCTIONS_AVAILABLE` (10)
    Ssse3InstructionsAvailable = 10,
    /// `PF_SSE4_1_INSTRUCTIONS_AVAILABLE` (11)
    Sse41InstructionsAvailable = 11,
    /// `PF_SSE4_2_INSTRUCTIONS_AVAILABLE` (12)
    Sse42InstructionsAvailable = 12,
}

impl ProcessorFeature {
    /// Returns the feature index as a `u32`.
    pub const fn index(self) -> u32 {
        self as u32
    }
}

/// The set of x86 processor features that are always present on ARM64EC.
/// Matches the `x86_features` bitmask in Wine's `BTCpu64IsProcessorFeaturePresent`.
const X86_FEATURES: u64 = (1u64 << ProcessorFeature::CompareExchangeDouble.index())
    | (1u64 << ProcessorFeature::MmxInstructionsAvailable.index())
    | (1u64 << ProcessorFeature::XmmiInstructionsAvailable.index())
    | (1u64 << ProcessorFeature::RdtscInstructionAvailable.index())
    | (1u64 << ProcessorFeature::Xmmi64InstructionsAvailable.index())
    | (1u64 << ProcessorFeature::NxEnabled.index())
    | (1u64 << ProcessorFeature::Sse3InstructionsAvailable.index())
    | (1u64 << ProcessorFeature::CompareExchange128.index())
    | (1u64 << ProcessorFeature::FastFailAvailable.index())
    | (1u64 << ProcessorFeature::RdtscpInstructionAvailable.index())
    | (1u64 << ProcessorFeature::Ssse3InstructionsAvailable.index())
    | (1u64 << ProcessorFeature::Sse41InstructionsAvailable.index())
    | (1u64 << ProcessorFeature::Sse42InstructionsAvailable.index());

/// Checks if an x86 processor feature is present on ARM64EC.
///
/// This is a direct port of `BTCpu64IsProcessorFeaturePresent` from Wine's `cpu.c`.
/// Returns `true` if `feature < 64` and the corresponding bit is set in the
/// `X86_FEATURES` bitmask.
pub fn is_processor_feature_present(feature: u32) -> bool {
    feature < 64 && (X86_FEATURES & (1u64 << feature)) != 0
}

#[cfg(test)]
mod edge_checks;

#[cfg(test)]
mod oracle_checks;
