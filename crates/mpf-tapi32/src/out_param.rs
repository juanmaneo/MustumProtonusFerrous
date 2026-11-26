/// `lineNegotiateAPIVersion` from `dlls/tapi32/line.c`. The C body stores the high version in
/// `*lpdwAPIVersion` and returns 0. The line handle, device ID, low version and extension ID
/// are not read.
pub fn line_negotiate_api_version(dw_api_high_version: u32, api_version: &mut u32) -> u32 {
    *api_version = dw_api_high_version;
    0
}

/// `phoneInitializeExA` and `phoneInitializeExW` from `dlls/tapi32/phone.c`. Both C bodies store 0
/// in `*lpdwNumDevs` and return 0. The other parameters are not read.
pub fn phone_initialize_ex(num_devs: &mut u32) -> u32 {
    *num_devs = 0;
    0
}
