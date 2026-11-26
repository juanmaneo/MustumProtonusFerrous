/// `ERROR_SUCCESS` from `include/winerror.h`.
pub const ERROR_SUCCESS: u32 = 0;
/// `ERROR_INVALID_PARAMETER` from `include/winerror.h`.
pub const ERROR_INVALID_PARAMETER: u32 = 87;
/// `ERROR_INSUFFICIENT_BUFFER` from `include/winerror.h`.
pub const ERROR_INSUFFICIENT_BUFFER: u32 = 122;

/// `offsetof(PROVIDER_ENUMERATION_INFO, TraceProviderInfoArray)` from `include/tdh.h`: the
/// `NumberOfProviders` and `Reserved` ULONGs that come before the provider array.
pub const PROVIDER_ENUMERATION_HEADER_SIZE: u32 = 8;

/// `TdhEnumerateProviders` from `dlls/tdh/tdh_main.c`.
///
/// `buffer` is the caller's `PROVIDER_ENUMERATION_INFO` and `buffer_size` its `ULONG *`; `None`
/// is a NULL pointer. The C code writes `NumberOfProviders` (the first ULONG, little-endian as on
/// the Windows targets) and leaves `Reserved` and the provider array alone. The C code trusts
/// `*buffer_size` without reading the memory behind it, so a `buffer` shorter than four bytes
/// when the size is large enough to write is a contract violation: the port panics where C would
/// overflow.
pub fn tdh_enumerate_providers(buffer: Option<&mut [u8]>, buffer_size: Option<&mut u32>) -> u32 {
    let Some(buffer_size) = buffer_size else {
        return ERROR_INVALID_PARAMETER;
    };
    let buffer = match buffer {
        Some(buffer) => buffer,
        None if *buffer_size != 0 => return ERROR_INVALID_PARAMETER,
        None => &mut [],
    };
    if *buffer_size < PROVIDER_ENUMERATION_HEADER_SIZE {
        *buffer_size = PROVIDER_ENUMERATION_HEADER_SIZE;
        return ERROR_INSUFFICIENT_BUFFER;
    }
    buffer[..4].copy_from_slice(&0u32.to_le_bytes());
    ERROR_SUCCESS
}
