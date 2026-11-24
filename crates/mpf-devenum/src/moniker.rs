//! Moniker identity and display-name text from Wine's `moniker_GetDisplayName` in
//! `mediacatenum.c`.

use crate::guid::{format_guid, Guid};
use crate::wide;

/// The two device kinds that share the `sw:` and `cm:` display-name forms.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeviceKind {
    /// `DEVICE_FILTER`, displayed as `@device:sw:`.
    Filter,
    /// `DEVICE_CODEC`, displayed as `@device:cm:`.
    Codec,
}

/// The fields of Wine's `struct moniker` that determine its display name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Moniker<'a> {
    /// A filter or codec, with an optional class GUID and the device name.
    Device {
        kind: DeviceKind,
        class: Option<Guid>,
        name: &'a [u16],
    },
    /// A DMO, identified by its CLSID and its category class.
    Dmo { clsid: Guid, class: Guid },
}

/// Port of `moniker_GetDisplayName`. The result is UTF-16 without a NUL terminator.
pub fn format_display_name(moniker: &Moniker<'_>) -> Vec<u16> {
    match moniker {
        Moniker::Device { kind, class, name } => {
            let mut text = wide(match kind {
                DeviceKind::Filter => "@device:sw:",
                DeviceKind::Codec => "@device:cm:",
            });
            if let Some(class) = class {
                text.extend(format_guid(class));
                text.push(u16::from(b'\\'));
            }
            text.extend_from_slice(name);
            text
        }
        Moniker::Dmo { clsid, class } => {
            let mut text = wide("@device:dmo:");
            text.extend(format_guid(clsid));
            text.extend(format_guid(class));
            text
        }
    }
}
