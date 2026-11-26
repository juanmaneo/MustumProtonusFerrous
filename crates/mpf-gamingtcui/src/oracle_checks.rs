//! Oracle-derived test vectors for the ported gamingtcui logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies from
//! `dlls/gamingtcui/gamingtcui_main.c` and `libs/winecrt0/stub.c` with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/gamingtcui/` and running
//! the harness. The harness prints one row per export: 26 rows (3 implemented, 23 `@ stub`).
//! ASan and UBSan flagged no row, so none is excluded.

use super::*;

/// `(export, code, flags, nargs)` for each `@ stub` export, in spec order, as printed by the harness.
const STUB_ROWS: [(&str, u32, u32, u32); 23] = [
    ("CheckGamingPrivilegeSilently", 0x8000_0100, 0x01, 2),
    ("CheckGamingPrivilegeSilentlyForUser", 0x8000_0100, 0x01, 2),
    ("CheckGamingPrivilegeWithUI", 0x8000_0100, 0x01, 2),
    ("CheckGamingPrivilegeWithUIForUser", 0x8000_0100, 0x01, 2),
    ("ShowChangeFriendRelationshipUI", 0x8000_0100, 0x01, 2),
    (
        "ShowChangeFriendRelationshipUIForUser",
        0x8000_0100,
        0x01,
        2,
    ),
    ("ShowCustomizeUserProfileUI", 0x8000_0100, 0x01, 2),
    ("ShowCustomizeUserProfileUIForUser", 0x8000_0100, 0x01, 2),
    ("ShowFindFriendsUI", 0x8000_0100, 0x01, 2),
    ("ShowFindFriendsUIForUser", 0x8000_0100, 0x01, 2),
    ("ShowGameInfoUI", 0x8000_0100, 0x01, 2),
    ("ShowGameInfoUIForUser", 0x8000_0100, 0x01, 2),
    ("ShowGameInviteUI", 0x8000_0100, 0x01, 2),
    ("ShowGameInviteUIForUser", 0x8000_0100, 0x01, 2),
    ("ShowGameInviteUIWithContext", 0x8000_0100, 0x01, 2),
    ("ShowGameInviteUIWithContextForUser", 0x8000_0100, 0x01, 2),
    ("ShowPlayerPickerUIForUser", 0x8000_0100, 0x01, 2),
    ("ShowProfileCardUIForUser", 0x8000_0100, 0x01, 2),
    ("ShowTitleAchievementsUI", 0x8000_0100, 0x01, 2),
    ("ShowTitleAchievementsUIForUser", 0x8000_0100, 0x01, 2),
    ("ShowUserSettingsUI", 0x8000_0100, 0x01, 2),
    ("ShowUserSettingsUIForUser", 0x8000_0100, 0x01, 2),
    ("TryCancelPendingGameUI", 0x8000_0100, 0x01, 2),
];

#[test]
fn oracle_process_pending_game_ui_row() {
    // ProcessPendingGameUI wait=1 -> rc=0
    assert_eq!(process_pending_game_ui(1), 0);
}

#[test]
fn oracle_show_player_picker_ui_row() {
    // ShowPlayerPickerUI prompt=0x1000 xuids=0x2000/3 preselected=0x3000/2 min=1 max=4
    //   completion=1 ctx=0x4000 -> rc=0 calls=1 completion(rc=0 ctx=0x4000 selected=0x3000 count=2)
    let mut calls: Vec<(Hresult, Handle, HStringArray)> = Vec::new();
    let rc = show_player_picker_ui(
        0x1000,
        HStringArray {
            ptr: 0x2000,
            count: 3,
        },
        HStringArray {
            ptr: 0x3000,
            count: 2,
        },
        1,
        4,
        Some(|rc, ctx, selected| calls.push((rc, ctx, selected))),
        0x4000,
    );
    assert_eq!(rc, 0);
    assert_eq!(
        calls,
        vec![(
            0,
            0x4000,
            HStringArray {
                ptr: 0x3000,
                count: 2
            }
        )]
    );
}

#[test]
fn oracle_show_profile_card_ui_row() {
    // ShowProfileCardUI target=0x5000 completion=1 ctx=0x6000 -> rc=0 calls=1 completion(rc=0 ctx=0x6000)
    let mut calls: Vec<(Hresult, Handle)> = Vec::new();
    let rc = show_profile_card_ui(0x5000, Some(|rc, ctx| calls.push((rc, ctx))), 0x6000);
    assert_eq!(rc, 0);
    assert_eq!(calls, vec![(0, 0x6000)]);
}

#[test]
fn oracle_stub_rows() {
    for (export, code, flags, nargs) in STUB_ROWS {
        assert_eq!(
            unimplemented_stub(MODULE_NAME, export),
            StubException {
                code,
                flags,
                number_parameters: nargs,
                module: "gamingtcui.dll",
                function: export,
            }
        );
    }
}
