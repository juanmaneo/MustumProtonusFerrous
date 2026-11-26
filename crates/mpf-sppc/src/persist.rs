//! `SLPersistApplicationPolicies` from `dlls/sppc/sppc.c`.

use super::session::{Hresult, E_INVALIDARG, S_OK};

/// `SLID`, a 16-byte GUID. Only whether a pointer to one is NULL matters to the port.
pub type Slid = [u8; 16];

/// `SLPersistApplicationPolicies`. A NULL `app` is `E_INVALIDARG`. Otherwise the call succeeds.
/// `product` and `flags` are not read.
pub fn sl_persist_application_policies(
    app: Option<&Slid>,
    _product: Option<&Slid>,
    _flags: u32,
) -> Hresult {
    if app.is_none() {
        return E_INVALIDARG;
    }
    S_OK
}
