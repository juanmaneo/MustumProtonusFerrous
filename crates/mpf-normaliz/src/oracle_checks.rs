//! Oracle-derived test vectors for the ported normaliz tables.
//!
//! Vectors were captured under `target/canon/normaliz/` by compiling the verbatim `get_args_size`
//! body (`tools/winebuild/utils.c`), the `ArgNames` table (`tools/winebuild/parser.c`) and the
//! `arg_type` enum (`tools/winebuild/build.h`) with `gcc -fsanitize=address,undefined -g`. A reader
//! for `dlls/normaliz/normaliz.spec`, shaped after winebuild's parser, supplies the five exports.
//! The harness printed 16 rows: 5 exports and 11 argument sweeps. ASan and UBSan flagged none of
//! them, so none is excluded.

use super::*;

/// `(export, argument types as written in normaliz.spec, stack bytes, forward module, forward name)`.
const EXPORT_ROWS: [(&str, &str, u32, &str, &str); 5] = [
    (
        "IdnToAscii",
        "long wstr long ptr long",
        20,
        "kernel32",
        "IdnToAscii",
    ),
    (
        "IdnToNameprepUnicode",
        "long wstr long ptr long",
        20,
        "kernel32",
        "IdnToNameprepUnicode",
    ),
    (
        "IdnToUnicode",
        "long wstr long ptr long",
        20,
        "kernel32",
        "IdnToUnicode",
    ),
    (
        "IsNormalizedString",
        "long wstr long",
        12,
        "kernel32",
        "IsNormalizedString",
    ),
    (
        "NormalizeString",
        "long wstr long ptr long",
        20,
        "kernel32",
        "NormalizeString",
    ),
];

/// `(argument types, stack bytes)` for argument lists that the sweep feeds to `get_args_size`.
const SWEEP_ROWS: [(&str, &[ArgKind], u32); 11] = [
    ("", &[], 0),
    ("long", &[ArgKind::Long], 4),
    ("ptr", &[ArgKind::Ptr], 4),
    ("wstr", &[ArgKind::Wstr], 4),
    ("long wstr", &[ArgKind::Long, ArgKind::Wstr], 8),
    ("ptr wstr", &[ArgKind::Ptr, ArgKind::Wstr], 8),
    (
        "long wstr long",
        &[ArgKind::Long, ArgKind::Wstr, ArgKind::Long],
        12,
    ),
    (
        "long wstr long ptr long",
        &[
            ArgKind::Long,
            ArgKind::Wstr,
            ArgKind::Long,
            ArgKind::Ptr,
            ArgKind::Long,
        ],
        20,
    ),
    (
        "long wstr long ptr long long",
        &[
            ArgKind::Long,
            ArgKind::Wstr,
            ArgKind::Long,
            ArgKind::Ptr,
            ArgKind::Long,
            ArgKind::Long,
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

/// The argument types of `args`, spelled the way `normaliz.spec` writes them.
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
