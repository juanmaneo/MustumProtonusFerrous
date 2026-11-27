//! Oracle-derived test vectors for the `security.spec` export table.
//!
//! The rows were captured by compiling the verbatim winebuild bodies of `parse_spec_export`,
//! `parse_spec_arguments`, `GetToken` and `assign_ordinals` (`tools/winebuild/parser.c`), with the
//! structs from `tools/winebuild/build.h`, using `gcc -fsanitize=address,undefined -g` under
//! `target/canon/security/`. The harness ran on `dlls/security/security.spec` and printed 36 rows,
//! one per `@ stdcall` line. No row is excluded: ASan and UBSan reported nothing.

use super::*;

/// `(name, ordinal, nb_args, forward)` per row, in spec order.
const ORACLE_ROWS: [(&str, u16, u8, &str); 36] = [
    (
        "AcceptSecurityContext",
        1,
        9,
        "secur32.AcceptSecurityContext",
    ), // row 01
    (
        "AcquireCredentialsHandleA",
        2,
        9,
        "secur32.AcquireCredentialsHandleA",
    ), // row 02
    (
        "AcquireCredentialsHandleW",
        3,
        9,
        "secur32.AcquireCredentialsHandleW",
    ), // row 03
    ("AddSecurityPackageA", 4, 2, "secur32.AddSecurityPackageA"), // row 04
    ("AddSecurityPackageW", 5, 2, "secur32.AddSecurityPackageW"), // row 05
    ("ApplyControlToken", 6, 2, "secur32.ApplyControlToken"),     // row 06
    ("CompleteAuthToken", 7, 2, "secur32.CompleteAuthToken"),     // row 07
    ("DecryptMessage", 8, 4, "secur32.DecryptMessage"),           // row 08
    (
        "DeleteSecurityContext",
        9,
        1,
        "secur32.DeleteSecurityContext",
    ), // row 09
    (
        "DeleteSecurityPackageA",
        10,
        1,
        "secur32.DeleteSecurityPackageA",
    ), // row 10
    (
        "DeleteSecurityPackageW",
        11,
        1,
        "secur32.DeleteSecurityPackageW",
    ), // row 11
    ("EncryptMessage", 12, 4, "secur32.EncryptMessage"),          // row 12
    (
        "EnumerateSecurityPackagesA",
        13,
        2,
        "secur32.EnumerateSecurityPackagesA",
    ), // row 13
    (
        "EnumerateSecurityPackagesW",
        14,
        2,
        "secur32.EnumerateSecurityPackagesW",
    ), // row 14
    (
        "ExportSecurityContext",
        15,
        4,
        "secur32.ExportSecurityContext",
    ), // row 15
    ("FreeContextBuffer", 16, 1, "secur32.FreeContextBuffer"),    // row 16
    (
        "FreeCredentialsHandle",
        17,
        1,
        "secur32.FreeCredentialsHandle",
    ), // row 17
    (
        "ImpersonateSecurityContext",
        18,
        1,
        "secur32.ImpersonateSecurityContext",
    ), // row 18
    (
        "ImportSecurityContextA",
        19,
        4,
        "secur32.ImportSecurityContextA",
    ), // row 19
    (
        "ImportSecurityContextW",
        20,
        4,
        "secur32.ImportSecurityContextW",
    ), // row 20
    (
        "InitSecurityInterfaceA",
        21,
        0,
        "secur32.InitSecurityInterfaceA",
    ), // row 21
    (
        "InitSecurityInterfaceW",
        22,
        0,
        "secur32.InitSecurityInterfaceW",
    ), // row 22
    (
        "InitializeSecurityContextA",
        23,
        12,
        "secur32.InitializeSecurityContextA",
    ), // row 23
    (
        "InitializeSecurityContextW",
        24,
        12,
        "secur32.InitializeSecurityContextW",
    ), // row 24
    ("MakeSignature", 25, 4, "secur32.MakeSignature"),            // row 25
    (
        "QueryContextAttributesA",
        26,
        3,
        "secur32.QueryContextAttributesA",
    ), // row 26
    (
        "QueryContextAttributesW",
        27,
        3,
        "secur32.QueryContextAttributesW",
    ), // row 27
    (
        "QueryCredentialsAttributesA",
        28,
        3,
        "secur32.QueryCredentialsAttributesA",
    ), // row 28
    (
        "QueryCredentialsAttributesW",
        29,
        3,
        "secur32.QueryCredentialsAttributesW",
    ), // row 29
    (
        "QuerySecurityContextToken",
        30,
        2,
        "secur32.QuerySecurityContextToken",
    ), // row 30
    (
        "QuerySecurityPackageInfoA",
        31,
        2,
        "secur32.QuerySecurityPackageInfoA",
    ), // row 31
    (
        "QuerySecurityPackageInfoW",
        32,
        2,
        "secur32.QuerySecurityPackageInfoW",
    ), // row 32
    (
        "RevertSecurityContext",
        33,
        1,
        "secur32.RevertSecurityContext",
    ), // row 33
    ("SealMessage", 34, 4, "secur32.SealMessage"),                // row 34
    ("UnsealMessage", 35, 4, "secur32.UnsealMessage"),            // row 35
    ("VerifySignature", 36, 4, "secur32.VerifySignature"),        // row 36
];

#[test]
fn oracle_export_rows() {
    assert_eq!(ORACLE_ROWS.len(), EXPORTS.len());
    for (index, &(name, ordinal, nb_args, forward)) in ORACLE_ROWS.iter().enumerate() {
        let export = &EXPORTS[index];
        assert_eq!(export.name, name);
        assert_eq!(export.ordinal, ordinal);
        assert_eq!(export.nb_args, nb_args);
        assert_eq!(export.forward, forward);
        assert_eq!(export_by_name(name), Some(export));
        assert_eq!(export_by_ordinal(ordinal), Some(export));
        assert_eq!(split_forward(forward), Some((IMPORTS, name)));
    }
}
