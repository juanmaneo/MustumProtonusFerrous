//! Port of the portable logic in Wine's `dlls/gameux/gameexplorer.c` and `dlls/gameux/gamestatistics.c`.
//!
//! Strings are UTF-16 code units (`&[u16]`), as in Wine. Like `lstrlenW` and
//! `lstrcatW`, each input string ends at its first NUL.
//!
//! Ported:
//! - `GAMEUX_buildGameRegistryPath`: scope selection, SID suffix, and GUID
//!   formatting: [`build_game_registry_path`], [`Guid`].
//! - `GAMEUX_buildStatisticsFilePath`: [`build_statistics_file_path`].
//! - The directory part of `GAMEUX_createStatsDirectory`: [`stats_directory`].
//! - `IGameStatistics` limits and the category/statistic store (`SetCategoryTitle`,
//!   `GetCategoryTitle`, `SetStatistic`, `GetStatistic`): [`GameStatistics`] and the `MAX_*` constants.
//!
//! Skipped:
//! - `GAMEUX_ParseGameDefinition`, `GAMEUX_ProcessGameDefinitionElement`,
//!   `GAMEUX_updateStatisticsFile`, `GAMEUX_loadStatisticsFromFile`, and
//!   `GameStatisticsImpl_Save`: XML DOM through COM.
//! - `GAMEUX_WriteRegistryRecord`, `GAMEUX_RemoveRegistryRecord`, `GAMEUX_RegisterGame`,
//!   `GAMEUX_UpdateGame`, `GAMEUX_IsGameKeyExist`, `GAMEUX_LoadRegistryString`,
//!   `GAMEUX_FindGameInstanceId`, `GAMEUX_getAppIdFromGDFPath`: registry access.
//! - `GAMEUX_ParseGDFBinary`: thread and file loading.
//! - `GAMEUX_createStatsDirectory` directory creation and its `S_FALSE` mapping: `SHCreateDirectoryExW`.
//! - The token and SID lookup in `GAMEUX_buildGameRegistryPath` (`OpenProcessToken`,
//!   `GetTokenInformation`, `ConvertSidToStringSidW`): the caller supplies the SID.
//! - The folder lookup in `GAMEUX_buildStatisticsFilePath` (`SHGetFolderPathW`): the caller supplies the folder.
//! - `GAMEUX_getAppIdFromGDFPath` scope fallback: depends on the registry lookup.
//! - `GAMEUX_initGameData` and `GAMEUX_uninitGameData`: memory management only.
//! - COM vtables, class factories, reference counting, `CoTaskMemAlloc` ownership,
//!   and the `FIXME` stubs (`SetLastPlayedCategory`, `GetLastPlayedCategory`).
//! - The `GetMax*Length`/`GetMax*` getters: they only return the `MAX_*` constants below.

mod paths;
mod registry;
mod stats;

pub use paths::{build_statistics_file_path, stats_directory};
pub use registry::{build_game_registry_path, GIS_ALL_USERS, GIS_CURRENT_USER};
pub use stats::{
    Category, GameStatistics, Stat, StatText, MAX_CATEGORIES, MAX_CATEGORY_LENGTH, MAX_NAME_LENGTH,
    MAX_STATS_PER_CATEGORY, MAX_VALUE_LENGTH,
};

/// `HRESULT`.
pub type Hresult = i32;

pub const S_OK: Hresult = 0;
pub const S_FALSE: Hresult = 1;
pub const E_INVALIDARG: Hresult = 0x8007_0057_u32 as i32;

/// `GUID` layout from `guiddef.h`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

fn wstr(s: &[u16]) -> &[u16] {
    let len = s.iter().position(|&c| c == 0).unwrap_or(s.len());
    &s[..len]
}

/// Stands in for the token and SID lookup and counts its calls.
#[cfg(test)]
fn sid_lookup(calls: &std::cell::Cell<u32>) -> impl FnMut() -> Result<Vec<u16>, Hresult> + '_ {
    move || {
        calls.set(calls.get() + 1);
        Ok(wide("S-1-5-21-111-222-333-1001"))
    }
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
