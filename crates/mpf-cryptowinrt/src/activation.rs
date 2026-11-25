//! `DllGetActivationFactory` class-name dispatch from Wine's `dlls/cryptowinrt/main.c`.

use crate::{Hresult, CLASS_E_CLASSNOTAVAILABLE};

const CRYPTOGRAPHIC_BUFFER: &str = "Windows.Security.Cryptography.CryptographicBuffer";
const KEY_CREDENTIAL_MANAGER: &str = "Windows.Security.Credentials.KeyCredentialManager";

/// The runtime classes this DLL can activate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActivationClass {
    CryptographicBuffer,
    KeyCredentialManager,
}

/// Looks up a runtime class name. Like `wcscmp`, the name ends at the first NUL,
/// so anything after an embedded NUL is ignored. Unknown names give
/// `CLASS_E_CLASSNOTAVAILABLE`.
pub fn dll_get_activation_factory(name: &[u16]) -> Result<ActivationClass, Hresult> {
    let end = name.iter().position(|&c| c == 0).unwrap_or(name.len());
    let name = &name[..end];
    if name.iter().copied().eq(CRYPTOGRAPHIC_BUFFER.encode_utf16()) {
        Ok(ActivationClass::CryptographicBuffer)
    } else if name
        .iter()
        .copied()
        .eq(KEY_CREDENTIAL_MANAGER.encode_utf16())
    {
        Ok(ActivationClass::KeyCredentialManager)
    } else {
        Err(CLASS_E_CLASSNOTAVAILABLE)
    }
}
