//! Exports whose C body returns a fixed value or only clears its out-parameters, and the
//! `HRESULT` and error values they use.

/// `S_OK` from `include/winerror.h`.
pub const S_OK: u32 = 0;
/// `S_FALSE` from `include/winerror.h`.
pub const S_FALSE: u32 = 1;
/// `E_NOINTERFACE` from `include/winerror.h`.
pub const E_NOINTERFACE: u32 = 0x8000_4002;
/// `E_NOTIMPL` from `include/winerror.h`.
pub const E_NOTIMPL: u32 = 0x8000_4001;
/// `E_FAIL` from `include/winerror.h`.
pub const E_FAIL: u32 = 0x8000_4005;
/// `E_INVALIDARG` from `include/winerror.h`.
pub const E_INVALIDARG: u32 = 0x8007_0057;
/// `ERROR_CALL_NOT_IMPLEMENTED` from `include/winerror.h`.
pub const ERROR_CALL_NOT_IMPLEMENTED: u32 = 120;

/// `WinList_Init`: returns the marker `0x0deadfeed`.
pub fn win_list_init() -> u32 {
    0x0dea_dfeed
}

/// `RunInstallUninstallStubs`: returns the marker `0x0deadbee`.
pub fn run_install_uninstall_stubs() -> u32 {
    0x0dea_dbee
}

/// `RunInstallUninstallStubs2`: returns the marker `0x0deadbee`. The `arg` is never read.
pub fn run_install_uninstall_stubs2() -> u32 {
    0x0dea_dbee
}

/// `SetQueryNetSessionCount`: returns 0. The `arg` is never read.
pub fn set_query_net_session_count() -> u32 {
    0
}

/// `IEParseDisplayNameWithBCW`: returns `E_FAIL`. The arguments are never read.
pub fn ie_parse_display_name_with_bcw() -> u32 {
    E_FAIL
}

/// `SHRestricted2W`: returns 0. The arguments are never read.
pub fn sh_restricted2_w() -> u32 {
    0
}

/// `SHRestricted2A`: converts the URL and returns the result of `SHRestricted2W`, which is 0.
pub fn sh_restricted2_a() -> u32 {
    sh_restricted2_w()
}

/// `ResetProfileSharing`: returns `E_NOTIMPL`.
pub fn reset_profile_sharing() -> u32 {
    E_NOTIMPL
}

/// `InstallReg_RunDLL`: does nothing.
pub fn install_reg_run_dll() {}

/// `DoFileDownload`: returns `FALSE`.
pub fn do_file_download() -> bool {
    false
}

/// `DoOrganizeFavDlgW`: returns `FALSE`.
pub fn do_organize_fav_dlg_w() -> bool {
    false
}

/// `DoOrganizeFavDlg`: converts the directory and returns the result of `DoOrganizeFavDlgW`.
pub fn do_organize_fav_dlg() -> bool {
    do_organize_fav_dlg_w()
}

/// `ImportPrivacySettings`: clears each flag that was passed in and returns `TRUE`. The file
/// name is never read.
pub fn import_privacy_settings(global: Option<&mut bool>, per_site: Option<&mut bool>) -> bool {
    if let Some(flag) = global {
        *flag = false;
    }
    if let Some(flag) = per_site {
        *flag = false;
    }
    true
}
