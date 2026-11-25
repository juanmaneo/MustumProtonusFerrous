//! `IKeyCredentialManagerStatics::IsSupportedAsync`'s callback from Wine's
//! `dlls/cryptowinrt/credentials.c`.

use crate::{CallbackResult, STATUS_PENDING, S_OK};

/// `IsSupportedAsync` callback. The synchronous call from `Start` (`called_async`
/// false) reports pending. The threadpool call (`called_async` true) reports `FALSE`.
pub fn is_supported_async(called_async: bool) -> CallbackResult {
    if !called_async {
        return CallbackResult {
            hr: STATUS_PENDING,
            value: None,
        };
    }
    CallbackResult {
        hr: S_OK,
        value: Some(false),
    }
}
