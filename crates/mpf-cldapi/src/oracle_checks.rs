//! Oracle-derived test vectors for the ported cldapi exports.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies from
//! `dlls/cldapi/main.c` and `libs/winecrt0/stub.c` with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/cldapi/` and running
//! the harness. It printed 64 rows: C01 to C10 and P01 to P05 for the two stdcall
//! exports, and S01 to S49 for the 49 `@ stub` exports. ASan and UBSan flagged none
//! of them, so none are excluded.
//!
//! The C bodies pass `h`, `info` and `len` only to the `FIXME` log, so the Rust
//! signatures omit them. The harness varies them per row, and they make no difference.

use super::*;

/// (id, buf fill byte, retlen before, return value, retlen after).
type PlaceholderRow = (&'static str, Option<u8>, Option<u32>, u32, Option<u32>);

#[test]
fn oracle_cf_get_placeholder_info_rows() {
    // A present buf is 64 bytes. Rows C01 to C10 of target/canon/cldapi/out.txt.
    // Every row leaves buf unchanged.
    let rows: [PlaceholderRow; 10] = [
        ("C01", None, None, 0x8000_4001, None),
        ("C02", Some(0xaa), Some(7), 0x8000_4001, Some(7)),
        ("C03", Some(0x00), None, 0x8000_4001, None),
        ("C04", None, None, 0x8000_4001, None),
        ("C05", None, Some(0), 0x8000_4001, Some(0)),
        (
            "C06",
            Some(0xff),
            Some(0xffff_ffff),
            0x8000_4001,
            Some(0xffff_ffff),
        ),
        ("C07", Some(0x00), None, 0x8000_4001, None),
        ("C08", Some(0x55), Some(7), 0x8000_4001, Some(7)),
        (
            "C09",
            None,
            Some(0xdead_beef),
            0x8000_4001,
            Some(0xdead_beef),
        ),
        ("C10", None, None, 0x8000_4001, None),
    ];
    for (id, fill, retlen_before, ret, retlen_after) in rows {
        let mut buf = fill.map(|byte| vec![byte; 64]);
        let before = buf.clone();
        let mut retlen = retlen_before;
        let hr = cf_get_placeholder_info(buf.as_deref_mut(), retlen.as_mut());
        assert_eq!(hr as u32, ret, "{id}");
        assert_eq!(buf, before, "{id}");
        assert_eq!(retlen, retlen_after, "{id}");
    }
}

#[test]
fn oracle_cf_get_platform_info_rows() {
    // (id, info before, return value). Rows P01 to P05 of target/canon/cldapi/out.txt.
    // Every row leaves *info unchanged.
    let rows: [(&str, Option<[u32; 3]>, u32); 5] = [
        ("P01", None, 0x8000_4001),
        ("P02", Some([0, 0, 0]), 0x8000_4001),
        ("P03", Some([u32::MAX; 3]), 0x8000_4001),
        ("P04", Some([1, 2, 3]), 0x8000_4001),
        ("P05", Some([0x1234_5678, 0, 0xdead_beef]), 0x8000_4001),
    ];
    for (id, before, ret) in rows {
        let mut info = before.map(|[build, revision, integration]| CfPlatformInfo {
            build_number: build,
            revision_number: revision,
            integration_number: integration,
        });
        let hr = cf_get_platform_info(info.as_mut());
        assert_eq!(hr as u32, ret, "{id}");
        let after = info.map(|v| [v.build_number, v.revision_number, v.integration_number]);
        assert_eq!(after, before, "{id}");
    }
}

#[test]
fn oracle_wine_spec_unimplemented_stub_rows() {
    // (id, export name). Rows S01 to S49 of target/canon/cldapi/out.txt. The C stub never
    // returns, so each row checks the one RaiseException it makes: code 0x80000100, flags 0x1,
    // and two arguments: module "cldapi.dll", then the export name.
    let rows: [(&str, &str); 49] = [
        ("S01", "CfAbortOperation"),
        ("S02", "CfCloseAppPolicy"),
        ("S03", "CfCloseHandle"),
        ("S04", "CfConnectSyncRoot"),
        ("S05", "CfConvertToPlaceholder"),
        ("S06", "CfCreatePlaceholders"),
        ("S07", "CfDehydratePlaceholder"),
        ("S08", "CfDehydratePlaceholderEx"),
        ("S09", "CfDisconnectSyncRoot"),
        ("S10", "CfEnumAppPolicy"),
        ("S11", "CfExecute"),
        ("S12", "CfGetCorrelationVector"),
        ("S13", "CfGetLastSyncStatus"),
        ("S14", "CfGetPlaceholderRangeInfo"),
        ("S15", "CfGetPlaceholderRangeInfoForHydration"),
        ("S16", "CfGetPlaceholderStateFromAttributeTag"),
        ("S17", "CfGetPlaceholderStateFromFileInfo"),
        ("S18", "CfGetPlaceholderStateFromFindData"),
        ("S19", "CfGetSyncRootInfoByHandle"),
        ("S20", "CfGetSyncRootInfoByPath"),
        ("S21", "CfGetTransferKey"),
        ("S22", "CfGetWin32HandleFromProtectedHandle"),
        ("S23", "CfHydratePlaceholder"),
        ("S24", "CfHydratePlaceholderWithAttributionPrivate"),
        ("S25", "CfLockProperties"),
        ("S26", "CfOpenAppPolicy"),
        ("S27", "CfOpenFileWithOplock"),
        ("S28", "CfOpenProgressEvent"),
        ("S29", "CfQueryProgress"),
        ("S30", "CfQuerySyncProviderStatus"),
        ("S31", "CfReferenceProtectedHandle"),
        ("S32", "CfRegisterSyncRoot"),
        ("S33", "CfReleaseProtectedHandle"),
        ("S34", "CfReleaseTransferKey"),
        ("S35", "CfReportProviderProgress"),
        ("S36", "CfReportProviderProgress2"),
        ("S37", "CfReportSyncStatus"),
        ("S38", "CfRetrieveProperties"),
        ("S39", "CfRevertPlaceholder"),
        ("S40", "CfSetAppPolicy"),
        ("S41", "CfSetCorrelationVector"),
        ("S42", "CfSetInSyncState"),
        ("S43", "CfSetPinState"),
        ("S44", "CfStoreProperties"),
        ("S45", "CfTestToggleSyncMsgThreadWaitEvent"),
        ("S46", "CfUnlockProperties"),
        ("S47", "CfUnregisterSyncRoot"),
        ("S48", "CfUpdatePlaceholder"),
        ("S49", "CfUpdateSyncProviderStatus"),
    ];
    for (i, (id, export)) in rows.iter().enumerate() {
        assert_eq!(STUB_EXPORTS[i], *export, "{id}");
        let raise = wine_spec_unimplemented_stub(MODULE_NAME, export);
        assert_eq!(raise.code, EXCEPTION_WINE_STUB, "{id}");
        assert_eq!(raise.flags, EXCEPTION_NONCONTINUABLE, "{id}");
        assert_eq!(raise.module, "cldapi.dll", "{id}");
        assert_eq!(raise.function, *export, "{id}");
    }
}
