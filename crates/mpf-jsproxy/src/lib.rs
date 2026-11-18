//! Headless subset of Wine's `dlls/jsproxy/main.c`, the auto-proxy (PAC) DLL.
//!
//! The ported functions return the value that Wine's C body computes from its
//! arguments. The JScript engine, COM, file loading, sockets and the computer
//! name are not modelled: the caller supplies their results.
//!
//! Ported:
//! - `dispex_GetDispID`: [`dispid_for_name`].
//! - `dispex_InvokeEx` argument checks: [`invoke_ex`]. The `dns_resolve` call it makes is the caller's; its result is [`first_ipv4_text`] of the lookup.
//! - `printf_addr`: [`ipv4_text`].
//! - `dns_resolve`, after `getaddrinfo`: [`first_ipv4_text`]. The lookup and the empty-name case (computer name) are the caller's.
//! - `JSPROXY_InternetInitializeAutoProxyDll`: [`initialize_script`]; the file load is the `load_file` argument.
//! - `InternetDeInitializeAutoProxyDll`: [`deinitialize_script`].
//! - `InternetGetProxyInfo`, the check that a script is loaded: [`proxy_script`]; the script run is the caller's.
//!
//! Skipped:
//! - `InternetInitializeAutoProxyDllEx`, `InternetDeInitializeAutoProxyDllEx`, `InternetGetProxyInfoEx`: `stub` in `jsproxy.spec`, no bodies.
//! - `load_script`, `get_computer_name`, `run_script`, `include_pac_utils`, `strdupAW` and `strdupWA`: file, computer-name, JScript and code-page work.
//! - The `IDispatchEx` and `IActiveScriptSite` vtables: COM.
//! - `pac.js`: the PAC helpers run inside JScript; they are not C.
//!
//! Divergences from C:
//! - Names are compared as UTF-16 code units. `lstrcmpW` compares with the locale's collation, which differs only for non-ASCII text.
//! - Names and the script are read up to their first NUL, as `lstrcmpW` and `WideCharToMultiByte` read them.

/// `DISPID_GLOBAL_DNSRESOLVE`, the dispatch id of the global `dns_resolve`.
pub const DISPID_GLOBAL_DNSRESOLVE: i32 = 0x1000;
/// `DISP_E_UNKNOWNNAME`, returned for a name other than `dns_resolve`.
pub const DISP_E_UNKNOWNNAME: i32 = 0x8002_0006u32 as i32;
/// `DISP_E_MEMBERNOTFOUND`, returned for a dispatch id other than `DISPID_GLOBAL_DNSRESOLVE`.
pub const DISP_E_MEMBERNOTFOUND: i32 = 0x8002_0003u32 as i32;
/// `DISP_E_BADPARAMCOUNT`, returned when `dns_resolve` is not given exactly one argument.
pub const DISP_E_BADPARAMCOUNT: i32 = 0x8002_000Eu32 as i32;
/// `DISP_E_BADVARTYPE`, returned when the one argument of `dns_resolve` is not a BSTR.
pub const DISP_E_BADVARTYPE: i32 = 0x8002_0008u32 as i32;
/// `ERROR_INVALID_PARAMETER`, which `InternetInitializeAutoProxyDll` sets for a zero-size buffer.
pub const ERROR_INVALID_PARAMETER: u32 = 87;
/// `ERROR_CAN_NOT_COMPLETE`, which `InternetGetProxyInfo` sets when no script is loaded.
pub const ERROR_CAN_NOT_COMPLETE: u32 = 1003;
/// `AF_INET`, the only address family that `dns_resolve` reads.
pub const AF_INET: i32 = 2;

/// The `sizeof(AUTO_PROXY_SCRIPT_BUFFER)` that `dwStructSize` must equal: a DWORD, a pointer and a DWORD, padded to pointer alignment.
pub const AUTO_PROXY_SCRIPT_BUFFER_SIZE: u32 = {
    let pointer = std::mem::size_of::<usize>() as u32;
    (2 * pointer + 4).next_multiple_of(pointer)
};

/// The code units of a NUL-terminated UTF-16 string, without the terminator.
fn wstr(text: &[u16]) -> &[u16] {
    match text.iter().position(|&c| c == 0) {
        Some(end) => &text[..end],
        None => text,
    }
}

/// Returns the dispatch id for `name`, or `DISP_E_UNKNOWNNAME`. Only `dns_resolve` is known, and the comparison is case-sensitive.
pub fn dispid_for_name(name: &[u16]) -> Result<i32, i32> {
    if wstr(name).iter().copied().eq("dns_resolve".encode_utf16()) {
        Ok(DISPID_GLOBAL_DNSRESOLVE)
    } else {
        Err(DISP_E_UNKNOWNNAME)
    }
}

/// An argument of an `InvokeEx` call: a BSTR, or a VARIANT of any other type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InvokeArg<'a> {
    /// A VARIANT with `VT_BSTR`, holding this text.
    Bstr(&'a [u16]),
    /// A VARIANT of any other type.
    Other,
}

/// Checks the arguments of `dispex_InvokeEx`. On success it returns the name to resolve, which `dns_resolve` then looks up.
///
/// Dispatch ids other than `DISPID_GLOBAL_DNSRESOLVE` return `DISP_E_MEMBERNOTFOUND` whatever the arguments. For that id, the argument count is checked before the type.
pub fn invoke_ex<'a>(id: i32, args: &[InvokeArg<'a>]) -> Result<&'a [u16], i32> {
    if id != DISPID_GLOBAL_DNSRESOLVE {
        return Err(DISP_E_MEMBERNOTFOUND);
    }
    match args {
        [InvokeArg::Bstr(name)] => Ok(wstr(name)),
        [InvokeArg::Other] => Err(DISP_E_BADVARTYPE),
        _ => Err(DISP_E_BADPARAMCOUNT),
    }
}

/// One `addrinfo` entry, as `dns_resolve` reads it: its family and, for `AF_INET`, the address in network byte order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AddrEntry {
    /// `ai_family`.
    pub family: i32,
    /// `sin_addr.s_addr` of an `AF_INET` entry, in network byte order.
    pub s_addr: u32,
}

/// Formats an IPv4 address given in network byte order as dotted decimal, as `printf_addr` does.
pub fn ipv4_text(s_addr: u32) -> String {
    let host = u32::from_be(s_addr);
    format!(
        "{}.{}.{}.{}",
        (host >> 24) as u8,
        (host >> 16) as u8,
        (host >> 8) as u8,
        host as u8
    )
}

/// The `dns_resolve` result for a lookup of a non-empty name: the text of the first `AF_INET` entry, which is `S_OK` with that BSTR. `None` stands for a failed `getaddrinfo` and for a lookup with no `AF_INET` entry; both are `S_FALSE`.
pub fn first_ipv4_text(lookup: Option<&[AddrEntry]>) -> Option<String> {
    lookup?
        .iter()
        .find(|entry| entry.family == AF_INET)
        .map(|entry| ipv4_text(entry.s_addr))
}

/// The `AUTO_PROXY_SCRIPT_BUFFER` argument of `InternetInitializeAutoProxyDll`, used when its pointer is given.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScriptBuffer<'a> {
    /// `dwStructSize`.
    pub struct_size: u32,
    /// `lpszScriptBuffer` decoded to UTF-16, or `None` when the pointer is NULL.
    pub script: Option<&'a [u16]>,
    /// `dwScriptBufferSize`, the number of bytes the script was decoded from.
    pub script_size: u32,
}

/// What `InternetInitializeAutoProxyDll` returns, and what it leaves in the script state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InitOutcome {
    /// `TRUE`: the new script is stored.
    Loaded,
    /// `FALSE` with no error code. The file did not load, and the previous script is gone.
    LoadFailed,
    /// `FALSE` with `ERROR_INVALID_PARAMETER`. The buffer has zero size, and the previous script is kept.
    InvalidParameter,
}

/// Runs the script-state part of `InternetInitializeAutoProxyDll`.
///
/// A buffer with the right `struct_size` and a non-NULL script is used: a zero `script_size` is an error that keeps the previous script, and otherwise the script is replaced. In every other case the previous script is dropped and `load_file` supplies the new one.
pub fn initialize_script(
    state: &mut Option<Vec<u16>>,
    buffer: Option<ScriptBuffer<'_>>,
    load_file: impl FnOnce() -> Option<Vec<u16>>,
) -> InitOutcome {
    if let Some(ScriptBuffer {
        struct_size,
        script: Some(text),
        script_size,
    }) = buffer
    {
        if struct_size == AUTO_PROXY_SCRIPT_BUFFER_SIZE {
            if script_size == 0 {
                return InitOutcome::InvalidParameter;
            }
            *state = Some(text.to_vec());
            return InitOutcome::Loaded;
        }
    }
    *state = load_file();
    if state.is_some() {
        InitOutcome::Loaded
    } else {
        InitOutcome::LoadFailed
    }
}

/// `InternetDeInitializeAutoProxyDll`: drops the stored script. The C body always returns `TRUE`.
pub fn deinitialize_script(state: &mut Option<Vec<u16>>) {
    *state = None;
}

/// The check at the start of `InternetGetProxyInfo`: returns the script to run, or `ERROR_CAN_NOT_COMPLETE` when none is loaded.
pub fn proxy_script(state: Option<&[u16]>) -> Result<&[u16], u32> {
    state.ok_or(ERROR_CAN_NOT_COMPLETE)
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
