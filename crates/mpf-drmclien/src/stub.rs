/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = drmclien.dll` in `dlls/drmclien/Makefile.in`.
const DRMCLIEN_DLL_NAME: &str = "drmclien.dll";

/// The record that `__wine_spec_unimplemented_stub` raises.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`. The C function raises
/// the record in a loop and never returns, so the port returns the record instead.
pub fn wine_spec_unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        number_parameters: 2,
        module,
        function,
    }
}

/// `CanDecrypt`, a `@ stub` export.
pub fn can_decrypt() -> StubException {
    wine_spec_unimplemented_stub(DRMCLIEN_DLL_NAME, "CanDecrypt")
}

/// `Decrypt`, a `@ stub` export.
pub fn decrypt() -> StubException {
    wine_spec_unimplemented_stub(DRMCLIEN_DLL_NAME, "Decrypt")
}

/// `RequestLicense`, a `@ stub` export.
pub fn request_license() -> StubException {
    wine_spec_unimplemented_stub(DRMCLIEN_DLL_NAME, "RequestLicense")
}

/// `ProcessResponse`, a `@ stub` export.
pub fn process_response() -> StubException {
    wine_spec_unimplemented_stub(DRMCLIEN_DLL_NAME, "ProcessResponse")
}

/// `Bind`, a `@ stub` export.
pub fn bind() -> StubException {
    wine_spec_unimplemented_stub(DRMCLIEN_DLL_NAME, "Bind")
}

/// `KeyExchange`, a `@ stub` export.
pub fn key_exchange() -> StubException {
    wine_spec_unimplemented_stub(DRMCLIEN_DLL_NAME, "KeyExchange")
}

/// `GetVersion`, a `@ stub` export.
pub fn get_version() -> StubException {
    wine_spec_unimplemented_stub(DRMCLIEN_DLL_NAME, "GetVersion")
}

/// `CanDecryptEx`, a `@ stub` export.
pub fn can_decrypt_ex() -> StubException {
    wine_spec_unimplemented_stub(DRMCLIEN_DLL_NAME, "CanDecryptEx")
}

/// `BindEx`, a `@ stub` export.
pub fn bind_ex() -> StubException {
    wine_spec_unimplemented_stub(DRMCLIEN_DLL_NAME, "BindEx")
}

/// `CDRMLiteCrypto::CDRMLiteCrypto` constructor (`??0CDRMLiteCrypto@@QAE@XZ`), a `@ stub` export.
pub fn cdrmlite_crypto_ctor() -> StubException {
    wine_spec_unimplemented_stub(DRMCLIEN_DLL_NAME, "??0CDRMLiteCrypto@@QAE@XZ")
}

/// `CDRMLiteCrypto::~CDRMLiteCrypto` destructor (`??1CDRMLiteCrypto@@QAE@XZ`), a `@ stub` export.
pub fn cdrmlite_crypto_dtor() -> StubException {
    wine_spec_unimplemented_stub(DRMCLIEN_DLL_NAME, "??1CDRMLiteCrypto@@QAE@XZ")
}

/// `SetAppSec`, a `@ stub` export.
pub fn set_app_sec() -> StubException {
    wine_spec_unimplemented_stub(DRMCLIEN_DLL_NAME, "SetAppSec")
}

/// `SetLicenseStore`, a `@ stub` export.
pub fn set_license_store() -> StubException {
    wine_spec_unimplemented_stub(DRMCLIEN_DLL_NAME, "SetLicenseStore")
}

/// `GetPMLicenseFileName`, a `@ stub` export.
pub fn get_pm_license_file_name() -> StubException {
    wine_spec_unimplemented_stub(DRMCLIEN_DLL_NAME, "GetPMLicenseFileName")
}

/// `GetPMLicenseSize`, a `@ stub` export.
pub fn get_pm_license_size() -> StubException {
    wine_spec_unimplemented_stub(DRMCLIEN_DLL_NAME, "GetPMLicenseSize")
}

/// `CreatePMLicense`, a `@ stub` export.
pub fn create_pm_license() -> StubException {
    wine_spec_unimplemented_stub(DRMCLIEN_DLL_NAME, "CreatePMLicense")
}

/// `GenerateNewLicense`, a `@ stub` export.
pub fn generate_new_license() -> StubException {
    wine_spec_unimplemented_stub(DRMCLIEN_DLL_NAME, "GenerateNewLicense")
}

/// `Encrypt`, a `@ stub` export.
pub fn encrypt() -> StubException {
    wine_spec_unimplemented_stub(DRMCLIEN_DLL_NAME, "Encrypt")
}

/// `InitAppCerts`, a `@ stub` export.
pub fn init_app_certs() -> StubException {
    wine_spec_unimplemented_stub(DRMCLIEN_DLL_NAME, "InitAppCerts")
}

/// `QueryXferToPM`, a `@ stub` export.
pub fn query_xfer_to_pm() -> StubException {
    wine_spec_unimplemented_stub(DRMCLIEN_DLL_NAME, "QueryXferToPM")
}

/// `CDRMLiteCrypto::BackupLicenses`, a `@ stub` export.
pub fn cdrmlite_crypto_backup_licenses() -> StubException {
    wine_spec_unimplemented_stub(
        DRMCLIEN_DLL_NAME,
        "?BackupLicenses@CDRMLiteCrypto@@QAEJKPAGPAUIUnknown@@PAHPAX@Z",
    )
}

/// `CDRMLiteCrypto::EncryptFast`, a `@ stub` export.
pub fn cdrmlite_crypto_encrypt_fast() -> StubException {
    wine_spec_unimplemented_stub(
        DRMCLIEN_DLL_NAME,
        "?EncryptFast@CDRMLiteCrypto@@QAEJPBDKPAE1@Z",
    )
}

/// `CDRMLiteCrypto::EncryptIndirectFast`, a `@ stub` export.
pub fn cdrmlite_crypto_encrypt_indirect_fast() -> StubException {
    wine_spec_unimplemented_stub(
        DRMCLIEN_DLL_NAME,
        "?EncryptIndirectFast@CDRMLiteCrypto@@QAEJPBDKPAE1@Z",
    )
}

/// `CDRMLiteCrypto::GenerateNewLicenseEx`, a `@ stub` export.
pub fn cdrmlite_crypto_generate_new_license_ex() -> StubException {
    wine_spec_unimplemented_stub(
        DRMCLIEN_DLL_NAME,
        "?GenerateNewLicenseEx@CDRMLiteCrypto@@QAEJKPAE00PAPAD10@Z",
    )
}

/// `CDRMLiteCrypto::GetLicenses`, a `@ stub` export.
pub fn cdrmlite_crypto_get_licenses() -> StubException {
    wine_spec_unimplemented_stub(
        DRMCLIEN_DLL_NAME,
        "?GetLicenses@CDRMLiteCrypto@@QAEJPBDPAUPMLICENSE@@PAKKPAX2PAE@Z",
    )
}

/// `CDRMLiteCrypto::GetPublicKey`, a `@ stub` export.
pub fn cdrmlite_crypto_get_public_key() -> StubException {
    wine_spec_unimplemented_stub(
        DRMCLIEN_DLL_NAME,
        "?GetPublicKey@CDRMLiteCrypto@@QAEJPAUPKCERT@@@Z",
    )
}

/// `CDRMLiteCrypto::QueryXferToPMEx`, a `@ stub` export.
pub fn cdrmlite_crypto_query_xfer_to_pm_ex() -> StubException {
    wine_spec_unimplemented_stub(
        DRMCLIEN_DLL_NAME,
        "?QueryXferToPMEx@CDRMLiteCrypto@@QAEJPBDKPAKPAEK2K12@Z",
    )
}

/// `CDRMLiteCrypto::RestoreLicenses`, a `@ stub` export.
pub fn cdrmlite_crypto_restore_licenses() -> StubException {
    wine_spec_unimplemented_stub(
        DRMCLIEN_DLL_NAME,
        "?RestoreLicenses@CDRMLiteCrypto@@QAEJKPAEPAGPAUIUnknown@@PAHPAX@Z",
    )
}
