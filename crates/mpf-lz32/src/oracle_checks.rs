//! Oracle-derived test vectors for the ported `lz32.spec` export table.
//!
//! Vectors were captured by running the verbatim Wine `tools/winebuild` (built with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/lz32/`) in `--def -m32` mode on
//! `dlls/lz32/lz32.spec`, through `target/canon/lz32/oracle.sh`. The 15 printed rows are the
//! `LIBRARY` line, the 12 exports, and two probe specs: `@ stdcall Dotted(long long) a.b.c` and
//! `@ stdcall Local(long) LocalImpl`. ASan and UBSan reported nothing for any row, so no vector
//! was excluded.

use super::*;

/// The `.def` line that `winebuild` printed for each export, in spec order. Rows 02 to 13.
const EXPORT_ROWS: [(&str, &str); 12] = [
    ("CopyLZFile", "  CopyLZFile@8=kernel32.CopyLZFile @1"),
    (
        "GetExpandedNameA",
        "  GetExpandedNameA@8=kernel32.GetExpandedNameA @2",
    ),
    (
        "GetExpandedNameW",
        "  GetExpandedNameW@8=kernel32.GetExpandedNameW @3",
    ),
    ("LZClose", "  LZClose@4=kernel32.LZClose @4"),
    ("LZCopy", "  LZCopy@8=kernel32.LZCopy @5"),
    ("LZDone", "  LZDone@0=kernel32.LZDone @6"),
    ("LZInit", "  LZInit@4=kernel32.LZInit @7"),
    ("LZOpenFileA", "  LZOpenFileA@12=kernel32.LZOpenFileA @8"),
    ("LZOpenFileW", "  LZOpenFileW@12=kernel32.LZOpenFileW @9"),
    ("LZRead", "  LZRead@12=kernel32.LZRead @10"),
    ("LZSeek", "  LZSeek@12=kernel32.LZSeek @11"),
    ("LZStart", "  LZStart@0=kernel32.LZStart @12"),
];

#[test]
fn oracle_library_row() {
    // row 01 | LIBRARY lz32.dll
    assert_eq!(def_body().lines().next(), Some("LIBRARY lz32.dll"));
}

#[test]
fn oracle_export_rows() {
    for (name, row) in EXPORT_ROWS {
        let export = export(name).unwrap();
        assert_eq!(export.def_line(), row, "export {name}");
    }
}

#[test]
fn oracle_def_body_rows() {
    // rows 01 to 13 printed by oracle.sh: the LIBRARY line, a blank line, EXPORTS, then the exports.
    let mut expected = String::from("LIBRARY lz32.dll\n\nEXPORTS\n");
    for (_, row) in EXPORT_ROWS {
        expected.push_str(row);
        expected.push('\n');
    }
    assert_eq!(def_body(), expected);
}

#[test]
fn oracle_probe_rows() {
    // row 14 | Dotted@8=a.b.c @1, from `@ stdcall Dotted(long long) a.b.c`
    let dotted = Export {
        name: "Dotted",
        ordinal: 1,
        args: &[ArgKind::Long, ArgKind::Long],
        target: "a.b.c",
    };
    assert_eq!(dotted.def_line(), "  Dotted@8=a.b.c @1");

    // row 15 | Local@4=LocalImpl @1, from `@ stdcall Local(long) LocalImpl`
    let local = Export {
        name: "Local",
        ordinal: 1,
        args: &[ArgKind::Long],
        target: "LocalImpl",
    };
    assert_eq!(local.def_line(), "  Local@4=LocalImpl @1");
}
