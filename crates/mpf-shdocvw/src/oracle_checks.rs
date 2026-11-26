//! Oracle-derived checks for the ported shdocvw logic.
//!
//! The rows were captured by compiling the verbatim Wine C bodies from `dlls/shdocvw` and
//! `dlls/ntdll/exception.c` with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/shdocvw/`, and running the harness. It prints 41 `EXPL` rows and 101 `STUB` rows,
//! 142 in total. ASan and UBSan reported nothing, so no vector is excluded. The stub recorder
//! uses `setjmp`/`longjmp` because the verbatim stub loops forever and never returns.

use super::*;
use std::cell::Cell;

/// `IID_Other` from the harness. Neither shdocvw nor its tests treat it as a known class or
/// interface.
const IID_OTHER: Guid = Guid {
    data1: 0x1234_5678,
    data2: 0x1234,
    data3: 0x1234,
    data4: [1, 2, 3, 4, 5, 6, 7, 8],
};

#[test]
fn oracle_e01_win_list_init() {
    assert_eq!(win_list_init(), 0xdead_feed);
}

#[test]
fn oracle_e02_run_install_uninstall_stubs() {
    assert_eq!(run_install_uninstall_stubs(), 0x0dea_dbee);
}

#[test]
fn oracle_e03_run_install_uninstall_stubs2() {
    // The harness passes arg=1, which the C body never reads.
    assert_eq!(run_install_uninstall_stubs2(), 0x0dea_dbee);
}

#[test]
fn oracle_e04_set_query_net_session_count() {
    // The harness passes arg=7, which the C body never reads.
    assert_eq!(set_query_net_session_count(), 0);
}

#[test]
fn oracle_e05_ie_parse_display_name_with_bcw() {
    assert_eq!(ie_parse_display_name_with_bcw(), 0x8000_4005);
}

#[test]
fn oracle_e06_sh_restricted2_w() {
    assert_eq!(sh_restricted2_w(), 0);
}

#[test]
fn oracle_e07_sh_restricted2_a() {
    assert_eq!(sh_restricted2_a(), 0);
}

#[test]
fn oracle_e08_import_privacy_settings_both_flags() {
    let mut global = true;
    let mut per_site = true;
    assert!(import_privacy_settings(
        Some(&mut global),
        Some(&mut per_site)
    ));
    assert!(!global);
    assert!(!per_site);
}

#[test]
fn oracle_e09_import_privacy_settings_no_flags() {
    assert!(import_privacy_settings(None, None));
}

#[test]
fn oracle_e10_import_privacy_settings_global_only() {
    let mut global = true;
    assert!(import_privacy_settings(Some(&mut global), None));
    assert!(!global);
}

#[test]
fn oracle_e11_reset_profile_sharing() {
    assert_eq!(reset_profile_sharing(), 0x8000_4001);
}

#[test]
fn oracle_e12_install_reg_run_dll() {
    // The C body returns void, so the row records only that the call returns.
    install_reg_run_dll();
}

#[test]
fn oracle_e13_do_file_download() {
    assert!(!do_file_download());
}

#[test]
fn oracle_e14_do_organize_fav_dlg_w() {
    assert!(!do_organize_fav_dlg_w());
}

#[test]
fn oracle_e15_do_organize_fav_dlg() {
    assert!(!do_organize_fav_dlg());
}

#[test]
fn oracle_e16_dll_install() {
    assert_eq!(dll_install(), 0);
}

#[test]
fn oracle_e17_dll_can_unload_now_no_references() {
    assert_eq!(dll_can_unload_now(0), 0);
}

#[test]
fn oracle_e18_dll_can_unload_now_two_references() {
    assert_eq!(dll_can_unload_now(2), 1);
}

#[test]
fn oracle_e19_to_e23_browser_classes_route_to_ieframe() {
    // E19 to E23: WebBrowser, WebBrowser_V1, InternetShortcut, CUrlHistory and TaskbarList.
    for clsid in [
        CLSID_WEB_BROWSER,
        CLSID_WEB_BROWSER_V1,
        CLSID_INTERNET_SHORTCUT,
        CLSID_CURL_HISTORY,
        CLSID_TASKBAR_LIST,
    ] {
        assert_eq!(dll_get_class_object_route(&clsid), ClassRoute::IeFrame);
    }
}

#[test]
fn oracle_e24_other_class_routes_to_shell_instance() {
    assert_eq!(
        dll_get_class_object_route(&IID_OTHER),
        ClassRoute::ShellInstance
    );
}

#[test]
fn oracle_e25_property_bag_query_interface_null_ppv() {
    assert_eq!(
        property_bag_query_interface(&IID_IUNKNOWN, false),
        0x8007_0057
    );
}

#[test]
fn oracle_e26_property_bag_query_interface_iunknown() {
    assert_eq!(property_bag_query_interface(&IID_IUNKNOWN, true), 0);
}

#[test]
fn oracle_e27_property_bag_query_interface_ipropertybag() {
    assert_eq!(property_bag_query_interface(&IID_IPROPERTY_BAG, true), 0);
}

#[test]
fn oracle_e28_property_bag_query_interface_other_iid() {
    assert_eq!(property_bag_query_interface(&IID_OTHER, true), 0x8000_4002);
}

#[test]
fn oracle_e29_property_bag_write() {
    assert_eq!(property_bag_write(), 0x8000_4001);
}

/// Calls `stop_watch_mode_forward` with a target that counts its calls, or with no target when
/// `present` is false. One closure expression serves both cases, so both arms share one instance.
fn stop_watch_mode(present: bool) -> u32 {
    let calls = Cell::new(0);
    let target = || calls.set(calls.get() + 1);
    stop_watch_mode_forward(|_| if present { Some(target) } else { None });
    calls.get()
}

/// The `StopWatchFlushFORWARD` counterpart of [`stop_watch_mode`].
fn stop_watch_flush(present: bool) -> u32 {
    let calls = Cell::new(0);
    let target = || calls.set(calls.get() + 1);
    stop_watch_flush_forward(|_| if present { Some(target) } else { None });
    calls.get()
}

/// `StopWatchAFORWARD` with the harness arguments `(1, "s", 2, 3, 4)`.
fn stop_watch_a(present: bool) -> u32 {
    let target = |class: u32, text: &str, unknown: u32, mode: u32, stamp: u32| {
        assert_eq!((class, text, unknown, mode, stamp), (1, "s", 2, 3, 4));
        class + unknown + mode + stamp
    };
    stop_watch_a_forward(
        1,
        "s",
        2,
        3,
        4,
        |_| if present { Some(target) } else { None },
    )
}

/// `StopWatchWFORWARD` with the harness arguments `(1, L"s", 2, 3, 4)`.
fn stop_watch_w(present: bool) -> u32 {
    let wide: [u16; 2] = [u16::from(b's'), 0];
    let target = |class: u32, text: &[u16], unknown: u32, mode: u32, stamp: u32| {
        assert_eq!((class, text, unknown, mode, stamp), (1, &wide[..], 2, 3, 4));
        class * 2 + unknown + mode + stamp
    };
    stop_watch_w_forward(1, &wide[..], 2, 3, 4, |_| {
        if present {
            Some(target)
        } else {
            None
        }
    })
}

/// `ShellDDEInit` with a target that records its `start` argument and returns TRUE. Returns the
/// result and the recorded argument, if the target ran.
fn shell_dde(shell32: bool, present: bool, start: bool) -> (bool, Option<bool>) {
    let seen = Cell::new(None);
    let target = |flag: bool| {
        seen.set(Some(flag));
        true
    };
    let ret = shell_dde_init(
        start,
        shell32,
        |_| if present { Some(target) } else { None },
    );
    (ret, seen.get())
}

#[test]
fn oracle_e30_stop_watch_mode_forward_absent() {
    // The shlwapi module is absent, so no target is found and nothing runs.
    assert_eq!(stop_watch_mode(false), 0);
}

#[test]
fn oracle_e31_stop_watch_flush_forward_absent() {
    assert_eq!(stop_watch_flush(false), 0);
}

#[test]
fn oracle_e32_stop_watch_a_forward_absent() {
    assert_eq!(stop_watch_a(false), 120);
}

#[test]
fn oracle_e33_stop_watch_w_forward_absent() {
    assert_eq!(stop_watch_w(false), 120);
}

#[test]
fn oracle_e34_shell_dde_init_shell32_absent() {
    assert_eq!(shell_dde(false, true, true), (false, None));
}

#[test]
fn oracle_e35_shell_dde_init_shell32_without_procedure() {
    assert_eq!(shell_dde(true, false, true), (false, None));
}

#[test]
fn oracle_e36_stop_watch_mode_forward_present() {
    assert_eq!(stop_watch_mode(true), 1);
}

#[test]
fn oracle_e37_stop_watch_flush_forward_present() {
    assert_eq!(stop_watch_flush(true), 1);
}

#[test]
fn oracle_e38_stop_watch_a_forward_present() {
    assert_eq!(stop_watch_a(true), 10);
}

#[test]
fn oracle_e39_stop_watch_w_forward_present() {
    assert_eq!(stop_watch_w(true), 11);
}

#[test]
fn oracle_e40_shell_dde_init_start_true() {
    assert_eq!(shell_dde(true, true, true), (true, Some(true)));
}

#[test]
fn oracle_e41_shell_dde_init_start_false() {
    assert_eq!(shell_dde(true, true, false), (true, Some(false)));
}

/// The `STUB` rows from the harness: the export, then the `StubException` fields it raises.
const STUB_ROWS: [(&str, u32, u32, u32, &str, &str); 101] = [
    (
        "CreateShortcutInDirA",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "CreateShortcutInDirA",
    ),
    (
        "CreateShortcutInDirW",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "CreateShortcutInDirW",
    ),
    (
        "CreateShortcutInDirEx",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "CreateShortcutInDirEx",
    ),
    (
        "HlinkFindFrame",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "HlinkFindFrame",
    ),
    (
        "SetShellOfflineState",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "SetShellOfflineState",
    ),
    (
        "AddUrlToFavorites",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "AddUrlToFavorites",
    ),
    (
        "WinList_Terminate",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "WinList_Terminate",
    ),
    (
        "CreateFromDesktop",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "CreateFromDesktop",
    ),
    (
        "DDECreatePostNotify",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "DDECreatePostNotify",
    ),
    (
        "DDEHandleViewFolderNotify",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "DDEHandleViewFolderNotify",
    ),
    (
        "SHCreateDesktop",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "SHCreateDesktop",
    ),
    (
        "SHDesktopMessageLoop",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "SHDesktopMessageLoop",
    ),
    (
        "SHCreateSplashScreen",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "SHCreateSplashScreen",
    ),
    ("IsFileUrl", 0x8000_0100, 1, 2, "shdocvw.dll", "IsFileUrl"),
    ("IsFileUrlW", 0x8000_0100, 1, 2, "shdocvw.dll", "IsFileUrlW"),
    (
        "PathIsFilePath",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "PathIsFilePath",
    ),
    (
        "URLSubLoadString",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "URLSubLoadString",
    ),
    (
        "OpenPidlOrderStream",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "OpenPidlOrderStream",
    ),
    ("DragDrop", 0x8000_0100, 1, 2, "shdocvw.dll", "DragDrop"),
    (
        "IEInvalidateImageList",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "IEInvalidateImageList",
    ),
    (
        "IEMapPIDLToSystemImageListIndex",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "IEMapPIDLToSystemImageListIndex",
    ),
    ("ILIsWeb", 0x8000_0100, 1, 2, "shdocvw.dll", "ILIsWeb"),
    (
        "IEGetAttributesOf",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "IEGetAttributesOf",
    ),
    (
        "IEBindToObject",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "IEBindToObject",
    ),
    (
        "IEGetNameAndFlags",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "IEGetNameAndFlags",
    ),
    (
        "IEGetDisplayName",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "IEGetDisplayName",
    ),
    (
        "IEBindToObjectEx",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "IEBindToObjectEx",
    ),
    (
        "_GetStdLocation",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "_GetStdLocation",
    ),
    (
        "URLSubRegQueryA",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "URLSubRegQueryA",
    ),
    (
        "CShellUIHelper_CreateInstance2",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "CShellUIHelper_CreateInstance2",
    ),
    ("IsURLChild", 0x8000_0100, 1, 2, "shdocvw.dll", "IsURLChild"),
    (
        "SHIsRestricted2W",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "SHIsRestricted2W",
    ),
    ("161", 0x8000_0100, 1, 2, "shdocvw.dll", "161"),
    (
        "CDDEAuto_Navigate",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "CDDEAuto_Navigate",
    ),
    (
        "SHAddSubscribeFavorite",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "SHAddSubscribeFavorite",
    ),
    (
        "URLSubstitution",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "URLSubstitution",
    ),
    (
        "IsIEDefaultBrowser",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "IsIEDefaultBrowser",
    ),
    (
        "ParseURLFromOutsideSourceA",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "ParseURLFromOutsideSourceA",
    ),
    (
        "ParseURLFromOutsideSourceW",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "ParseURLFromOutsideSourceW",
    ),
    (
        "_DeletePidlDPA",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "_DeletePidlDPA",
    ),
    (
        "IURLQualify",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "IURLQualify",
    ),
    (
        "SHIsRestricted",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "SHIsRestricted",
    ),
    (
        "SHIsGlobalOffline",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "SHIsGlobalOffline",
    ),
    (
        "DetectAndFixAssociations",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "DetectAndFixAssociations",
    ),
    (
        "EnsureWebViewRegSettings",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "EnsureWebViewRegSettings",
    ),
    (
        "WinList_NotifyNewLocation",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "WinList_NotifyNewLocation",
    ),
    (
        "WinList_FindFolderWindow",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "WinList_FindFolderWindow",
    ),
    (
        "WinList_GetShellWindows",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "WinList_GetShellWindows",
    ),
    (
        "WinList_RegisterPending",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "WinList_RegisterPending",
    ),
    (
        "WinList_Revoke",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "WinList_Revoke",
    ),
    (
        "SHMapNbspToSp",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "SHMapNbspToSp",
    ),
    (
        "FireEvent_Quit",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "FireEvent_Quit",
    ),
    (
        "SHDGetPageLocation",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "SHDGetPageLocation",
    ),
    (
        "SHIEErrorMsgBox",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "SHIEErrorMsgBox",
    ),
    ("189", 0x8000_0100, 1, 2, "shdocvw.dll", "189"),
    (
        "SHRunIndirectRegClientCommandForward",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "SHRunIndirectRegClientCommandForward",
    ),
    (
        "SHIsRegisteredClient",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "SHIsRegisteredClient",
    ),
    (
        "SHGetHistoryPIDL",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "SHGetHistoryPIDL",
    ),
    (
        "IECleanUpAutomationObject",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "IECleanUpAutomationObject",
    ),
    (
        "IEOnFirstBrowserCreation",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "IEOnFirstBrowserCreation",
    ),
    (
        "IEDDE_WindowDestroyed",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "IEDDE_WindowDestroyed",
    ),
    (
        "IEDDE_NewWindow",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "IEDDE_NewWindow",
    ),
    ("IsErrorUrl", 0x8000_0100, 1, 2, "shdocvw.dll", "IsErrorUrl"),
    ("199", 0x8000_0100, 1, 2, "shdocvw.dll", "199"),
    (
        "SHGetViewStream",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "SHGetViewStream",
    ),
    (
        "NavToUrlUsingIEA",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "NavToUrlUsingIEA",
    ),
    (
        "NavToUrlUsingIEW",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "NavToUrlUsingIEW",
    ),
    (
        "SearchForElementInHead",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "SearchForElementInHead",
    ),
    (
        "JITCoCreateInstance",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "JITCoCreateInstance",
    ),
    (
        "UrlHitsNetW",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "UrlHitsNetW",
    ),
    (
        "ClearAutoSuggestForForms",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "ClearAutoSuggestForForms",
    ),
    (
        "GetLinkInfo",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "GetLinkInfo",
    ),
    (
        "UseCustomInternetSearch",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "UseCustomInternetSearch",
    ),
    (
        "GetSearchAssistantUrlW",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "GetSearchAssistantUrlW",
    ),
    (
        "GetSearchAssistantUrlA",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "GetSearchAssistantUrlA",
    ),
    (
        "GetDefaultInternetSearchUrlW",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "GetDefaultInternetSearchUrlW",
    ),
    (
        "GetDefaultInternetSearchUrlA",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "GetDefaultInternetSearchUrlA",
    ),
    (
        "IEILIsEqual",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "IEILIsEqual",
    ),
    ("220", 0x8000_0100, 1, 2, "shdocvw.dll", "220"),
    (
        "IECreateFromPathCPWithBCA",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "IECreateFromPathCPWithBCA",
    ),
    (
        "IECreateFromPathCPWithBCW",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "IECreateFromPathCPWithBCW",
    ),
    (
        "ResetWebSettings",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "ResetWebSettings",
    ),
    (
        "IsResetWebSettingsRequired",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "IsResetWebSettingsRequired",
    ),
    (
        "PrepareURLForDisplayUTF8W",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "PrepareURLForDisplayUTF8W",
    ),
    (
        "IEIsLinkSafe",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "IEIsLinkSafe",
    ),
    (
        "SHUseClassicToolbarGlyphs",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "SHUseClassicToolbarGlyphs",
    ),
    (
        "SafeOpenPromptForShellExec",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "SafeOpenPromptForShellExec",
    ),
    (
        "SafeOpenPromptForPackager",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "SafeOpenPromptForPackager",
    ),
    (
        "DllRegisterWindowClasses",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "DllRegisterWindowClasses",
    ),
    (
        "DoAddToFavDlg",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "DoAddToFavDlg",
    ),
    (
        "DoAddToFavDlgW",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "DoAddToFavDlgW",
    ),
    (
        "DoFileDownloadEx",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "DoFileDownloadEx",
    ),
    (
        "DoPrivacyDlg",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "DoPrivacyDlg",
    ),
    (
        "HlinkFrameNavigate",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "HlinkFrameNavigate",
    ),
    (
        "HlinkFrameNavigateNHL",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "HlinkFrameNavigateNHL",
    ),
    ("IEAboutBox", 0x8000_0100, 1, 2, "shdocvw.dll", "IEAboutBox"),
    (
        "IEWriteErrorLog",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "IEWriteErrorLog",
    ),
    (
        "SHGetIDispatchForFolder",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "SHGetIDispatchForFolder",
    ),
    (
        "SoftwareUpdateMessageBox",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "SoftwareUpdateMessageBox",
    ),
    (
        "URLQualifyA",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "URLQualifyA",
    ),
    (
        "URLQualifyW",
        0x8000_0100,
        1,
        2,
        "shdocvw.dll",
        "URLQualifyW",
    ),
];

#[test]
fn oracle_stub_rows_match_stub_for_export() {
    for (key, code, flags, number_parameters, module, function) in STUB_ROWS {
        assert_eq!(
            stub_for_export(key),
            Some(StubException {
                code,
                flags,
                number_parameters,
                module,
                function,
            }),
            "export {key}"
        );
    }
}
