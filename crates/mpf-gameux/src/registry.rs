//! `GAMEUX_buildGameRegistryPath` from Wine's `dlls/gameux/gameexplorer.c`.

use crate::{wide, wstr, Guid, Hresult, E_INVALIDARG};

/// `GIS_CURRENT_USER` from `gameux.idl`.
pub const GIS_CURRENT_USER: i32 = 2;
/// `GIS_ALL_USERS` from `gameux.idl`.
pub const GIS_ALL_USERS: i32 = 3;

const GAMEUX_KEY: &str = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\GameUX\\";

/// Builds the registry path, relative to HKLM, for a game's data.
///
/// `user_sid` is called only for `GIS_CURRENT_USER`, as in the C code. It stands in
/// for the token and SID lookup, and its `Err` is returned unchanged. Any other
/// scope gives `E_INVALIDARG`. With `instance_id`, the path ends in `\{GUID}`.
pub fn build_game_registry_path(
    scope: i32,
    user_sid: &mut dyn FnMut() -> Result<Vec<u16>, Hresult>,
    instance_id: Option<&Guid>,
) -> Result<Vec<u16>, Hresult> {
    let mut path = wide(GAMEUX_KEY);
    match scope {
        GIS_CURRENT_USER => path.extend_from_slice(wstr(&user_sid()?)),
        GIS_ALL_USERS => path.extend(wide("Games")),
        _ => return Err(E_INVALIDARG),
    }
    if let Some(id) = instance_id {
        path.extend(wide("\\"));
        path.extend(format_guid(id));
    }
    Ok(path)
}

/// `StringFromGUID2`: `{XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX}`, uppercase hex.
fn format_guid(id: &Guid) -> Vec<u16> {
    let d = &id.data4;
    wide(&format!(
        "{{{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}}}",
        id.data1, id.data2, id.data3, d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7]
    ))
}
