//! Oracle-derived test vectors for the ported aero.msstyles logic.
//!
//! Vectors were captured by compiling the verbatim `resources.h` IDs and `aero.rc` string table,
//! the `version_accepted` and list-walk bodies of `dlls/uxtheme/msstyles.c`, and the user-data
//! literals of `aero.rc` with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/aero.msstyles/`, then running the harness. All 20 printed rows ran clean under the
//! sanitizers, so none are excluded.
//!
//! The last test is not oracle-derived. It checks the port against the resources of Wine's
//! `aero.res`, built with `wrc -u` and dumped by `res_dump.py`.

use super::*;

/// FNV-1a over `bytes`, the hash the harness prints for the text resources.
fn fnv1a(bytes: &[u8]) -> u32 {
    bytes.iter().fold(0x811c_9dc5, |hash, &byte| {
        (hash ^ u32::from(byte)).wrapping_mul(0x0100_0193)
    })
}

/// The UTF-16LE bytes of `text`, which is how `wrc -u` stores the text resources.
fn utf16le(text: &str) -> Vec<u8> {
    text.encode_utf16().flat_map(u16::to_le_bytes).collect()
}

/// The names in a double-NUL-terminated UTF-16 list, decoded to `String`s.
fn names(list: &str) -> Vec<String> {
    let units: Vec<u16> = list.encode_utf16().collect();
    multi_sz_entries(&units)
        .into_iter()
        .map(|entry| String::from_utf16(entry).unwrap())
        .collect()
}

/// The unit count and FNV-1a hash of a `TEXTFILE` resource, as the harness prints them.
fn text_fingerprint(name: &str) -> Option<(usize, u32)> {
    textfile(name).map(|text| (text.encode_utf16().count(), fnv1a(&utf16le(text))))
}

#[test]
fn oracle_string_table_rows() {
    // row 01: string_table 1000 -> "Blue"
    assert_eq!(string_table_entry(1000), Some("Blue"));
    // row 02: string_table 2000 -> "Blue"
    assert_eq!(string_table_entry(2000), Some("Blue"));
    // row 03: string_table 3000 -> "Normal"
    assert_eq!(string_table_entry(3000), Some("Normal"));
    // row 04: string_table 4000 -> "Normal"
    assert_eq!(string_table_entry(4000), Some("Normal"));
    // row 05: string_table 5000 -> (null)
    assert_eq!(string_table_entry(5000), None);
}

#[test]
fn oracle_version_rows() {
    // row 06: version size=2 data=0x0003 -> accept
    assert!(is_supported_version(&[0x03, 0x00]));
    // row 07: version size=2 data=0x0004 -> reject
    assert!(!is_supported_version(&[0x04, 0x00]));
    // row 08: version size=1 data=0x0003 -> reject
    assert!(!is_supported_version(&[0x03]));
}

#[test]
fn oracle_name_list_rows() {
    // row 09: COLORNAMES -> [0]="Blue" entries=1
    assert_eq!(names(COLOR_NAMES), ["Blue"]);
    // row 10: SIZENAMES -> [0]="NormalSize" entries=1
    assert_eq!(names(SIZE_NAMES), ["NormalSize"]);
    // row 11: FILERESNAMES -> [0]="BLUE_INI" entries=1
    assert_eq!(names(FILE_RES_NAMES), ["BLUE_INI"]);
}

#[test]
fn oracle_textfile_rows() {
    // row 12: textfile THEMES_INI -> units=66 fnv1a=0x9f7b6b9d
    assert_eq!(text_fingerprint("THEMES_INI"), Some((66, 0x9f7b_6b9d)));
    // row 13: textfile BLUE_INI -> units=78968 fnv1a=0x4cb4309a
    assert_eq!(text_fingerprint("BLUE_INI"), Some((78_968, 0x4cb4_309a)));
    // row 14: textfile NO_SUCH -> (null)
    assert_eq!(text_fingerprint("NO_SUCH"), None);
    // row 15: textfile themes_ini -> (null)
    assert_eq!(text_fingerprint("themes_ini"), None);
}

#[test]
fn oracle_bitmap_rows() {
    // row 16: bitmap BLUE_BUTTON_BMP -> blue_button.bmp
    assert_eq!(bitmap_file("BLUE_BUTTON_BMP"), Some("blue_button.bmp"));
    // row 17: bitmap BLUE_CHECKBOX_13PX_BMP -> blue_checkbox_13px.bmp
    assert_eq!(
        bitmap_file("BLUE_CHECKBOX_13PX_BMP"),
        Some("blue_checkbox_13px.bmp")
    );
    // row 18: bitmap BLUE_WINDOW_SMALL_FRAME_RIGHT_BMP -> blue_window_small_frame_right.bmp
    assert_eq!(
        bitmap_file("BLUE_WINDOW_SMALL_FRAME_RIGHT_BMP"),
        Some("blue_window_small_frame_right.bmp")
    );
    // row 19: bitmap blue_button_bmp -> (null)
    assert_eq!(bitmap_file("blue_button_bmp"), None);
    // row 20: bitmap NO_SUCH -> (null)
    assert_eq!(bitmap_file("NO_SUCH"), None);
}

#[test]
fn wine_build_resources_match_wrc_output() {
    // Not oracle-derived: the byte sizes and FNV-1a hashes of the resources in Wine's aero.res,
    // built with `wrc -u` and dumped by target/canon/aero.msstyles/res_dump.py. The UTF-16 data
    // is hashed as little-endian bytes, as the dump does.
    assert_eq!(PACKTHEM_VERSION.to_le_bytes(), [0x03, 0x00]);
    for (text, size, hash) in [
        (COLOR_NAMES, 12, 0xd8d9_f353),
        (SIZE_NAMES, 24, 0xbf13_b9d3),
        (FILE_RES_NAMES, 20, 0xf792_1292),
        (THEMES_INI, 132, 0x9f7b_6b9d),
        (BLUE_INI, 157_936, 0x4cb4_309a),
    ] {
        let bytes = utf16le(text);
        assert_eq!((bytes.len(), fnv1a(&bytes)), (size, hash));
    }
}
