//! Port of the portable logic in Wine's `dlls/mssign32/mssign32_main.c` and `dlls/mssign32/mssign32.spec`.
//!
//! Ported:
//! - The explicit exports that return a fixed value: [`pvk_free_crypt_prov`], [`pvk_get_crypt_prov`],
//!   [`pvk_private_key_acquire_context_from_memory`], [`signer_sign`], [`signer_sign_ex`] and
//!   [`signer_free_signer_context`], with the [`E_FAIL`], [`E_NOTIMPL`] and [`S_OK`] values from
//!   `include/winerror.h`.
//! - The 22 `@ stub` exports, modelled as the `__wine_spec_unimplemented_stub` record they raise:
//!   [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per export, such as
//!   [`sign_error`] and [`spc_get_cert_from_key`].
//!
//! Skipped:
//! - The `FIXME` traces and the parameters of the explicit exports. The C bodies never read them, so
//!   the results do not depend on them.
//! - `DllRegisterServer` and `DllUnregisterServer` (`@ stdcall -private`). `mssign32_main.c` does not
//!   define them. `libs/winecrt0/dll_register.c` provides them and forwards to `__wine_register_resources`
//!   (`libs/winecrt0/register.c`), which registers the module's registry resources through the OS registrar.

mod pvk;
mod signer;
mod stub;

pub use pvk::{
    pvk_free_crypt_prov, pvk_get_crypt_prov, pvk_private_key_acquire_context_from_memory, E_FAIL,
};
pub use signer::{signer_free_signer_context, signer_sign, signer_sign_ex, E_NOTIMPL, S_OK};
pub use stub::{
    free_crypt_prov_from_cert, get_crypt_prov_from_cert, pvk_private_key_acquire_context,
    pvk_private_key_acquire_context_a, pvk_private_key_acquire_context_from_memory_a,
    pvk_private_key_load, pvk_private_key_load_a, pvk_private_key_load_from_memory,
    pvk_private_key_load_from_memory_a, pvk_private_key_release_context,
    pvk_private_key_release_context_a, pvk_private_key_save, pvk_private_key_save_a,
    pvk_private_key_save_to_memory, pvk_private_key_save_to_memory_a, sign_error,
    signer_add_time_stamp_response, signer_add_time_stamp_response_ex,
    signer_create_time_stamp_request, signer_time_stamp, signer_time_stamp_ex,
    spc_get_cert_from_key, wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
