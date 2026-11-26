//! Oracle-derived test vectors for the ported powrprof logic.
//!
//! Rows were printed by `target/canon/powrprof/harness`, which compiles the
//! verbatim Wine bodies of `IsPwrHibernateAllowed`, `IsPwrShutdownAllowed`,
//! `IsPwrSuspendAllowed` and `GetPwrDiskSpindownRange` from
//! `dlls/powrprof/powrprof.c` with `gcc -fsanitize=address,undefined -g`. The
//! NtPowerInformation and registry calls are mocked. The harness printed 23 rows
//! (8 capability rows, 15 spindown rows). ASan and UBSan reported nothing, so no
//! rows were excluded.

use super::*;

/// Builds the UTF-16LE bytes of a `REG_SZ` value: the text plus a NUL code unit.
fn reg_sz(text: &str) -> Vec<u8> {
    text.encode_utf16()
        .chain(std::iter::once(0))
        .flat_map(u16::to_le_bytes)
        .collect()
}

/// Builds the capabilities from `[SystemS1, SystemS2, SystemS3, SystemS4, SystemS5, HiberFilePresent]`.
fn caps(s: [u8; 6]) -> SystemPowerCapabilities {
    SystemPowerCapabilities {
        system_s1: s[0],
        system_s2: s[1],
        system_s3: s[2],
        system_s4: s[3],
        system_s5: s[4],
        hiber_file_present: s[5],
    }
}

/// `(DiskSpindownMax, DiskSpindownMin, (RangeMax, RangeMin))`.
type SpindownRow = (Option<&'static str>, Option<&'static str>, (u32, u32));

/// Calls `GetPwrDiskSpindownRange` after its registry reads, with `None` for an unreadable value.
fn spindown(max: Option<&str>, min: Option<&str>) -> (u32, u32) {
    let max = max.map(reg_sz);
    let min = min.map(reg_sz);
    disk_spindown_range(max.as_deref(), min.as_deref())
}

#[test]
fn oracle_capability_rows() {
    // [s1, s2, s3, s4, s5, hiber] -> [hibernate, shutdown, suspend]
    let rows: [([u8; 6], [u8; 3]); 8] = [
        ([0, 0, 0, 0, 0, 0], [0, 0, 0]),
        ([1, 1, 1, 1, 1, 1], [1, 1, 1]),
        ([1, 1, 1, 0, 1, 0], [0, 1, 1]),
        ([0, 1, 1, 1, 0, 1], [1, 0, 0]),
        ([1, 0, 1, 1, 0x80, 1], [1, 0x80, 0]),
        ([0x02, 0x80, 0xff, 0x02, 0xff, 0x00], [0, 0xff, 1]),
        ([0, 0, 0, 0x80, 0, 0x02], [1, 0, 0]),
        ([1, 1, 0, 0, 2, 1], [0, 2, 0]),
    ];
    for (input, expected) in rows {
        let c = caps(input);
        let actual = [
            is_pwr_hibernate_allowed(&c),
            is_pwr_shutdown_allowed(&c),
            is_pwr_suspend_allowed(&c),
        ];
        assert_eq!(actual, expected, "capabilities {input:?}");
    }
}

#[test]
fn oracle_spindown_rows() {
    // (DiskSpindownMax, DiskSpindownMin) -> (RangeMax, RangeMin)
    let rows: [SpindownRow; 15] = [
        // open=no: a failed key open is modelled as both values absent.
        (None, None, (3600, 3)),
        (Some("1200"), Some("5"), (1200, 5)),
        (Some("  -1"), Some("0"), (4_294_967_295, 0)),
        (Some("abc"), Some("2"), (0, 2)),
        (
            Some("99999999999"),
            Some("-99999999999"),
            (2_147_483_647, 2_147_483_648),
        ),
        (None, Some("4"), (3600, 4)),
        (Some("+42"), None, (42, 3)),
        (Some("12abc"), Some("\x09 9"), (12, 9)),
        (Some("1234567890123456789012345"), Some("1"), (3600, 1)),
        (Some("1234567890123456789"), Some("0"), (2_147_483_647, 0)),
        (Some("-2147483649"), Some("+"), (2_147_483_648, 0)),
        (Some(""), Some(""), (0, 0)),
        (
            Some("2147483647"),
            Some("2147483648"),
            (2_147_483_647, 2_147_483_647),
        ),
        (Some("0x1F"), Some("\x0b\x0c8"), (0, 8)),
        (Some("-0"), Some("1 2"), (0, 1)),
    ];
    for (max, min, expected) in rows {
        assert_eq!(spindown(max, min), expected, "spindown {max:?} {min:?}");
    }
}
