//! Oracle-derived test vectors for the ported slbcsp stub exceptions.
//!
//! Vectors were captured by compiling the verbatim Wine C body of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c` lines 504-516) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/slbcsp/`, and running the harness `oracle.c` once per `@ stub` export of
//! `dlls/slbcsp/slbcsp.spec`. The 25 printed rows were all clean under ASan and UBSan, so none is
//! excluded.

use super::*;

type OracleRow = (fn() -> StubException, u32, u32, u32, &'static str, &'static str);

/// One row per `@ stub` export, in spec order: the helper, then the `code`, `flags`, number of
/// parameters, `module` and `function` that the harness printed.
const ORACLE_ROWS: [OracleRow; 25] = [
    // row 01 CPAcquireContext -> raise code=0x80000100 flags=0x1 nargs=2 p0=slbcsp.dll p1=CPAcquireContext
    (
        cp_acquire_context,
        0x8000_0100,
        0x01,
        2,
        "slbcsp.dll",
        "CPAcquireContext",
    ),
    // row 02 CPSetHashParam -> raise code=0x80000100 flags=0x1 nargs=2 p0=slbcsp.dll p1=CPSetHashParam
    (
        cp_set_hash_param,
        0x8000_0100,
        0x01,
        2,
        "slbcsp.dll",
        "CPSetHashParam",
    ),
    // row 03 CPSetKeyParam -> raise code=0x80000100 flags=0x1 nargs=2 p0=slbcsp.dll p1=CPSetKeyParam
    (
        cp_set_key_param,
        0x8000_0100,
        0x01,
        2,
        "slbcsp.dll",
        "CPSetKeyParam",
    ),
    // row 04 CPSetProvParam -> raise code=0x80000100 flags=0x1 nargs=2 p0=slbcsp.dll p1=CPSetProvParam
    (
        cp_set_prov_param,
        0x8000_0100,
        0x01,
        2,
        "slbcsp.dll",
        "CPSetProvParam",
    ),
    // row 05 CPCreateHash -> raise code=0x80000100 flags=0x1 nargs=2 p0=slbcsp.dll p1=CPCreateHash
    (
        cp_create_hash,
        0x8000_0100,
        0x01,
        2,
        "slbcsp.dll",
        "CPCreateHash",
    ),
    // row 06 CPDecrypt -> raise code=0x80000100 flags=0x1 nargs=2 p0=slbcsp.dll p1=CPDecrypt
    (cp_decrypt, 0x8000_0100, 0x01, 2, "slbcsp.dll", "CPDecrypt"),
    // row 07 CPDeriveKey -> raise code=0x80000100 flags=0x1 nargs=2 p0=slbcsp.dll p1=CPDeriveKey
    (
        cp_derive_key,
        0x8000_0100,
        0x01,
        2,
        "slbcsp.dll",
        "CPDeriveKey",
    ),
    // row 08 CPDestroyHash -> raise code=0x80000100 flags=0x1 nargs=2 p0=slbcsp.dll p1=CPDestroyHash
    (
        cp_destroy_hash,
        0x8000_0100,
        0x01,
        2,
        "slbcsp.dll",
        "CPDestroyHash",
    ),
    // row 09 CPDestroyKey -> raise code=0x80000100 flags=0x1 nargs=2 p0=slbcsp.dll p1=CPDestroyKey
    (
        cp_destroy_key,
        0x8000_0100,
        0x01,
        2,
        "slbcsp.dll",
        "CPDestroyKey",
    ),
    // row 10 CPDuplicateHash -> raise code=0x80000100 flags=0x1 nargs=2 p0=slbcsp.dll p1=CPDuplicateHash
    (
        cp_duplicate_hash,
        0x8000_0100,
        0x01,
        2,
        "slbcsp.dll",
        "CPDuplicateHash",
    ),
    // row 11 CPDuplicateKey -> raise code=0x80000100 flags=0x1 nargs=2 p0=slbcsp.dll p1=CPDuplicateKey
    (
        cp_duplicate_key,
        0x8000_0100,
        0x01,
        2,
        "slbcsp.dll",
        "CPDuplicateKey",
    ),
    // row 12 CPEncrypt -> raise code=0x80000100 flags=0x1 nargs=2 p0=slbcsp.dll p1=CPEncrypt
    (cp_encrypt, 0x8000_0100, 0x01, 2, "slbcsp.dll", "CPEncrypt"),
    // row 13 CPExportKey -> raise code=0x80000100 flags=0x1 nargs=2 p0=slbcsp.dll p1=CPExportKey
    (
        cp_export_key,
        0x8000_0100,
        0x01,
        2,
        "slbcsp.dll",
        "CPExportKey",
    ),
    // row 14 CPGenKey -> raise code=0x80000100 flags=0x1 nargs=2 p0=slbcsp.dll p1=CPGenKey
    (cp_gen_key, 0x8000_0100, 0x01, 2, "slbcsp.dll", "CPGenKey"),
    // row 15 CPGenRandom -> raise code=0x80000100 flags=0x1 nargs=2 p0=slbcsp.dll p1=CPGenRandom
    (
        cp_gen_random,
        0x8000_0100,
        0x01,
        2,
        "slbcsp.dll",
        "CPGenRandom",
    ),
    // row 16 CPGetHashParam -> raise code=0x80000100 flags=0x1 nargs=2 p0=slbcsp.dll p1=CPGetHashParam
    (
        cp_get_hash_param,
        0x8000_0100,
        0x01,
        2,
        "slbcsp.dll",
        "CPGetHashParam",
    ),
    // row 17 CPGetKeyParam -> raise code=0x80000100 flags=0x1 nargs=2 p0=slbcsp.dll p1=CPGetKeyParam
    (
        cp_get_key_param,
        0x8000_0100,
        0x01,
        2,
        "slbcsp.dll",
        "CPGetKeyParam",
    ),
    // row 18 CPGetProvParam -> raise code=0x80000100 flags=0x1 nargs=2 p0=slbcsp.dll p1=CPGetProvParam
    (
        cp_get_prov_param,
        0x8000_0100,
        0x01,
        2,
        "slbcsp.dll",
        "CPGetProvParam",
    ),
    // row 19 CPGetUserKey -> raise code=0x80000100 flags=0x1 nargs=2 p0=slbcsp.dll p1=CPGetUserKey
    (
        cp_get_user_key,
        0x8000_0100,
        0x01,
        2,
        "slbcsp.dll",
        "CPGetUserKey",
    ),
    // row 20 CPHashData -> raise code=0x80000100 flags=0x1 nargs=2 p0=slbcsp.dll p1=CPHashData
    (
        cp_hash_data,
        0x8000_0100,
        0x01,
        2,
        "slbcsp.dll",
        "CPHashData",
    ),
    // row 21 CPHashSessionKey -> raise code=0x80000100 flags=0x1 nargs=2 p0=slbcsp.dll p1=CPHashSessionKey
    (
        cp_hash_session_key,
        0x8000_0100,
        0x01,
        2,
        "slbcsp.dll",
        "CPHashSessionKey",
    ),
    // row 22 CPImportKey -> raise code=0x80000100 flags=0x1 nargs=2 p0=slbcsp.dll p1=CPImportKey
    (
        cp_import_key,
        0x8000_0100,
        0x01,
        2,
        "slbcsp.dll",
        "CPImportKey",
    ),
    // row 23 CPReleaseContext -> raise code=0x80000100 flags=0x1 nargs=2 p0=slbcsp.dll p1=CPReleaseContext
    (
        cp_release_context,
        0x8000_0100,
        0x01,
        2,
        "slbcsp.dll",
        "CPReleaseContext",
    ),
    // row 24 CPSignHash -> raise code=0x80000100 flags=0x1 nargs=2 p0=slbcsp.dll p1=CPSignHash
    (
        cp_sign_hash,
        0x8000_0100,
        0x01,
        2,
        "slbcsp.dll",
        "CPSignHash",
    ),
    // row 25 CPVerifySignature -> raise code=0x80000100 flags=0x1 nargs=2 p0=slbcsp.dll p1=CPVerifySignature
    (
        cp_verify_signature,
        0x8000_0100,
        0x01,
        2,
        "slbcsp.dll",
        "CPVerifySignature",
    ),
];

#[test]
fn oracle_stub_rows() {
    for (helper, code, flags, number_parameters, module, function) in ORACLE_ROWS {
        assert_eq!(
            helper(),
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
