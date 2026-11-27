//! Oracle-derived test vectors for the ported softpub export table and stub records.
//!
//! The 24 export rows are the `winebuild --def` output for `dlls/softpub/softpub.spec`, saved as
//! `target/canon/softpub/softpub_export.def` (winebuild 11.19; the reference tree is 11.22). The
//! 3 stub records are printed by `target/canon/softpub/stub_harness.c`, which copies
//! `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c` and was built with
//! `gcc -fsanitize=address,undefined -g`. ASan and UBSan reported nothing, so no vector is excluded.

use super::*;

/// One row of the `winebuild --def` output: ordinal, name, `PRIVATE`, and the forward target, or
/// `None` for a `stub` line.
type DefRow = (
    u16,
    &'static str,
    bool,
    Option<(&'static str, &'static str)>,
);

const DEF_ROWS: [DefRow; 24] = [
    (
        1,
        "GenericChainCertificateTrust",
        false,
        Some(("wintrust", "GenericChainCertificateTrust")),
    ),
    (
        2,
        "GenericChainFinalProv",
        false,
        Some(("wintrust", "GenericChainFinalProv")),
    ),
    (
        3,
        "HTTPSCertificateTrust",
        false,
        Some(("wintrust", "HTTPSCertificateTrust")),
    ),
    (
        4,
        "SoftpubDefCertInit",
        false,
        Some(("wintrust", "SoftpubDefCertInit")),
    ),
    (5, "SoftpubFreeDefUsageCallData", true, None),
    (6, "SoftpubLoadDefUsageCallData", true, None),
    (7, "AddPersonalTrustDBPages", true, None),
    (
        8,
        "DllRegisterServer",
        true,
        Some(("wintrust", "SoftpubDllRegisterServer")),
    ),
    (
        9,
        "DllUnregisterServer",
        true,
        Some(("wintrust", "SoftpubDllUnregisterServer")),
    ),
    (
        10,
        "DriverCleanupPolicy",
        false,
        Some(("wintrust", "DriverCleanupPolicy")),
    ),
    (
        11,
        "DriverFinalPolicy",
        false,
        Some(("wintrust", "DriverFinalPolicy")),
    ),
    (
        12,
        "DriverInitializePolicy",
        false,
        Some(("wintrust", "DriverInitializePolicy")),
    ),
    (
        13,
        "FindCertsByIssuer",
        false,
        Some(("wintrust", "FindCertsByIssuer")),
    ),
    (
        14,
        "HTTPSFinalProv",
        false,
        Some(("wintrust", "HTTPSFinalProv")),
    ),
    (
        15,
        "OfficeCleanupPolicy",
        false,
        Some(("wintrust", "OfficeCleanupPolicy")),
    ),
    (
        16,
        "OfficeInitializePolicy",
        false,
        Some(("wintrust", "OfficeInitializePolicy")),
    ),
    (
        17,
        "OpenPersonalTrustDBDialog",
        false,
        Some(("wintrust", "OpenPersonalTrustDBDialog")),
    ),
    (
        18,
        "SoftpubAuthenticode",
        false,
        Some(("wintrust", "SoftpubAuthenticode")),
    ),
    (
        19,
        "SoftpubCheckCert",
        false,
        Some(("wintrust", "SoftpubCheckCert")),
    ),
    (
        20,
        "SoftpubCleanup",
        false,
        Some(("wintrust", "SoftpubCleanup")),
    ),
    (
        21,
        "SoftpubDumpStructure",
        false,
        Some(("wintrust", "SoftpubDumpStructure")),
    ),
    (
        22,
        "SoftpubInitialize",
        false,
        Some(("wintrust", "SoftpubInitialize")),
    ),
    (
        23,
        "SoftpubLoadMessage",
        false,
        Some(("wintrust", "SoftpubLoadMessage")),
    ),
    (
        24,
        "SoftpubLoadSignature",
        false,
        Some(("wintrust", "SoftpubLoadSignature")),
    ),
];

#[test]
fn oracle_export_table_rows() {
    for (index, &(ordinal, name, private, forward)) in DEF_ROWS.iter().enumerate() {
        let row = index + 1;
        let kind = match forward {
            Some((module, function)) => ExportKind::Forward { module, function },
            None => ExportKind::Stub,
        };
        assert_eq!(
            find_export(name),
            Some(Export {
                ordinal,
                name,
                private,
                kind,
            }),
            "row {row:02} {name}"
        );
        assert_eq!(forward_target(name), forward, "row {row:02} {name}");
    }
}

#[test]
fn oracle_stub_rows() {
    // stub 05 code=0x80000100 flags=0x1 nparams=2 module=softpub.dll function=SoftpubFreeDefUsageCallData raises=1
    assert_eq!(
        softpub_free_def_usage_call_data(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "softpub.dll",
            function: "SoftpubFreeDefUsageCallData",
        }
    );

    // stub 06 code=0x80000100 flags=0x1 nparams=2 module=softpub.dll function=SoftpubLoadDefUsageCallData raises=1
    assert_eq!(
        softpub_load_def_usage_call_data(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "softpub.dll",
            function: "SoftpubLoadDefUsageCallData",
        }
    );

    // stub 07 code=0x80000100 flags=0x1 nparams=2 module=softpub.dll function=AddPersonalTrustDBPages raises=1
    assert_eq!(
        add_personal_trust_db_pages(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "softpub.dll",
            function: "AddPersonalTrustDBPages",
        }
    );
}
