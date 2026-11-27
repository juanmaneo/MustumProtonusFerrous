//! HRESULT values from include/winerror.h and include/dsound.h, and the MAKE_HRESULT and
//! MAKE_DSHRESULT macros that build DSERR_ALREADYINITIALIZED.

/// A COM HRESULT. The C type is LONG; the bits are kept unsigned here.
pub type Hresult = u32;

pub const S_OK: Hresult = 0;
pub const DS_OK: Hresult = 0;
pub const E_FAIL: Hresult = 0x8000_4005;
pub const E_NOINTERFACE: Hresult = 0x8000_4002;
pub const E_OUTOFMEMORY: Hresult = 0x8007_000E;
pub const E_INVALIDARG: Hresult = 0x8007_0057;
pub const CLASS_E_NOAGGREGATION: Hresult = 0x8004_0110;
pub const CLASS_E_CLASSNOTAVAILABLE: Hresult = 0x8004_0111;

pub const DSERR_INVALIDPARAM: Hresult = E_INVALIDARG;
pub const DSERR_OUTOFMEMORY: Hresult = E_OUTOFMEMORY;
pub const DSERR_NOAGGREGATION: Hresult = CLASS_E_NOAGGREGATION;
pub const DSERR_ALREADYINITIALIZED: Hresult = make_dshresult(130);

/// _FACDS from the DirectSound private header.
const FACILITY_DS: u32 = 0x878;

/// MAKE_HRESULT from include/winerror.h.
pub const fn make_hresult(severity: u32, facility: u32, code: u32) -> Hresult {
    (severity << 31) | (facility << 16) | code
}

/// MAKE_DSHRESULT: severity 1 in the DirectSound facility.
pub const fn make_dshresult(code: u32) -> Hresult {
    make_hresult(1, FACILITY_DS, code)
}
