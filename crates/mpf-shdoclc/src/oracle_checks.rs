//! Oracle-derived test vectors for the ported shdoclc constants, resources and menu.
//!
//! The vectors were printed by two C harnesses under target/canon/shdoclc, built with
//! gcc -fsanitize=address,undefined -g. oracle_consts.c prints every IDM, IDR, IDS and _IDM
//! constant that shdoclc.rc uses, with the values of shdoclc.h and mshtmcid.h. res_oracle.c reads
//! the binary shdoclc.res that wrc (Wine 11.19, wrc -u) wrote from the verbatim shdoclc.rc, and
//! applies the duplicate rule of winebuild (remove_duplicate_resources in tools/winebuild/res32.c):
//! of two resources with the same type, name and language, the one with the higher Version wins.
//! Neither harness printed a sanitizer report, so no vector is excluded. There are 23 rows.
//!
//! Each fingerprint is the 32-bit FNV-1a hash of the bytes that the harness hashed.

use super::*;

/// The 32-bit FNV-1a hash that the C harnesses print.
fn fnv1a(bytes: &[u8]) -> u32 {
    bytes.iter().fold(0x811c_9dc5, |hash, &byte| {
        (hash ^ u32::from(byte)).wrapping_mul(0x0100_0193)
    })
}

/// The bytes that res_oracle.c hashes for a menu item: "C" text TAB id LF for a command, and "P"
/// text LF, the children, "E" LF for a popup.
fn serialize(item: &MenuItem, out: &mut Vec<u8>) {
    match item {
        MenuItem::Command { text, id } => {
            out.extend_from_slice(format!("C{text}\t{id}\n").as_bytes());
        }
        MenuItem::Popup { text, items } => {
            out.extend_from_slice(format!("P{text}\n").as_bytes());
            for child in *items {
                serialize(child, out);
            }
            out.extend_from_slice(b"E\n");
        }
    }
}

/// The child count and the fingerprint of a top-level POPUP, the two values that res_oracle.c prints.
fn popup_row(popup: &MenuItem) -> (usize, u32) {
    let mut bytes = Vec::new();
    serialize(popup, &mut bytes);
    (popup.items().len(), fnv1a(&bytes))
}

/// Every constant of shdoclc.rc in name order, as oracle_consts.c prints them.
const ALL_CONSTANTS: [(&str, u32); 72] = [
    ("IDM_ADDFAVORITES", IDM_ADDFAVORITES),
    ("IDM_BROWSEMODE", IDM_BROWSEMODE),
    ("IDM_CELLPROPERTIES", IDM_CELLPROPERTIES),
    ("IDM_CELLSELECT", IDM_CELLSELECT),
    ("IDM_COLUMNSELECT", IDM_COLUMNSELECT),
    ("IDM_COPY", IDM_COPY),
    ("IDM_COPYBACKGROUND", IDM_COPYBACKGROUND),
    ("IDM_COPYSHORTCUT", IDM_COPYSHORTCUT),
    ("IDM_CREATESHORTCUT", IDM_CREATESHORTCUT),
    ("IDM_CUT", IDM_CUT),
    ("IDM_DEBUGBREAK", IDM_DEBUGBREAK),
    ("IDM_DEBUGVIEW", IDM_DEBUGVIEW),
    ("IDM_DELETE", IDM_DELETE),
    ("IDM_DUMPDISPLAYTREE", IDM_DUMPDISPLAYTREE),
    ("IDM_DUMPFORMATCACHES", IDM_DUMPFORMATCACHES),
    ("IDM_DUMPLAYOUTRECTS", IDM_DUMPLAYOUTRECTS),
    ("IDM_DUMPLINES", IDM_DUMPLINES),
    ("IDM_DUMPTRACKINGINFO", IDM_DUMPTRACKINGINFO),
    ("IDM_DUMPTREE", IDM_DUMPTREE),
    ("IDM_DYNSRCPLAY", IDM_DYNSRCPLAY),
    ("IDM_DYNSRCSTOP", IDM_DYNSRCSTOP),
    ("IDM_EDITMODE", IDM_EDITMODE),
    ("IDM_FOLLOWLINKC", IDM_FOLLOWLINKC),
    ("IDM_FOLLOWLINKN", IDM_FOLLOWLINKN),
    ("IDM_GOBACKWARD", IDM_GOBACKWARD),
    ("IDM_GOFORWARD", IDM_GOFORWARD),
    ("IDM_IMGARTPLAY", IDM_IMGARTPLAY),
    ("IDM_IMGARTREWIND", IDM_IMGARTREWIND),
    ("IDM_IMGARTSTOP", IDM_IMGARTSTOP),
    ("IDM_LANGUAGE", IDM_LANGUAGE),
    ("IDM_MEMORYMONITOR", IDM_MEMORYMONITOR),
    ("IDM_MP_EMAILPICTURE", IDM_MP_EMAILPICTURE),
    ("IDM_MP_MYPICS", IDM_MP_MYPICS),
    ("IDM_MP_PRINTPICTURE", IDM_MP_PRINTPICTURE),
    ("IDM_PASTE", IDM_PASTE),
    ("IDM_PERFORMANCEMETERS", IDM_PERFORMANCEMETERS),
    ("IDM_PRINT", IDM_PRINT),
    ("IDM_PRINTTARGET", IDM_PRINTTARGET),
    ("IDM_PROPERTIES", IDM_PROPERTIES),
    ("IDM_RESOURCEFAILURES", IDM_RESOURCEFAILURES),
    ("IDM_ROWSELECT", IDM_ROWSELECT),
    ("IDM_SAVEBACKGROUND", IDM_SAVEBACKGROUND),
    ("IDM_SAVEHTML", IDM_SAVEHTML),
    ("IDM_SAVEPICTURE", IDM_SAVEPICTURE),
    ("IDM_SAVETARGET", IDM_SAVETARGET),
    ("IDM_SCROLL_BOTTOM", IDM_SCROLL_BOTTOM),
    ("IDM_SCROLL_DOWN", IDM_SCROLL_DOWN),
    ("IDM_SCROLL_HERE", IDM_SCROLL_HERE),
    ("IDM_SCROLL_LEFT", IDM_SCROLL_LEFT),
    ("IDM_SCROLL_LEFTEDGE", IDM_SCROLL_LEFTEDGE),
    ("IDM_SCROLL_PAGEDOWN", IDM_SCROLL_PAGEDOWN),
    ("IDM_SCROLL_PAGELEFT", IDM_SCROLL_PAGELEFT),
    ("IDM_SCROLL_PAGERIGHT", IDM_SCROLL_PAGERIGHT),
    ("IDM_SCROLL_PAGEUP", IDM_SCROLL_PAGEUP),
    ("IDM_SCROLL_RIGHT", IDM_SCROLL_RIGHT),
    ("IDM_SCROLL_RIGHTEDGE", IDM_SCROLL_RIGHTEDGE),
    ("IDM_SCROLL_TOP", IDM_SCROLL_TOP),
    ("IDM_SCROLL_UP", IDM_SCROLL_UP),
    ("IDM_SELECTALL", IDM_SELECTALL),
    ("IDM_SETDESKTOPITEM", IDM_SETDESKTOPITEM),
    ("IDM_SETWALLPAPER", IDM_SETWALLPAPER),
    ("IDM_SHOWPICTURE", IDM_SHOWPICTURE),
    ("IDM_TABLEPROPERTIES", IDM_TABLEPROPERTIES),
    ("IDM_TABLESELECT", IDM_TABLESELECT),
    ("IDM_TRACETAGS", IDM_TRACETAGS),
    ("IDM_UNDO", IDM_UNDO),
    ("IDM_VIEWSOURCE", IDM_VIEWSOURCE),
    ("IDR_BROWSE_CONTEXT_MENU", IDR_BROWSE_CONTEXT_MENU),
    ("IDS_MESSAGE_BOX_TITLE", IDS_MESSAGE_BOX_TITLE),
    ("IDS_PRINT_FOOTER_TEMPLATE", IDS_PRINT_FOOTER_TEMPLATE),
    ("IDS_PRINT_HEADER_TEMPLATE", IDS_PRINT_HEADER_TEMPLATE),
    ("_IDM_REFRESH", _IDM_REFRESH),
];

#[test]
fn row01_constants_fingerprint() {
    // row 01: constants count=72 fnv=0x0e437451
    let mut lines = Vec::new();
    for (name, value) in ALL_CONSTANTS {
        lines.extend_from_slice(format!("{name} {value}\n").as_bytes());
    }
    assert_eq!(fnv1a(&lines), 0x0e43_7451);
}

#[test]
fn row02_ids_message_box_title() {
    // row 02: IDS_MESSAGE_BOX_TITLE 2213
    assert_eq!(IDS_MESSAGE_BOX_TITLE, 2213);
}

#[test]
fn row03_idr_browse_context_menu() {
    // row 03: IDR_BROWSE_CONTEXT_MENU 24641
    assert_eq!(IDR_BROWSE_CONTEXT_MENU, 24641);
}

#[test]
fn row04_idm_gobackward() {
    // row 04: IDM_GOBACKWARD 2282
    assert_eq!(IDM_GOBACKWARD, 2282);
}

#[test]
fn row05_idm_refresh() {
    // row 05: _IDM_REFRESH 6042
    assert_eq!(_IDM_REFRESH, 6042);
}

#[test]
fn row06_lang_id() {
    // row 06: resources count=4 lang=0x0409 equal=4
    assert_eq!(LANG_ID, 0x0409);
}

#[test]
fn row07_string_message_box_title() {
    // row 07: string 2213 "Wine Internet Explorer"
    assert_eq!(
        string_table_entry(IDS_MESSAGE_BOX_TITLE),
        Some("Wine Internet Explorer")
    );
}

#[test]
fn row08_string_print_header_template() {
    // row 08: string 8403 "&w&bPage &p"
    assert_eq!(
        string_table_entry(IDS_PRINT_HEADER_TEMPLATE),
        Some("&w&bPage &p")
    );
}

#[test]
fn row09_string_print_footer_template() {
    // row 09: string 8404 "&u&b&d"
    assert_eq!(
        string_table_entry(IDS_PRINT_FOOTER_TEMPLATE),
        Some("&u&b&d")
    );
}

#[test]
fn row10_string_absent_2212() {
    // row 10: string 2212 (null)
    assert_eq!(string_table_entry(2212), None);
}

#[test]
fn row11_menu_popup_00() {
    // row 11: menu popup 00 items=21 fnv=0x771ba74c
    // The oracle prints twelve popups.
    assert_eq!(BROWSE_CONTEXT_MENU.len(), 12);
    assert_eq!(popup_row(&BROWSE_CONTEXT_MENU[0]), (21, 0x771ba74c));
}

#[test]
fn row12_menu_popup_01() {
    // row 12: menu popup 01 items=21 fnv=0xcc1590c7
    assert_eq!(popup_row(&BROWSE_CONTEXT_MENU[1]), (21, 0xcc1590c7));
}

#[test]
fn row13_menu_popup_02() {
    // row 13: menu popup 02 items=8 fnv=0xafd33358
    assert_eq!(popup_row(&BROWSE_CONTEXT_MENU[2]), (8, 0xafd33358));
}

#[test]
fn row14_menu_popup_03() {
    // row 14: menu popup 03 items=4 fnv=0x0470e618
    assert_eq!(popup_row(&BROWSE_CONTEXT_MENU[3]), (4, 0x0470e618));
}

#[test]
fn row15_menu_popup_04() {
    // row 15: menu popup 04 items=6 fnv=0x8ac42eea
    assert_eq!(popup_row(&BROWSE_CONTEXT_MENU[4]), (6, 0x8ac42eea));
}

#[test]
fn row16_menu_popup_05() {
    // row 16: menu popup 05 items=13 fnv=0xf146f655
    assert_eq!(popup_row(&BROWSE_CONTEXT_MENU[5]), (13, 0xf146f655));
}

#[test]
fn row17_menu_popup_06() {
    // row 17: menu popup 06 items=1 fnv=0xae7cd5b6
    assert_eq!(popup_row(&BROWSE_CONTEXT_MENU[6]), (1, 0xae7cd5b6));
}

#[test]
fn row18_menu_popup_07() {
    // row 18: menu popup 07 items=20 fnv=0x9bb859a7
    assert_eq!(popup_row(&BROWSE_CONTEXT_MENU[7]), (20, 0x9bb859a7));
}

#[test]
fn row19_menu_popup_08() {
    // row 19: menu popup 08 items=22 fnv=0x5da83054
    assert_eq!(popup_row(&BROWSE_CONTEXT_MENU[8]), (22, 0x5da83054));
}

#[test]
fn row20_menu_popup_09() {
    // row 20: menu popup 09 items=16 fnv=0xd0b2c27b
    assert_eq!(popup_row(&BROWSE_CONTEXT_MENU[9]), (16, 0xd0b2c27b));
}

#[test]
fn row21_menu_popup_10() {
    // row 21: menu popup 10 items=10 fnv=0x5c190a8b
    assert_eq!(popup_row(&BROWSE_CONTEXT_MENU[10]), (10, 0x5c190a8b));
}

#[test]
fn row22_menu_popup_11() {
    // row 22: menu popup 11 items=10 fnv=0x437579c5
    assert_eq!(popup_row(&BROWSE_CONTEXT_MENU[11]), (10, 0x437579c5));
}

#[test]
fn row23_html_error_htm() {
    // row 23: html ERROR.HTM size=452 fnv=0xf022c65d
    let html = html_resource("ERROR.HTM").unwrap();
    assert_eq!(html.len(), 452);
    assert_eq!(fnv1a(html.as_bytes()), 0xf022_c65d);
}
