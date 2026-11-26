/// `IDS_COLOR_DISPLAY_NAME_BLUE` from `dlls/aero.msstyles/resources.h`.
pub const IDS_COLOR_DISPLAY_NAME_BLUE: u32 = 1000;

/// `IDS_COLOR_TOOLTIP_BLUE` from `dlls/aero.msstyles/resources.h`.
pub const IDS_COLOR_TOOLTIP_BLUE: u32 = 2000;

/// `IDS_SIZE_DISPLAY_NAME_NORMAL` from `dlls/aero.msstyles/resources.h`.
pub const IDS_SIZE_DISPLAY_NAME_NORMAL: u32 = 3000;

/// `IDS_SIZE_TOOLTIP_NORMAL` from `dlls/aero.msstyles/resources.h`.
pub const IDS_SIZE_TOOLTIP_NORMAL: u32 = 4000;

/// The English `STRINGTABLE` entries of `aero.rc`, in `.rc` order.
const STRING_TABLE: [(u32, &str); 4] = [
    (IDS_COLOR_DISPLAY_NAME_BLUE, "Blue"),
    (IDS_COLOR_TOOLTIP_BLUE, "Blue"),
    (IDS_SIZE_DISPLAY_NAME_NORMAL, "Normal"),
    (IDS_SIZE_TOOLTIP_NORMAL, "Normal"),
];

/// The `STRINGTABLE` text for `id`, or `None` when `aero.rc` defines no string with that ID.
pub fn string_table_entry(id: u32) -> Option<&'static str> {
    STRING_TABLE
        .iter()
        .find(|(entry_id, _)| *entry_id == id)
        .map(|&(_, text)| text)
}
