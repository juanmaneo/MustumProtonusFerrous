//! Headless subset of Wine's `dlls/oledlg` (the OLE dialog library).
//!
//! The ported functions compute the decisions that Wine's C bodies make from
//! their arguments: which verbs an object's menu gets and in what order, and the
//! flag and list state of the Paste Special dialog. COM objects, clipboard data,
//! menus and dialog controls are not modelled; the caller performs the side
//! effects that the returned plans describe.
//!
//! Ported:
//! - `OleUIAddVerbMenuW`: `plan_add_verb_menu`, built on `is_verb_in_range` and `next_insertable_verb` (`get_next_insertable_verb`).
//! - `init_pastelist`: `paste_entry_matches`.
//! - `init_linklist`: `supported_link_mask` and `link_entry_visible`.
//! - `init_lists`: `init_lists_state`.
//! - `mode_change`: `mode_change`.
//! - `update_as_icon`: `update_as_icon_flags`.
//! - `update_result_text`: `result_text_id` and `substitute_percent_s`.
//! - `update_structure`: `export_result`.
//!
//! Skipped:
//! - `DllMain`, `register_clipboard_formats`, `register_messages`: process-wide clipboard format and message registration.
//! - `OleUIAddVerbMenuA`: ANSI to wide conversion; the decisions are in `OleUIAddVerbMenuW`.
//! - `OleUICanConvertOrActivateAs`, `OleUIInsertObjectW`, `OleUIEditLinksA/W`, `OleUIChangeIconA/W`, `OleUIConvertA/W`, `OleUIBusyA/W`, `OleUIUpdateLinksA/W`, `OleUIPromptUserA/W`, `OleUIObjectPropertiesA/W`, `OleUIChangeSourceA/W`: `FIXME` stubs that fail with `ERROR_CALL_NOT_IMPLEMENTED`; no logic to port.
//! - `OleUIInsertObjectA`, `UIInsertObjectDlgProc` and the other `UIINSERTOBJECTDLG_*` helpers: dialog templates, registry-backed object types, file browsing and COM object creation.
//! - `OleUIPasteSpecialA/W`, `ps_dlg_proc`, `get_descriptors`, `add_entry_to_lb`, `update_display_list`, `update_src_text`, `selection_change`, `post_help_msg`, `send_end_dialog_msg`, `free_structure`: dialog messages, listbox items, clipboard and `IDataObject` I/O. `add_entry_to_lb` also depends on `LB_FINDSTRING` semantics.
//! - `dump_ps_flags`, `dump_pastespecial`: trace output.
//! - `strdupAtoW`: code page conversion.

/// `OLEVERBATTRIB_ONCONTAINERMENU`: the verb is listed in the container's menu.
pub const OLEVERBATTRIB_ONCONTAINERMENU: u32 = 2;
/// `PS_MAXLINKTYPES`: the number of link formats `init_linklist` inspects.
pub const PS_MAXLINKTYPES: usize = 8;
/// The number of formats `init_pastelist` takes from the enumerator (`ARRAY_SIZE(fmts)`).
pub const PS_MAX_ENUM_FORMATS: usize = 20;
/// `PSF_SELECTPASTE`: the Paste radio button is selected.
pub const PSF_SELECTPASTE: u32 = 0x2;
/// `PSF_SELECTPASTELINK`: the Paste Link radio button is selected.
pub const PSF_SELECTPASTELINK: u32 = 0x4;
/// `PSF_CHECKDISPLAYASICON`: the display-as-icon check box is set.
pub const PSF_CHECKDISPLAYASICON: u32 = 0x8;
/// `OLEUIPASTE_LINKANYTYPE`: a paste entry accepts any supported link type.
pub const OLEUIPASTE_LINKANYTYPE: u32 = 1024;
/// `IDC_PS_PASTE`: the Paste radio button.
pub const IDC_PS_PASTE: u32 = 500;
/// `IDC_PS_PASTELINK`: the Paste Link radio button.
pub const IDC_PS_PASTELINK: u32 = 501;
/// `IDS_PS_PASTE_DATA`: result text for pasting data.
pub const IDS_PS_PASTE_DATA: u32 = 0x190;
/// `IDS_PS_PASTE_OBJECT_AS_ICON`: result text for pasting an object as an icon.
pub const IDS_PS_PASTE_OBJECT_AS_ICON: u32 = 0x192;
/// `IDS_PS_PASTE_LINK_DATA`: result text for pasting a link to data.
pub const IDS_PS_PASTE_LINK_DATA: u32 = 0x193;
/// `IDS_PS_PASTE_LINK_OBJECT_AS_ICON`: result text for pasting a link to an object as an icon.
pub const IDS_PS_PASTE_LINK_OBJECT_AS_ICON: u32 = 0x195;

/// One `OLEVERB` entry. The verb name does not affect the menu decisions, so it is not carried.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Verb {
    /// `lVerb`, a signed verb number that the menu id is derived from.
    pub lverb: i32,
    /// `fuFlags`, the menu flags of the verb.
    pub fu_flags: u32,
    /// `grfAttribs`, the `OLEVERBATTRIB_*` bits.
    pub attribs: u32,
}

/// One menu entry that `OleUIAddVerbMenuW` inserts into the verb submenu.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MenuItem {
    /// The command id, `lVerb + idmin` (unsigned, wrapping).
    pub id: u32,
    /// The verb's `fuFlags`, passed through as the menu flags.
    pub fu_flags: u32,
}

/// What `OleUIAddVerbMenuW` does, as data for the caller to apply to its menus.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AddVerbMenu {
    /// `hMenu` or the submenu output is missing. Nothing is changed and the call returns `FALSE`.
    Invalid,
    /// There is no object or no verb enumerator. The root item becomes a grayed placeholder with id `idmin`, and the call returns `FALSE`.
    Grayed { id: u32 },
    /// No convert entry is needed and at most one verb can go in the menu. The root item runs the verb directly with id `idmin`. `verb` indexes the verbs slice; `None` means no verb qualified, and Wine then uses a zeroed verb with no name.
    Single { id: u32, verb: Option<usize> },
    /// A popup submenu with `items` in insertion order. When `convert` is set, a separator and then the convert entry with that id follow the items. The call returns `TRUE`.
    Popup {
        items: Vec<MenuItem>,
        convert: Option<u32>,
    },
}

/// `is_verb_in_range`: with `idmax == 0` every verb is in range. Otherwise `lVerb + idmin` must not exceed `idmax`. The sum is unsigned and wraps, as the C `LONG + UINT` addition does.
pub fn is_verb_in_range(verb: &Verb, idmin: u32, idmax: u32) -> bool {
    if idmax == 0 {
        return true;
    }
    (verb.lverb as u32).wrapping_add(idmin) <= idmax
}

/// Index of the first verb at or after `start` that `get_next_insertable_verb` returns: in range and flagged `OLEVERBATTRIB_ONCONTAINERMENU`.
pub fn next_insertable_verb(verbs: &[Verb], idmin: u32, idmax: u32, start: usize) -> Option<usize> {
    (start..verbs.len()).find(|&index| {
        is_verb_in_range(&verbs[index], idmin, idmax)
            && verbs[index].attribs & OLEVERBATTRIB_ONCONTAINERMENU != 0
    })
}

fn menu_item(verb: &Verb, idmin: u32) -> MenuItem {
    MenuItem {
        id: (verb.lverb as u32).wrapping_add(idmin),
        fu_flags: verb.fu_flags,
    }
}

/// Decides what `OleUIAddVerbMenuW` does.
///
/// `verbs` is `None` when the object or its verb enumerator is missing, and an empty slice when the enumerator has no verbs. `has_menu` and `has_submenu_slot` say whether the caller passed `hMenu` and the submenu output.
///
/// Wine inserts the first insertable verb even when none qualifies, so a popup then holds one zeroed entry with id `idmin`.
pub fn plan_add_verb_menu(
    verbs: Option<&[Verb]>,
    has_menu: bool,
    has_submenu_slot: bool,
    idmin: u32,
    idmax: u32,
    add_convert: bool,
    id_convert: u32,
) -> AddVerbMenu {
    if !has_menu || !has_submenu_slot {
        return AddVerbMenu::Invalid;
    }
    let Some(verbs) = verbs else {
        return AddVerbMenu::Grayed { id: idmin };
    };
    let first = next_insertable_verb(verbs, idmin, idmax, 0);
    let second = first.and_then(|index| next_insertable_verb(verbs, idmin, idmax, index + 1));
    if second.is_none() && !add_convert {
        return AddVerbMenu::Single {
            id: idmin,
            verb: first,
        };
    }
    let mut items = vec![match first {
        Some(index) => menu_item(&verbs[index], idmin),
        None => MenuItem {
            id: idmin,
            fu_flags: 0,
        },
    }];
    if let Some(second_index) = second {
        items.push(menu_item(&verbs[second_index], idmin));
        let mut cursor = second_index + 1;
        while let Some(index) = next_insertable_verb(verbs, idmin, idmax, cursor) {
            items.push(menu_item(&verbs[index], idmin));
            cursor = index + 1;
        }
    }
    AddVerbMenu::Popup {
        items,
        convert: if add_convert { Some(id_convert) } else { None },
    }
}

/// Which radio button of the Paste Special dialog is selected.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PasteMode {
    /// `PSF_SELECTPASTE`.
    Paste,
    /// `PSF_SELECTPASTELINK`.
    PasteLink,
}

/// The Paste Special state that `init_lists` settles on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ListState {
    /// The `PSF_*` flags after the list population rules ran.
    pub flags: u32,
    /// The radio button to check, or `None` when neither list has entries.
    pub check: Option<PasteMode>,
    /// `true` when the OK button must be disabled, which happens when `check` is `None`.
    pub disable_ok: bool,
}

/// `init_pastelist`: for each requested format, whether one of the enumerated source formats matches it. Only the first `PS_MAX_ENUM_FORMATS` offered formats are considered.
pub fn paste_entry_matches(requested: &[u32], offered: &[u32]) -> Vec<bool> {
    let offered = &offered[..offered.len().min(PS_MAX_ENUM_FORMATS)];
    requested
        .iter()
        .map(|format| offered.contains(format))
        .collect()
}

/// The mask of link types that `init_linklist` found supported. Bit `n` is set when `query_ok[n]` is set. Only the first `PS_MAXLINKTYPES` entries count.
pub fn supported_link_mask(query_ok: &[bool]) -> u32 {
    query_ok
        .iter()
        .take(PS_MAXLINKTYPES)
        .enumerate()
        .fold(
            0,
            |mask, (link, &ok)| if ok { mask | 1 << link } else { mask },
        )
}

/// Whether a paste entry with `entry_flags` gets a link list item: `OLEUIPASTE_LINKANYTYPE` accepts every link type, otherwise the entry's low byte is the set of accepted link types.
pub fn link_entry_visible(entry_flags: u32, supported_mask: u32) -> bool {
    let link_types = if entry_flags & OLEUIPASTE_LINKANYTYPE != 0 {
        0xff
    } else {
        entry_flags & 0xff
    };
    link_types & supported_mask != 0
}

/// `init_lists`: settles the selected mode from the flags and the number of entries in each list.
///
/// A dialog that has no mode selected defaults to Paste. A selected mode with an empty list switches to the other mode. When both lists are empty, both mode flags are cleared.
pub fn init_lists_state(flags: u32, pastes_added: u32, links_added: u32) -> ListState {
    let mut flags = flags;
    if flags & (PSF_SELECTPASTE | PSF_SELECTPASTELINK) == 0 {
        flags |= PSF_SELECTPASTE;
    }
    if pastes_added == 0 && links_added == 0 {
        flags &= !(PSF_SELECTPASTE | PSF_SELECTPASTELINK);
    } else if pastes_added == 0 && flags & PSF_SELECTPASTE != 0 {
        flags &= !PSF_SELECTPASTE;
        flags |= PSF_SELECTPASTELINK;
    } else if links_added == 0 && flags & PSF_SELECTPASTELINK != 0 {
        flags &= !PSF_SELECTPASTELINK;
        flags |= PSF_SELECTPASTE;
    }
    let check = if flags & PSF_SELECTPASTE != 0 {
        Some(PasteMode::Paste)
    } else if flags & PSF_SELECTPASTELINK != 0 {
        Some(PasteMode::PasteLink)
    } else {
        None
    };
    ListState {
        flags,
        check,
        disable_ok: check.is_none(),
    }
}

/// `mode_change`: a click on the Paste button selects Paste, any other button selects Paste Link.
pub fn mode_change(flags: u32, button_id: u32) -> u32 {
    if button_id == IDC_PS_PASTE {
        (flags & !PSF_SELECTPASTELINK) | PSF_SELECTPASTE
    } else {
        (flags & !PSF_SELECTPASTE) | PSF_SELECTPASTELINK
    }
}

/// `update_as_icon`: display-as-icon is not implemented, so the flag is always cleared.
pub fn update_as_icon_flags(flags: u32) -> u32 {
    flags & !PSF_CHECKDISPLAYASICON
}

/// `update_result_text`: the resource id of the result text for the selected mode and icon flag.
pub fn result_text_id(flags: u32) -> u32 {
    match (
        flags & PSF_SELECTPASTE != 0,
        flags & PSF_CHECKDISPLAYASICON != 0,
    ) {
        (true, true) => IDS_PS_PASTE_OBJECT_AS_ICON,
        (true, false) => IDS_PS_PASTE_DATA,
        (false, true) => IDS_PS_PASTE_LINK_OBJECT_AS_ICON,
        (false, false) => IDS_PS_PASTE_LINK_DATA,
    }
}

/// Replaces the first `%s` in a resource string with `value`, as `update_result_text` does. Without `%s`, the template is returned unchanged. Both slices exclude the NUL terminator.
pub fn substitute_percent_s(template: &[u16], value: &[u16]) -> Vec<u16> {
    let placeholder = [b'%' as u16, b's' as u16];
    match template.windows(2).position(|pair| pair == placeholder) {
        Some(at) => {
            let mut text = template[..at].to_vec();
            text.extend_from_slice(value);
            text.extend_from_slice(&template[at + 2..]);
            text
        }
        None => template.to_vec(),
    }
}

/// The result of `update_structure`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PasteResult {
    /// `nSelectedIndex`.
    pub n_selected_index: u32,
    /// `dwFlags`.
    pub dw_flags: u32,
    /// `fLink`, set when the Paste Link mode is selected.
    pub f_link: bool,
}

/// `update_structure`: the values written back to the caller's structure. `selected_index` is the index of the selected list item, or `None` when nothing is selected, and in that case `n_selected_index` keeps its previous value.
pub fn export_result(
    flags: u32,
    selected_index: Option<u32>,
    n_selected_index: u32,
) -> PasteResult {
    PasteResult {
        n_selected_index: selected_index.unwrap_or(n_selected_index),
        dw_flags: flags,
        f_link: flags & PSF_SELECTPASTELINK != 0,
    }
}

#[cfg(test)]
mod oracle_checks {
    use super::*;

    /// The expected plan, written with slices so it can be a const table.
    enum Expect {
        Invalid,
        Grayed(u32),
        Single(u32, Option<usize>),
        Popup(&'static [(u32, u32)], Option<u32>),
    }

    /// The same shape as `AddVerbMenu`, owned, so the two can be compared.
    #[derive(Debug, PartialEq, Eq)]
    enum Shape {
        Invalid,
        Grayed(u32),
        Single(u32, Option<usize>),
        Popup(Vec<(u32, u32)>, Option<u32>),
    }

    struct VerbRow {
        ret: bool,
        menu: bool,
        slot: bool,
        object: Option<&'static [Verb]>,
        idmin: u32,
        idmax: u32,
        convert: bool,
        id_convert: u32,
        expect: Expect,
    }

    struct PasteRow {
        flags: u32,
        entries: &'static [(u32, u32)],
        offered: &'static [u32],
        link_ok: &'static [bool],
        pastes: u32,
        links: u32,
        out_flags: u32,
        check: Option<PasteMode>,
        disable_ok: bool,
    }

    /// Rows generated by running `target/canon/oledlg/harness_verb.c`, which compiles Wine's `OleUIAddVerbMenuW` verbatim.
    const VERB_ROWS: &[VerbRow] = &[
        VerbRow {
            ret: false,
            menu: true,
            slot: false,
            object: Some(&[Verb {
                lverb: 1,
                fu_flags: 0,
                attribs: 2,
            }]),
            idmin: 100,
            idmax: 0,
            convert: false,
            id_convert: 900,
            expect: Expect::Invalid,
        }, // case 0
        VerbRow {
            ret: false,
            menu: true,
            slot: true,
            object: None,
            idmin: 100,
            idmax: 0,
            convert: false,
            id_convert: 900,
            expect: Expect::Grayed(100),
        }, // case 1
        VerbRow {
            ret: true,
            menu: true,
            slot: true,
            object: Some(&[]),
            idmin: 100,
            idmax: 0,
            convert: false,
            id_convert: 0,
            expect: Expect::Single(100, None),
        }, // case 2
        VerbRow {
            ret: true,
            menu: true,
            slot: true,
            object: Some(&[]),
            idmin: 100,
            idmax: 0,
            convert: true,
            id_convert: 900,
            expect: Expect::Popup(&[(100, 0)], Some(900)),
        }, // case 3
        VerbRow {
            ret: true,
            menu: true,
            slot: true,
            object: Some(&[Verb {
                lverb: 1,
                fu_flags: 0,
                attribs: 2,
            }]),
            idmin: 100,
            idmax: 0,
            convert: false,
            id_convert: 900,
            expect: Expect::Single(100, Some(0)),
        }, // case 4
        VerbRow {
            ret: true,
            menu: true,
            slot: true,
            object: Some(&[Verb {
                lverb: 1,
                fu_flags: 0,
                attribs: 2,
            }]),
            idmin: 100,
            idmax: 0,
            convert: true,
            id_convert: 900,
            expect: Expect::Popup(&[(101, 0)], Some(900)),
        }, // case 5
        VerbRow {
            ret: true,
            menu: true,
            slot: true,
            object: Some(&[
                Verb {
                    lverb: 1,
                    fu_flags: 0,
                    attribs: 2,
                },
                Verb {
                    lverb: 2,
                    fu_flags: 1,
                    attribs: 2,
                },
            ]),
            idmin: 100,
            idmax: 0,
            convert: false,
            id_convert: 900,
            expect: Expect::Popup(&[(101, 0), (102, 1)], None),
        }, // case 6
        VerbRow {
            ret: true,
            menu: true,
            slot: true,
            object: Some(&[
                Verb {
                    lverb: 1,
                    fu_flags: 0,
                    attribs: 2,
                },
                Verb {
                    lverb: 2,
                    fu_flags: 0,
                    attribs: 0,
                },
                Verb {
                    lverb: 3,
                    fu_flags: 8,
                    attribs: 2,
                },
            ]),
            idmin: 100,
            idmax: 0,
            convert: true,
            id_convert: 900,
            expect: Expect::Popup(&[(101, 0), (103, 8)], Some(900)),
        }, // case 7
        VerbRow {
            ret: true,
            menu: true,
            slot: true,
            object: Some(&[
                Verb {
                    lverb: 0,
                    fu_flags: 0,
                    attribs: 2,
                },
                Verb {
                    lverb: 1,
                    fu_flags: 0,
                    attribs: 2,
                },
                Verb {
                    lverb: 2,
                    fu_flags: 0,
                    attribs: 2,
                },
            ]),
            idmin: 100,
            idmax: 101,
            convert: false,
            id_convert: 900,
            expect: Expect::Popup(&[(100, 0), (101, 0)], None),
        }, // case 8
        VerbRow {
            ret: true,
            menu: true,
            slot: true,
            object: Some(&[
                Verb {
                    lverb: 10,
                    fu_flags: 0,
                    attribs: 2,
                },
                Verb {
                    lverb: 3,
                    fu_flags: 0,
                    attribs: 2,
                },
            ]),
            idmin: 100,
            idmax: 105,
            convert: false,
            id_convert: 900,
            expect: Expect::Single(100, Some(1)),
        }, // case 9
        VerbRow {
            ret: true,
            menu: true,
            slot: true,
            object: Some(&[
                Verb {
                    lverb: -1,
                    fu_flags: 0,
                    attribs: 2,
                },
                Verb {
                    lverb: 1,
                    fu_flags: 0,
                    attribs: 2,
                },
            ]),
            idmin: 5,
            idmax: 10,
            convert: false,
            id_convert: 900,
            expect: Expect::Popup(&[(4, 0), (6, 0)], None),
        }, // case 10
        VerbRow {
            ret: true,
            menu: true,
            slot: true,
            object: Some(&[
                Verb {
                    lverb: 1000,
                    fu_flags: 0,
                    attribs: 2,
                },
                Verb {
                    lverb: 1001,
                    fu_flags: 1,
                    attribs: 2,
                },
            ]),
            idmin: 5,
            idmax: 0,
            convert: true,
            id_convert: 900,
            expect: Expect::Popup(&[(1005, 0), (1006, 1)], Some(900)),
        }, // case 11
        VerbRow {
            ret: true,
            menu: true,
            slot: true,
            object: Some(&[
                Verb {
                    lverb: 1,
                    fu_flags: 0,
                    attribs: 0,
                },
                Verb {
                    lverb: 2,
                    fu_flags: 8,
                    attribs: 2,
                },
            ]),
            idmin: 100,
            idmax: 0,
            convert: false,
            id_convert: 900,
            expect: Expect::Single(100, Some(1)),
        }, // case 12
    ];

    /// Rows generated by running `target/canon/oledlg/harness_paste.c`, which compiles Wine's `init_pastelist`, `init_linklist` and `init_lists` verbatim.
    const PASTE_ROWS: &[PasteRow] = &[
        PasteRow {
            flags: 0,
            entries: &[(1, 0)],
            offered: &[1],
            link_ok: &[],
            pastes: 1,
            links: 0,
            out_flags: 2,
            check: Some(PasteMode::Paste),
            disable_ok: false,
        }, // case 0
        PasteRow {
            flags: 0,
            entries: &[(1, 0)],
            offered: &[],
            link_ok: &[],
            pastes: 0,
            links: 0,
            out_flags: 0,
            check: None,
            disable_ok: true,
        }, // case 1
        PasteRow {
            flags: 0,
            entries: &[(49409, 1)],
            offered: &[],
            link_ok: &[true],
            pastes: 0,
            links: 1,
            out_flags: 4,
            check: Some(PasteMode::PasteLink),
            disable_ok: false,
        }, // case 2
        PasteRow {
            flags: 4,
            entries: &[(1, 0), (49409, 1)],
            offered: &[1],
            link_ok: &[true],
            pastes: 1,
            links: 1,
            out_flags: 4,
            check: Some(PasteMode::PasteLink),
            disable_ok: false,
        }, // case 3
        PasteRow {
            flags: 4,
            entries: &[(1, 0)],
            offered: &[1],
            link_ok: &[],
            pastes: 1,
            links: 0,
            out_flags: 2,
            check: Some(PasteMode::Paste),
            disable_ok: false,
        }, // case 4
        PasteRow {
            flags: 6,
            entries: &[(1, 0)],
            offered: &[1],
            link_ok: &[],
            pastes: 1,
            links: 0,
            out_flags: 2,
            check: Some(PasteMode::Paste),
            disable_ok: false,
        }, // case 5
        PasteRow {
            flags: 1,
            entries: &[(49409, 1)],
            offered: &[],
            link_ok: &[false],
            pastes: 0,
            links: 0,
            out_flags: 1,
            check: None,
            disable_ok: true,
        }, // case 6
        PasteRow {
            flags: 0,
            entries: &[(49409, 1024)],
            offered: &[],
            link_ok: &[false, true, false],
            pastes: 0,
            links: 1,
            out_flags: 4,
            check: Some(PasteMode::PasteLink),
            disable_ok: false,
        }, // case 7
        PasteRow {
            flags: 0,
            entries: &[(49409, 1024)],
            offered: &[],
            link_ok: &[false, false, false, false, false, false, false, false, true],
            pastes: 0,
            links: 0,
            out_flags: 0,
            check: None,
            disable_ok: true,
        }, // case 8
        PasteRow {
            flags: 0,
            entries: &[(1, 0)],
            offered: &[
                36864, 36865, 36866, 36867, 36868, 36869, 36870, 36871, 36872, 36873, 36874, 36875,
                36876, 36877, 36878, 36879, 36880, 36881, 36882, 36883, 1,
            ],
            link_ok: &[],
            pastes: 0,
            links: 0,
            out_flags: 0,
            check: None,
            disable_ok: true,
        }, // case 9
        PasteRow {
            flags: 0,
            entries: &[],
            offered: &[1],
            link_ok: &[],
            pastes: 0,
            links: 0,
            out_flags: 0,
            check: None,
            disable_ok: true,
        }, // case 10
        PasteRow {
            flags: 12,
            entries: &[(49409, 1)],
            offered: &[],
            link_ok: &[true],
            pastes: 0,
            links: 1,
            out_flags: 12,
            check: Some(PasteMode::PasteLink),
            disable_ok: false,
        }, // case 11
    ];

    fn shape(plan: &AddVerbMenu) -> Shape {
        match plan {
            AddVerbMenu::Invalid => Shape::Invalid,
            AddVerbMenu::Grayed { id } => Shape::Grayed(*id),
            AddVerbMenu::Single { id, verb } => Shape::Single(*id, *verb),
            AddVerbMenu::Popup { items, convert } => Shape::Popup(
                items.iter().map(|item| (item.id, item.fu_flags)).collect(),
                *convert,
            ),
        }
    }

    fn expected(expect: &Expect) -> Shape {
        match expect {
            Expect::Invalid => Shape::Invalid,
            Expect::Grayed(id) => Shape::Grayed(*id),
            Expect::Single(id, verb) => Shape::Single(*id, *verb),
            Expect::Popup(items, convert) => Shape::Popup(items.to_vec(), *convert),
        }
    }

    fn planned(row: &VerbRow) -> AddVerbMenu {
        plan_add_verb_menu(
            row.object,
            row.menu,
            row.slot,
            row.idmin,
            row.idmax,
            row.convert,
            row.id_convert,
        )
    }

    #[test]
    fn verb_menu_plans_match_the_wine_oracle() {
        assert_eq!(VERB_ROWS.len(), 13);
        for row in VERB_ROWS {
            let plan = planned(row);
            assert_eq!(
                row.ret,
                !matches!(plan, AddVerbMenu::Invalid | AddVerbMenu::Grayed { .. })
            );
            assert_eq!(shape(&plan), expected(&row.expect));
        }
    }

    #[test]
    fn paste_state_matches_the_wine_oracle() {
        assert_eq!(PASTE_ROWS.len(), 12);
        for row in PASTE_ROWS {
            let requested: Vec<u32> = row.entries.iter().map(|entry| entry.0).collect();
            let matches = paste_entry_matches(&requested, row.offered);
            let pastes = matches.iter().filter(|&&found| found).count() as u32;
            let mask = supported_link_mask(row.link_ok);
            let links = row
                .entries
                .iter()
                .filter(|entry| link_entry_visible(entry.1, mask))
                .count() as u32;
            assert_eq!((pastes, links), (row.pastes, row.links));
            let state = init_lists_state(row.flags, pastes, links);
            assert_eq!(
                (state.flags, state.check, state.disable_ok),
                (row.out_flags, row.check, row.disable_ok)
            );
        }
    }

    fn w(text: &str) -> Vec<u16> {
        text.encode_utf16().collect()
    }

    fn verb(lverb: i32, fu_flags: u32, attribs: u32) -> Verb {
        Verb {
            lverb,
            fu_flags,
            attribs,
        }
    }

    #[test]
    fn verb_range_treats_zero_idmax_as_unbounded() {
        assert!(is_verb_in_range(&verb(7, 0, 0), 3, 0));
        assert!(is_verb_in_range(&verb(7, 0, 0), 3, 10));
        assert!(!is_verb_in_range(&verb(8, 0, 0), 3, 10));
    }

    #[test]
    fn negative_verbs_wrap_like_the_unsigned_addition() {
        assert!(is_verb_in_range(&verb(-1, 0, 0), 5, 4));
        assert_eq!(
            menu_item(&verb(-1, 2, 0), 5),
            MenuItem { id: 4, fu_flags: 2 }
        );
    }

    #[test]
    fn next_insertable_verb_skips_to_the_qualifying_verb() {
        let verbs = [
            verb(1, 0, 0),
            verb(2, 0, OLEVERBATTRIB_ONCONTAINERMENU),
            verb(3, 0, OLEVERBATTRIB_ONCONTAINERMENU),
        ];
        assert_eq!(next_insertable_verb(&verbs, 0, 0, 0), Some(1));
        assert_eq!(next_insertable_verb(&verbs, 0, 0, 2), Some(2));
        assert_eq!(next_insertable_verb(&verbs, 0, 0, 3), None);
        assert_eq!(next_insertable_verb(&verbs, 0, 2, 0), Some(1));
        assert_eq!(next_insertable_verb(&verbs, 0, 2, 2), None);
    }

    #[test]
    fn invalid_arguments_plan_nothing() {
        let verbs = [verb(1, 0, OLEVERBATTRIB_ONCONTAINERMENU)];
        assert_eq!(
            plan_add_verb_menu(Some(&verbs), false, true, 0, 0, false, 0),
            AddVerbMenu::Invalid
        );
        assert_eq!(
            plan_add_verb_menu(Some(&verbs), true, false, 0, 0, false, 0),
            AddVerbMenu::Invalid
        );
        assert_eq!(
            plan_add_verb_menu(None, false, false, 0, 0, false, 0),
            AddVerbMenu::Invalid
        );
    }

    #[test]
    fn convert_entry_is_appended_to_a_single_verb_popup() {
        let verbs = [verb(4, 1, OLEVERBATTRIB_ONCONTAINERMENU)];
        assert_eq!(
            plan_add_verb_menu(Some(&verbs), true, true, 10, 0, true, 99),
            AddVerbMenu::Popup {
                items: vec![MenuItem {
                    id: 14,
                    fu_flags: 1
                }],
                convert: Some(99),
            }
        );
    }

    /// Hand-derived from the insertion loop at `oledlg_main.c:230`, which has no oracle row: with three qualifying verbs every verb after the first two goes into the popup in order, with no separator.
    #[test]
    fn popup_lists_every_insertable_verb_in_order() {
        let verbs = [
            verb(1, 0, OLEVERBATTRIB_ONCONTAINERMENU),
            verb(2, 0, 0),
            verb(3, 1, OLEVERBATTRIB_ONCONTAINERMENU),
            verb(4, 8, OLEVERBATTRIB_ONCONTAINERMENU),
        ];
        assert_eq!(
            shape(&plan_add_verb_menu(
                Some(&verbs),
                true,
                true,
                100,
                0,
                false,
                0
            )),
            Shape::Popup(vec![(101, 0), (103, 1), (104, 8)], None)
        );
    }

    #[test]
    fn paste_matching_ignores_formats_past_the_twentieth() {
        let offered: Vec<u32> = (0..20).map(|n| 100 + n).chain([7]).collect();
        assert_eq!(paste_entry_matches(&[7, 105], &offered), vec![false, true]);
        assert_eq!(paste_entry_matches(&[], &offered), Vec::<bool>::new());
    }

    #[test]
    fn link_mask_ignores_link_types_past_the_limit() {
        let mut query = [false; PS_MAXLINKTYPES + 1];
        query[0] = true;
        query[2] = true;
        query[PS_MAXLINKTYPES] = true;
        assert_eq!(supported_link_mask(&query), 0b101);
        assert_eq!(supported_link_mask(&[]), 0);
    }

    #[test]
    fn link_entries_use_the_low_byte_unless_any_type_is_set() {
        assert!(link_entry_visible(0x2, 0x3));
        assert!(!link_entry_visible(0x4, 0x3));
        assert!(link_entry_visible(OLEUIPASTE_LINKANYTYPE | 0x4, 0x8));
        assert!(!link_entry_visible(OLEUIPASTE_LINKANYTYPE, 0));
    }

    #[test]
    fn mode_change_selects_exactly_one_mode() {
        assert_eq!(
            mode_change(PSF_SELECTPASTELINK | PSF_CHECKDISPLAYASICON, IDC_PS_PASTE),
            PSF_SELECTPASTE | PSF_CHECKDISPLAYASICON
        );
        assert_eq!(
            mode_change(PSF_SELECTPASTE, IDC_PS_PASTELINK),
            PSF_SELECTPASTELINK
        );
    }

    #[test]
    fn as_icon_flag_is_always_cleared() {
        assert_eq!(
            update_as_icon_flags(PSF_CHECKDISPLAYASICON | PSF_SELECTPASTE),
            PSF_SELECTPASTE
        );
        assert_eq!(update_as_icon_flags(0), 0);
    }

    #[test]
    fn result_text_depends_on_mode_and_icon_flag() {
        assert_eq!(result_text_id(PSF_SELECTPASTE), IDS_PS_PASTE_DATA);
        assert_eq!(
            result_text_id(PSF_SELECTPASTE | PSF_CHECKDISPLAYASICON),
            IDS_PS_PASTE_OBJECT_AS_ICON
        );
        assert_eq!(result_text_id(PSF_SELECTPASTELINK), IDS_PS_PASTE_LINK_DATA);
        assert_eq!(
            result_text_id(PSF_SELECTPASTELINK | PSF_CHECKDISPLAYASICON),
            IDS_PS_PASTE_LINK_OBJECT_AS_ICON
        );
    }

    #[test]
    fn percent_s_is_replaced_once_and_only_when_present() {
        assert_eq!(
            substitute_percent_s(&w("Paste %s here %s"), &w("Doc")),
            w("Paste Doc here %s")
        );
        assert_eq!(
            substitute_percent_s(&w("Paste data"), &w("Doc")),
            w("Paste data")
        );
        assert_eq!(substitute_percent_s(&w("%s"), &[]), Vec::<u16>::new());
        assert_eq!(substitute_percent_s(&w("%"), &w("x")), w("%"));
    }

    #[test]
    fn selection_is_exported_only_when_present() {
        let kept = export_result(PSF_SELECTPASTELINK, None, 5);
        assert_eq!(
            kept,
            PasteResult {
                n_selected_index: 5,
                dw_flags: PSF_SELECTPASTELINK,
                f_link: true
            }
        );
        let set = export_result(PSF_SELECTPASTE, Some(2), 5);
        assert_eq!(
            set,
            PasteResult {
                n_selected_index: 2,
                dw_flags: PSF_SELECTPASTE,
                f_link: false
            }
        );
    }

    #[test]
    fn init_lists_state_keeps_both_modes_when_both_lists_have_entries() {
        let state = init_lists_state(PSF_SELECTPASTE | PSF_SELECTPASTELINK, 1, 1);
        assert_eq!(
            state,
            ListState {
                flags: PSF_SELECTPASTE | PSF_SELECTPASTELINK,
                check: Some(PasteMode::Paste),
                disable_ok: false
            }
        );
    }
}
