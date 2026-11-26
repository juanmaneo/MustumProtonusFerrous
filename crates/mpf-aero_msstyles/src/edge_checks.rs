//! Edge-case tests for the ported aero.msstyles logic.
//!
//! These cover the boundaries of the name-list walk and of the version check, the resource
//! tables, and the constants. Not every expectation is oracle-derived; the ones that aren't say so
//! in a comment.

use super::*;
use std::collections::HashSet;

fn names(list: &[u16]) -> Vec<String> {
    multi_sz_entries(list)
        .into_iter()
        .map(|entry| String::from_utf16(entry).unwrap())
        .collect()
}

fn units(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

#[test]
fn constants_match_resources_h_and_makefile() {
    // Not oracle-derived: the values are the resources.h macros, the aero.rc version and the
    // Makefile.in version fields. Oracle rows 01 to 04 print the four IDs and their values.
    assert_eq!(IDS_COLOR_DISPLAY_NAME_BLUE, 1000);
    assert_eq!(IDS_COLOR_TOOLTIP_BLUE, 2000);
    assert_eq!(IDS_SIZE_DISPLAY_NAME_NORMAL, 3000);
    assert_eq!(IDS_SIZE_TOOLTIP_NORMAL, 4000);
    assert_eq!(PACKTHEM_VERSION, 0x0003);
    assert_eq!(VER_FILEDESCRIPTION_STR, "Aero Theme");
    assert_eq!(VER_PRODUCTVERSION, [1, 0, 0, 1]);
}

#[test]
fn string_table_misses_outside_the_four_entries() {
    // Not oracle-derived: an ID of zero and the IDs next to the defined ones have no entry.
    assert_eq!(string_table_entry(0), None);
    assert_eq!(string_table_entry(999), None);
    assert_eq!(string_table_entry(1001), None);
}

#[test]
fn name_list_walk_boundaries() {
    // Not oracle-derived: an empty slice and a lone NUL both hold no names.
    assert!(multi_sz_entries(&[]).is_empty());
    assert!(multi_sz_entries(&[0]).is_empty());
    // Not oracle-derived: with no final NUL, the last name ends at the end of the slice. Wine's
    // walk would read past the buffer here.
    assert_eq!(names(&units("A")), ["A"]);
    // Not oracle-derived: a single NUL ends the only name, and the slice ends with it.
    assert_eq!(names(&units("A\0")), ["A"]);
    // Not oracle-derived: an empty name ends the list, so the "B" after it is never read.
    assert_eq!(names(&units("A\0\0B\0")), ["A"]);
    // Not oracle-derived: two names, then the empty name that ends the list.
    assert_eq!(names(&units("A\0B\0\0")), ["A", "B"]);
}

#[test]
fn version_check_length_and_byte_order() {
    // Not oracle-derived: an empty resource is rejected, and so is one byte, which is short of the
    // two-byte WORD.
    assert!(!is_supported_version(&[]));
    assert!(!is_supported_version(&[0x03]));
    // Not oracle-derived: a three-byte resource is longer than the WORD, so it is rejected even
    // though its first two bytes hold the version.
    assert!(!is_supported_version(&[0x03, 0x00, 0x00]));
    // Not oracle-derived: the WORD is little-endian, so the bytes [0x00, 0x03] are 0x0300.
    assert!(!is_supported_version(&[0x00, 0x03]));
    // Not oracle-derived: the accepted two-byte encoding, the same as oracle row 06.
    assert!(is_supported_version(&[0x03, 0x00]));
}

#[test]
fn every_file_res_name_resolves_to_a_textfile() {
    // Not oracle-derived: every name in FILERESNAMES must resolve through textfile, so the list
    // and the TEXTFILE table cannot drift apart.
    let list = names(&units(FILE_RES_NAMES));
    assert_eq!(list, ["BLUE_INI"]);
    assert!(list.iter().all(|name| textfile(name).is_some()));
}

#[test]
fn bitmap_resource_names_are_unique_and_end_in_bmp() {
    // Not oracle-derived: aero.rc has 482 BITMAP resources, the same count as the BITMAP entries
    // of Wine's aero.res. Each resource name is unique and ends in _BMP.
    assert_eq!(BITMAPS.len(), 482);
    let unique: HashSet<&str> = BITMAPS.iter().map(|(name, _)| *name).collect();
    assert_eq!(unique.len(), 482);
    assert!(BITMAPS.iter().all(|(name, _)| name.ends_with("_BMP")));
}
