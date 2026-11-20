use crate::{wide_str, HResult, PATH_NOT_FOUND};

/// Registry root handles used by the Win32 API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum RootKey {
    ClassesRoot = 0x8000_0000,
    CurrentUser = 0x8000_0001,
    LocalMachine = 0x8000_0002,
    Users = 0x8000_0003,
    CurrentConfig = 0x8000_0005,
}

impl RootKey {
    /// The predefined `HKEY` value associated with this root.
    pub const fn hkey(self) -> u32 {
        self as u32
    }
}

struct RootPrefix {
    full: &'static [u8],
    abbrev: Option<&'static [u8]>,
    key: RootKey,
}

/// Resolve a Wine `WshShell3` registry path prefix to its predefined hive.
///
/// Matching is case-sensitive and does not require a separator after the root, matching
/// `get_root_key`'s `wcsncmp` calls.
pub fn root_key(path: &[u16]) -> Option<RootKey> {
    const ROOTS: [RootPrefix; 5] = [
        RootPrefix {
            full: b"HKEY_CURRENT_USER",
            abbrev: Some(b"HKCU"),
            key: RootKey::CurrentUser,
        },
        RootPrefix {
            full: b"HKEY_LOCAL_MACHINE",
            abbrev: Some(b"HKLM"),
            key: RootKey::LocalMachine,
        },
        RootPrefix {
            full: b"HKEY_CLASSES_ROOT",
            abbrev: Some(b"HKCR"),
            key: RootKey::ClassesRoot,
        },
        RootPrefix {
            full: b"HKEY_USERS",
            abbrev: None,
            key: RootKey::Users,
        },
        RootPrefix {
            full: b"HKEY_CURRENT_CONFIG",
            abbrev: None,
            key: RootKey::CurrentConfig,
        },
    ];

    let path = wide_str(path);
    ROOTS.iter().find_map(|root| {
        (starts_with_ascii(path, root.full)
            || root
                .abbrev
                .is_some_and(|short| starts_with_ascii(path, short)))
        .then_some(root.key)
    })
}

/// The borrowed parts returned by Wine's `split_reg_path`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegPath<'a> {
    pub subkey: &'a [u16],
    pub value: Option<&'a [u16]>,
}

/// Split a registry path into its key path and optional value name.
///
/// For a default-value path ending in a backslash, `value` is `None`. The source has a notable
/// one-separator quirk: when a value is present but there is no second backslash, `subkey`
/// includes the first backslash (for example, `HKCU\Value` yields `\Value` and `Value`).
pub fn split_reg_path(input: &[u16]) -> Result<RegPath<'_>, HResult> {
    let path = wide_str(input);
    let Some(first) = path.iter().position(|&unit| unit == b'\\' as u16) else {
        return Err(PATH_NOT_FOUND);
    };

    if path.last() == Some(&(b'\\' as u16)) {
        return Ok(RegPath {
            subkey: &path[first + 1..],
            value: None,
        });
    }

    let last = path
        .iter()
        .rposition(|&unit| unit == b'\\' as u16)
        .expect("the first separator guarantees a last separator");
    if last - first > 1 {
        Ok(RegPath {
            subkey: &path[first + 1..last],
            value: Some(&path[last + 1..]),
        })
    } else {
        Ok(RegPath {
            subkey: &path[first..],
            value: Some(&path[last + 1..]),
        })
    }
}

fn starts_with_ascii(path: &[u16], prefix: &[u8]) -> bool {
    path.get(..prefix.len()).is_some_and(|start| {
        start
            .iter()
            .zip(prefix)
            .all(|(&unit, &byte)| unit == byte as u16)
    })
}
