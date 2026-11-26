//! `GAMEUX_buildStatisticsFilePath` and the directory split in `GAMEUX_createStatsDirectory`,
//! from Wine's `dlls/gameux/gamestatistics.c`.

use crate::{wide, wstr};

const STATISTICS_SUBDIR: &str = "\\Microsoft\\Windows\\GameExplorer\\GameStatistics\\";
const BACKSLASH: u16 = b'\\' as u16;

/// Builds `<local_appdata>\Microsoft\Windows\GameExplorer\GameStatistics\<app>\<app>.gamestats`.
pub fn build_statistics_file_path(local_appdata: &[u16], app_id: &[u16]) -> Vec<u16> {
    let app_id = wstr(app_id);
    let mut path = wstr(local_appdata).to_vec();
    path.extend(wide(STATISTICS_SUBDIR));
    path.extend_from_slice(app_id);
    path.extend(wide("\\"));
    path.extend_from_slice(app_id);
    path.extend(wide(".gamestats"));
    path
}

/// The part of `path` before its last backslash.
///
/// Returns `None` when there is no backslash. The C code then computes a length
/// from a NULL pointer, so that input has no defined result.
pub fn stats_directory(path: &[u16]) -> Option<&[u16]> {
    let path = wstr(path);
    let end = path.iter().rposition(|&c| c == BACKSLASH)?;
    Some(&path[..end])
}
