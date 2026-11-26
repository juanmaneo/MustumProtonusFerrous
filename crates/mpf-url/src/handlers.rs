/// `HRESULT`, as its 32-bit pattern.
pub type HResult = u32;

/// `S_OK` from `include/winerror.h`.
pub const S_OK: HResult = 0;
/// `E_NOTIMPL` from `include/winerror.h`.
pub const E_NOTIMPL: HResult = 0x8000_4001;

/// `AddMIMEFileTypesPS`, which returns `0` and reads neither argument.
pub fn add_mime_file_types_ps() -> u32 {
    0
}

/// `InetIsOffline`, which returns `FALSE` for every `flags` value.
pub fn inet_is_offline() -> bool {
    false
}

/// `OpenURLA`, which has no return value and no effect.
pub fn open_url_a() {}

/// `TelnetProtocolHandlerA`, which returns `E_NOTIMPL`.
pub fn telnet_protocol_handler_a() -> HResult {
    E_NOTIMPL
}

/// The string `FileProtocolHandlerA` passes to `ShellExecuteA`.
///
/// `create_path` is the `HRESULT` from `PathCreateFromUrlA`, and `path` is the buffer it filled.
/// Only an exact `S_OK` selects `path`. Any other value, `S_FALSE` included, selects `url`, as
/// the C ternary `createpath==S_OK` does.
pub fn file_protocol_handler_a_target<'a>(
    create_path: HResult,
    path: &'a str,
    url: &'a str,
) -> &'a str {
    if create_path == S_OK {
        path
    } else {
        url
    }
}
