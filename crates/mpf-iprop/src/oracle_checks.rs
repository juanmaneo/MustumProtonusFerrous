//! Oracle-derived test vectors for the ported iprop tables.
//!
//! Vectors were captured under `target/canon/iprop/` by compiling the verbatim `get_args_size`
//! body (`tools/winebuild/utils.c`), the `ArgNames` table (`tools/winebuild/parser.c`) and the
//! `arg_type` enum (`tools/winebuild/build.h`) with `gcc -fsanitize=address,undefined -g`. A reader
//! for `dlls/iprop/iprop.spec`, shaped after winebuild's parser, supplies the eight exports. The
//! harness printed 15 rows: 8 exports and 7 argument sweeps. ASan and UBSan flagged none of them,
//! so none is excluded.

use super::*;

/// `(export, argument types as written in iprop.spec, stack bytes, forward module, forward name)`.
const EXPORT_ROWS: [(&str, &str, u32, &str, &str); 8] = [
    (
        "FmtIdToPropStgName",
        "ptr wstr",
        8,
        "ole32",
        "FmtIdToPropStgName",
    ),
    (
        "FreePropVariantArray",
        "long ptr",
        8,
        "ole32",
        "FreePropVariantArray",
    ),
    (
        "PropStgNameToFmtId",
        "wstr ptr",
        8,
        "ole32",
        "PropStgNameToFmtId",
    ),
    ("PropVariantClear", "ptr", 4, "ole32", "PropVariantClear"),
    ("PropVariantCopy", "ptr ptr", 8, "ole32", "PropVariantCopy"),
    (
        "StgCreatePropSetStg",
        "ptr long ptr",
        12,
        "ole32",
        "StgCreatePropSetStg",
    ),
    (
        "StgCreatePropStg",
        "ptr ptr ptr long long ptr",
        24,
        "ole32",
        "StgCreatePropStg",
    ),
    (
        "StgOpenPropStg",
        "ptr ptr long long ptr",
        20,
        "ole32",
        "StgOpenPropStg",
    ),
];

/// `(argument types, stack bytes)` for argument lists that the sweep feeds to `get_args_size`.
const SWEEP_ROWS: [(&str, &[ArgKind], u32); 7] = [
    ("", &[], 0),
    ("long", &[ArgKind::Long], 4),
    ("ptr wstr", &[ArgKind::Ptr, ArgKind::Wstr], 8),
    (
        "long long ptr",
        &[ArgKind::Long, ArgKind::Long, ArgKind::Ptr],
        12,
    ),
    (
        "ptr ptr ptr long long ptr",
        &[
            ArgKind::Ptr,
            ArgKind::Ptr,
            ArgKind::Ptr,
            ArgKind::Long,
            ArgKind::Long,
            ArgKind::Ptr,
        ],
        24,
    ),
    (
        "long long long long long long long long",
        &[ArgKind::Long; 8],
        32,
    ),
    (
        "wstr wstr wstr wstr wstr wstr wstr wstr wstr wstr wstr wstr wstr wstr wstr wstr",
        &[ArgKind::Wstr; 16],
        64,
    ),
];

/// The argument types of `args`, spelled the way `iprop.spec` writes them.
fn spell(args: &[ArgKind]) -> String {
    args.iter()
        .map(|arg| match arg {
            ArgKind::Long => "long",
            ArgKind::Ptr => "ptr",
            ArgKind::Wstr => "wstr",
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn export_rows_match_the_spec_and_its_forwards() {
    assert_eq!(EXPORT_ROWS.len(), EXPORTS.len());
    for (name, spelled, bytes, module, function) in EXPORT_ROWS {
        let export = find_export(name).unwrap();
        assert_eq!(spell(export.args), spelled);
        assert_eq!(stack_bytes(export.args), bytes);
        assert_eq!(forward_target(name), Some((module, function)));
    }
}

#[test]
fn sweep_rows_match_get_args_size() {
    for (spelled, args, bytes) in SWEEP_ROWS {
        assert_eq!(spell(args), spelled);
        assert_eq!(stack_bytes(args), bytes);
    }
}
