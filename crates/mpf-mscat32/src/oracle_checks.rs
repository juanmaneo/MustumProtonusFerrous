//! Oracle-derived test vectors for the ported mscat32 logic.
//!
//! `dlls/mscat32` has no C sources, so the export rows come from the reference `winebuild` in
//! `tools/winebuild` (Wine 11.22), run with `--def` on `dlls/mscat32/mscat32.spec`. The
//! `EXPORT_ROWS` vector holds its 35 `EXPORTS` lines. The `STUB_ROWS` vector comes from the
//! verbatim `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`) compiled with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/mscat32/stub/`. The module and
//! function strings match the `winebuild --dll` assembly of the same spec. ASan and UBSan
//! reported nothing, so no row was excluded.

use super::*;

/// The `winebuild --def` rows: name, ordinal, forward target (`None` for a stub), and `PRIVATE`.
const EXPORT_ROWS: [(&str, u16, Option<&str>, bool); 35] = [
    ("CryptCATVerifyMember", 1, None, true),
    (
        "CryptCATAdminAcquireContext",
        2,
        Some("wintrust.CryptCATAdminAcquireContext"),
        false,
    ),
    (
        "CryptCATAdminAddCatalog",
        3,
        Some("wintrust.CryptCATAdminAddCatalog"),
        false,
    ),
    (
        "CryptCATAdminCalcHashFromFileHandle",
        4,
        Some("wintrust.CryptCATAdminCalcHashFromFileHandle"),
        false,
    ),
    (
        "CryptCATAdminEnumCatalogFromHash",
        5,
        Some("wintrust.CryptCATAdminEnumCatalogFromHash"),
        false,
    ),
    (
        "CryptCATAdminReleaseCatalogContext",
        6,
        Some("wintrust.CryptCATAdminReleaseCatalogContext"),
        false,
    ),
    (
        "CryptCATAdminReleaseContext",
        7,
        Some("wintrust.CryptCATAdminReleaseContext"),
        false,
    ),
    (
        "CryptCATCDFClose",
        8,
        Some("wintrust.CryptCATCDFClose"),
        false,
    ),
    ("CryptCATCDFEnumAttributes", 9, None, true),
    (
        "CryptCATCDFEnumAttributesWithCDFTag",
        10,
        Some("wintrust.CryptCATCDFEnumAttributesWithCDFTag"),
        false,
    ),
    (
        "CryptCATCDFEnumCatAttributes",
        11,
        Some("wintrust.CryptCATCDFEnumCatAttributes"),
        false,
    ),
    ("CryptCATCDFEnumMembers", 12, None, true),
    (
        "CryptCATCDFEnumMembersByCDFTag",
        13,
        Some("wintrust.CryptCATCDFEnumMembersByCDFTag"),
        false,
    ),
    (
        "CryptCATCDFEnumMembersByCDFTagEx",
        14,
        Some("wintrust.CryptCATCDFEnumMembersByCDFTagEx"),
        false,
    ),
    (
        "CryptCATCDFOpen",
        15,
        Some("wintrust.CryptCATCDFOpen"),
        false,
    ),
    (
        "CryptCATCatalogInfoFromContext",
        16,
        Some("wintrust.CryptCATCatalogInfoFromContext"),
        false,
    ),
    ("CryptCATClose", 17, Some("wintrust.CryptCATClose"), false),
    (
        "CryptCATEnumerateAttr",
        18,
        Some("wintrust.CryptCATEnumerateAttr"),
        false,
    ),
    (
        "CryptCATEnumerateCatAttr",
        19,
        Some("wintrust.CryptCATEnumerateCatAttr"),
        false,
    ),
    (
        "CryptCATEnumerateMember",
        20,
        Some("wintrust.CryptCATEnumerateMember"),
        false,
    ),
    (
        "CryptCATGetAttrInfo",
        21,
        Some("wintrust.CryptCATGetAttrInfo"),
        false,
    ),
    (
        "CryptCATGetCatAttrInfo",
        22,
        Some("wintrust.CryptCATGetCatAttrInfo"),
        false,
    ),
    (
        "CryptCATGetMemberInfo",
        23,
        Some("wintrust.CryptCATGetMemberInfo"),
        false,
    ),
    (
        "CryptCATHandleFromStore",
        24,
        Some("wintrust.CryptCATHandleFromStore"),
        false,
    ),
    ("CryptCATOpen", 25, Some("wintrust.CryptCATOpen"), false),
    (
        "CryptCATPersistStore",
        26,
        Some("wintrust.CryptCATPersistStore"),
        false,
    ),
    (
        "CryptCATPutAttrInfo",
        27,
        Some("wintrust.CryptCATPutAttrInfo"),
        false,
    ),
    (
        "CryptCATPutCatAttrInfo",
        28,
        Some("wintrust.CryptCATPutCatAttrInfo"),
        false,
    ),
    (
        "CryptCATPutMemberInfo",
        29,
        Some("wintrust.CryptCATPutMemberInfo"),
        false,
    ),
    (
        "CryptCATStoreFromHandle",
        30,
        Some("wintrust.CryptCATStoreFromHandle"),
        false,
    ),
    (
        "DllRegisterServer",
        31,
        Some("wintrust.mscat32DllRegisterServer"),
        true,
    ),
    (
        "DllUnregisterServer",
        32,
        Some("wintrust.mscat32DllUnregisterServer"),
        true,
    ),
    ("IsCatalogFile", 33, Some("wintrust.IsCatalogFile"), false),
    ("MsCatConstructHashTag", 34, None, true),
    ("MsCatFreeHashTag", 35, None, true),
];

/// The stub oracle rows, in spec order. Each is the record `__wine_spec_unimplemented_stub` raises.
const STUB_ROWS: [StubException; 5] = [
    StubException {
        code: 0x8000_0100,
        flags: 0x1,
        number_parameters: 2,
        module: "mscat32.dll",
        function: "CryptCATVerifyMember",
    },
    StubException {
        code: 0x8000_0100,
        flags: 0x1,
        number_parameters: 2,
        module: "mscat32.dll",
        function: "CryptCATCDFEnumAttributes",
    },
    StubException {
        code: 0x8000_0100,
        flags: 0x1,
        number_parameters: 2,
        module: "mscat32.dll",
        function: "CryptCATCDFEnumMembers",
    },
    StubException {
        code: 0x8000_0100,
        flags: 0x1,
        number_parameters: 2,
        module: "mscat32.dll",
        function: "MsCatConstructHashTag",
    },
    StubException {
        code: 0x8000_0100,
        flags: 0x1,
        number_parameters: 2,
        module: "mscat32.dll",
        function: "MsCatFreeHashTag",
    },
];

#[test]
fn oracle_export_rows() {
    for (index, &(name, ordinal, target, private)) in EXPORT_ROWS.iter().enumerate() {
        assert_eq!(EXPORTS[index].name, name);
        let row = export(name).expect("every oracle row is exported");
        assert_eq!(row.ordinal, ordinal);
        assert_eq!(row.private, private);
        match target {
            Some(target) => assert_eq!(row.kind, ExportKind::Forwarded(target)),
            None => assert_eq!(row.kind, ExportKind::Stub),
        }
        assert_eq!(row, EXPORTS[index]);
    }
}

#[test]
fn oracle_stub_rows() {
    let helpers = [
        crypt_cat_verify_member(),
        crypt_cat_cdf_enum_attributes(),
        crypt_cat_cdf_enum_members(),
        ms_cat_construct_hash_tag(),
        ms_cat_free_hash_tag(),
    ];
    for (row, helper) in STUB_ROWS.iter().zip(helpers) {
        assert_eq!(helper, *row);
        assert_eq!(stub_for_export(row.function), Some(*row));
        assert_eq!(export(row.function).map(|e| e.kind), Some(ExportKind::Stub));
    }
}
