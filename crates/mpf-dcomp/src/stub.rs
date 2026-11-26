/// `EXCEPTION_NONCONTINUABLE` from `winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// `EXCEPTION_WINE_STUB` from `winternl.h`: "stub entry point called".
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// Named exports that `dcomp.spec` declares as `stub`.
pub const SPEC_STUBS: [&str; 12] = [
    "CompileEffectDescription",
    "CreateEffectDescription",
    "DCompositionAttachMouseDragToHwnd",
    "DCompositionAttachMouseWheelToHwnd",
    "DCompositionCreateSurfaceHandle",
    "DeserializeEffectDescription",
    "DllGetActivationFactory",
    "DllGetClassObject",
    "DwmEnableMMCSS",
    "DwmFlush",
    "DwmpEnableDDASupport",
    "SerializeEffectDescription",
];

/// The `EXCEPTION_RECORD` fields that `__wine_spec_unimplemented_stub` sets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
}

/// What a spec `stub` export does when called. Wine's handler raises the
/// exception in a loop, so it never returns to the caller.
pub fn unimplemented_stub_exception() -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        number_parameters: 2,
    }
}

/// Whether `name` is one of the named `stub` exports in [`SPEC_STUBS`].
pub fn is_spec_stub(name: &str) -> bool {
    SPEC_STUBS.contains(&name)
}
