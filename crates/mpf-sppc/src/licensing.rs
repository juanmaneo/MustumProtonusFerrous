//! `SLGetLicensingStatusInformation` from `dlls/sppc/sppc.c`.

use super::session::Hresult;

/// `SL_E_RIGHT_NOT_CONSUMED` from `include/slerror.h`.
pub const SL_E_RIGHT_NOT_CONSUMED: Hresult = 0xC004_F002;

/// `SLGetLicensingStatusInformation`. The C body ignores all six arguments and returns
/// `SL_E_RIGHT_NOT_CONSUMED`.
pub fn sl_get_licensing_status_information() -> Hresult {
    SL_E_RIGHT_NOT_CONSUMED
}
