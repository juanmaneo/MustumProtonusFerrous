//! Headless subset of Wine's `dlls/odbccp32/odbccp32.c` data source name checks.
//!
//! The two ported exports are pure functions of their string argument. Each C
//! body also calls `clear_errors`, which empties Wine's process-wide installer
//! error stack. That stack is not modelled, so the port has no side effect; the
//! oracle counts these calls and always sees exactly one per call.
//!
//! Ported (exact, pure string checks):
//! - `SQLValidDSN`: `valid_dsn`. Rejects a null, empty or over-long name, and any name containing a character from `INVALID_DSN_CHARS`. Only the bytes before the first NUL are checked.
//! - `SQLValidDSNW`: `valid_dsn_w`. The same rules over 16-bit units, so a surrogate pair counts as two units.
//!
//! Skipped (55 of the 57 `odbccp32.spec` entries):
//! - Stubs, 23 entries: bodies that only log `FIXME` and fail or return fixed values, and spec-only names with no C body (`SQLCreateDataSourceEx(W)`, `SelectTransDlg`, `SQLLoadDriverListBox`, `SQLLoadDataSourcesListBox`). Porting them would invent behaviour. This group includes `ODBCCPlApplet`, `SQLCreateDataSource(W)`, `SQLGetAvailableDrivers(W)`, `SQLGetTranslator(W)`, `SQLInstallODBC(W)`, `SQLManageDataSources`, `SQLPostInstallerError(W)`, `SQLRemoveDefaultDataSource`, `SQLRemoveDriverManager` (it returns a fixed `TRUE` and usage count), and the file DSN calls `SQLReadFileDSN(W)` and `SQLWriteFileDSN(W)`.
//! - Registry DSN and driver configuration, 12 entries: `SQLConfigDataSource(W)`, `SQLConfigDriver(W)`, `SQLWriteDSNToIni(W)`, `SQLRemoveDSNFromIni(W)`, `SQLWritePrivateProfileString(W)`, `SQLGetPrivateProfileString(W)`. They read or write the registry and load driver modules.
//! - Driver and translator install, remove and list, 16 entries: `SQLInstallDriver(W)`, `SQLInstallDriverEx(W)`, `SQLInstallDriverManager(W)`, `SQLGetInstalledDrivers(W)`, `SQLInstallTranslator(W)`, `SQLInstallTranslatorEx(W)`, `SQLRemoveDriver(W)`, `SQLRemoveTranslator(W)`. They depend on registry, driver manager and installer state.
//! - Installer error stack and config mode, 4 entries: `SQLInstallerError(W)`, `SQLGetConfigMode`, `SQLSetConfigMode`. The error stack and the config mode are process-wide state.

/// The longest data source name, in characters, that `SQLValidDSN` accepts (`SQL_MAX_DSN_LENGTH` in `sqlext.h`).
pub const SQL_MAX_DSN_LENGTH: usize = 32;

/// The characters that make a data source name invalid wherever they appear.
pub const INVALID_DSN_CHARS: &[u8] = b"[]{}(),;?*=!@\\";

/// Returns whether `name` is a valid data source name, as Wine's `SQLValidDSN` decides.
///
/// A null, empty or over-long name is invalid, and so is one containing a character from `INVALID_DSN_CHARS`. Only the bytes before the first NUL count.
pub fn valid_dsn(name: Option<&[u8]>) -> bool {
    is_valid_dsn(name, |byte| INVALID_DSN_CHARS.contains(&byte))
}

/// Returns whether `name` is a valid data source name, as Wine's `SQLValidDSNW` decides.
///
/// The rules match `valid_dsn`, with the length counted in 16-bit units. A unit that is not ASCII is never invalid, even when its low byte is one of `INVALID_DSN_CHARS`.
pub fn valid_dsn_w(name: Option<&[u16]>) -> bool {
    is_valid_dsn(name, |unit| {
        u8::try_from(unit).is_ok_and(|byte| INVALID_DSN_CHARS.contains(&byte))
    })
}

fn is_valid_dsn<T, F>(name: Option<&[T]>, is_invalid: F) -> bool
where
    T: Copy + Default + PartialEq,
    F: Fn(T) -> bool,
{
    let Some(name) = name else {
        return false;
    };
    let name = until_nul(name);
    !name.is_empty() && name.len() <= SQL_MAX_DSN_LENGTH && !name.iter().copied().any(is_invalid)
}

/// The units before the first NUL, or all of them when there is no NUL.
fn until_nul<T: Copy + Default + PartialEq>(units: &[T]) -> &[T] {
    match units.iter().position(|&unit| unit == T::default()) {
        Some(end) => &units[..end],
        None => units,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wide(text: Option<&str>) -> Option<Vec<u16>> {
        text.map(|text| text.encode_utf16().collect())
    }

    #[test]
    fn ansi_names_match_wine_oracle() {
        // Rows A01 to A24 of target/canon/odbccp32/out_odbccp32.txt, in input order.
        let cases: [(&str, Option<&[u8]>, bool); 24] = [
            ("A01", Some(b"MyDSN"), true),
            ("A02", Some(b""), false),
            ("A03", None, false),
            ("A04", Some(b"Wine Vinegar"), true),
            ("A05", Some(b"12345678901234567890123456789012"), true),
            ("A06", Some(b"123456789012345678901234567890123"), false),
            ("A07", Some(b"wine[10"), false),
            ("A08", Some(b"wine]10"), false),
            ("A09", Some(b"wine{10"), false),
            ("A10", Some(b"wine}10"), false),
            ("A11", Some(b"wine(10"), false),
            ("A12", Some(b"wine)10"), false),
            ("A13", Some(b"wine,10"), false),
            ("A14", Some(b"wine;10"), false),
            ("A15", Some(b"wine?10"), false),
            ("A16", Some(b"wine*10"), false),
            ("A17", Some(b"wine=10"), false),
            ("A18", Some(b"wine!10"), false),
            ("A19", Some(b"wine@10"), false),
            ("A20", Some(b"wine\\10"), false),
            ("A21", Some(b"wine\x00[10"), true),
            ("A22", Some(b"\x00wine"), false),
            ("A23", Some(b"caf\xe9"), true),
            ("A24", Some(b"1234567890123456789012345678901]"), false),
        ];
        for (id, name, want) in cases {
            assert_eq!(valid_dsn(name), want, "{id}");
        }
    }

    #[test]
    fn wide_names_match_wine_oracle() {
        // Rows W01 to W24 of target/canon/odbccp32/out_odbccp32.txt, in input order.
        let cases: [(&str, Option<&str>, bool); 24] = [
            ("W01", Some("MyDSN"), true),
            ("W02", Some(""), false),
            ("W03", None, false),
            ("W04", Some("Wine Vinegar"), true),
            ("W05", Some("12345678901234567890123456789012"), true),
            ("W06", Some("123456789012345678901234567890123"), false),
            ("W07", Some("wine[10"), false),
            ("W08", Some("wine]10"), false),
            ("W09", Some("wine{10"), false),
            ("W10", Some("wine}10"), false),
            ("W11", Some("wine(10"), false),
            ("W12", Some("wine)10"), false),
            ("W13", Some("wine,10"), false),
            ("W14", Some("wine;10"), false),
            ("W15", Some("wine?10"), false),
            ("W16", Some("wine*10"), false),
            ("W17", Some("wine=10"), false),
            ("W18", Some("wine!10"), false),
            ("W19", Some("wine@10"), false),
            ("W20", Some("wine\\10"), false),
            ("W21", Some("wine\u{0}[10"), true),
            ("W22", Some("\u{0}wine"), false),
            ("W23", Some("caf\u{e9}"), true),
            ("W24", Some("123456789012345678901234567890\u{1f600}"), true),
        ];
        for (id, name, want) in cases {
            let units = wide(name);
            assert_eq!(valid_dsn_w(units.as_deref()), want, "{id}");
        }
    }

    #[test]
    fn null_and_empty_names_are_rejected() {
        assert!(!valid_dsn(None));
        assert!(!valid_dsn(Some(&[])));
        assert!(!valid_dsn_w(None));
        assert!(!valid_dsn_w(Some(&[])));
    }

    #[test]
    fn nul_ends_the_name() {
        assert!(valid_dsn(Some(b"wine\0]")));
        assert!(valid_dsn(Some(b"wine\0\0")));
        assert!(!valid_dsn(Some(b"\0")));
        assert!(!valid_dsn(Some(b"\0wine")));
        assert!(valid_dsn_w(Some(&[b'w' as u16, 0, b']' as u16])));
        assert!(!valid_dsn_w(Some(&[0u16])));
        assert!(!valid_dsn_w(Some(&[0u16, b'w' as u16])));
    }

    #[test]
    fn name_without_nul_is_read_to_the_end() {
        assert!(valid_dsn(Some(b"wine")));
        assert!(!valid_dsn(Some(b"wine]")));
        assert!(valid_dsn_w(Some(&[b'w' as u16])));
        assert!(!valid_dsn_w(Some(&[b'w' as u16, b']' as u16])));
    }

    #[test]
    fn every_invalid_char_is_rejected_at_every_position() {
        assert_eq!(INVALID_DSN_CHARS.len(), 14);
        for &invalid in INVALID_DSN_CHARS {
            for position in 0..5 {
                let mut name = b"wine_".to_vec();
                name[position] = invalid;
                assert!(!valid_dsn(Some(&name)), "{}", char::from(invalid));
                let units: Vec<u16> = name.iter().map(|&byte| u16::from(byte)).collect();
                assert!(!valid_dsn_w(Some(&units)), "{}", char::from(invalid));
            }
        }
    }

    #[test]
    fn other_ascii_chars_are_accepted() {
        for byte in 0x20u8..0x7f {
            let want = !INVALID_DSN_CHARS.contains(&byte);
            assert_eq!(valid_dsn(Some(&[byte])), want, "{byte:#x}");
            assert_eq!(valid_dsn_w(Some(&[u16::from(byte)])), want, "{byte:#x}");
        }
    }

    #[test]
    fn high_units_are_never_invalid() {
        for byte in 0x80u8..=0xff {
            assert!(valid_dsn(Some(&[byte])), "{byte:#x}");
        }
        // Low byte 0x5d is ']', but these units are not ASCII, so they are valid names.
        assert!(valid_dsn_w(Some(&[0x015d])));
        assert!(valid_dsn_w(Some(&[0x015b])));
        assert!(valid_dsn_w(Some(&[0xff5d])));
        assert!(valid_dsn(Some(&[0xdd])));
    }

    #[test]
    fn ansi_length_limit_is_32_bytes() {
        assert!(valid_dsn(Some(&[b'a'; SQL_MAX_DSN_LENGTH])));
        assert!(!valid_dsn(Some(&[b'a'; SQL_MAX_DSN_LENGTH + 1])));
        let mut padded = vec![b'a'; 40];
        padded[SQL_MAX_DSN_LENGTH] = 0;
        assert!(valid_dsn(Some(&padded)));
    }

    #[test]
    fn wide_length_limit_counts_units() {
        assert!(valid_dsn_w(Some(&[b'a' as u16; SQL_MAX_DSN_LENGTH])));
        assert!(!valid_dsn_w(Some(&[b'a' as u16; SQL_MAX_DSN_LENGTH + 1])));
        // A surrogate pair is two units, so 31 ASCII units and one pair is 33 units.
        let mut pair: Vec<u16> = vec![b'a' as u16; 31];
        pair.extend("\u{1f600}".encode_utf16());
        assert!(!valid_dsn_w(Some(&pair)));
        let mut padded = vec![b'a' as u16; 40];
        padded[SQL_MAX_DSN_LENGTH] = 0;
        assert!(valid_dsn_w(Some(&padded)));
    }

    #[test]
    fn constants_match_the_wine_sources() {
        // SQL_MAX_DSN_LENGTH is `#define SQL_MAX_DSN_LENGTH 32` in include/sqlext.h. INVALID_DSN_CHARS is the string literal in SQLValidDSN.
        assert_eq!(SQL_MAX_DSN_LENGTH, 32);
        assert_eq!(INVALID_DSN_CHARS, b"[]{}(),;?*=!@\\");
    }
}
