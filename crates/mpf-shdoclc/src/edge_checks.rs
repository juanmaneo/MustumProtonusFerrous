//! Edge-case tests for the ported shdoclc data.
//!
//! These cover the STRINGTABLE lookup outside the table, the exact name matching of html_resource,
//! the MENU lookup around its ID, the items helper and the derived traits of MenuItem, and the
//! MAKELANGID arithmetic. Each test says whether its expectations come from the oracle.

use super::*;

#[test]
fn string_lookup_outside_the_table_is_none() {
    // Not oracle-derived: IDs beside the STRINGTABLE entries and the ends of the u32 range. The
    // oracle checks only 2212, in row 10.
    for id in [0, 2214, 8402, 8405, u32::MAX] {
        assert_eq!(string_table_entry(id), None);
    }
}

#[test]
fn html_resource_matches_the_name_exactly() {
    // Not oracle-derived: find_entry_by_name in dlls/ntdll/resource.c compares the names with
    // wcsncmp and then checks the length, so only the exact, case-sensitive name matches.
    assert_eq!(html_resource("error.htm"), None);
    assert_eq!(html_resource("ERROR.HT"), None);
    assert_eq!(html_resource("ERROR.HTML"), None);
    assert_eq!(html_resource("ERROR.HTM "), None);
    assert_eq!(html_resource(""), None);
}

#[test]
fn menu_resource_answers_only_its_own_id() {
    // Not oracle-derived: the oracle reads only IDR_BROWSE_CONTEXT_MENU. The IDs beside it and 0
    // name no MENU.
    assert!(menu_resource(IDR_BROWSE_CONTEXT_MENU - 1).is_none());
    assert!(menu_resource(IDR_BROWSE_CONTEXT_MENU + 1).is_none());
    assert!(menu_resource(0).is_none());
    let menu = menu_resource(IDR_BROWSE_CONTEXT_MENU).unwrap();
    assert_eq!(menu, BROWSE_CONTEXT_MENU);
}

#[test]
fn items_is_empty_for_a_command() {
    // Not oracle-derived: a command holds no items. Rows 11 to 22 read the popups through the
    // same helper.
    let command = MenuItem::Command {
        text: "&Back",
        id: IDM_GOBACKWARD,
    };
    assert!(command.items().is_empty());
}

#[test]
fn items_returns_the_children_of_a_popup() {
    // Not oracle-derived: a popup built here with one child.
    let popup = MenuItem::Popup {
        text: "",
        items: &[MenuItem::Command {
            text: "&Back",
            id: IDM_GOBACKWARD,
        }],
    };
    assert_eq!(
        popup.items(),
        &[MenuItem::Command {
            text: "&Back",
            id: IDM_GOBACKWARD
        }]
    );
}

#[test]
fn menu_items_compare_by_kind_and_fields() {
    // Not oracle-derived: the derived PartialEq for each kind of item.
    let back = MenuItem::Command {
        text: "&Back",
        id: IDM_GOBACKWARD,
    };
    assert_eq!(
        back,
        MenuItem::Command {
            text: "&Back",
            id: IDM_GOBACKWARD
        }
    );
    assert_ne!(
        back,
        MenuItem::Command {
            text: "&Back",
            id: IDM_GOFORWARD
        }
    );
    assert_ne!(
        back,
        MenuItem::Command {
            text: "F&orward",
            id: IDM_GOBACKWARD
        }
    );
    assert_ne!(
        back,
        MenuItem::Popup {
            text: "",
            items: &[]
        }
    );
    let popup = MenuItem::Popup {
        text: "",
        items: &[],
    };
    assert_eq!(
        popup,
        MenuItem::Popup {
            text: "",
            items: &[]
        }
    );
    assert_ne!(
        popup,
        MenuItem::Popup {
            text: "",
            items: &[MenuItem::Command {
                text: "&Back",
                id: IDM_GOBACKWARD,
            }]
        }
    );
}

#[test]
fn debug_output_names_each_kind() {
    // Not oracle-derived: the derived Debug output of each kind of item.
    assert_eq!(
        format!(
            "{:?}",
            MenuItem::Command {
                text: "&Back",
                id: 2282
            }
        ),
        "Command { text: \"&Back\", id: 2282 }"
    );
    assert_eq!(
        format!(
            "{:?}",
            MenuItem::Popup {
                text: "",
                items: &[]
            }
        ),
        "Popup { text: \"\", items: [] }"
    );
}

#[test]
fn makelangid_packs_the_sublanguage_above_the_language() {
    // Not oracle-derived: MAKELANGID from include/winnt.h, which is (sublang << 10) | primary in
    // 16 bits.
    assert_eq!(makelangid(LANG_ENGLISH, SUBLANG_DEFAULT), 0x0409);
    assert_eq!(makelangid(0x0c, 0x03), 0x0c0c);
    assert_eq!(makelangid(0x3ff, 0x3f), 0xffff);
    assert_eq!(makelangid(0, 0), 0);
    // Bits above bit 15 are dropped, as the WORD result of the C macro drops them.
    assert_eq!(makelangid(0, 0x40), 0);
}
