//! The `new:` split of `new_moniker_parse_displayname` from `main.c`.

use crate::{guid_from_string, Guid};

const PREFIX: [u8; 4] = *b"new:";

/// What a `new:` display name names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NewMonikerTarget<'a> {
    /// The text after `new:` is a GUID.
    Clsid(Guid),
    /// The text after `new:` is a ProgID. `clsid` is what the lookup returned, and `name` is
    /// the text the C code keeps in `progid`.
    ProgId { clsid: Guid, name: &'a [u16] },
}

/// Parses a display name the way `new_moniker_parse_displayname` does. The name is read up to
/// its first NUL. The result is the target and the number of units consumed (`lstrlenW`), or
/// `None` for `MK_E_SYNTAX`.
///
/// `resolve_progid` stands in for `CLSIDFromProgID`, which reads the registry. It is called
/// only when the text is not a GUID, and its `None` is also `MK_E_SYNTAX`.
pub fn parse_displayname(
    name: &[u16],
    resolve_progid: impl FnOnce(&[u16]) -> Option<Guid>,
) -> Option<(NewMonikerTarget<'_>, usize)> {
    let name = &name[..name
        .iter()
        .position(|&unit| unit == 0)
        .unwrap_or(name.len())];
    let prefix = name.get(..PREFIX.len())?;
    let is_prefix = prefix
        .iter()
        .zip(PREFIX)
        .all(|(&unit, want)| u8::try_from(unit).is_ok_and(|byte| byte.eq_ignore_ascii_case(&want)));
    if !is_prefix {
        return None;
    }
    let text = &name[PREFIX.len()..];
    if let Some(clsid) = guid_from_string(text) {
        return Some((NewMonikerTarget::Clsid(clsid), name.len()));
    }
    let clsid = resolve_progid(text)?;
    Some((NewMonikerTarget::ProgId { clsid, name: text }, name.len()))
}
