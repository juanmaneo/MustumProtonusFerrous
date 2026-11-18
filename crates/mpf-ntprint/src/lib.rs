//! Headless subset of Wine's `dlls/ntprint/ntprint.c` monitor enumeration.
//!
//! The C file has three exports. Only `PSetupEnumMonitor` has logic that does
//! not depend on the spooler, so only its index, buffer size and copy rules are
//! ported. Those rules are a pure function of a monitor name list.
//!
//! Ported (partial):
//! - `PSetupEnumMonitor`: `enum_monitor_name`. Given the monitor names (the
//!   slice that Wine's `monitorinfo_t` would hold), an index and a buffer
//!   capacity in WCHARs, it returns the name plus its NUL, or the size the
//!   caller needs, or "no more items". The `HANDLE` is replaced by the slice,
//!   `*psize` by `capacity`, and the write into `buffer` by the returned `Vec`.
//!   Divergence: Wine's `lstrlenW` reads past a name that has no NUL, which is
//!   undefined behaviour here. A name without a NUL ends at its slice end.
//!   Wine also crashes on a NULL `psize` or `buffer`; the port takes neither.
//!
//! Skipped (35 `ntprint.spec` entries: 2 real exports and 32 stubs):
//! - `PSetupCreateMonitorInfo`: calls `EnumMonitorsW` on a print server and
//!   allocates the monitor handle. It needs spooler state.
//! - `PSetupDestroyMonitorInfo`: frees the handle and its spooler buffer. There
//!   is no result to port.
//! - 32 `stub` entries (for example `PSetupInstallMonitor`, `PSetupGetDriverInfo3`,
//!   `PSetupSelectDriver`): spec-only names with no body in `ntprint.c`. They
//!   cover driver, monitor and ICM installation, printer device lists, driver
//!   dialogs and INF search. Porting them would invent behaviour.

/// `ERROR_INSUFFICIENT_BUFFER`, the last error Wine sets when the name does not fit.
pub const ERROR_INSUFFICIENT_BUFFER: u32 = 122;

/// `ERROR_NO_MORE_ITEMS`, the last error Wine sets when the index is past the last monitor.
pub const ERROR_NO_MORE_ITEMS: u32 = 259;

/// Why `enum_monitor_name` did not copy a name.
#[derive(Debug, PartialEq, Eq)]
pub enum EnumMonitorError {
    /// The name and its NUL need `needed` WCHARs, which Wine stores in `*psize`.
    InsufficientBuffer { needed: usize },
    /// `index` is not below the number of monitors.
    NoMoreItems,
}

impl EnumMonitorError {
    /// The Win32 error code that Wine leaves in `GetLastError`.
    pub fn win32_error(&self) -> u32 {
        match self {
            Self::InsufficientBuffer { .. } => ERROR_INSUFFICIENT_BUFFER,
            Self::NoMoreItems => ERROR_NO_MORE_ITEMS,
        }
    }
}

/// Returns the monitor name at `index` with its NUL terminator, as Wine's `PSetupEnumMonitor` copies it.
///
/// The name is read up to its first NUL, as `lstrlenW` does. The copy has `len + 1` units and is returned only when that count is at most `capacity`. Otherwise the error carries the count the caller must allocate.
pub fn enum_monitor_name(
    names: &[&[u16]],
    index: usize,
    capacity: usize,
) -> Result<Vec<u16>, EnumMonitorError> {
    let Some(name) = names.get(index) else {
        return Err(EnumMonitorError::NoMoreItems);
    };
    let len = name
        .iter()
        .position(|&unit| unit == 0)
        .unwrap_or(name.len());
    let needed = len + 1;
    if needed > capacity {
        return Err(EnumMonitorError::InsufficientBuffer { needed });
    }
    let mut copied = name[..len].to_vec();
    copied.push(0);
    Ok(copied)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wide(text: &str) -> Vec<u16> {
        text.encode_utf16().collect()
    }

    fn run(names: &[&str], index: usize, capacity: usize) -> Result<Vec<u16>, EnumMonitorError> {
        let wide_names: Vec<Vec<u16>> = names.iter().map(|name| wide(name)).collect();
        let slices: Vec<&[u16]> = wide_names.iter().map(Vec::as_slice).collect();
        enum_monitor_name(&slices, index, capacity)
    }

    /// What one `PSetupEnumMonitor` call did, as the oracle reports it.
    enum Outcome {
        /// The buffer received this text, with its trailing NUL.
        Copied(&'static str),
        /// `*psize` came back as this count.
        Insufficient(usize),
        NoMoreItems,
    }

    #[test]
    fn enum_monitor_matches_wine_oracle() {
        // Rows E01 to E20 of target/canon/ntprint/out_enum_monitor.txt, in input order.
        let cases: [(&str, &[&str], usize, usize, Outcome); 20] = [
    ("E01", &[], 0, 10, Outcome::NoMoreItems),
    ("E02", &["Local Port"], 0, 11, Outcome::Copied("Local Port\u{0}")),
    ("E03", &["Local Port"], 0, 10, Outcome::Insufficient(11)),
    ("E04", &["Local Port"], 0, 0, Outcome::Insufficient(11)),
    ("E05", &["Local Port"], 1, 100, Outcome::NoMoreItems),
    ("E06", &["Local Port"], 1, 0, Outcome::NoMoreItems),
    ("E07", &[""], 0, 1, Outcome::Copied("\u{0}")),
    ("E08", &[""], 0, 0, Outcome::Insufficient(1)),
    ("E09", &["A"], 0, 2, Outcome::Copied("A\u{0}")),
    ("E10", &["A"], 0, 1, Outcome::Insufficient(2)),
    ("E11", &["Local Port", "Standard TCP/IP Port", "USB Monitor"], 1, 100, Outcome::Copied("Standard TCP/IP Port\u{0}")),
    ("E12", &["Local Port", "Standard TCP/IP Port", "USB Monitor"], 2, 12, Outcome::Copied("USB Monitor\u{0}")),
    ("E13", &["Local Port", "Standard TCP/IP Port", "USB Monitor"], 1, 20, Outcome::Insufficient(21)),
    ("E14", &["Local Port", "Standard TCP/IP Port", "USB Monitor"], 0, 20, Outcome::Copied("Local Port\u{0}")),
    ("E15", &["Local Port", "Standard TCP/IP Port", "USB Monitor"], 3, 500, Outcome::NoMoreItems),
    ("E16", &["Local Port", "Standard TCP/IP Port", "USB Monitor"], 99, 0, Outcome::NoMoreItems),
    ("E17", &["ab\u{0}cd"], 0, 10, Outcome::Copied("ab\u{0}")),
    ("E18", &["Ünïcode Mon"], 0, 12, Outcome::Copied("Ünïcode Mon\u{0}")),
    ("E19", &["emoji 😀 port"], 0, 13, Outcome::Insufficient(14)),
    ("E20", &["WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW"], 0, 260, Outcome::Copied("WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW\u{0}")),
        ];
        for (id, names, index, capacity, want) in cases {
            let got = run(names, index, capacity);
            match want {
                Outcome::Copied(text) => assert_eq!(got, Ok(wide(text)), "{id}"),
                Outcome::Insufficient(needed) => assert_eq!(
                    got,
                    Err(EnumMonitorError::InsufficientBuffer { needed }),
                    "{id}"
                ),
                Outcome::NoMoreItems => assert_eq!(got, Err(EnumMonitorError::NoMoreItems), "{id}"),
            }
        }
    }

    #[test]
    fn index_is_checked_before_capacity() {
        assert_eq!(run(&[], 0, 0), Err(EnumMonitorError::NoMoreItems));
        assert_eq!(run(&["A"], 1, 0), Err(EnumMonitorError::NoMoreItems));
        assert_eq!(
            run(&["A"], usize::MAX, 0),
            Err(EnumMonitorError::NoMoreItems)
        );
    }

    #[test]
    fn every_capacity_boundary_for_one_name() {
        for capacity in 0..=12 {
            let want = if capacity >= 11 {
                Ok(wide("Local Port\0"))
            } else {
                Err(EnumMonitorError::InsufficientBuffer { needed: 11 })
            };
            assert_eq!(
                run(&["Local Port"], 0, capacity),
                want,
                "capacity {capacity}"
            );
        }
    }

    #[test]
    fn leading_nul_gives_an_empty_name() {
        assert_eq!(run(&["\0abc"], 0, 1), Ok(vec![0]));
        assert_eq!(
            run(&["\0abc"], 0, 0),
            Err(EnumMonitorError::InsufficientBuffer { needed: 1 })
        );
    }

    #[test]
    fn name_without_nul_ends_at_the_slice_end() {
        let units = [u16::from(b'A'), u16::from(b'B')];
        assert_eq!(enum_monitor_name(&[&units], 0, 3), Ok(vec![65, 66, 0]));
        assert_eq!(
            enum_monitor_name(&[&units], 0, 2),
            Err(EnumMonitorError::InsufficientBuffer { needed: 3 })
        );
    }

    #[test]
    fn empty_monitor_list_has_no_items() {
        assert_eq!(
            enum_monitor_name(&[], 0, usize::MAX),
            Err(EnumMonitorError::NoMoreItems)
        );
    }

    #[test]
    fn errors_carry_the_wine_last_error_codes() {
        assert_eq!(ERROR_INSUFFICIENT_BUFFER, 122);
        assert_eq!(ERROR_NO_MORE_ITEMS, 259);
        assert_eq!(
            EnumMonitorError::InsufficientBuffer { needed: 5 }.win32_error(),
            122
        );
        assert_eq!(EnumMonitorError::NoMoreItems.win32_error(), 259);
    }

    #[test]
    fn errors_are_comparable_and_debug_printable() {
        let error = EnumMonitorError::InsufficientBuffer { needed: 7 };
        assert_eq!(error, EnumMonitorError::InsufficientBuffer { needed: 7 });
        assert_ne!(error, EnumMonitorError::NoMoreItems);
        assert_eq!(format!("{error:?}"), "InsufficientBuffer { needed: 7 }");
        assert_eq!(
            format!("{:?}", EnumMonitorError::NoMoreItems),
            "NoMoreItems"
        );
    }
}
