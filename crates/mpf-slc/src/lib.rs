//! Port of the portable logic in Wine's `dlls/slc/slc.c`.
//!
//! `slc.spec` exports 41 functions. Two have bodies in `slc.c`, and only
//! `SLGetWindowsInformationDWORD` has logic to port: its argument checks and
//! the mapping from the licence query's result to an HRESULT. The query itself
//! (`NtQueryLicenseValue`) belongs to the licensing service, so the caller
//! supplies it as a closure. This crate has no licence store, activation or
//! handles.
//!
//! Ported (1 of 41):
//! - `SLGetWindowsInformationDWORD`: [`get_windows_information_dword`]; the licence query is the `query_license_value` argument.
//!
//! Skipped (40 of 41):
//! - `SLGetWindowsInformation`: `FIXME` stub returning `SL_E_RIGHT_NOT_GRANTED`; no logic to port.
//! - `SLClose`, `SLOpen`, `SLGetLicensingStatusInformation`: forwarded to `sppc` in the spec; not implemented here.
//! - The 36 other `slc.spec` entries (`SLp*` and `SL*`): `@ stub` in the spec, with no body in `slc.c`.
//!
//! Divergences from C:
//! - The value name is read up to its first NUL, or to the end of the slice when there is none. A zero-length slice counts as an empty name. C would read past the buffer in both cases.

/// Success HRESULT.
pub const S_OK: i32 = 0;
/// `E_FAIL`, returned when the licence query fails with a status that is not named below.
pub const E_FAIL: i32 = 0x8000_4005u32 as i32;
/// `E_INVALIDARG`, returned when the value name or the value slot is null.
pub const E_INVALIDARG: i32 = 0x8007_0057u32 as i32;
/// `SL_E_VALUE_NOT_FOUND`, returned when the licence query reports the value does not exist.
pub const SL_E_VALUE_NOT_FOUND: i32 = 0xC004_F012u32 as i32;
/// `SL_E_RIGHT_NOT_GRANTED`, returned for an empty value name.
pub const SL_E_RIGHT_NOT_GRANTED: i32 = 0xC004_F013u32 as i32;
/// `SL_E_DATATYPE_MISMATCHED`, returned when the value is not a `REG_DWORD`.
pub const SL_E_DATATYPE_MISMATCHED: i32 = 0xC004_F01Eu32 as i32;

const STATUS_SUCCESS: i32 = 0;
const STATUS_BUFFER_TOO_SMALL: i32 = 0xC000_0023u32 as i32;
const STATUS_OBJECT_NAME_NOT_FOUND: i32 = 0xC000_0034u32 as i32;
const REG_DWORD: u32 = 4;

/// The code units of a NUL-terminated UTF-16 string, without the terminator.
fn wstr(text: &[u16]) -> &[u16] {
    match text.iter().position(|&c| c == 0) {
        Some(end) => &text[..end],
        None => text,
    }
}

/// Ports `SLGetWindowsInformationDWORD`.
///
/// `value_name` is a NUL-terminated UTF-16 string. `value` is the caller's output slot.
/// `query_license_value` stands in for `NtQueryLicenseValue`: it receives the name and
/// the slot, may store the value in the slot, and returns the NTSTATUS and the value
/// type. It is called only after both arguments pass the checks, so the slot is written
/// only by the query and never by this function.
///
/// Returns the HRESULT that the C body returns.
pub fn get_windows_information_dword(
    value_name: Option<&[u16]>,
    value: Option<&mut u32>,
    query_license_value: impl FnOnce(&[u16], &mut u32) -> (i32, u32),
) -> i32 {
    let (Some(name), Some(value)) = (value_name, value) else {
        return E_INVALIDARG;
    };
    let name = wstr(name);
    if name.is_empty() {
        return SL_E_RIGHT_NOT_GRANTED;
    }
    let (status, value_type) = query_license_value(name, value);
    if status == STATUS_OBJECT_NAME_NOT_FOUND {
        return SL_E_VALUE_NOT_FOUND;
    }
    if (status == STATUS_SUCCESS || status == STATUS_BUFFER_TOO_SMALL) && value_type != REG_DWORD {
        return SL_E_DATATYPE_MISMATCHED;
    }
    if status == STATUS_SUCCESS {
        S_OK
    } else {
        E_FAIL
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    /// One vector of `target/canon/slc/vectors.txt`, with the output of the Wine oracle.
    struct OracleCase {
        name: Option<&'static str>,
        has_out: bool,
        status: i32,
        value_type: u32,
        writes: Option<u32>,
        hr: i32,
        out: u32,
        queried: Option<usize>,
    }

    /// Vectors from the Wine oracle (`target/canon/slc/build.sh`): the `slc.c` body of
    /// `SLGetWindowsInformationDWORD`, with a scripted `NtQueryLicenseValue`. `out` is the
    /// slot after the call, `0xdead_beef` when the call left it alone.
    const ORACLE: [OracleCase; 22] = [
        OracleCase {
            name: None,
            has_out: true,
            status: STATUS_SUCCESS,
            value_type: 0,
            writes: None,
            hr: E_INVALIDARG,
            out: 0xdead_beef,
            queried: None,
        },
        OracleCase {
            name: Some("Nonexistent-License-Value"),
            has_out: false,
            status: STATUS_OBJECT_NAME_NOT_FOUND,
            value_type: 0,
            writes: None,
            hr: E_INVALIDARG,
            out: 0xdead_beef,
            queried: None,
        },
        OracleCase {
            name: None,
            has_out: false,
            status: STATUS_SUCCESS,
            value_type: 0,
            writes: None,
            hr: E_INVALIDARG,
            out: 0xdead_beef,
            queried: None,
        },
        OracleCase {
            name: Some(""),
            has_out: true,
            status: STATUS_SUCCESS,
            value_type: 4,
            writes: Some(0x0000_0007),
            hr: SL_E_RIGHT_NOT_GRANTED,
            out: 0xdead_beef,
            queried: None,
        },
        OracleCase {
            name: Some("\0abc"),
            has_out: true,
            status: STATUS_SUCCESS,
            value_type: 4,
            writes: Some(0x0000_0007),
            hr: SL_E_RIGHT_NOT_GRANTED,
            out: 0xdead_beef,
            queried: None,
        },
        OracleCase {
            name: Some("Nonexistent-License-Value"),
            has_out: true,
            status: STATUS_OBJECT_NAME_NOT_FOUND,
            value_type: 4,
            writes: None,
            hr: SL_E_VALUE_NOT_FOUND,
            out: 0xdead_beef,
            queried: Some(25),
        },
        OracleCase {
            name: Some("Nonexistent-License-Value"),
            has_out: true,
            status: STATUS_OBJECT_NAME_NOT_FOUND,
            value_type: 1,
            writes: None,
            hr: SL_E_VALUE_NOT_FOUND,
            out: 0xdead_beef,
            queried: Some(25),
        },
        OracleCase {
            name: Some("Kernel-MUI-Language-Allowed"),
            has_out: true,
            status: STATUS_SUCCESS,
            value_type: 1,
            writes: None,
            hr: SL_E_DATATYPE_MISMATCHED,
            out: 0xdead_beef,
            queried: Some(27),
        },
        OracleCase {
            name: Some("Kernel-MUI-Language-Allowed"),
            has_out: true,
            status: STATUS_SUCCESS,
            value_type: 0,
            writes: Some(0x0000_0009),
            hr: SL_E_DATATYPE_MISMATCHED,
            out: 0x0000_0009,
            queried: Some(27),
        },
        OracleCase {
            name: Some("Kernel-MUI-Language-Allowed"),
            has_out: true,
            status: STATUS_SUCCESS,
            value_type: 5,
            writes: None,
            hr: SL_E_DATATYPE_MISMATCHED,
            out: 0xdead_beef,
            queried: Some(27),
        },
        OracleCase {
            name: Some("Kernel-MUI-Number-Allowed"),
            has_out: true,
            status: STATUS_SUCCESS,
            value_type: 4,
            writes: Some(0x0000_002a),
            hr: S_OK,
            out: 0x0000_002a,
            queried: Some(25),
        },
        OracleCase {
            name: Some("Kernel-MUI-Number-Allowed"),
            has_out: true,
            status: STATUS_SUCCESS,
            value_type: 4,
            writes: Some(0xffff_ffff),
            hr: S_OK,
            out: 0xffff_ffff,
            queried: Some(25),
        },
        OracleCase {
            name: Some("Kernel-MUI-Number-Allowed"),
            has_out: true,
            status: STATUS_SUCCESS,
            value_type: 4,
            writes: Some(0x0000_0000),
            hr: S_OK,
            out: 0x0000_0000,
            queried: Some(25),
        },
        OracleCase {
            name: Some("Kernel-MUI-Number-Allowed\0junk"),
            has_out: true,
            status: STATUS_SUCCESS,
            value_type: 4,
            writes: Some(0x0000_0007),
            hr: S_OK,
            out: 0x0000_0007,
            queried: Some(25),
        },
        OracleCase {
            name: Some("x"),
            has_out: true,
            status: STATUS_SUCCESS,
            value_type: 4,
            writes: None,
            hr: S_OK,
            out: 0xdead_beef,
            queried: Some(1),
        },
        OracleCase {
            name: Some("Nonexistent-License-Value"),
            has_out: true,
            status: STATUS_BUFFER_TOO_SMALL,
            value_type: 4,
            writes: None,
            hr: E_FAIL,
            out: 0xdead_beef,
            queried: Some(25),
        },
        OracleCase {
            name: Some("Nonexistent-License-Value"),
            has_out: true,
            status: STATUS_BUFFER_TOO_SMALL,
            value_type: 1,
            writes: None,
            hr: SL_E_DATATYPE_MISMATCHED,
            out: 0xdead_beef,
            queried: Some(25),
        },
        OracleCase {
            name: Some("Nonexistent-License-Value"),
            has_out: true,
            status: STATUS_BUFFER_TOO_SMALL,
            value_type: 4,
            writes: Some(0x0000_0009),
            hr: E_FAIL,
            out: 0x0000_0009,
            queried: Some(25),
        },
        OracleCase {
            name: Some("Nonexistent-License-Value"),
            has_out: true,
            status: 0xC000_0001u32 as i32,
            value_type: 4,
            writes: None,
            hr: E_FAIL,
            out: 0xdead_beef,
            queried: Some(25),
        },
        OracleCase {
            name: Some("Nonexistent-License-Value"),
            has_out: true,
            status: 0xC000_0001u32 as i32,
            value_type: 1,
            writes: None,
            hr: E_FAIL,
            out: 0xdead_beef,
            queried: Some(25),
        },
        OracleCase {
            name: Some("Nonexistent-License-Value"),
            has_out: true,
            status: 0x0000_0103u32 as i32,
            value_type: 4,
            writes: None,
            hr: E_FAIL,
            out: 0xdead_beef,
            queried: Some(25),
        },
        OracleCase {
            name: Some("Nonexistent-License-Value"),
            has_out: true,
            status: 0x8000_0005u32 as i32,
            value_type: 1,
            writes: Some(0x0000_0005),
            hr: E_FAIL,
            out: 0x0000_0005,
            queried: Some(25),
        },
    ];

    /// A scripted `NtQueryLicenseValue`: records the name length it saw and stores `writes` in the slot.
    fn scripted<'a>(
        seen: &'a Cell<Option<usize>>,
        status: i32,
        value_type: u32,
        writes: Option<u32>,
    ) -> impl FnOnce(&[u16], &mut u32) -> (i32, u32) + 'a {
        move |name, value| {
            seen.set(Some(name.len()));
            if let Some(stored) = writes {
                *value = stored;
            }
            (status, value_type)
        }
    }

    /// Calls the port with a scripted query. Returns the HRESULT, the slot after the call,
    /// and the length of the name the query saw.
    fn call(
        name: Option<&[u16]>,
        has_out: bool,
        status: i32,
        value_type: u32,
        writes: Option<u32>,
    ) -> (i32, u32, Option<usize>) {
        let seen = Cell::new(None);
        let mut out = 0xdead_beef;
        let slot = has_out.then_some(&mut out);
        let query = scripted(&seen, status, value_type, writes);
        let hr = get_windows_information_dword(name, slot, query);
        (hr, out, seen.get())
    }

    fn w(text: &str) -> Vec<u16> {
        text.encode_utf16().collect()
    }

    #[test]
    fn matches_wine_oracle_vectors() {
        for (index, case) in ORACLE.iter().enumerate() {
            let name = case.name.map(w);
            let actual = call(
                name.as_deref(),
                case.has_out,
                case.status,
                case.value_type,
                case.writes,
            );
            assert_eq!(actual, (case.hr, case.out, case.queried), "vector {index}");
        }
    }

    #[test]
    fn name_without_nul_is_read_to_its_end() {
        let name = [u16::from(b'A'), u16::from(b'B')];
        let actual = call(Some(&name[..]), true, STATUS_SUCCESS, REG_DWORD, Some(7));
        assert_eq!(actual, (S_OK, 7, Some(2)));
    }

    #[test]
    fn empty_slice_is_an_empty_name() {
        let empty: &[u16] = &[];
        let actual = call(Some(empty), true, STATUS_SUCCESS, REG_DWORD, Some(7));
        assert_eq!(actual, (SL_E_RIGHT_NOT_GRANTED, 0xdead_beef, None));
    }
}
