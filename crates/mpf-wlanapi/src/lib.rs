//! wlanapi subset for MPF: the argument checks of `WlanOpenHandle` and
//! `WlanCloseHandle`, and the buffer behind `WlanAllocateMemory`.
//!
//! Reference Wine's `dlls/wlanapi/main.c` has bodies for 12 of the 39 entries in
//! `wlanapi.spec`; the other 27 are spec-only `stub` entries. Three of the 12 are
//! ported here, two of them only in part. The port takes no WLAN service, handle
//! table, adapter, or registry input.
//!
//! Ported (table tests against a verbatim Wine C oracle, see the tests):
//! - `WlanAllocateMemory` as `allocate_memory`: the zero-size check, then a buffer
//!   of the requested length. The buffer is zero-filled; C's `HeapAlloc` is not.
//!   The owned `Vec<u8>` replaces `WlanFreeMemory`, so that export is not ported.
//! - `WlanOpenHandle`, argument checks only, as `check_open_handle`. They run in
//!   C's order: reserved and null out-pointers, then the client version (1 or 2).
//!   `Ok(())` means C would go on to create a handle, which the port does not
//!   model. C's `ERROR_REMOTE_SESSION_LIMIT_EXCEEDED` on a full table is not
//!   produced. On success C stores `client_version` as the negotiated version;
//!   the caller keeps that.
//! - `WlanCloseHandle`, argument checks only, as `check_close_handle`. A null
//!   handle or a reserved argument fails with `ERROR_INVALID_PARAMETER`. `Ok(())`
//!   means C would look the handle up in its table, which the port does not
//!   model. That lookup returns `ERROR_INVALID_HANDLE` (6) unless the handle is open.
//!
//! Divergences from the C bodies:
//! - Pointers are passed as booleans that say whether they are non-null. The
//!   caller holds the values.
//! - The `FIXME` and `TRACE` logging is not modelled.
//!
//! Skipped (9 of the 12 bodies, and the 27 stubs):
//! - `WlanEnumInterfaces`: a `FIXME` "semi-stub" that looks up a handle and
//!   returns an empty list. Porting it would invent the list behaviour.
//! - `WlanFreeMemory`: `HeapFree` on a raw pointer. The owned `Vec` in
//!   `allocate_memory` makes it unnecessary.
//! - `WlanScan`, `WlanRegisterNotification`, `WlanGetAvailableNetworkList`,
//!   `WlanQueryInterface`, `WlanHostedNetworkQueryProperty`,
//!   `WlanHostedNetworkQuerySecondaryKey`, `WlanHostedNetworkQueryStatus`: `FIXME`
//!   stubs that return `ERROR_CALL_NOT_IMPLEMENTED`. Not ported, since that would
//!   turn stubs into implementations.
//! - The 27 `@ stub` entries (`WlanConnect`, `WlanGetProfile`, and so on): no C body.
//! - No SSID or profile-string handling is ported. The C reference has none:
//!   `WlanScan` takes a `DOT11_SSID` but is a stub.

/// `ERROR_NOT_ENOUGH_MEMORY`: the buffer could not be reserved.
pub const ERROR_NOT_ENOUGH_MEMORY: u32 = 8;
/// `ERROR_NOT_SUPPORTED`: the client version is not 1 or 2.
pub const ERROR_NOT_SUPPORTED: u32 = 50;
/// `ERROR_INVALID_PARAMETER`: a zero size, or a null or reserved argument.
pub const ERROR_INVALID_PARAMETER: u32 = 87;

/// `WlanAllocateMemory`: a zero-filled buffer of `size` bytes.
///
/// A zero size fails with `ERROR_INVALID_PARAMETER`, and a failed reservation
/// with `ERROR_NOT_ENOUGH_MEMORY`.
pub fn allocate_memory(size: u32) -> Result<Vec<u8>, u32> {
    allocate_bytes(size as usize)
}

fn allocate_bytes(size: usize) -> Result<Vec<u8>, u32> {
    if size == 0 {
        return Err(ERROR_INVALID_PARAMETER);
    }
    let mut buffer = Vec::new();
    if buffer.try_reserve_exact(size).is_err() {
        return Err(ERROR_NOT_ENOUGH_MEMORY);
    }
    buffer.resize(size, 0);
    Ok(buffer)
}

/// The argument checks of `WlanOpenHandle`, in C's order.
///
/// Each flag says whether that argument is non-null, or for `reserved`, whether
/// it is set. `Ok(())` means the arguments pass and C would create a handle.
pub fn check_open_handle(
    client_version: u32,
    reserved_set: bool,
    negotiated_version_set: bool,
    handle_set: bool,
) -> Result<(), u32> {
    if reserved_set || !negotiated_version_set || !handle_set {
        return Err(ERROR_INVALID_PARAMETER);
    }
    match client_version {
        1 | 2 => Ok(()),
        _ => Err(ERROR_NOT_SUPPORTED),
    }
}

/// The argument checks of `WlanCloseHandle`.
///
/// `Ok(())` means the arguments pass and C would look the handle up.
pub fn check_close_handle(handle_set: bool, reserved_set: bool) -> Result<(), u32> {
    if !handle_set || reserved_set {
        return Err(ERROR_INVALID_PARAMETER);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Oracle rows `A01`–`A17`: (id, size, expected length or error).
    const ALLOCATE_ROWS: &[(&str, u32, Result<usize, u32>)] = &[
        ("A01", 0, Err(ERROR_INVALID_PARAMETER)),
        ("A02", 1, Ok(1)),
        ("A03", 2, Ok(2)),
        ("A04", 3, Ok(3)),
        ("A05", 7, Ok(7)),
        ("A06", 8, Ok(8)),
        ("A07", 9, Ok(9)),
        ("A08", 15, Ok(15)),
        ("A09", 16, Ok(16)),
        ("A10", 17, Ok(17)),
        ("A11", 63, Ok(63)),
        ("A12", 64, Ok(64)),
        ("A13", 255, Ok(255)),
        ("A14", 256, Ok(256)),
        ("A15", 4096, Ok(4096)),
        ("A16", 65536, Ok(65536)),
        ("A17", 1048576, Ok(1048576)),
    ];

    /// Oracle rows `O01`–`O21`: (id, client version, reserved, negotiated, handle,
    /// the C return code).
    const OPEN_ROWS: &[(&str, u32, bool, bool, bool, u32)] = &[
        ("O01", 1, false, true, true, 0),
        ("O02", 2, false, true, true, 0),
        ("O03", 0, false, true, true, ERROR_NOT_SUPPORTED),
        ("O04", 3, false, true, true, ERROR_NOT_SUPPORTED),
        ("O05", 10, false, true, true, ERROR_NOT_SUPPORTED),
        ("O06", 4294967295, false, true, true, ERROR_NOT_SUPPORTED),
        ("O07", 1, true, true, true, ERROR_INVALID_PARAMETER),
        ("O08", 2, true, true, true, ERROR_INVALID_PARAMETER),
        ("O09", 0, true, true, true, ERROR_INVALID_PARAMETER),
        ("O10", 1, false, false, true, ERROR_INVALID_PARAMETER),
        ("O11", 1, false, true, false, ERROR_INVALID_PARAMETER),
        ("O12", 1, false, false, false, ERROR_INVALID_PARAMETER),
        ("O13", 2, false, false, true, ERROR_INVALID_PARAMETER),
        ("O14", 2, false, true, false, ERROR_INVALID_PARAMETER),
        ("O15", 0, false, false, true, ERROR_INVALID_PARAMETER),
        ("O16", 3, true, false, false, ERROR_INVALID_PARAMETER),
        ("O17", 10, false, false, false, ERROR_INVALID_PARAMETER),
        ("O18", 0, false, true, false, ERROR_INVALID_PARAMETER),
        ("O19", 1, true, false, false, ERROR_INVALID_PARAMETER),
        ("O20", 2, true, true, false, ERROR_INVALID_PARAMETER),
        ("O21", 1, false, true, true, 0),
    ];

    /// Oracle rows `C01`–`C16`: (id, handle non-null, reserved set, the C return
    /// code). The handle value does not change the argument checks; the C codes
    /// 0 and 6 come from the handle table, so they must pass the checks.
    const CLOSE_ROWS: &[(&str, bool, bool, u32)] = &[
        ("C01", true, false, 0),
        ("C02", true, true, ERROR_INVALID_PARAMETER),
        ("C03", false, false, ERROR_INVALID_PARAMETER),
        ("C04", false, true, ERROR_INVALID_PARAMETER),
        ("C05", true, false, 6),
        ("C06", true, true, ERROR_INVALID_PARAMETER),
        ("C07", true, false, 6),
        ("C08", true, true, ERROR_INVALID_PARAMETER),
        ("C09", true, false, 6),
        ("C10", true, false, 6),
        ("C11", true, true, ERROR_INVALID_PARAMETER),
        ("C12", true, false, 6),
        ("C13", true, false, 6),
        ("C14", true, true, ERROR_INVALID_PARAMETER),
        ("C15", true, false, 6),
        ("C16", true, true, ERROR_INVALID_PARAMETER),
    ];

    fn code(result: Result<(), u32>) -> u32 {
        result.err().unwrap_or(0)
    }

    #[test]
    fn allocate_matches_wine_oracle() {
        for &(id, size, expected) in ALLOCATE_ROWS {
            let got = allocate_memory(size).map(|buffer| buffer.len());
            assert_eq!(got, expected, "{id}");
        }
    }

    #[test]
    fn open_handle_matches_wine_oracle() {
        for &(id, version, reserved, negotiated, handle, ret) in OPEN_ROWS {
            let got = check_open_handle(version, reserved, negotiated, handle);
            assert_eq!(code(got), ret, "{id}");
        }
    }

    #[test]
    fn close_handle_matches_wine_oracle() {
        for &(id, handle, reserved, ret) in CLOSE_ROWS {
            let got = check_close_handle(handle, reserved);
            let want = if ret == ERROR_INVALID_PARAMETER {
                Err(ret)
            } else {
                Ok(())
            };
            assert_eq!(got, want, "{id}");
        }
    }

    #[test]
    fn allocate_zero_size_is_invalid_parameter() {
        assert_eq!(allocate_memory(0), Err(ERROR_INVALID_PARAMETER));
    }

    #[test]
    fn allocate_returns_zeroed_buffer_of_requested_length() {
        let buffer = allocate_memory(37).unwrap();
        assert_eq!(buffer, vec![0u8; 37]);
    }

    #[test]
    fn allocate_reports_failed_reservation() {
        // Capacity above `isize::MAX` is rejected before any memory is touched.
        assert_eq!(allocate_bytes(usize::MAX), Err(ERROR_NOT_ENOUGH_MEMORY));
    }

    #[test]
    fn open_handle_checks_every_pointer_combination() {
        for bits in 0..8u32 {
            let reserved = (bits & 1) != 0;
            let negotiated = (bits & 2) != 0;
            let handle = (bits & 4) != 0;
            let want = if !reserved && negotiated && handle {
                Ok(())
            } else {
                Err(ERROR_INVALID_PARAMETER)
            };
            assert_eq!(check_open_handle(1, reserved, negotiated, handle), want);
        }
    }

    #[test]
    fn open_handle_pointer_checks_run_before_the_version_check() {
        assert_eq!(check_open_handle(0, true, true, true), Err(87));
        assert_eq!(check_open_handle(0, false, false, true), Err(87));
        assert_eq!(check_open_handle(3, false, true, false), Err(87));
    }

    #[test]
    fn open_handle_accepts_only_versions_1_and_2() {
        for version in [0, 3, 10, u32::MAX] {
            assert_eq!(
                check_open_handle(version, false, true, true),
                Err(ERROR_NOT_SUPPORTED)
            );
        }
        for version in [1, 2] {
            assert_eq!(check_open_handle(version, false, true, true), Ok(()));
        }
    }

    #[test]
    fn close_handle_checks_handle_and_reserved() {
        assert_eq!(check_close_handle(true, false), Ok(()));
        assert_eq!(check_close_handle(true, true), Err(87));
        assert_eq!(check_close_handle(false, false), Err(87));
        assert_eq!(check_close_handle(false, true), Err(87));
    }

    #[test]
    fn error_codes_match_winerror_h() {
        assert_eq!(
            [
                ERROR_NOT_ENOUGH_MEMORY,
                ERROR_NOT_SUPPORTED,
                ERROR_INVALID_PARAMETER
            ],
            [8, 50, 87]
        );
    }
}
