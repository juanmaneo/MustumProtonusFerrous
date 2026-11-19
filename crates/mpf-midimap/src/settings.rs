//! `MIDIMAP_LoadSettings` and its helpers: which port each of the 16 channels is routed to,
//! read from the user's MIDIMap settings and the system-wide scheme keys.

use crate::ports::{find_port, OutPort};

/// `HKCU` key that holds the user's mapping.
pub const USER_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Multimedia\MIDIMap";
/// `HKLM` key that holds the named schemes.
pub const SCHEMES_KEY: &str =
    r"System\CurrentControlSet\Control\MediaProperties\PrivateProperties\Midi\Schemes";

/// `RegEnumKeyW` buffer size in code units: a subkey name of this length or more ends the walk.
const MAX_KEY_NAME: usize = 256;
/// The byte size of the `szPname` and `CurrentScheme` buffers (`WCHAR[256]`).
const MAX_STRING_BYTES: usize = 512;
const DWORD_BYTES: usize = 4;

/// The port index chosen for each channel, `None` for an unmapped channel.
pub type ChannelMap = [Option<usize>; 16];

/// The two registry roots that the mapper opens.
pub enum Root {
    /// `HKEY_CURRENT_USER`.
    CurrentUser,
    /// `HKEY_LOCAL_MACHINE`.
    LocalMachine,
}

/// The registry calls that `MIDIMAP_LoadSettings` makes.
///
/// Values are returned as raw bytes, so the caller applies the Win32 size limits.
pub trait Registry {
    /// An open key.
    type Key;
    /// `RegOpenKeyA` on a root key. `None` if the key does not exist.
    fn open_root(&self, root: Root, path: &str) -> Option<Self::Key>;
    /// `RegOpenKeyW` on a subkey of `key`. `None` if the key does not exist.
    fn open_sub(&self, key: &Self::Key, name: &[u16]) -> Option<Self::Key>;
    /// `RegEnumKeyW`: the name of the subkey at `index`, or `None` past the last one.
    fn sub_key_name(&self, key: &Self::Key, index: u32) -> Option<Vec<u16>>;
    /// The raw data of a value (`""` for the default value), or `None` if it is absent.
    fn value(&self, key: &Self::Key, name: &str) -> Option<Vec<u8>>;
    /// `RegCloseKey`.
    fn close(&self, key: Self::Key);
}

/// `MIDIMAP_LoadSettings`: the channel map for a new instance, or `None` when the
/// settings name no usable port (the C `MIDIERR_INVALIDSETUP` path).
pub fn load<R: Registry>(reg: &R, ports: &[OutPort]) -> Option<ChannelMap> {
    let Some(user) = reg.open_root(Root::CurrentUser, USER_KEY) else {
        return default_map(ports, None);
    };
    let map = user_map(reg, ports, &user);
    reg.close(user);
    map
}

fn user_map<R: Registry>(reg: &R, ports: &[OutPort], user: &R::Key) -> Option<ChannelMap> {
    if let Some(name) = string(reg, user, "szPname") {
        return default_map(ports, Some(name.as_slice()));
    }
    if dword(reg, user, "UseScheme").unwrap_or(0) != 0 {
        if let Some(scheme) = string(reg, user, "CurrentScheme") {
            return scheme_map(reg, ports, &scheme).or_else(|| default_map(ports, None));
        }
    }
    match string(reg, user, "CurrentInstrument") {
        Some(name) if !name.is_empty() => default_map(ports, Some(name.as_slice())),
        _ => default_map(ports, None),
    }
}

/// `MIDIMAP_LoadSettingsDefault`: every channel to one port. `port` is looked up by name,
/// and falls back to port 0 when it is absent or unknown.
fn default_map(ports: &[OutPort], port: Option<&[u16]>) -> Option<ChannelMap> {
    let dev = port.and_then(|name| find_port(ports, name)).unwrap_or(0);
    (dev < ports.len()).then_some([Some(dev); 16])
}

/// `MIDIMAP_LoadSettingsScheme`. Each subkey of the scheme names a port in its default
/// value, and a `Channels` bitmask. Later subkeys overwrite earlier ones.
fn scheme_map<R: Registry>(reg: &R, ports: &[OutPort], scheme: &[u16]) -> Option<ChannelMap> {
    let schemes = reg.open_root(Root::LocalMachine, SCHEMES_KEY)?;
    let Some(key) = reg.open_sub(&schemes, scheme) else {
        reg.close(schemes);
        return None;
    };
    let mut map = [None; 16];
    let mut index = 0;
    while let Some(name) = reg.sub_key_name(&key, index) {
        if name.len() >= MAX_KEY_NAME {
            break;
        }
        if let Some((dev, mask)) = scheme_entry(reg, ports, &key, &name) {
            for (channel, slot) in map.iter_mut().enumerate() {
                if mask & (1 << channel) != 0 {
                    *slot = Some(dev);
                }
            }
        }
        index += 1;
    }
    reg.close(key);
    reg.close(schemes);
    Some(map)
}

/// One subkey of a scheme: the port index and channel bitmask, when both are usable.
fn scheme_entry<R: Registry>(
    reg: &R,
    ports: &[OutPort],
    scheme: &R::Key,
    name: &[u16],
) -> Option<(usize, u32)> {
    let key = reg.open_sub(scheme, name)?;
    let port = string(reg, &key, "");
    let mask = dword(reg, &key, "Channels");
    reg.close(key);
    let dev = find_port(ports, &port?)?;
    Some((dev, mask?))
}

/// A `REG_SZ` value read into a buffer of `MAX_STRING_BYTES`, up to its first NUL.
fn string<R: Registry>(reg: &R, key: &R::Key, name: &str) -> Option<Vec<u16>> {
    let raw = reg.value(key, name)?;
    if raw.len() > MAX_STRING_BYTES {
        return None;
    }
    Some(
        raw.as_chunks::<2>()
            .0
            .iter()
            .map(|&unit| u16::from_le_bytes(unit))
            .take_while(|&unit| unit != 0)
            .collect(),
    )
}

/// A `REG_DWORD` value read into a 4-byte buffer. A shorter value is zero-extended.
fn dword<R: Registry>(reg: &R, key: &R::Key, name: &str) -> Option<u32> {
    let raw = reg.value(key, name)?;
    if raw.len() > DWORD_BYTES {
        return None;
    }
    let mut bytes = [0u8; DWORD_BYTES];
    bytes[..raw.len()].copy_from_slice(&raw);
    Some(u32::from_le_bytes(bytes))
}
