//! NTSTATUS codes, mirroring `winternl.h` / `dlls/ntdll/unixlib.h`.

pub type NtStatus = i32;

pub const STATUS_SUCCESS: NtStatus = 0x0000_0000;
pub const STATUS_NOT_IMPLEMENTED: NtStatus = 0xC000_0002u32 as i32;
pub const STATUS_INVALID_HANDLE: NtStatus = 0xC000_0008u32 as i32;
pub const STATUS_BUFFER_TOO_SMALL: NtStatus = 0xC000_0023u32 as i32;
pub const STATUS_OBJECT_NAME_NOT_FOUND: NtStatus = 0xC000_0034u32 as i32;
pub const STATUS_BUFFER_OVERFLOW: NtStatus = 0x8000_0005u32 as i32;

/// NTSTATUS severity: 0 = success, 1 = informational, 2 = warning, 3 = error.
pub fn severity(status: NtStatus) -> u32 {
    (status as u32) >> 30
}

pub fn is_success(status: NtStatus) -> bool {
    status >= 0
}

pub fn is_warning(status: NtStatus) -> bool {
    severity(status) == 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn severity_bits() {
        assert!(is_success(STATUS_SUCCESS));
        assert!(!is_success(STATUS_INVALID_HANDLE));
        assert!(is_warning(STATUS_BUFFER_OVERFLOW));
        assert_eq!(severity(STATUS_BUFFER_TOO_SMALL), 3);
    }
}
