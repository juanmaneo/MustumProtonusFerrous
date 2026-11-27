//! Oracle-derived test vectors for the ported dpnlobby export table.
//!
//! The reference has no C body to compile, since its only code is the `dpnlobby.spec` export
//! table. The rows are what the reference `winebuild` writes for that spec. `winebuild` comes
//! from `tools/winebuild` and was built under `gcc -fsanitize=address,undefined -g`. The driver
//! `target/canon/dpnlobby/run_oracle.sh` runs it over the spec and prints `ORACLE_ROWS.txt`. The
//! driver reads each data-only PE's export directory with objdump. There are 18 rows: 10 `.def`
//! lines, 3 export directories and 5 data-only builds. ASan and UBSan flagged none of them, so none
//! is excluded. Leak detection was off, because `winebuild` leaks its own option strings at exit
//! in every run, outside the export data.

use super::*;

/// Rows 01 to 10: `(target, i386, kill_at, expected .def line)`.
const DEF_ROWS: [(&str, bool, bool, &str); 10] = [
    (
        "i386-windows",
        true,
        false,
        "DirectPlay8LobbyCreate@12=dpnet.DirectPlay8Create @1",
    ),
    (
        "i386-windows",
        true,
        true,
        "DirectPlay8LobbyCreate=dpnet.DirectPlay8Create @1",
    ),
    (
        "x86_64-windows",
        false,
        false,
        "DirectPlay8LobbyCreate=dpnet.DirectPlay8Create @1",
    ),
    (
        "x86_64-windows",
        false,
        true,
        "DirectPlay8LobbyCreate=dpnet.DirectPlay8Create @1",
    ),
    (
        "arm-windows",
        false,
        false,
        "DirectPlay8LobbyCreate=dpnet.DirectPlay8Create @1",
    ),
    (
        "arm-windows",
        false,
        true,
        "DirectPlay8LobbyCreate=dpnet.DirectPlay8Create @1",
    ),
    (
        "aarch64-windows",
        false,
        false,
        "DirectPlay8LobbyCreate=dpnet.DirectPlay8Create @1",
    ),
    (
        "aarch64-windows",
        false,
        true,
        "DirectPlay8LobbyCreate=dpnet.DirectPlay8Create @1",
    ),
    (
        "arm64ec-windows",
        false,
        false,
        "DirectPlay8LobbyCreate=dpnet.DirectPlay8Create @1",
    ),
    (
        "arm64ec-windows",
        false,
        true,
        "DirectPlay8LobbyCreate=dpnet.DirectPlay8Create @1",
    ),
];

/// Rows 11 to 13: `(target, dll, ordinal base, ordinal, name, forwarder)`, read from the export
/// directory of the data-only PE.
const PE_ROWS: [(&str, &str, u16, u16, &str, &str); 3] = [
    (
        "i386-windows",
        "dpnlobby.dll",
        1,
        1,
        "DirectPlay8LobbyCreate",
        "dpnet.DirectPlay8Create",
    ),
    (
        "x86_64-windows",
        "dpnlobby.dll",
        1,
        1,
        "DirectPlay8LobbyCreate",
        "dpnet.DirectPlay8Create",
    ),
    (
        "arm64ec-windows",
        "dpnlobby.dll",
        1,
        1,
        "DirectPlay8LobbyCreate",
        "dpnet.DirectPlay8Create",
    ),
];

/// Rows 14 to 18: the targets whose data-only build exited 0.
const DATA_ONLY_TARGETS: [&str; 5] = [
    "i386-windows",
    "x86_64-windows",
    "arm-windows",
    "aarch64-windows",
    "arm64ec-windows",
];

#[test]
fn oracle_def_rows() {
    for &(target, i386, kill_at, expected) in &DEF_ROWS {
        assert_eq!(
            EXPORTS[0].def_line(i386, kill_at),
            expected,
            "{target} kill_at={kill_at}"
        );
    }
}

#[test]
fn oracle_pe_export_rows() {
    for &(target, dll, base, ordinal, name, forwarder) in &PE_ROWS {
        let export = EXPORTS[0];
        assert_eq!(DLL_NAME, dll, "{target}");
        assert_eq!(
            EXPORTS.iter().map(|e| e.ordinal).min(),
            Some(base),
            "{target}"
        );
        assert_eq!(export.ordinal, ordinal, "{target}");
        assert_eq!(export.name, name, "{target}");
        assert_eq!(export.link_name, forwarder, "{target}");
        assert!(export.is_forward(), "{target}");
    }
}

#[test]
fn oracle_data_only_rows() {
    for target in DATA_ONLY_TARGETS {
        assert!(
            EXPORTS.iter().all(Export::is_forward),
            "{target}: data-only builds accept only forwarded entries"
        );
    }
}
