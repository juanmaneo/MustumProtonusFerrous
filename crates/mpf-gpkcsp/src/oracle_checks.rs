//! Oracle-derived test vectors for the ported gpkcsp stub records.
//!
//! Vectors were captured by compiling the verbatim Wine C body of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`, lines 504-516) under `target/canon/gpkcsp/` with
//! `gcc -fsanitize=address,undefined -g`, and running the harness. The C body loops forever, so
//! the harness replaces `RtlRaiseException` with a recorder that stores the record and `longjmp`s
//! out of the loop. The 23 printed rows, one per `@ stub` export, are all kept: ASan and UBSan
//! reported nothing, so no vector is excluded.

use super::*;

/// `(helper, code, flags, number_parameters, module, function)` for each `@ stub` export, in spec order.
type StubRow = (
    fn() -> StubException,
    u32,
    u32,
    u32,
    &'static str,
    &'static str,
);

const STUB_ROWS: [StubRow; 23] = [
    // row 01 CPAcquireContext -> raise code=0x80000100 flags=0x1 nargs=2 p0=gpkcsp.dll p1=CPAcquireContext
    (
        cp_acquire_context,
        0x8000_0100,
        0x01,
        2,
        "gpkcsp.dll",
        "CPAcquireContext",
    ),
    // row 02 CPCreateHash -> raise code=0x80000100 flags=0x1 nargs=2 p0=gpkcsp.dll p1=CPCreateHash
    (
        cp_create_hash,
        0x8000_0100,
        0x01,
        2,
        "gpkcsp.dll",
        "CPCreateHash",
    ),
    // row 03 CPDecrypt -> raise code=0x80000100 flags=0x1 nargs=2 p0=gpkcsp.dll p1=CPDecrypt
    (cp_decrypt, 0x8000_0100, 0x01, 2, "gpkcsp.dll", "CPDecrypt"),
    // row 04 CPDeriveKey -> raise code=0x80000100 flags=0x1 nargs=2 p0=gpkcsp.dll p1=CPDeriveKey
    (
        cp_derive_key,
        0x8000_0100,
        0x01,
        2,
        "gpkcsp.dll",
        "CPDeriveKey",
    ),
    // row 05 CPDestroyHash -> raise code=0x80000100 flags=0x1 nargs=2 p0=gpkcsp.dll p1=CPDestroyHash
    (
        cp_destroy_hash,
        0x8000_0100,
        0x01,
        2,
        "gpkcsp.dll",
        "CPDestroyHash",
    ),
    // row 06 CPDestroyKey -> raise code=0x80000100 flags=0x1 nargs=2 p0=gpkcsp.dll p1=CPDestroyKey
    (
        cp_destroy_key,
        0x8000_0100,
        0x01,
        2,
        "gpkcsp.dll",
        "CPDestroyKey",
    ),
    // row 07 CPEncrypt -> raise code=0x80000100 flags=0x1 nargs=2 p0=gpkcsp.dll p1=CPEncrypt
    (cp_encrypt, 0x8000_0100, 0x01, 2, "gpkcsp.dll", "CPEncrypt"),
    // row 08 CPExportKey -> raise code=0x80000100 flags=0x1 nargs=2 p0=gpkcsp.dll p1=CPExportKey
    (
        cp_export_key,
        0x8000_0100,
        0x01,
        2,
        "gpkcsp.dll",
        "CPExportKey",
    ),
    // row 09 CPGenKey -> raise code=0x80000100 flags=0x1 nargs=2 p0=gpkcsp.dll p1=CPGenKey
    (cp_gen_key, 0x8000_0100, 0x01, 2, "gpkcsp.dll", "CPGenKey"),
    // row 10 CPGenRandom -> raise code=0x80000100 flags=0x1 nargs=2 p0=gpkcsp.dll p1=CPGenRandom
    (
        cp_gen_random,
        0x8000_0100,
        0x01,
        2,
        "gpkcsp.dll",
        "CPGenRandom",
    ),
    // row 11 CPGetHashParam -> raise code=0x80000100 flags=0x1 nargs=2 p0=gpkcsp.dll p1=CPGetHashParam
    (
        cp_get_hash_param,
        0x8000_0100,
        0x01,
        2,
        "gpkcsp.dll",
        "CPGetHashParam",
    ),
    // row 12 CPGetKeyParam -> raise code=0x80000100 flags=0x1 nargs=2 p0=gpkcsp.dll p1=CPGetKeyParam
    (
        cp_get_key_param,
        0x8000_0100,
        0x01,
        2,
        "gpkcsp.dll",
        "CPGetKeyParam",
    ),
    // row 13 CPGetProvParam -> raise code=0x80000100 flags=0x1 nargs=2 p0=gpkcsp.dll p1=CPGetProvParam
    (
        cp_get_prov_param,
        0x8000_0100,
        0x01,
        2,
        "gpkcsp.dll",
        "CPGetProvParam",
    ),
    // row 14 CPGetUserKey -> raise code=0x80000100 flags=0x1 nargs=2 p0=gpkcsp.dll p1=CPGetUserKey
    (
        cp_get_user_key,
        0x8000_0100,
        0x01,
        2,
        "gpkcsp.dll",
        "CPGetUserKey",
    ),
    // row 15 CPHashData -> raise code=0x80000100 flags=0x1 nargs=2 p0=gpkcsp.dll p1=CPHashData
    (
        cp_hash_data,
        0x8000_0100,
        0x01,
        2,
        "gpkcsp.dll",
        "CPHashData",
    ),
    // row 16 CPHashSessionKey -> raise code=0x80000100 flags=0x1 nargs=2 p0=gpkcsp.dll p1=CPHashSessionKey
    (
        cp_hash_session_key,
        0x8000_0100,
        0x01,
        2,
        "gpkcsp.dll",
        "CPHashSessionKey",
    ),
    // row 17 CPImportKey -> raise code=0x80000100 flags=0x1 nargs=2 p0=gpkcsp.dll p1=CPImportKey
    (
        cp_import_key,
        0x8000_0100,
        0x01,
        2,
        "gpkcsp.dll",
        "CPImportKey",
    ),
    // row 18 CPReleaseContext -> raise code=0x80000100 flags=0x1 nargs=2 p0=gpkcsp.dll p1=CPReleaseContext
    (
        cp_release_context,
        0x8000_0100,
        0x01,
        2,
        "gpkcsp.dll",
        "CPReleaseContext",
    ),
    // row 19 CPSetHashParam -> raise code=0x80000100 flags=0x1 nargs=2 p0=gpkcsp.dll p1=CPSetHashParam
    (
        cp_set_hash_param,
        0x8000_0100,
        0x01,
        2,
        "gpkcsp.dll",
        "CPSetHashParam",
    ),
    // row 20 CPSetKeyParam -> raise code=0x80000100 flags=0x1 nargs=2 p0=gpkcsp.dll p1=CPSetKeyParam
    (
        cp_set_key_param,
        0x8000_0100,
        0x01,
        2,
        "gpkcsp.dll",
        "CPSetKeyParam",
    ),
    // row 21 CPSetProvParam -> raise code=0x80000100 flags=0x1 nargs=2 p0=gpkcsp.dll p1=CPSetProvParam
    (
        cp_set_prov_param,
        0x8000_0100,
        0x01,
        2,
        "gpkcsp.dll",
        "CPSetProvParam",
    ),
    // row 22 CPSignHash -> raise code=0x80000100 flags=0x1 nargs=2 p0=gpkcsp.dll p1=CPSignHash
    (
        cp_sign_hash,
        0x8000_0100,
        0x01,
        2,
        "gpkcsp.dll",
        "CPSignHash",
    ),
    // row 23 CPVerifySignature -> raise code=0x80000100 flags=0x1 nargs=2 p0=gpkcsp.dll p1=CPVerifySignature
    (
        cp_verify_signature,
        0x8000_0100,
        0x01,
        2,
        "gpkcsp.dll",
        "CPVerifySignature",
    ),
];

#[test]
fn oracle_stub_rows() {
    for (raise, code, flags, number_parameters, module, function) in STUB_ROWS {
        assert_eq!(
            raise(),
            StubException {
                code,
                flags,
                number_parameters,
                module,
                function,
            }
        );
    }
}
