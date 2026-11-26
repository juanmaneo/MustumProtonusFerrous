/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = shdocvw.dll` in `dlls/shdocvw/Makefile.in`.
const SHDOCVW_DLL_NAME: &str = "shdocvw.dll";

/// The record that `__wine_spec_unimplemented_stub` raises.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`. The C function raises
/// the record in a loop and never returns, so the port returns the record instead.
pub fn wine_spec_unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        // The module and the function name.
        number_parameters: 2,
        module,
        function,
    }
}

/// The `@ stub` export helpers. Each one passes the export name, or the decimal ordinal when the
/// export is nameless, as `winebuild` does.
/// `CreateShortcutInDirA`, a `stub` export.
pub fn create_shortcut_in_dir_a() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "CreateShortcutInDirA")
}

/// `CreateShortcutInDirW`, a `stub` export.
pub fn create_shortcut_in_dir_w() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "CreateShortcutInDirW")
}

/// `CreateShortcutInDirEx`, a `stub` export.
pub fn create_shortcut_in_dir_ex() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "CreateShortcutInDirEx")
}

/// `HlinkFindFrame`, a `stub` export.
pub fn hlink_find_frame() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "HlinkFindFrame")
}

/// `SetShellOfflineState`, a `stub` export.
pub fn set_shell_offline_state() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "SetShellOfflineState")
}

/// `AddUrlToFavorites`, a `stub` export.
pub fn add_url_to_favorites() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "AddUrlToFavorites")
}

/// `WinList_Terminate`, a `stub` export.
pub fn win_list_terminate() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "WinList_Terminate")
}

/// `CreateFromDesktop`, a `stub` export.
pub fn create_from_desktop() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "CreateFromDesktop")
}

/// `DDECreatePostNotify`, a `stub` export.
pub fn dde_create_post_notify() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "DDECreatePostNotify")
}

/// `DDEHandleViewFolderNotify`, a `stub` export.
pub fn dde_handle_view_folder_notify() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "DDEHandleViewFolderNotify")
}

/// `SHCreateDesktop`, a `stub` export.
pub fn sh_create_desktop() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "SHCreateDesktop")
}

/// `SHDesktopMessageLoop`, a `stub` export.
pub fn sh_desktop_message_loop() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "SHDesktopMessageLoop")
}

/// `SHCreateSplashScreen`, a `stub` export.
pub fn sh_create_splash_screen() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "SHCreateSplashScreen")
}

/// `IsFileUrl`, a `stub` export.
pub fn is_file_url() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "IsFileUrl")
}

/// `IsFileUrlW`, a `stub` export.
pub fn is_file_url_w() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "IsFileUrlW")
}

/// `PathIsFilePath`, a `stub` export.
pub fn path_is_file_path() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "PathIsFilePath")
}

/// `URLSubLoadString`, a `stub` export.
pub fn url_sub_load_string() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "URLSubLoadString")
}

/// `OpenPidlOrderStream`, a `stub` export.
pub fn open_pidl_order_stream() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "OpenPidlOrderStream")
}

/// `DragDrop`, a `stub` export.
pub fn drag_drop() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "DragDrop")
}

/// `IEInvalidateImageList`, a `stub` export.
pub fn ie_invalidate_image_list() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "IEInvalidateImageList")
}

/// `IEMapPIDLToSystemImageListIndex`, a `stub` export.
pub fn ie_map_pidl_to_system_image_list_index() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "IEMapPIDLToSystemImageListIndex")
}

/// `ILIsWeb`, a `stub` export.
pub fn il_is_web() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "ILIsWeb")
}

/// `IEGetAttributesOf`, a `stub` export.
pub fn ie_get_attributes_of() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "IEGetAttributesOf")
}

/// `IEBindToObject`, a `stub` export.
pub fn ie_bind_to_object() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "IEBindToObject")
}

/// `IEGetNameAndFlags`, a `stub` export.
pub fn ie_get_name_and_flags() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "IEGetNameAndFlags")
}

/// `IEGetDisplayName`, a `stub` export.
pub fn ie_get_display_name() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "IEGetDisplayName")
}

/// `IEBindToObjectEx`, a `stub` export.
pub fn ie_bind_to_object_ex() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "IEBindToObjectEx")
}

/// `_GetStdLocation`, a `stub` export.
pub fn get_std_location() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "_GetStdLocation")
}

/// `URLSubRegQueryA`, a `stub` export.
pub fn url_sub_reg_query_a() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "URLSubRegQueryA")
}

/// `CShellUIHelper_CreateInstance2`, a `stub` export.
pub fn c_shell_ui_helper_create_instance2() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "CShellUIHelper_CreateInstance2")
}

/// `IsURLChild`, a `stub` export.
pub fn is_url_child() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "IsURLChild")
}

/// `SHIsRestricted2W`, a `stub` export.
pub fn sh_is_restricted2_w() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "SHIsRestricted2W")
}

/// Ordinal 161, a nameless `stub` export, so the function is the ordinal.
pub fn ordinal_161() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "161")
}

/// `CDDEAuto_Navigate`, a `stub` export.
pub fn cdde_auto_navigate() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "CDDEAuto_Navigate")
}

/// `SHAddSubscribeFavorite`, a `stub` export.
pub fn sh_add_subscribe_favorite() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "SHAddSubscribeFavorite")
}

/// `URLSubstitution`, a `stub` export.
pub fn url_substitution() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "URLSubstitution")
}

/// `IsIEDefaultBrowser`, a `stub` export.
pub fn is_ie_default_browser() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "IsIEDefaultBrowser")
}

/// `ParseURLFromOutsideSourceA`, a `stub` export.
pub fn parse_url_from_outside_source_a() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "ParseURLFromOutsideSourceA")
}

/// `ParseURLFromOutsideSourceW`, a `stub` export.
pub fn parse_url_from_outside_source_w() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "ParseURLFromOutsideSourceW")
}

/// `_DeletePidlDPA`, a `stub` export.
pub fn delete_pidl_dpa() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "_DeletePidlDPA")
}

/// `IURLQualify`, a `stub` export.
pub fn iurl_qualify() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "IURLQualify")
}

/// `SHIsRestricted`, a `stub` export.
pub fn sh_is_restricted() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "SHIsRestricted")
}

/// `SHIsGlobalOffline`, a `stub` export.
pub fn sh_is_global_offline() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "SHIsGlobalOffline")
}

/// `DetectAndFixAssociations`, a `stub` export.
pub fn detect_and_fix_associations() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "DetectAndFixAssociations")
}

/// `EnsureWebViewRegSettings`, a `stub` export.
pub fn ensure_web_view_reg_settings() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "EnsureWebViewRegSettings")
}

/// `WinList_NotifyNewLocation`, a `stub` export.
pub fn win_list_notify_new_location() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "WinList_NotifyNewLocation")
}

/// `WinList_FindFolderWindow`, a `stub` export.
pub fn win_list_find_folder_window() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "WinList_FindFolderWindow")
}

/// `WinList_GetShellWindows`, a `stub` export.
pub fn win_list_get_shell_windows() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "WinList_GetShellWindows")
}

/// `WinList_RegisterPending`, a `stub` export.
pub fn win_list_register_pending() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "WinList_RegisterPending")
}

/// `WinList_Revoke`, a `stub` export.
pub fn win_list_revoke() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "WinList_Revoke")
}

/// `SHMapNbspToSp`, a `stub` export.
pub fn sh_map_nbsp_to_sp() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "SHMapNbspToSp")
}

/// `FireEvent_Quit`, a `stub` export.
pub fn fire_event_quit() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "FireEvent_Quit")
}

/// `SHDGetPageLocation`, a `stub` export.
pub fn shd_get_page_location() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "SHDGetPageLocation")
}

/// `SHIEErrorMsgBox`, a `stub` export.
pub fn shie_error_msg_box() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "SHIEErrorMsgBox")
}

/// Ordinal 189, a nameless `stub` export, so the function is the ordinal.
pub fn ordinal_189() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "189")
}

/// `SHRunIndirectRegClientCommandForward`, a `stub` export.
pub fn sh_run_indirect_reg_client_command_forward() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "SHRunIndirectRegClientCommandForward")
}

/// `SHIsRegisteredClient`, a `stub` export.
pub fn sh_is_registered_client() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "SHIsRegisteredClient")
}

/// `SHGetHistoryPIDL`, a `stub` export.
pub fn sh_get_history_pidl() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "SHGetHistoryPIDL")
}

/// `IECleanUpAutomationObject`, a `stub` export.
pub fn ie_clean_up_automation_object() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "IECleanUpAutomationObject")
}

/// `IEOnFirstBrowserCreation`, a `stub` export.
pub fn ie_on_first_browser_creation() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "IEOnFirstBrowserCreation")
}

/// `IEDDE_WindowDestroyed`, a `stub` export.
pub fn iedde_window_destroyed() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "IEDDE_WindowDestroyed")
}

/// `IEDDE_NewWindow`, a `stub` export.
pub fn iedde_new_window() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "IEDDE_NewWindow")
}

/// `IsErrorUrl`, a `stub` export.
pub fn is_error_url() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "IsErrorUrl")
}

/// Ordinal 199, a nameless `stub` export, so the function is the ordinal.
pub fn ordinal_199() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "199")
}

/// `SHGetViewStream`, a `stub` export.
pub fn sh_get_view_stream() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "SHGetViewStream")
}

/// `NavToUrlUsingIEA`, a `stub` export.
pub fn nav_to_url_using_iea() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "NavToUrlUsingIEA")
}

/// `NavToUrlUsingIEW`, a `stub` export.
pub fn nav_to_url_using_iew() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "NavToUrlUsingIEW")
}

/// `SearchForElementInHead`, a `stub` export.
pub fn search_for_element_in_head() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "SearchForElementInHead")
}

/// `JITCoCreateInstance`, a `stub` export.
pub fn jit_co_create_instance() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "JITCoCreateInstance")
}

/// `UrlHitsNetW`, a `stub` export.
pub fn url_hits_net_w() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "UrlHitsNetW")
}

/// `ClearAutoSuggestForForms`, a `stub` export.
pub fn clear_auto_suggest_for_forms() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "ClearAutoSuggestForForms")
}

/// `GetLinkInfo`, a `stub` export.
pub fn get_link_info() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "GetLinkInfo")
}

/// `UseCustomInternetSearch`, a `stub` export.
pub fn use_custom_internet_search() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "UseCustomInternetSearch")
}

/// `GetSearchAssistantUrlW`, a `stub` export.
pub fn get_search_assistant_url_w() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "GetSearchAssistantUrlW")
}

/// `GetSearchAssistantUrlA`, a `stub` export.
pub fn get_search_assistant_url_a() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "GetSearchAssistantUrlA")
}

/// `GetDefaultInternetSearchUrlW`, a `stub` export.
pub fn get_default_internet_search_url_w() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "GetDefaultInternetSearchUrlW")
}

/// `GetDefaultInternetSearchUrlA`, a `stub` export.
pub fn get_default_internet_search_url_a() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "GetDefaultInternetSearchUrlA")
}

/// `IEILIsEqual`, a `stub` export.
pub fn ieil_is_equal() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "IEILIsEqual")
}

/// Ordinal 220, a nameless `stub` export, so the function is the ordinal.
pub fn ordinal_220() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "220")
}

/// `IECreateFromPathCPWithBCA`, a `stub` export.
pub fn ie_create_from_path_cp_with_bca() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "IECreateFromPathCPWithBCA")
}

/// `IECreateFromPathCPWithBCW`, a `stub` export.
pub fn ie_create_from_path_cp_with_bcw() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "IECreateFromPathCPWithBCW")
}

/// `ResetWebSettings`, a `stub` export.
pub fn reset_web_settings() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "ResetWebSettings")
}

/// `IsResetWebSettingsRequired`, a `stub` export.
pub fn is_reset_web_settings_required() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "IsResetWebSettingsRequired")
}

/// `PrepareURLForDisplayUTF8W`, a `stub` export.
pub fn prepare_url_for_display_utf8_w() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "PrepareURLForDisplayUTF8W")
}

/// `IEIsLinkSafe`, a `stub` export.
pub fn ie_is_link_safe() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "IEIsLinkSafe")
}

/// `SHUseClassicToolbarGlyphs`, a `stub` export.
pub fn sh_use_classic_toolbar_glyphs() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "SHUseClassicToolbarGlyphs")
}

/// `SafeOpenPromptForShellExec`, a `stub` export.
pub fn safe_open_prompt_for_shell_exec() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "SafeOpenPromptForShellExec")
}

/// `SafeOpenPromptForPackager`, a `stub` export.
pub fn safe_open_prompt_for_packager() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "SafeOpenPromptForPackager")
}

/// `DllRegisterWindowClasses`, a `stub` export.
pub fn dll_register_window_classes() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "DllRegisterWindowClasses")
}

/// `DoAddToFavDlg`, a `stub` export.
pub fn do_add_to_fav_dlg() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "DoAddToFavDlg")
}

/// `DoAddToFavDlgW`, a `stub` export.
pub fn do_add_to_fav_dlg_w() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "DoAddToFavDlgW")
}

/// `DoFileDownloadEx`, a `stub` export.
pub fn do_file_download_ex() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "DoFileDownloadEx")
}

/// `DoPrivacyDlg`, a `stub` export.
pub fn do_privacy_dlg() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "DoPrivacyDlg")
}

/// `HlinkFrameNavigate`, a `stub` export.
pub fn hlink_frame_navigate() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "HlinkFrameNavigate")
}

/// `HlinkFrameNavigateNHL`, a `stub` export.
pub fn hlink_frame_navigate_nhl() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "HlinkFrameNavigateNHL")
}

/// `IEAboutBox`, a `stub` export.
pub fn ie_about_box() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "IEAboutBox")
}

/// `IEWriteErrorLog`, a `stub` export.
pub fn ie_write_error_log() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "IEWriteErrorLog")
}

/// `SHGetIDispatchForFolder`, a `stub` export.
pub fn sh_get_i_dispatch_for_folder() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "SHGetIDispatchForFolder")
}

/// `SoftwareUpdateMessageBox`, a `stub` export.
pub fn software_update_message_box() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "SoftwareUpdateMessageBox")
}

/// `URLQualifyA`, a `stub` export.
pub fn url_qualify_a() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "URLQualifyA")
}

/// `URLQualifyW`, a `stub` export.
pub fn url_qualify_w() -> StubException {
    wine_spec_unimplemented_stub(SHDOCVW_DLL_NAME, "URLQualifyW")
}

/// The stub record for export `name` (its key in `EXPORTS`), or `None` when it is not a stub.
pub fn stub_for_export(name: &str) -> Option<StubException> {
    let stub = match name {
        "CreateShortcutInDirA" => create_shortcut_in_dir_a(),
        "CreateShortcutInDirW" => create_shortcut_in_dir_w(),
        "CreateShortcutInDirEx" => create_shortcut_in_dir_ex(),
        "HlinkFindFrame" => hlink_find_frame(),
        "SetShellOfflineState" => set_shell_offline_state(),
        "AddUrlToFavorites" => add_url_to_favorites(),
        "WinList_Terminate" => win_list_terminate(),
        "CreateFromDesktop" => create_from_desktop(),
        "DDECreatePostNotify" => dde_create_post_notify(),
        "DDEHandleViewFolderNotify" => dde_handle_view_folder_notify(),
        "SHCreateDesktop" => sh_create_desktop(),
        "SHDesktopMessageLoop" => sh_desktop_message_loop(),
        "SHCreateSplashScreen" => sh_create_splash_screen(),
        "IsFileUrl" => is_file_url(),
        "IsFileUrlW" => is_file_url_w(),
        "PathIsFilePath" => path_is_file_path(),
        "URLSubLoadString" => url_sub_load_string(),
        "OpenPidlOrderStream" => open_pidl_order_stream(),
        "DragDrop" => drag_drop(),
        "IEInvalidateImageList" => ie_invalidate_image_list(),
        "IEMapPIDLToSystemImageListIndex" => ie_map_pidl_to_system_image_list_index(),
        "ILIsWeb" => il_is_web(),
        "IEGetAttributesOf" => ie_get_attributes_of(),
        "IEBindToObject" => ie_bind_to_object(),
        "IEGetNameAndFlags" => ie_get_name_and_flags(),
        "IEGetDisplayName" => ie_get_display_name(),
        "IEBindToObjectEx" => ie_bind_to_object_ex(),
        "_GetStdLocation" => get_std_location(),
        "URLSubRegQueryA" => url_sub_reg_query_a(),
        "CShellUIHelper_CreateInstance2" => c_shell_ui_helper_create_instance2(),
        "IsURLChild" => is_url_child(),
        "SHIsRestricted2W" => sh_is_restricted2_w(),
        "161" => ordinal_161(),
        "CDDEAuto_Navigate" => cdde_auto_navigate(),
        "SHAddSubscribeFavorite" => sh_add_subscribe_favorite(),
        "URLSubstitution" => url_substitution(),
        "IsIEDefaultBrowser" => is_ie_default_browser(),
        "ParseURLFromOutsideSourceA" => parse_url_from_outside_source_a(),
        "ParseURLFromOutsideSourceW" => parse_url_from_outside_source_w(),
        "_DeletePidlDPA" => delete_pidl_dpa(),
        "IURLQualify" => iurl_qualify(),
        "SHIsRestricted" => sh_is_restricted(),
        "SHIsGlobalOffline" => sh_is_global_offline(),
        "DetectAndFixAssociations" => detect_and_fix_associations(),
        "EnsureWebViewRegSettings" => ensure_web_view_reg_settings(),
        "WinList_NotifyNewLocation" => win_list_notify_new_location(),
        "WinList_FindFolderWindow" => win_list_find_folder_window(),
        "WinList_GetShellWindows" => win_list_get_shell_windows(),
        "WinList_RegisterPending" => win_list_register_pending(),
        "WinList_Revoke" => win_list_revoke(),
        "SHMapNbspToSp" => sh_map_nbsp_to_sp(),
        "FireEvent_Quit" => fire_event_quit(),
        "SHDGetPageLocation" => shd_get_page_location(),
        "SHIEErrorMsgBox" => shie_error_msg_box(),
        "189" => ordinal_189(),
        "SHRunIndirectRegClientCommandForward" => sh_run_indirect_reg_client_command_forward(),
        "SHIsRegisteredClient" => sh_is_registered_client(),
        "SHGetHistoryPIDL" => sh_get_history_pidl(),
        "IECleanUpAutomationObject" => ie_clean_up_automation_object(),
        "IEOnFirstBrowserCreation" => ie_on_first_browser_creation(),
        "IEDDE_WindowDestroyed" => iedde_window_destroyed(),
        "IEDDE_NewWindow" => iedde_new_window(),
        "IsErrorUrl" => is_error_url(),
        "199" => ordinal_199(),
        "SHGetViewStream" => sh_get_view_stream(),
        "NavToUrlUsingIEA" => nav_to_url_using_iea(),
        "NavToUrlUsingIEW" => nav_to_url_using_iew(),
        "SearchForElementInHead" => search_for_element_in_head(),
        "JITCoCreateInstance" => jit_co_create_instance(),
        "UrlHitsNetW" => url_hits_net_w(),
        "ClearAutoSuggestForForms" => clear_auto_suggest_for_forms(),
        "GetLinkInfo" => get_link_info(),
        "UseCustomInternetSearch" => use_custom_internet_search(),
        "GetSearchAssistantUrlW" => get_search_assistant_url_w(),
        "GetSearchAssistantUrlA" => get_search_assistant_url_a(),
        "GetDefaultInternetSearchUrlW" => get_default_internet_search_url_w(),
        "GetDefaultInternetSearchUrlA" => get_default_internet_search_url_a(),
        "IEILIsEqual" => ieil_is_equal(),
        "220" => ordinal_220(),
        "IECreateFromPathCPWithBCA" => ie_create_from_path_cp_with_bca(),
        "IECreateFromPathCPWithBCW" => ie_create_from_path_cp_with_bcw(),
        "ResetWebSettings" => reset_web_settings(),
        "IsResetWebSettingsRequired" => is_reset_web_settings_required(),
        "PrepareURLForDisplayUTF8W" => prepare_url_for_display_utf8_w(),
        "IEIsLinkSafe" => ie_is_link_safe(),
        "SHUseClassicToolbarGlyphs" => sh_use_classic_toolbar_glyphs(),
        "SafeOpenPromptForShellExec" => safe_open_prompt_for_shell_exec(),
        "SafeOpenPromptForPackager" => safe_open_prompt_for_packager(),
        "DllRegisterWindowClasses" => dll_register_window_classes(),
        "DoAddToFavDlg" => do_add_to_fav_dlg(),
        "DoAddToFavDlgW" => do_add_to_fav_dlg_w(),
        "DoFileDownloadEx" => do_file_download_ex(),
        "DoPrivacyDlg" => do_privacy_dlg(),
        "HlinkFrameNavigate" => hlink_frame_navigate(),
        "HlinkFrameNavigateNHL" => hlink_frame_navigate_nhl(),
        "IEAboutBox" => ie_about_box(),
        "IEWriteErrorLog" => ie_write_error_log(),
        "SHGetIDispatchForFolder" => sh_get_i_dispatch_for_folder(),
        "SoftwareUpdateMessageBox" => software_update_message_box(),
        "URLQualifyA" => url_qualify_a(),
        "URLQualifyW" => url_qualify_w(),
        _ => return None,
    };
    Some(stub)
}
