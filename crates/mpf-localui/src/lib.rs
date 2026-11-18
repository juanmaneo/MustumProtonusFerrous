//! Headless subset of Wine's `dlls/localui/localui.c`, the local print-monitor UI.
//!
//! The entry points `AddPortUI`, `ConfigurePortUI` and `DeletePortUI` show dialogs and call the monitor through `XcvData`. The port keeps the decisions that do not depend on a window or a live monitor: the port type of a name, the name that is opened, the COM short name, the LPT retry-timeout limits, how a `PortIsValid` result is read, which dialog command was sent, and the result and `GetLastError` value of each entry point. The live results come in as arguments.
//!
//! Ported:
//! - `get_type_from_name`: `port_type_from_name`. The `LPT` and `COM` prefixes match without regard to case. `FILE:` must match the whole name without regard to case. `/`, `|`, `CUPS:` and `LPR:` follow, and `CUPS:` and `LPR:` are case-sensitive. Any other name goes to the file probe, which stands for Wine's `CreateFileW` pair (open existing, then open always with delete on close). The probe is called only when no prefix matches.
//! - `strdupWW` in `open_monitor_by_name`: `open_name`, the `,XcvPort ` or `,XcvMonitor ` name for `OpenPrinterW`.
//! - `dlg_configure_com`: `strip_port_colon`, the short name sent to `GetDefaultCommConfig`.
//! - `dlgproc_lptconfig`: `lpt_retry_timeout_request`, the value sent to `ConfigureLPTPortCommandOK`.
//! - `dlgproc_addport`: `port_name_check` for the `PortIsValid` result, and `dialog_command` for `WM_COMMAND`.
//! - `localui_AddPortUI`: `add_port_outcome`.
//! - `localui_ConfigurePortUI`: `configure_port_outcome`.
//! - `localui_DeletePortUI`: `delete_port_precheck` and `delete_port_outcome`.
//!
//! Skipped. `localui.spec` exports only `InitializePrintMonitorUI`, so there are no other exports and no FIXME stubs.
//! - `InitializePrintMonitorUI`: returns a pointer to a static function table, with no logic to port.
//! - `DllMain`: records the module handle and calls `DisableThreadLibraryCalls`.
//! - Dialog, message-box and system-error text (`LoadStringW`, `MessageBoxW`, `FormatMessageW`, `swprintf` into the message): resource and system text.
//! - `XcvData`, `OpenPrinterW`, `ClosePrinter`, `CreateFileW`, the `GlobalAlloc` copy of the published name, and the initial LPT value read from the monitor: live monitor and file state.
//! - `DWLP_USER`, `EndDialog` and `DialogBoxParamW` plumbing: window state.
//!
//! Oracle provenance: a harness compiles Wine's `localui.c` verbatim (sha256 in `target/canon/localui/sources.sha256`) with `gcc -std=gnu11 -fshort-wchar -g -fsanitize=address,undefined`, against Win32 shims that log each call. Its 25 rows are in `src/oracle_checks.rs`. The Wine NLS upcase table cannot be loaded there, so the harness uses glibc `towupper` in its place. `fold_upper` relies on the Unicode mapping for the two non-ASCII units that fold to ASCII, `U+0131` and `U+017F`. That is an inference, not checked against Wine's table.

/// `PORT_IS_UNKNOWN`: the name is neither a port nor a file that can be created.
pub const PORT_IS_UNKNOWN: u32 = 0;
/// `PORT_IS_LPT`: a parallel port such as `LPT1:`.
pub const PORT_IS_LPT: u32 = 1;
/// `PORT_IS_COM`: a serial port such as `COM1:`.
pub const PORT_IS_COM: u32 = 2;
/// `PORT_IS_FILE`: the `FILE:` port.
pub const PORT_IS_FILE: u32 = 3;
/// `PORT_IS_FILENAME`: a file name that the probe could open or create.
pub const PORT_IS_FILENAME: u32 = 4;
/// `PORT_IS_UNIXNAME` (also `PORT_IS_WINE`): a name that starts with `/`.
pub const PORT_IS_UNIXNAME: u32 = 5;
/// `PORT_IS_PIPE`: a name that starts with `|`.
pub const PORT_IS_PIPE: u32 = 6;
/// `PORT_IS_CUPS`: a name that starts with `CUPS:`.
pub const PORT_IS_CUPS: u32 = 7;
/// `PORT_IS_LPR`: a name that starts with `LPR:`.
pub const PORT_IS_LPR: u32 = 8;

/// `ERROR_SUCCESS`.
pub const ERROR_SUCCESS: u32 = 0;
/// `ERROR_INVALID_PARAMETER`: a required port name is missing or empty.
pub const ERROR_INVALID_PARAMETER: u32 = 87;
/// `ERROR_INVALID_NAME`: the monitor rejected a port name.
pub const ERROR_INVALID_NAME: u32 = 123;
/// `ERROR_ALREADY_EXISTS`: the port is already present.
pub const ERROR_ALREADY_EXISTS: u32 = 183;
/// `ERROR_CANCELLED`: the user cancelled, or the port has nothing to configure.
pub const ERROR_CANCELLED: u32 = 1223;
/// `ERROR_UNKNOWN_PORT`: the monitor could not be opened for the port.
pub const ERROR_UNKNOWN_PORT: u32 = 1796;

/// `IDOK`.
pub const IDOK: u16 = 1;
/// `IDCANCEL`.
pub const IDCANCEL: u16 = 2;
/// `BN_CLICKED`, the notification code of a button click.
pub const BN_CLICKED: u16 = 0;

/// The LPT retry timeout is accepted when it is above 0 and below this value.
const LPT_RETRY_LIMIT: u32 = 1_000_000;

/// The `PortIsValid` decision in the add-port dialog.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortNameCheck {
    /// The name is valid, and the dialog closes with success.
    Valid,
    /// The monitor rejected the name. The dialog shows the invalid-name notice and stays open.
    InvalidName,
    /// Any other result. The dialog shows the system message for this error and stays open.
    Failed(u32),
}

/// The `WM_COMMAND` decision of the dialogs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogCommand {
    /// `IDOK` with `BN_CLICKED`.
    Ok,
    /// `IDCANCEL` with `BN_CLICKED`.
    Cancel,
    /// Any other command, which the dialog does not handle.
    Other,
}

/// The `BOOL` that an entry point returns, and the last error it leaves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PortOutcome {
    /// The `BOOL` the entry point returns.
    pub success: bool,
    /// The value that `SetLastError` sets, or `None` when the last error is left alone.
    pub last_error: Option<u32>,
}

/// What `AddPortUI` returns, and what it does with the port name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AddPortOutcome {
    /// The `BOOL` the entry point returns. It is the `XcvData` result, not the dialog result.
    pub success: bool,
    /// The value that `SetLastError` sets, or `None` when the last error is left alone.
    pub last_error: Option<u32>,
    /// The port name is returned to the caller through `ppPortName`.
    pub publish_name: bool,
    /// The already-exists notice is shown.
    pub already_exists_notice: bool,
}

/// Returns the port type of `name`, as Wine's `get_type_from_name` does.
///
/// `name` counts up to its first NUL. `file_probe` is called only when no port prefix matches. It reports whether the name can be opened or created as a file.
pub fn port_type_from_name(name: &[u16], file_probe: impl FnOnce() -> bool) -> u32 {
    if let Some(port_type) = port_prefix_type(wstr(name)) {
        port_type
    } else if file_probe() {
        PORT_IS_FILENAME
    } else {
        PORT_IS_UNKNOWN
    }
}

/// Returns the port type that a prefix or a fixed name gives, or `None` when the name needs the file probe.
fn port_prefix_type(name: &[u16]) -> Option<u32> {
    if has_prefix(name, b"LPT", true) {
        Some(PORT_IS_LPT)
    } else if has_prefix(name, b"COM", true) {
        Some(PORT_IS_COM)
    } else if equals(name, b"FILE:", true) {
        Some(PORT_IS_FILE)
    } else if name.first() == Some(&u16::from(b'/')) {
        Some(PORT_IS_UNIXNAME)
    } else if name.first() == Some(&u16::from(b'|')) {
        Some(PORT_IS_PIPE)
    } else if has_prefix(name, b"CUPS:", false) {
        Some(PORT_IS_CUPS)
    } else if has_prefix(name, b"LPR:", false) {
        Some(PORT_IS_LPR)
    } else {
        None
    }
}

/// Returns the name that `open_monitor_by_name` passes to `OpenPrinterW`: `prefix`, then `port` up to its NUL, then a NUL.
pub fn open_name(prefix: &str, port: Option<&[u16]>) -> Vec<u16> {
    let mut name: Vec<u16> = prefix.encode_utf16().collect();
    if let Some(port) = port {
        name.extend_from_slice(wstr(port));
    }
    name.push(0);
    name
}

/// Returns the short name that `dlg_configure_com` sends to the monitor: `port` without its last unit.
///
/// Wine drops the last unit whatever it is. `port` counts up to its first NUL, so an empty name has no last unit and gives `None`. Wine's own code would underflow on an empty name, but only names that start with `COM` reach it.
pub fn strip_port_colon(port: &[u16]) -> Option<Vec<u16>> {
    wstr(port).split_last().map(|(_, short)| short.to_vec())
}

/// Returns the timeout text that the LPT dialog sends, or `None` when the dialog shows the value again.
///
/// `value` and `translated` are the `GetDlgItemInt` result and its success flag. The value must be above 0 and below 1000000. The text is decimal, with a NUL, as sent to `ConfigureLPTPortCommandOK`.
pub fn lpt_retry_timeout_request(value: u32, translated: bool) -> Option<Vec<u16>> {
    if (1..LPT_RETRY_LIMIT).contains(&value) && translated {
        let mut text: Vec<u16> = value.to_string().encode_utf16().collect();
        text.push(0);
        Some(text)
    } else {
        None
    }
}

/// Reads the `PortIsValid` result in the add-port dialog.
pub fn port_name_check(res: bool, status: u32) -> PortNameCheck {
    match (res, status) {
        (true, ERROR_SUCCESS) => PortNameCheck::Valid,
        (true, ERROR_INVALID_NAME) => PortNameCheck::InvalidName,
        _ => PortNameCheck::Failed(status),
    }
}

/// Packs a `WM_COMMAND` `wparam` the way the `MAKEWPARAM` macro does.
pub fn make_wparam(low: u16, high: u16) -> usize {
    usize::from(low) | (usize::from(high) << 16)
}

/// Reads a `WM_COMMAND` `wparam` the way the dialogs do.
pub fn dialog_command(wparam: usize) -> DialogCommand {
    if wparam == make_wparam(IDOK, BN_CLICKED) {
        DialogCommand::Ok
    } else if wparam == make_wparam(IDCANCEL, BN_CLICKED) {
        DialogCommand::Cancel
    } else {
        DialogCommand::Other
    }
}

/// Returns what `AddPortUI` returns after its dialog.
///
/// `add` is `None` when the dialog was cancelled. Otherwise it is the `XcvData` `AddPort` result as `(res, status)`. `want_name` tells whether the caller asked for the port name.
pub fn add_port_outcome(add: Option<(bool, u32)>, want_name: bool) -> AddPortOutcome {
    match add {
        None => AddPortOutcome {
            success: false,
            last_error: Some(ERROR_CANCELLED),
            publish_name: false,
            already_exists_notice: false,
        },
        Some((res, status)) => AddPortOutcome {
            success: res,
            last_error: None,
            publish_name: res && status == ERROR_SUCCESS && want_name,
            already_exists_notice: res && status == ERROR_ALREADY_EXISTS,
        },
    }
}

/// Checks the `DeletePortUI` name before the monitor is opened. A missing name, or one whose first unit is NUL, is `ERROR_INVALID_PARAMETER`.
pub fn delete_port_precheck(port: Option<&[u16]>) -> Result<(), u32> {
    match port {
        Some(port) if !wstr(port).is_empty() => Ok(()),
        _ => Err(ERROR_INVALID_PARAMETER),
    }
}

/// Returns what `DeletePortUI` returns after the monitor was opened.
///
/// `xcv` is `None` when the monitor could not be opened for the port. Otherwise it is the `XcvData` `DeletePort` result as `(res, status)`.
pub fn delete_port_outcome(xcv: Option<(bool, u32)>) -> PortOutcome {
    match xcv {
        None => PortOutcome {
            success: false,
            last_error: Some(ERROR_UNKNOWN_PORT),
        },
        Some((false, _)) => PortOutcome {
            success: false,
            last_error: None,
        },
        Some((true, status)) => PortOutcome {
            success: status == ERROR_SUCCESS,
            last_error: (status != ERROR_SUCCESS).then_some(status),
        },
    }
}

/// Returns what `ConfigurePortUI` returns for a port of type `port_type`.
///
/// `ok` is the result of the COM or LPT path. For COM it covers both the dialog and the store. Other port types have nothing to configure, so they return `ERROR_CANCELLED` whatever `ok` is.
pub fn configure_port_outcome(port_type: u32, ok: bool) -> PortOutcome {
    match (port_type, ok) {
        (PORT_IS_COM, _) => PortOutcome {
            success: ok,
            last_error: None,
        },
        (PORT_IS_LPT, true) => PortOutcome {
            success: true,
            last_error: None,
        },
        _ => PortOutcome {
            success: false,
            last_error: Some(ERROR_CANCELLED),
        },
    }
}

/// Returns the units of `text` before the first NUL.
fn wstr(text: &[u16]) -> &[u16] {
    let len = text
        .iter()
        .position(|&unit| unit == 0)
        .unwrap_or(text.len());
    &text[..len]
}

/// Checks whether `name` starts with `prefix`. With `ignore_case`, both sides are upper-cased first.
fn has_prefix(name: &[u16], prefix: &[u8], ignore_case: bool) -> bool {
    name.len() >= prefix.len()
        && name.iter().zip(prefix).all(|(&unit, &want)| {
            if ignore_case {
                fold_upper(unit) == u32::from(want)
            } else {
                unit == u16::from(want)
            }
        })
}

/// Checks whether `name` is exactly `text`, with the same case rule as `has_prefix`.
fn equals(name: &[u16], text: &[u8], ignore_case: bool) -> bool {
    name.len() == text.len() && has_prefix(name, text, ignore_case)
}

/// Upper-cases one UTF-16 unit for the name comparisons.
///
/// ASCII is mapped directly. Other units use the Unicode uppercase mapping when it is one character. A surrogate, or a character whose uppercase is several characters (such as `ß`), stays as it is.
fn fold_upper(unit: u16) -> u32 {
    if unit < 0x80 {
        return u32::from((unit as u8).to_ascii_uppercase());
    }
    let Some(ch) = char::from_u32(u32::from(unit)) else {
        return u32::from(unit);
    };
    let mut upper = ch.to_uppercase();
    match (upper.next(), upper.next()) {
        (Some(single), None) => u32::from(single),
        _ => u32::from(unit),
    }
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod tests {
    use super::*;

    fn w(text: &str) -> Vec<u16> {
        text.encode_utf16().collect()
    }

    fn port_type(name: &str, probe: bool) -> u32 {
        port_type_from_name(&w(name), || probe)
    }

    #[test]
    fn wstr_stops_at_first_nul() {
        assert_eq!(wstr(&[65, 0, 66]), &[65][..]);
        assert_eq!(wstr(&[65, 66]), &[65, 66][..]);
        assert_eq!(wstr(&[]), &[] as &[u16]);
    }

    #[test]
    fn fold_upper_maps_ascii_and_unicode() {
        assert_eq!(fold_upper(u16::from(b'a')), u32::from(b'A'));
        assert_eq!(fold_upper(u16::from(b'Z')), u32::from(b'Z'));
        assert_eq!(fold_upper(u16::from(b'1')), u32::from(b'1'));
        assert_eq!(fold_upper(0xE9), 0xC9);
        assert_eq!(fold_upper(0xDF), 0xDF);
        assert_eq!(fold_upper(0xD800), 0xD800);
        assert_eq!(fold_upper(0x131), u32::from(b'I'));
        assert_eq!(fold_upper(0x17F), u32::from(b'S'));
    }

    #[test]
    fn prefix_and_equality_rules() {
        assert!(has_prefix(&w("lpt9"), b"LPT", true));
        assert!(!has_prefix(&w("LP"), b"LPT", true));
        assert!(!has_prefix(&w("LPX"), b"LPT", true));
        assert!(has_prefix(&w("CUPS:q"), b"CUPS:", false));
        assert!(!has_prefix(&w("cups:q"), b"CUPS:", false));
        assert!(equals(&w("file:"), b"FILE:", true));
        assert!(!equals(&w("FILE"), b"FILE:", true));
        assert!(!equals(&w("FILE:x"), b"FILE:", true));
    }

    #[test]
    fn port_type_from_name_edges() {
        assert_eq!(port_type("LPT", false), PORT_IS_LPT);
        assert_eq!(port_type("Com3:", false), PORT_IS_COM);
        assert_eq!(port_type("COM1\0LPT", false), PORT_IS_COM);
        assert_eq!(port_type("FILE:", false), PORT_IS_FILE);
        assert_eq!(port_type("|cat", false), PORT_IS_PIPE);
        assert_eq!(port_type("/", false), PORT_IS_UNIXNAME);
        assert_eq!(port_type("LPR:q", false), PORT_IS_LPR);
        assert_eq!(port_type("lpr:q", true), PORT_IS_FILENAME);
        assert_eq!(port_type("LP", false), PORT_IS_UNKNOWN);
        assert_eq!(port_type("LP", true), PORT_IS_FILENAME);
        assert_eq!(port_type("FILE:x", true), PORT_IS_FILENAME);
        assert_eq!(port_type("FILE", false), PORT_IS_UNKNOWN);
        assert_eq!(port_type("", false), PORT_IS_UNKNOWN);
        assert_eq!(port_type("\0LPT1", true), PORT_IS_FILENAME);
    }

    #[test]
    fn open_name_builds_nul_terminated_name() {
        assert_eq!(
            open_name(",XcvPort ", Some(&w("LPT1:"))),
            w(",XcvPort LPT1:\0")
        );
        assert_eq!(open_name(",XcvMonitor ", None), w(",XcvMonitor \0"));
        assert_eq!(
            open_name(",XcvPort ", Some(&w("COM1\0junk"))),
            w(",XcvPort COM1\0")
        );
    }

    #[test]
    fn strip_port_colon_drops_last_unit() {
        assert_eq!(strip_port_colon(&w("COM1:")), Some(w("COM1")));
        assert_eq!(strip_port_colon(&w("X")), Some(Vec::new()));
        assert_eq!(strip_port_colon(&w("COM1\0zz")), Some(w("COM")));
        assert_eq!(strip_port_colon(&[]), None);
        assert_eq!(strip_port_colon(&[0, 65]), None);
    }

    #[test]
    fn lpt_retry_limits() {
        assert_eq!(
            lpt_retry_timeout_request(999_999, true),
            Some(w("999999\0"))
        );
        assert_eq!(lpt_retry_timeout_request(0, true), None);
        assert_eq!(lpt_retry_timeout_request(u32::MAX, true), None);
        assert_eq!(lpt_retry_timeout_request(45, false), None);
    }

    #[test]
    fn port_name_check_reads_result() {
        assert_eq!(port_name_check(true, ERROR_SUCCESS), PortNameCheck::Valid);
        assert_eq!(
            port_name_check(true, ERROR_INVALID_NAME),
            PortNameCheck::InvalidName
        );
        assert_eq!(
            port_name_check(true, ERROR_ALREADY_EXISTS),
            PortNameCheck::Failed(183)
        );
        assert_eq!(
            port_name_check(false, ERROR_INVALID_NAME),
            PortNameCheck::Failed(123)
        );
        assert_eq!(
            port_name_check(false, ERROR_SUCCESS),
            PortNameCheck::Failed(0)
        );
    }

    #[test]
    fn dialog_command_reads_wparam() {
        assert_eq!(make_wparam(IDOK, BN_CLICKED), 1);
        assert_eq!(make_wparam(IDCANCEL, BN_CLICKED), 2);
        assert_eq!(
            dialog_command(make_wparam(IDOK, BN_CLICKED)),
            DialogCommand::Ok
        );
        assert_eq!(
            dialog_command(make_wparam(IDCANCEL, BN_CLICKED)),
            DialogCommand::Cancel
        );
        assert_eq!(dialog_command(make_wparam(IDOK, 1)), DialogCommand::Other);
        assert_eq!(
            dialog_command(make_wparam(7, BN_CLICKED)),
            DialogCommand::Other
        );
    }

    #[test]
    fn add_port_outcome_cases() {
        let cancelled = add_port_outcome(None, true);
        assert_eq!(cancelled.last_error, Some(ERROR_CANCELLED));
        assert!(!cancelled.success);

        let no_name = add_port_outcome(Some((true, ERROR_SUCCESS)), false);
        assert!(no_name.success && !no_name.publish_name && no_name.last_error.is_none());

        let failed = add_port_outcome(Some((false, ERROR_ALREADY_EXISTS)), true);
        assert!(!failed.success && !failed.already_exists_notice && !failed.publish_name);

        let other = add_port_outcome(Some((true, ERROR_INVALID_NAME)), true);
        assert!(other.success && !other.publish_name && !other.already_exists_notice);
    }

    #[test]
    fn delete_port_precheck_rejects_missing_names() {
        assert_eq!(delete_port_precheck(None), Err(ERROR_INVALID_PARAMETER));
        assert_eq!(
            delete_port_precheck(Some(&[0])),
            Err(ERROR_INVALID_PARAMETER)
        );
        assert_eq!(delete_port_precheck(Some(&w("COM3:"))), Ok(()));
    }

    #[test]
    fn delete_port_outcome_cases() {
        assert_eq!(
            delete_port_outcome(None),
            PortOutcome {
                success: false,
                last_error: Some(ERROR_UNKNOWN_PORT)
            }
        );
        assert_eq!(
            delete_port_outcome(Some((false, 50))),
            PortOutcome {
                success: false,
                last_error: None
            }
        );
        assert_eq!(
            delete_port_outcome(Some((true, ERROR_SUCCESS))),
            PortOutcome {
                success: true,
                last_error: None
            }
        );
        assert_eq!(
            delete_port_outcome(Some((true, 50))),
            PortOutcome {
                success: false,
                last_error: Some(50)
            }
        );
    }

    #[test]
    fn configure_port_outcome_cases() {
        let com_ok = configure_port_outcome(PORT_IS_COM, true);
        assert!(com_ok.success && com_ok.last_error.is_none());
        let com_failed = configure_port_outcome(PORT_IS_COM, false);
        assert!(!com_failed.success && com_failed.last_error.is_none());
        let lpt_ok = configure_port_outcome(PORT_IS_LPT, true);
        assert!(lpt_ok.success && lpt_ok.last_error.is_none());
        let lpt_cancelled = configure_port_outcome(PORT_IS_LPT, false);
        assert!(!lpt_cancelled.success && lpt_cancelled.last_error == Some(ERROR_CANCELLED));
        let nothing = configure_port_outcome(PORT_IS_FILE, true);
        assert!(!nothing.success && nothing.last_error == Some(ERROR_CANCELLED));
    }
}
