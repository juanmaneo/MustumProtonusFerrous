//! Port of the portable logic in Wine's `dlls/cryptowinrt/` (`async.c`,
//! `credentials.c`, `main.c`) and of `encodeBase64W` from `dlls/crypt32/base64.c`,
//! which `EncodeToBase64String` uses.
//!
//! Ported:
//! - `EncodeToBase64String` payload (`encodeBase64W` with `CRYPT_STRING_BASE64 |
//!   CRYPT_STRING_NOCRLF`): [`encode_base64`]
//! - `DllGetActivationFactory` class-name dispatch: [`dll_get_activation_factory`],
//!   [`ActivationClass`]
//! - `is_supported_async` (`KeyCredentialManager.IsSupportedAsync`): [`is_supported_async`]
//! - The `IAsyncInfo` state machine (`async.c`): [`AsyncInfo`], [`AsyncStatus`],
//!   [`Handler`], [`Completion`], [`Callback`], [`CallbackResult`]
//! - `FAILED`: [`failed`]
//!
//! Skipped:
//! - `GenerateRandomNumber`: `BCryptGenRandom`, OS randomness.
//! - HSTRING, `IBuffer` and `IBufferByteAccess` plumbing. For non-empty input, the
//!   HSTRING that `EncodeToBase64String` returns has length `chars + 1` with a trailing
//!   NUL code unit (`alloc_string` keeps the NUL-inclusive length and
//!   `WindowsPromoteStringBuffer` does not change it); the port returns only the
//!   encoded text.
//! - `FIXME` stubs that return `E_NOTIMPL`, and `DllGetClassObject`, which only
//!   returns `CLASS_E_CLASSNOTAVAILABLE`.
//! - COM vtables, `QueryInterface`, reference counts, the critical section, threadpool
//!   submission and the `IInspectable` outer object. Handler dispatch is reported
//!   through return values instead of a callback.

mod activation;
mod async_info;
mod base64;
mod credentials;

pub use activation::{dll_get_activation_factory, ActivationClass};
pub use async_info::{AsyncInfo, AsyncStatus, Callback, CallbackResult, Completion, Handler};
pub use base64::encode_base64;
pub use credentials::is_supported_async;

/// `HRESULT`, as a signed 32-bit value so that [`failed`] is a sign test.
pub type Hresult = i32;

pub const S_OK: Hresult = 0;
pub const E_FAIL: Hresult = 0x8000_4005u32 as i32;
pub const E_ILLEGAL_STATE_CHANGE: Hresult = 0x8000_000Du32 as i32;
pub const E_ILLEGAL_METHOD_CALL: Hresult = 0x8000_000Eu32 as i32;
pub const E_ILLEGAL_DELEGATE_ASSIGNMENT: Hresult = 0x8000_0018u32 as i32;
pub const CLASS_E_CLASSNOTAVAILABLE: Hresult = 0x8004_0111u32 as i32;
pub const STATUS_PENDING: Hresult = 0x0000_0103;

/// `FAILED(hr)`.
pub fn failed(hr: Hresult) -> bool {
    hr < 0
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
