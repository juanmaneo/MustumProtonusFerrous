//! Members that `main.c` leaves as stubs. Each one returns a fixed `HRESULT` and writes no outputs.

use crate::hresult::{Hresult, E_NOTIMPL, S_OK};

/// Members of `IInspectable` (also used by `IActivationFactory`), `IGeolocator`, and the event
/// registration calls.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Member {
    GetIids,
    GetRuntimeClassName,
    GetTrustLevel,
    GetDesiredAccuracy,
    SetDesiredAccuracy,
    GetMovementThreshold,
    SetMovementThreshold,
    GetReportInterval,
    SetReportInterval,
    LocationStatus,
    GetGeopositionAsync,
    GetGeopositionAsyncWithAgeAndTimeout,
    AddPositionChanged,
    RemovePositionChanged,
    AddStatusChanged,
    RemoveStatusChanged,
}

/// Return value of a stub member. `add_PositionChanged` returns `S_OK` without storing the
/// handler, and every other member returns `E_NOTIMPL`.
pub fn stub_hresult(member: Member) -> Hresult {
    match member {
        Member::AddPositionChanged => S_OK,
        _ => E_NOTIMPL,
    }
}
