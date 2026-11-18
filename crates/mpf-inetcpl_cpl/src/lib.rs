//! inetcpl.cpl subset for MPF: the pure level-selection logic of `dlls/inetcpl.cpl/security.c`.
//!
//! Ported (pure logic, verified against the reference by oracle table tests):
//! - `index_from_urltemplate` (`index_from_urltemplate`): maps a `URLTEMPLATE` to its level index.
//! - `current_index`: the `tb_index` choice in `update_security_level`.
//! - `trackbar_pos`: the `TBM_SETPOS` position for a level index.
//!
//! Skipped: the dialog procedures, zone enumeration, listview and trackbar messages,
//! and string resources. They need Win32 windows, COM zone managers and the registry.
//! The reference's `FIXME` for an unsupported template is unreachable (the only index
//! that reaches it is `URLTEMPLATE_CUSTOM`, whose value is 0), so it is not ported.

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub type UrlTemplate = u32;

pub const URLTEMPLATE_CUSTOM: UrlTemplate = 0x00000;
pub const URLTEMPLATE_LOW: UrlTemplate = 0x10000;
pub const URLTEMPLATE_MEDLOW: UrlTemplate = 0x10500;
pub const URLTEMPLATE_MEDIUM: UrlTemplate = 0x11000;
pub const URLTEMPLATE_MEDHIGH: UrlTemplate = 0x11500;
pub const URLTEMPLATE_HIGH: UrlTemplate = 0x12000;

/// `NUM_TRACKBAR_POS`.
pub const NUM_TRACKBAR_POS: u32 = 5;

/// `url_templates` in `security.c`, in level order.
const URL_TEMPLATES: [UrlTemplate; 6] = [
    URLTEMPLATE_CUSTOM,
    URLTEMPLATE_LOW,
    URLTEMPLATE_MEDLOW,
    URLTEMPLATE_MEDIUM,
    URLTEMPLATE_MEDHIGH,
    URLTEMPLATE_HIGH,
];

/// Returns the level index of `value`, or `u32::MAX` if `value` is not a template.
/// The `u32::MAX` result is the unsigned wrap of the reference's `DWORD index--`.
pub fn index_from_urltemplate(value: UrlTemplate) -> u32 {
    let mut index = URL_TEMPLATES.len();
    while index > 0 && URL_TEMPLATES[index - 1] != value {
        index -= 1;
    }
    (index as u32).wrapping_sub(1)
}

/// A non-zero trackbar index wins over the index of the zone's level.
pub fn current_index(tb_index: u32, level: UrlTemplate) -> u32 {
    if tb_index > 0 {
        tb_index
    } else {
        index_from_urltemplate(level)
    }
}

/// `NUM_TRACKBAR_POS - current_index`, in unsigned arithmetic as in the reference.
pub fn trackbar_pos(current_index: u32) -> u32 {
    NUM_TRACKBAR_POS.wrapping_sub(current_index)
}
