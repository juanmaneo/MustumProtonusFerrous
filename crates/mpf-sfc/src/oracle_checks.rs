//! Oracle-derived test vectors for the `dlls/sfc/sfc.spec` export table.
//!
//! Vectors were printed by running winebuild's own parser on `sfc.spec`. The harness
//! `target/canon/sfc/tools/winebuild/sfc_oracle.c` links the verbatim `parser.c`, `utils.c`,
//! `main.c` and the other winebuild sources, built with `gcc -fsanitize=address,undefined -g`.
//! It prints 16 entry rows and 7 name-hint rows. ASan and UBSan reported nothing, so no row is
//! excluded.

use super::*;

/// The 16 entry rows, in spec order. Every row has `forward=1`.
const ORACLE_EXPORTS: [Export; 16] = [
    // row 01 line=1 ordinal=1 name=- link_name=sfc_os.SfcInitProt
    Export {
        line: 1,
        ordinal: 1,
        name: None,
        link_name: "sfc_os.SfcInitProt",
    },
    // row 02 line=2 ordinal=2 name=- link_name=sfc_os.SfcTerminateWatcherThread
    Export {
        line: 2,
        ordinal: 2,
        name: None,
        link_name: "sfc_os.SfcTerminateWatcherThread",
    },
    // row 03 line=3 ordinal=3 name=- link_name=sfc_os.SfcConnectToServer
    Export {
        line: 3,
        ordinal: 3,
        name: None,
        link_name: "sfc_os.SfcConnectToServer",
    },
    // row 04 line=4 ordinal=4 name=- link_name=sfc_os.SfcClose
    Export {
        line: 4,
        ordinal: 4,
        name: None,
        link_name: "sfc_os.SfcClose",
    },
    // row 05 line=5 ordinal=5 name=- link_name=sfc_os.SfcFileException
    Export {
        line: 5,
        ordinal: 5,
        name: None,
        link_name: "sfc_os.SfcFileException",
    },
    // row 06 line=6 ordinal=6 name=- link_name=sfc_os.SfcInitiateScan
    Export {
        line: 6,
        ordinal: 6,
        name: None,
        link_name: "sfc_os.SfcInitiateScan",
    },
    // row 07 line=7 ordinal=7 name=- link_name=sfc_os.SfcInstallProtectedFiles
    Export {
        line: 7,
        ordinal: 7,
        name: None,
        link_name: "sfc_os.SfcInstallProtectedFiles",
    },
    // row 08 line=8 ordinal=8 name=- link_name=sfc_os.SfpInstallCatalog
    Export {
        line: 8,
        ordinal: 8,
        name: None,
        link_name: "sfc_os.SfpInstallCatalog",
    },
    // row 09 line=9 ordinal=9 name=- link_name=sfc_os.SfpDeleteCatalog
    Export {
        line: 9,
        ordinal: 9,
        name: None,
        link_name: "sfc_os.SfpDeleteCatalog",
    },
    // row 10 line=10 ordinal=10 name=SRSetRestorePoint link_name=sfc_os.SRSetRestorePointA
    Export {
        line: 10,
        ordinal: 10,
        name: Some("SRSetRestorePoint"),
        link_name: "sfc_os.SRSetRestorePointA",
    },
    // row 11 line=11 ordinal=11 name=SRSetRestorePointA link_name=sfc_os.SRSetRestorePointA
    Export {
        line: 11,
        ordinal: 11,
        name: Some("SRSetRestorePointA"),
        link_name: "sfc_os.SRSetRestorePointA",
    },
    // row 12 line=12 ordinal=12 name=SRSetRestorePointW link_name=sfc_os.SRSetRestorePointW
    Export {
        line: 12,
        ordinal: 12,
        name: Some("SRSetRestorePointW"),
        link_name: "sfc_os.SRSetRestorePointW",
    },
    // row 13 line=13 ordinal=13 name=SfcGetNextProtectedFile link_name=sfc_os.SfcGetNextProtectedFile
    Export {
        line: 13,
        ordinal: 13,
        name: Some("SfcGetNextProtectedFile"),
        link_name: "sfc_os.SfcGetNextProtectedFile",
    },
    // row 14 line=14 ordinal=14 name=SfcIsFileProtected link_name=sfc_os.SfcIsFileProtected
    Export {
        line: 14,
        ordinal: 14,
        name: Some("SfcIsFileProtected"),
        link_name: "sfc_os.SfcIsFileProtected",
    },
    // row 15 line=15 ordinal=15 name=SfcIsKeyProtected link_name=sfc_os.SfcIsKeyProtected
    Export {
        line: 15,
        ordinal: 15,
        name: Some("SfcIsKeyProtected"),
        link_name: "sfc_os.SfcIsKeyProtected",
    },
    // row 16 line=16 ordinal=16 name=SfpVerifyFile link_name=sfc_os.SfpVerifyFile
    Export {
        line: 16,
        ordinal: 16,
        name: Some("SfpVerifyFile"),
        link_name: "sfc_os.SfpVerifyFile",
    },
];

/// The seven name-hint rows as (name, ordinal), in hint order.
const ORACLE_NAME_TABLE: [(&str, u16); 7] = [
    // hint 00
    ("SRSetRestorePoint", 10),
    // hint 01
    ("SRSetRestorePointA", 11),
    // hint 02
    ("SRSetRestorePointW", 12),
    // hint 03
    ("SfcGetNextProtectedFile", 13),
    // hint 04
    ("SfcIsFileProtected", 14),
    // hint 05
    ("SfcIsKeyProtected", 15),
    // hint 06
    ("SfpVerifyFile", 16),
];

#[test]
fn oracle_export_rows() {
    // Rows 01-16: the entries in spec order, every one with forward=1.
    assert_eq!(EXPORTS, ORACLE_EXPORTS);
    for export in EXPORTS {
        assert!(export.forward_target().is_some());
    }
}

#[test]
fn oracle_name_table_rows() {
    // Hint rows 00-06: the name table sorted by name, with each hint equal to its index.
    let table = name_table();
    assert_eq!(table.len(), ORACLE_NAME_TABLE.len());
    for (hint, (name, ordinal)) in ORACLE_NAME_TABLE.iter().enumerate() {
        assert_eq!(table[hint].name, Some(*name));
        assert_eq!(table[hint].ordinal, *ordinal);
    }
}
