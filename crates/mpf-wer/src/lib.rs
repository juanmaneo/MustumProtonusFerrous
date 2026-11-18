//! Headless subset of Wine's `dlls/wer/main.c` error-reporting API.
//!
//! The ported functions return the HRESULT or value that Wine's C body
//! computes from its arguments. The registry, report-handle table, dialogs and
//! upload are not modelled.
//!
//! Ported (partial, the side effects are skipped):
//! - `WerAddExcludedApplication`: `excluded_application_value_name`; the registry write is skipped.
//! - `WerRemoveExcludedApplication`: `excluded_application_value_name`; the registry delete is skipped.
//! - `WerReportCreate`: `report_event_type`; the report allocation, handle output and global report list are skipped.
//! - `WerReportSubmit`: `report_submit`; the consent prompt and upload are skipped, and the result is always `WerDisabled`.
//!
//! Skipped:
//! - `WerReportCloseHandle`: finds the handle in a process-wide list keyed by raw pointers.
//! - `WerReportAddDump`: `FIXME` stub returning `E_NOTIMPL`; no logic to port.
//! - `WerReportAddFile`: `FIXME` stub returning `S_OK`; no logic to port.
//! - `WerReportSetParameter`: `FIXME` stub returning `S_OK`; no logic to port.
//! - `WerReportSetUIOption`: `FIXME` stub returning `E_NOTIMPL`; no logic to port.
//! - The 68 other `wer.spec` entries (`WerSysprep*`, `WerUnattendedSetup`, `Werp*`): `@ stub` in the spec, not implemented in `main.c`.

/// Success HRESULT.
pub const S_OK: i32 = 0;
/// `E_INVALIDARG`, returned for null or malformed arguments.
pub const E_INVALIDARG: i32 = 0x8007_0057u32 as i32;
/// `WerReportInvalid`, the first `WER_REPORT_TYPE` value that is not a report type.
pub const WER_REPORT_INVALID: u32 = 5;
/// `WerDisabled`, the `WER_SUBMIT_RESULT` that `WerReportSubmit` writes.
pub const WER_DISABLED: u32 = 5;

const BACKSLASH: u16 = b'\\' as u16;

/// The code units of a NUL-terminated UTF-16 string, without the terminator.
fn wstr(text: &[u16]) -> &[u16] {
    match text.iter().position(|&c| c == 0) {
        Some(end) => &text[..end],
        None => text,
    }
}

/// Returns the value name that `WerAddExcludedApplication` and `WerRemoveExcludedApplication` use: the text after the last backslash of `exe_name`, or the whole name when it has none.
///
/// Returns `E_INVALIDARG` when the name is null or empty, or when nothing follows its last backslash.
pub fn excluded_application_value_name(exe_name: Option<&[u16]>) -> Result<&[u16], i32> {
    let name = wstr(exe_name.ok_or(E_INVALIDARG)?);
    let value = match name.iter().rposition(|&c| c == BACKSLASH) {
        Some(index) => &name[index + 1..],
        None => name,
    };
    if value.is_empty() {
        Err(E_INVALIDARG)
    } else {
        Ok(value)
    }
}

/// Checks the arguments of `WerReportCreate` and returns the event type the report stores, NUL-terminated.
///
/// `has_phandle` says whether the caller passed an output handle slot. Report types of `WER_REPORT_INVALID` or more are rejected. The type is unsigned, as the enum is in a GCC build of Wine, so negative values are rejected too.
pub fn report_event_type(
    eventtype: Option<&[u16]>,
    reporttype: u32,
    has_phandle: bool,
) -> Result<Vec<u16>, i32> {
    let name = wstr(eventtype.ok_or(E_INVALIDARG)?);
    if name.is_empty() || !has_phandle || reporttype >= WER_REPORT_INVALID {
        return Err(E_INVALIDARG);
    }
    let mut stored = name.to_vec();
    stored.push(0);
    Ok(stored)
}

/// Writes `WerDisabled` to the result slot when the caller passed one, and fails otherwise.
pub fn report_submit(presult: Option<&mut u32>) -> i32 {
    match presult {
        Some(result) => {
            *result = WER_DISABLED;
            S_OK
        }
        None => E_INVALIDARG,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn w(text: &str) -> Vec<u16> {
        text.encode_utf16().collect()
    }

    fn text(units: &[u16]) -> String {
        String::from_utf16_lossy(units)
    }

    fn stored_text(result: Result<Vec<u16>, i32>) -> Result<String, i32> {
        result.map(|stored| {
            assert_eq!(stored.last(), Some(&0));
            text(&stored[..stored.len() - 1])
        })
    }

    #[test]
    fn excluded_application_value_names_match_wine_oracle() {
        // WerAddExcludedApplication and WerRemoveExcludedApplication share this check.
        let cases: [(Option<&str>, Result<&str, i32>); 9] = [
            (Some("game.exe"), Ok("game.exe")),
            (Some("C:\\Games\\Foo\\game.exe"), Ok("game.exe")),
            (Some("C:\\Games\\Foo"), Ok("Foo")),
            (Some("C:\\Games\\Foo\\"), Err(E_INVALIDARG)),
            (Some(""), Err(E_INVALIDARG)),
            (None, Err(E_INVALIDARG)),
            (Some("\\"), Err(E_INVALIDARG)),
            (Some("game.exe\0junk"), Ok("game.exe")),
            (Some("tool"), Ok("tool")),
        ];
        for (exe, expected) in cases {
            let buffer = exe.map(w);
            let actual = excluded_application_value_name(buffer.as_deref()).map(text);
            assert_eq!(actual, expected.map(String::from), "{exe:?}");
        }
    }

    #[test]
    fn report_event_types_match_wine_oracle() {
        let cases = [
            (Some("APPCRASH"), 0, true, Ok("APPCRASH")),
            (Some("Crash32"), 2, true, Ok("Crash32")),
            (Some("APPCRASH"), 4, true, Ok("APPCRASH")),
            (Some("APPCRASH"), 5, true, Err(E_INVALIDARG)),
            (Some("APPCRASH"), u32::MAX, true, Err(E_INVALIDARG)),
            (Some("APPCRASH"), 0, false, Err(E_INVALIDARG)),
            (Some(""), 0, true, Err(E_INVALIDARG)),
            (None, 1, true, Err(E_INVALIDARG)),
            (Some("APPCRASH\0junk"), 3, true, Ok("APPCRASH")),
        ];
        for (event, reporttype, has_phandle, expected) in cases {
            let buffer = event.map(w);
            let actual = report_event_type(buffer.as_deref(), reporttype, has_phandle);
            assert_eq!(
                stored_text(actual),
                expected.map(String::from),
                "{event:?} {reporttype} {has_phandle}"
            );
        }
    }

    #[test]
    fn submit_result_matches_wine_oracle() {
        for (has_slot, expected_hr, expected_value) in
            [(false, E_INVALIDARG, 0), (true, S_OK, WER_DISABLED)]
        {
            let mut result = 0;
            let hr = report_submit(has_slot.then_some(&mut result));
            assert_eq!((hr, result), (expected_hr, expected_value));
        }
    }
}
