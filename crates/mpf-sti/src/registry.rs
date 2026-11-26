//! `RegisterLaunchApplication` and `UnregisterLaunchApplication` from `dlls/sti/sti.c`. The
//! registry calls are injected through [`Registry`], so the control flow and the error mapping
//! run without the OS.

use crate::com::{hresult_from_win32, Hresult, E_OUTOFMEMORY};

/// `registeredAppsLaunchPath` in `dlls/sti/sti.c`. The key is under `HKEY_LOCAL_MACHINE`.
pub const REGISTERED_APPS_LAUNCH_PATH: &str =
    "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\StillImage\\Registered Applications";

/// `commandLineSuffix` in `RegisterLaunchApplication`.
const COMMAND_LINE_SUFFIX: &str = "/StiDevice:%1 /StiEvent:%2";

const ERROR_SUCCESS: u32 = 0;

/// The registry calls that the two methods make. Each returns a Win32 error code.
pub trait Registry {
    /// `RegCreateKeyW(HKEY_LOCAL_MACHINE, path, ...)`. The key stays open until `close_key`.
    fn create_key(&mut self, path: &[u16]) -> u32;
    /// `RegSetValueExW` with `REG_SZ`. `data` includes the terminating NUL.
    fn set_value(&mut self, name: &[u16], data: &[u16]) -> u32;
    /// `RegDeleteValueW`.
    fn delete_value(&mut self, name: &[u16]) -> u32;
    /// `RegCloseKey` for the key that `create_key` opened.
    fn close_key(&mut self);
}

/// `RegisterLaunchApplication`. `alloc_ok` is the `HeapAlloc` result for the value buffer.
/// `RegCloseKey` runs whenever the key was opened, including after an allocation failure.
pub fn register_launch_application(
    registry: &mut impl Registry,
    app_name: &[u16],
    command_line: &[u16],
    alloc_ok: bool,
) -> Hresult {
    let ret = registry.create_key(&registered_apps_path());
    if ret != ERROR_SUCCESS {
        return hresult_from_win32(ret);
    }
    let hr = if alloc_ok {
        let data = launch_command_data(command_line);
        hresult_from_win32(registry.set_value(c_str(app_name), &data))
    } else {
        E_OUTOFMEMORY
    };
    registry.close_key();
    hr
}

/// `UnregisterLaunchApplication`.
pub fn unregister_launch_application(registry: &mut impl Registry, app_name: &[u16]) -> Hresult {
    let ret = registry.create_key(&registered_apps_path());
    if ret != ERROR_SUCCESS {
        return hresult_from_win32(ret);
    }
    let hr = hresult_from_win32(registry.delete_value(c_str(app_name)));
    registry.close_key();
    hr
}

/// The `REG_SZ` data that `RegisterLaunchApplication` writes. The `swprintf(L"%s %s")` call
/// stops each `%s` at its NUL. The data is `command_line`, a space, the suffix, and a NUL.
pub fn launch_command_data(command_line: &[u16]) -> Vec<u16> {
    let mut data = c_str(command_line).to_vec();
    data.push(u16::from(b' '));
    data.extend(COMMAND_LINE_SUFFIX.encode_utf16());
    data.push(0);
    data
}

fn registered_apps_path() -> Vec<u16> {
    REGISTERED_APPS_LAUNCH_PATH.encode_utf16().collect()
}

/// A wide string as the C API reads it: up to the first NUL, or all of it if there is none.
fn c_str(s: &[u16]) -> &[u16] {
    let end = s.iter().position(|&c| c == 0).unwrap_or(s.len());
    &s[..end]
}
