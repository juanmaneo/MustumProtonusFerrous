//! Oracle-derived test vectors for the ported mssign32 logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of the six explicit exports
//! (`dlls/mssign32/mssign32_main.c` lines 32-81) and of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c` lines 504-516) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/mssign32/`, and running the harness. It prints 28 rows, one per export of
//! `dlls/mssign32/mssign32.spec` except the two `-private` DllRegisterServer and DllUnregisterServer
//! lines. ASan and UBSan flagged no row, so none is excluded.

use super::*;

/// The record the harness prints for a stub row: code `0x80000100`, flags `0x1`, two parameters,
/// `mssign32.dll` as the module and the export name as the function.
fn raised(function: &'static str) -> StubException {
    StubException {
        code: 0x8000_0100,
        flags: 0x01,
        number_parameters: 2,
        module: "mssign32.dll",
        function,
    }
}

#[test]
fn oracle_explicit_rows() {
    // row 3 PvkFreeCryptProv hProv=0x1234 provtype=1 caption=Cap tmp=Tmp -> void, returned
    pvk_free_crypt_prov();
    // row 4 PvkGetCryptProv all NULL/0 -> hr=0x80004005
    assert_eq!(pvk_get_crypt_prov(), 0x8000_4005);
    // row 7 PvkPrivateKeyAcquireContextFromMemory prov=Cap type=1 data=4 bytes owner=NULL key=Cap -> 0
    assert!(!pvk_private_key_acquire_context_from_memory());
    // row 23 SignerFreeSignerContext NULL -> hr=0x00000000
    assert_eq!(signer_free_signer_context(), 0x0000_0000);
    // row 24 SignerSign all NULL -> hr=0x80004001
    assert_eq!(signer_sign(), 0x8000_4001);
    // row 25 SignerSignEx flags=0 all NULL -> hr=0x80004001
    assert_eq!(signer_sign_ex(), 0x8000_4001);
}

#[test]
fn oracle_stub_rows() {
    // row 1 FreeCryptProvFromCert -> raise code=0x80000100 flags=0x1 nargs=2 p0=mssign32.dll p1=FreeCryptProvFromCert
    assert_eq!(free_crypt_prov_from_cert(), raised("FreeCryptProvFromCert"));
    // row 2 GetCryptProvFromCert
    assert_eq!(get_crypt_prov_from_cert(), raised("GetCryptProvFromCert"));
    // row 5 PvkPrivateKeyAcquireContext
    assert_eq!(
        pvk_private_key_acquire_context(),
        raised("PvkPrivateKeyAcquireContext")
    );
    // row 6 PvkPrivateKeyAcquireContextA
    assert_eq!(
        pvk_private_key_acquire_context_a(),
        raised("PvkPrivateKeyAcquireContextA")
    );
    // row 8 PvkPrivateKeyAcquireContextFromMemoryA
    assert_eq!(
        pvk_private_key_acquire_context_from_memory_a(),
        raised("PvkPrivateKeyAcquireContextFromMemoryA")
    );
    // row 9 PvkPrivateKeyLoad
    assert_eq!(pvk_private_key_load(), raised("PvkPrivateKeyLoad"));
    // row 10 PvkPrivateKeyLoadA
    assert_eq!(pvk_private_key_load_a(), raised("PvkPrivateKeyLoadA"));
    // row 11 PvkPrivateKeyLoadFromMemory
    assert_eq!(
        pvk_private_key_load_from_memory(),
        raised("PvkPrivateKeyLoadFromMemory")
    );
    // row 12 PvkPrivateKeyLoadFromMemoryA
    assert_eq!(
        pvk_private_key_load_from_memory_a(),
        raised("PvkPrivateKeyLoadFromMemoryA")
    );
    // row 13 PvkPrivateKeyReleaseContext
    assert_eq!(
        pvk_private_key_release_context(),
        raised("PvkPrivateKeyReleaseContext")
    );
    // row 14 PvkPrivateKeyReleaseContextA
    assert_eq!(
        pvk_private_key_release_context_a(),
        raised("PvkPrivateKeyReleaseContextA")
    );
    // row 15 PvkPrivateKeySave
    assert_eq!(pvk_private_key_save(), raised("PvkPrivateKeySave"));
    // row 16 PvkPrivateKeySaveA
    assert_eq!(pvk_private_key_save_a(), raised("PvkPrivateKeySaveA"));
    // row 17 PvkPrivateKeySaveToMemory
    assert_eq!(
        pvk_private_key_save_to_memory(),
        raised("PvkPrivateKeySaveToMemory")
    );
    // row 18 PvkPrivateKeySaveToMemoryA
    assert_eq!(
        pvk_private_key_save_to_memory_a(),
        raised("PvkPrivateKeySaveToMemoryA")
    );
    // row 19 SignError
    assert_eq!(sign_error(), raised("SignError"));
    // row 20 SignerAddTimeStampResponse
    assert_eq!(
        signer_add_time_stamp_response(),
        raised("SignerAddTimeStampResponse")
    );
    // row 21 SignerAddTimeStampResponseEx
    assert_eq!(
        signer_add_time_stamp_response_ex(),
        raised("SignerAddTimeStampResponseEx")
    );
    // row 22 SignerCreateTimeStampRequest
    assert_eq!(
        signer_create_time_stamp_request(),
        raised("SignerCreateTimeStampRequest")
    );
    // row 26 SignerTimeStamp
    assert_eq!(signer_time_stamp(), raised("SignerTimeStamp"));
    // row 27 SignerTimeStampEx
    assert_eq!(signer_time_stamp_ex(), raised("SignerTimeStampEx"));
    // row 28 SpcGetCertFromKey
    assert_eq!(spc_get_cert_from_key(), raised("SpcGetCertFromKey"));
}
