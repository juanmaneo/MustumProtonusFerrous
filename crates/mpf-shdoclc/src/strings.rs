/// IDS_MESSAGE_BOX_TITLE from shdoclc.h.
pub const IDS_MESSAGE_BOX_TITLE: u32 = 2213;

/// IDS_PRINT_HEADER_TEMPLATE from shdoclc.h.
pub const IDS_PRINT_HEADER_TEMPLATE: u32 = 8403;

/// IDS_PRINT_FOOTER_TEMPLATE from shdoclc.h.
pub const IDS_PRINT_FOOTER_TEMPLATE: u32 = 8404;

/// The English STRINGTABLE of shdoclc.rc, in .rc order. wrc also writes a Version 0 copy of the
/// block that holds the footer, with a #msgctxt# prefix. winebuild keeps the Version 1 copy
/// (remove_duplicate_resources in tools/winebuild/res32.c), so only the plain text is listed here.
const STRING_TABLE: [(u32, &str); 3] = [
    (IDS_MESSAGE_BOX_TITLE, "Wine Internet Explorer"),
    (IDS_PRINT_HEADER_TEMPLATE, "&w&bPage &p"),
    (IDS_PRINT_FOOTER_TEMPLATE, "&u&b&d"),
];

/// The STRINGTABLE text with the given ID, or None when shdoclc.rc defines no string with that ID.
pub fn string_table_entry(id: u32) -> Option<&'static str> {
    STRING_TABLE
        .iter()
        .find(|(entry_id, _)| *entry_id == id)
        .map(|&(_, text)| text)
}
