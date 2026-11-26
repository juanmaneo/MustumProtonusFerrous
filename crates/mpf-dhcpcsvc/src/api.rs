//! `dhcpcsvc.c` entry points that need no host services, and the Win32 error codes the port uses.

pub const ERROR_SUCCESS: u32 = 0;
pub const ERROR_INVALID_DATA: u32 = 13;
pub const ERROR_OUTOFMEMORY: u32 = 14;
pub const ERROR_INVALID_PARAMETER: u32 = 87;
pub const ERROR_INSUFFICIENT_BUFFER: u32 = 122;
pub const ERROR_MORE_DATA: u32 = 234;

/// Version reported by `DhcpCApiInitialize` (what Windows 98, XP and 8 report).
pub const DHCP_API_VERSION: u32 = 2;

/// `DhcpCApiInitialize`.
pub fn dhcp_c_api_initialize(version: &mut u32) -> u32 {
    *version = DHCP_API_VERSION;
    ERROR_SUCCESS
}

/// `DhcpCApiCleanup`: nothing is acquired, so nothing is released.
pub fn dhcp_c_api_cleanup() {}
