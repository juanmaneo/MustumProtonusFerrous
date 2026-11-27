//! Oracle-derived test vectors for the ported sccbase logic.
//!
//! Vectors were captured under `target/canon/sccbase/` by compiling the verbatim
//! `__wine_spec_unimplemented_stub` body from `dlls/ntdll/exception.c` with one stub per
//! `@ stub` line of `dlls/sccbase/sccbase.spec`, and running the harness with
//! `gcc -fsanitize=address,undefined -g`. The harness printed 24 rows. ASan and UBSan reported
//! nothing, so no row is excluded. The two `stdcall -private` entries have no C body and no stub,
//! so they have no row.

use super::*;

/// One oracle row: the export's helper, then the code, flags, number of parameters, module and
/// function of the record the harness printed for that export.
type Row = (
    fn() -> StubException,
    u32,
    u32,
    u32,
    &'static str,
    &'static str,
);

#[test]
fn oracle_stub_rows() {
    let rows: [Row; 24] = [
        (
            cp_acquire_context,
            0x8000_0100,
            0x01,
            2,
            "sccbase.dll",
            "CPAcquireContext",
        ),
        (
            cp_acquire_context_w,
            0x8000_0100,
            0x01,
            2,
            "sccbase.dll",
            "CPAcquireContextW",
        ),
        (
            cp_create_hash,
            0x8000_0100,
            0x01,
            2,
            "sccbase.dll",
            "CPCreateHash",
        ),
        (cp_decrypt, 0x8000_0100, 0x01, 2, "sccbase.dll", "CPDecrypt"),
        (
            cp_derive_key,
            0x8000_0100,
            0x01,
            2,
            "sccbase.dll",
            "CPDeriveKey",
        ),
        (
            cp_destroy_hash,
            0x8000_0100,
            0x01,
            2,
            "sccbase.dll",
            "CPDestroyHash",
        ),
        (
            cp_destroy_key,
            0x8000_0100,
            0x01,
            2,
            "sccbase.dll",
            "CPDestroyKey",
        ),
        (cp_encrypt, 0x8000_0100, 0x01, 2, "sccbase.dll", "CPEncrypt"),
        (
            cp_export_key,
            0x8000_0100,
            0x01,
            2,
            "sccbase.dll",
            "CPExportKey",
        ),
        (cp_gen_key, 0x8000_0100, 0x01, 2, "sccbase.dll", "CPGenKey"),
        (
            cp_gen_random,
            0x8000_0100,
            0x01,
            2,
            "sccbase.dll",
            "CPGenRandom",
        ),
        (
            cp_get_hash_param,
            0x8000_0100,
            0x01,
            2,
            "sccbase.dll",
            "CPGetHashParam",
        ),
        (
            cp_get_key_param,
            0x8000_0100,
            0x01,
            2,
            "sccbase.dll",
            "CPGetKeyParam",
        ),
        (
            cp_get_prov_param,
            0x8000_0100,
            0x01,
            2,
            "sccbase.dll",
            "CPGetProvParam",
        ),
        (
            cp_get_user_key,
            0x8000_0100,
            0x01,
            2,
            "sccbase.dll",
            "CPGetUserKey",
        ),
        (
            cp_hash_data,
            0x8000_0100,
            0x01,
            2,
            "sccbase.dll",
            "CPHashData",
        ),
        (
            cp_hash_session_key,
            0x8000_0100,
            0x01,
            2,
            "sccbase.dll",
            "CPHashSessionKey",
        ),
        (
            cp_import_key,
            0x8000_0100,
            0x01,
            2,
            "sccbase.dll",
            "CPImportKey",
        ),
        (
            cp_release_context,
            0x8000_0100,
            0x01,
            2,
            "sccbase.dll",
            "CPReleaseContext",
        ),
        (
            cp_set_hash_param,
            0x8000_0100,
            0x01,
            2,
            "sccbase.dll",
            "CPSetHashParam",
        ),
        (
            cp_set_key_param,
            0x8000_0100,
            0x01,
            2,
            "sccbase.dll",
            "CPSetKeyParam",
        ),
        (
            cp_set_prov_param,
            0x8000_0100,
            0x01,
            2,
            "sccbase.dll",
            "CPSetProvParam",
        ),
        (
            cp_sign_hash,
            0x8000_0100,
            0x01,
            2,
            "sccbase.dll",
            "CPSignHash",
        ),
        (
            cp_verify_signature,
            0x8000_0100,
            0x01,
            2,
            "sccbase.dll",
            "CPVerifySignature",
        ),
    ];
    for (stub, code, flags, number_parameters, module, function) in rows {
        assert_eq!(
            stub(),
            StubException {
                code,
                flags,
                number_parameters,
                module,
                function,
            }
        );
        assert_eq!(export_kind(function), Some(ExportKind::Stub));
    }
}
