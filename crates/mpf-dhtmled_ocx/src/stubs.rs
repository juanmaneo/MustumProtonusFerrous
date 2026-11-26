//! Exports in `edit.c` whose body only logs a FIXME and returns a fixed HRESULT.

use crate::hresult::{HResult, E_NOTIMPL, S_OK};

/// The two return shapes of the stub exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StubReturn {
    /// `FIXME(...) stub; return E_NOTIMPL;`
    NotImplemented,
    /// `FIXME(...); return S_OK;`, e.g. `DHTMLEdit_NewDocument`.
    Ok,
}

/// Return value of a stub export.
pub fn stub_result(kind: StubReturn) -> HResult {
    match kind {
        StubReturn::NotImplemented => E_NOTIMPL,
        StubReturn::Ok => S_OK,
    }
}
