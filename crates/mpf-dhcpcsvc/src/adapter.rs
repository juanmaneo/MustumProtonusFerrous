//! Adapter name resolution from `dhcpcsvc.c`: `get_adapter_luid` and `get_adapter_name`.

use crate::api::ERROR_INVALID_PARAMETER;

/// `IF_NAMESIZE`: size of the unix interface name buffer, including the terminating NUL.
pub const IF_NAMESIZE: usize = 16;

/// `NET_LUID`.
pub type Luid = u64;

/// `GUID`, in memory order.
pub type Guid = [u8; 16];

/// The host calls that adapter resolution makes.
pub trait Os {
    /// `RtlGUIDFromString`: `None` when `s` is not a GUID.
    fn guid_from_string(&mut self, s: &[u16]) -> Option<Guid>;
    /// `ConvertInterfaceGuidToLuid`.
    fn guid_to_luid(&mut self, guid: &Guid) -> Result<Luid, u32>;
    /// `ConvertInterfaceNameToLuidW`.
    fn name_to_luid(&mut self, name: &[u16]) -> Result<Luid, u32>;
    /// `ConvertInterfaceLuidToAlias`, without the terminating NUL.
    fn luid_to_alias(&mut self, luid: Luid) -> Result<Vec<u16>, u32>;
    /// `WideCharToMultiByte(CP_UNIXCP, ...)` of `s`, without the terminating NUL. `None` on failure.
    fn wide_to_unix(&mut self, s: &[u16]) -> Option<Vec<u8>>;
}

/// The units of a NUL-terminated `WCHAR` string that starts at `units`.
fn until_nul(units: &[u16]) -> &[u16] {
    match units.iter().position(|&c| c == 0) {
        Some(end) => &units[..end],
        None => units,
    }
}

/// `get_adapter_luid`: a name that starts with `{` is tried as a GUID first, and falls back to
/// the interface name when it does not parse.
pub fn get_adapter_luid<O: Os>(os: &mut O, adapter: &[u16]) -> Result<Luid, u32> {
    let adapter = until_nul(adapter);
    if adapter.first() == Some(&u16::from(b'{')) {
        if let Some(guid) = os.guid_from_string(adapter) {
            return os.guid_to_luid(&guid);
        }
    }
    os.name_to_luid(adapter)
}

/// `get_adapter_name`: the unix interface name of `adapter`, without its NUL. The alias is read
/// through an `IF_NAMESIZE` buffer, and the converted name must fit that buffer with its NUL.
pub fn get_adapter_name<O: Os>(os: &mut O, adapter: &[u16]) -> Result<Vec<u8>, u32> {
    let luid = get_adapter_luid(os, adapter)?;
    let alias = os.luid_to_alias(luid)?;
    let alias = until_nul(&alias[..alias.len().min(IF_NAMESIZE - 1)]);
    let name = os.wide_to_unix(alias).ok_or(ERROR_INVALID_PARAMETER)?;
    if name.len() < IF_NAMESIZE {
        Ok(name)
    } else {
        Err(ERROR_INVALID_PARAMETER)
    }
}
