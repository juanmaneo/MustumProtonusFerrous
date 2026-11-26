//! Oracle-derived test vectors for the ported drmclien stub records.
//!
//! Vectors were captured by compiling the verbatim Wine C body of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`, lines 504-516) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/drmclien/`, and calling it once for each `@ stub` export of
//! `dlls/drmclien/drmclien.spec` with the module `drmclien.dll`. `RtlRaiseException` is replaced
//! by a recorder that longjmps out of the stub's `for (;;)` loop. The 28 printed rows, one per
//! export, are all kept: ASan and UBSan reported nothing, so no vector was excluded.

use super::*;

/// One printed row: the helper, then the code, flags, number of parameters, module and function.
type Row = (
    fn() -> StubException,
    u32,
    u32,
    u32,
    &'static str,
    &'static str,
);

/// One row per `@ stub` export, in spec order.
const ROWS: [Row; 28] = [
    (can_decrypt, 0x8000_0100, 1, 2, "drmclien.dll", "CanDecrypt"),
    (decrypt, 0x8000_0100, 1, 2, "drmclien.dll", "Decrypt"),
    (
        request_license,
        0x8000_0100,
        1,
        2,
        "drmclien.dll",
        "RequestLicense",
    ),
    (
        process_response,
        0x8000_0100,
        1,
        2,
        "drmclien.dll",
        "ProcessResponse",
    ),
    (bind, 0x8000_0100, 1, 2, "drmclien.dll", "Bind"),
    (
        key_exchange,
        0x8000_0100,
        1,
        2,
        "drmclien.dll",
        "KeyExchange",
    ),
    (get_version, 0x8000_0100, 1, 2, "drmclien.dll", "GetVersion"),
    (
        can_decrypt_ex,
        0x8000_0100,
        1,
        2,
        "drmclien.dll",
        "CanDecryptEx",
    ),
    (bind_ex, 0x8000_0100, 1, 2, "drmclien.dll", "BindEx"),
    (
        cdrmlite_crypto_ctor,
        0x8000_0100,
        1,
        2,
        "drmclien.dll",
        "??0CDRMLiteCrypto@@QAE@XZ",
    ),
    (
        cdrmlite_crypto_dtor,
        0x8000_0100,
        1,
        2,
        "drmclien.dll",
        "??1CDRMLiteCrypto@@QAE@XZ",
    ),
    (set_app_sec, 0x8000_0100, 1, 2, "drmclien.dll", "SetAppSec"),
    (
        set_license_store,
        0x8000_0100,
        1,
        2,
        "drmclien.dll",
        "SetLicenseStore",
    ),
    (
        get_pm_license_file_name,
        0x8000_0100,
        1,
        2,
        "drmclien.dll",
        "GetPMLicenseFileName",
    ),
    (
        get_pm_license_size,
        0x8000_0100,
        1,
        2,
        "drmclien.dll",
        "GetPMLicenseSize",
    ),
    (
        create_pm_license,
        0x8000_0100,
        1,
        2,
        "drmclien.dll",
        "CreatePMLicense",
    ),
    (
        generate_new_license,
        0x8000_0100,
        1,
        2,
        "drmclien.dll",
        "GenerateNewLicense",
    ),
    (encrypt, 0x8000_0100, 1, 2, "drmclien.dll", "Encrypt"),
    (
        init_app_certs,
        0x8000_0100,
        1,
        2,
        "drmclien.dll",
        "InitAppCerts",
    ),
    (
        query_xfer_to_pm,
        0x8000_0100,
        1,
        2,
        "drmclien.dll",
        "QueryXferToPM",
    ),
    (
        cdrmlite_crypto_backup_licenses,
        0x8000_0100,
        1,
        2,
        "drmclien.dll",
        "?BackupLicenses@CDRMLiteCrypto@@QAEJKPAGPAUIUnknown@@PAHPAX@Z",
    ),
    (
        cdrmlite_crypto_encrypt_fast,
        0x8000_0100,
        1,
        2,
        "drmclien.dll",
        "?EncryptFast@CDRMLiteCrypto@@QAEJPBDKPAE1@Z",
    ),
    (
        cdrmlite_crypto_encrypt_indirect_fast,
        0x8000_0100,
        1,
        2,
        "drmclien.dll",
        "?EncryptIndirectFast@CDRMLiteCrypto@@QAEJPBDKPAE1@Z",
    ),
    (
        cdrmlite_crypto_generate_new_license_ex,
        0x8000_0100,
        1,
        2,
        "drmclien.dll",
        "?GenerateNewLicenseEx@CDRMLiteCrypto@@QAEJKPAE00PAPAD10@Z",
    ),
    (
        cdrmlite_crypto_get_licenses,
        0x8000_0100,
        1,
        2,
        "drmclien.dll",
        "?GetLicenses@CDRMLiteCrypto@@QAEJPBDPAUPMLICENSE@@PAKKPAX2PAE@Z",
    ),
    (
        cdrmlite_crypto_get_public_key,
        0x8000_0100,
        1,
        2,
        "drmclien.dll",
        "?GetPublicKey@CDRMLiteCrypto@@QAEJPAUPKCERT@@@Z",
    ),
    (
        cdrmlite_crypto_query_xfer_to_pm_ex,
        0x8000_0100,
        1,
        2,
        "drmclien.dll",
        "?QueryXferToPMEx@CDRMLiteCrypto@@QAEJPBDKPAKPAEK2K12@Z",
    ),
    (
        cdrmlite_crypto_restore_licenses,
        0x8000_0100,
        1,
        2,
        "drmclien.dll",
        "?RestoreLicenses@CDRMLiteCrypto@@QAEJKPAEPAGPAUIUnknown@@PAHPAX@Z",
    ),
];

#[test]
fn oracle_stub_rows() {
    for (i, (helper, code, flags, number_parameters, module, function)) in
        ROWS.into_iter().enumerate()
    {
        assert_eq!(
            helper(),
            StubException {
                code,
                flags,
                number_parameters,
                module,
                function,
            },
            "row {:02}",
            i + 1
        );
    }
}
