//! The `@ stub` exports of `dlls/gamingtcui/gamingtcui.spec`.
//!
//! winebuild routes each stub to `__wine_spec_unimplemented_stub(module, function)`,
//! which loops on `RaiseException(EXCEPTION_WINE_STUB, EXCEPTION_NONCONTINUABLE, 2, args)`
//! and never returns. The module string is the spec file name.

/// Module string passed to the stub: the spec file name from the `MODULE` makefile variable.
pub const MODULE_NAME: &str = "gamingtcui.dll";

/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The record that `__wine_spec_unimplemented_stub` raises. `number_parameters` is always 2:
/// the module and function strings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// `__wine_spec_unimplemented_stub`. The C version never returns, so this returns the record
/// it would raise.
pub fn unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        number_parameters: 2,
        module,
        function,
    }
}

/// Names of the 23 `@ stub` exports, in spec order.
pub const STUB_EXPORTS: [&str; 23] = [
    "CheckGamingPrivilegeSilently",
    "CheckGamingPrivilegeSilentlyForUser",
    "CheckGamingPrivilegeWithUI",
    "CheckGamingPrivilegeWithUIForUser",
    "ShowChangeFriendRelationshipUI",
    "ShowChangeFriendRelationshipUIForUser",
    "ShowCustomizeUserProfileUI",
    "ShowCustomizeUserProfileUIForUser",
    "ShowFindFriendsUI",
    "ShowFindFriendsUIForUser",
    "ShowGameInfoUI",
    "ShowGameInfoUIForUser",
    "ShowGameInviteUI",
    "ShowGameInviteUIForUser",
    "ShowGameInviteUIWithContext",
    "ShowGameInviteUIWithContextForUser",
    "ShowPlayerPickerUIForUser",
    "ShowProfileCardUIForUser",
    "ShowTitleAchievementsUI",
    "ShowTitleAchievementsUIForUser",
    "ShowUserSettingsUI",
    "ShowUserSettingsUIForUser",
    "TryCancelPendingGameUI",
];
