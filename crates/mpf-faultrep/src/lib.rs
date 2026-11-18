//! Headless subset of Wine's `dlls/faultrep/faultrep.c` fault-report API.
//!
//! The ported function returns the value name and the last error that Wine's C
//! body computes from its argument. The registry write, the ANSI wrapper and
//! the report pipeline are not modelled.
//!
//! Ported (partial, the side effects are skipped):
//! - `AddERExcludedApplicationW`: `excluded_application_value_name`; the registry write is skipped, and its key and data are `EXCLUSION_LIST_KEY` and `EXCLUSION_VALUE_DATA`.
//!
//! Skipped:
//! - `AddERExcludedApplicationA`: converts its name with `MultiByteToWideChar(CP_ACP, ...)`, a host code-page conversion, before calling the W export. Its own check tests only the converter's result, so porting it would mean inventing the conversion.
//! - `ReportFault`: `FIXME` stub returning `frrvOk`; no logic to port.
//! - The 11 other `faultrep.spec` entries (`CreateMinidump*`, `ReportEREvent*`, `ReportFaultDWM`, `ReportFaultFromQueue`, `ReportFaultToQueue`, `ReportHang`, `ReportKernelFault*`): `@ stub` in the spec, not implemented in `faultrep.c`.

/// Win32 `ERROR_INVALID_PARAMETER`, the last error that a failed `AddERExcludedApplicationW` sets.
pub const ERROR_INVALID_PARAMETER: u32 = 87;
/// The key under `HKEY_LOCAL_MACHINE` that `AddERExcludedApplicationW` writes its value to.
pub const EXCLUSION_LIST_KEY: &str = "Software\\Microsoft\\PCHealth\\ErrorReporting\\ExclusionList";
/// The `REG_DWORD` data stored under each excluded application's value name.
pub const EXCLUSION_VALUE_DATA: u32 = 1;

const BACKSLASH: u16 = b'\\' as u16;

/// The code units of a NUL-terminated UTF-16 string, without the terminator.
fn wstr(text: &[u16]) -> &[u16] {
    match text.iter().position(|&c| c == 0) {
        Some(end) => &text[..end],
        None => text,
    }
}

/// Returns the value name that `AddERExcludedApplicationW` registers: the text after the last backslash of `app_file_name`, or the whole name when it has none.
///
/// Returns `ERROR_INVALID_PARAMETER` when the name is null or empty, or when nothing follows its last backslash. A null name has no oracle row, because the C body passes it to `wcsrchr`.
pub fn excluded_application_value_name(app_file_name: Option<&[u16]>) -> Result<&[u16], u32> {
    let name = wstr(app_file_name.ok_or(ERROR_INVALID_PARAMETER)?);
    let value = match name.iter().rposition(|&c| c == BACKSLASH) {
        Some(index) => &name[index + 1..],
        None => name,
    };
    if value.is_empty() {
        Err(ERROR_INVALID_PARAMETER)
    } else {
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn w(text: &str) -> Vec<u16> {
        text.encode_utf16().collect()
    }

    #[test]
    fn value_names_match_wine_oracle() {
        // Rows W01 to W22 of target/canon/faultrep/out_faultrep.txt, in input order.
        let cases: [(&str, Result<&str, u32>); 22] = [
            ("game.exe", Ok("game.exe")),
            ("C:\\Games\\Foo\\game.exe", Ok("game.exe")),
            ("C:\\Games\\Foo", Ok("Foo")),
            ("C:\\Games\\Foo\\", Err(ERROR_INVALID_PARAMETER)),
            ("", Err(ERROR_INVALID_PARAMETER)),
            ("\\", Err(ERROR_INVALID_PARAMETER)),
            ("\\\\server\\share\\app.exe", Ok("app.exe")),
            ("tool", Ok("tool")),
            ("game.exe\0junk", Ok("game.exe")),
            ("C:\\a\0\\b", Ok("a")),
            ("\0", Err(ERROR_INVALID_PARAMETER)),
            ("\0\\app.exe", Err(ERROR_INVALID_PARAMETER)),
            ("C:\\\u{e9}\\app\u{e9}.exe", Ok("app\u{e9}.exe")),
            ("a\\b\\c\\d.exe", Ok("d.exe")),
            ("C:/Games/app.exe", Ok("C:/Games/app.exe")),
            ("\\\\", Err(ERROR_INVALID_PARAMETER)),
            ("app.exe\\", Err(ERROR_INVALID_PARAMETER)),
            ("C:\\Program Files\\My App\\app.exe", Ok("app.exe")),
            ("C:\\\u{1F600}.exe", Ok("\u{1F600}.exe")),
            ("game\u{FFFF}", Ok("game\u{FFFF}")),
            ("C:\\x\\\0", Err(ERROR_INVALID_PARAMETER)),
            ("\\app.exe", Ok("app.exe")),
        ];
        for (index, (name, expected)) in cases.into_iter().enumerate() {
            let buffer = w(name);
            let actual =
                excluded_application_value_name(Some(buffer.as_slice())).map(<[u16]>::to_vec);
            assert_eq!(actual, expected.map(w), "W{:02} {name:?}", index + 1);
        }
    }

    #[test]
    fn null_name_is_rejected() {
        assert_eq!(
            excluded_application_value_name(None),
            Err(ERROR_INVALID_PARAMETER)
        );
    }

    #[test]
    fn registry_target_matches_wine_oracle() {
        // The W rows of the oracle record this key, REG_DWORD type 4, size 4 and data 1.
        assert_eq!(
            EXCLUSION_LIST_KEY,
            "Software\\Microsoft\\PCHealth\\ErrorReporting\\ExclusionList"
        );
        assert_eq!(EXCLUSION_VALUE_DATA, 1);
    }
}
