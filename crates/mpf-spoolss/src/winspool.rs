//! The table check at the end of `SplInitializeWinSpoolDrv` (`spoolss_main.c`).
//!
//! The function loads `winspool.drv` and fills nine slots: `OpenPrinterW`, `ClosePrinter`,
//! `SpoolerDevQueryPrintW`, `SpoolerPrinterEvent`, `DocumentPropertiesW`, then ordinals 212 to
//! 215. The load and the `GetProcAddress` calls are OS work and stay with the caller.

/// Number of table slots that `SplInitializeWinSpoolDrv` requires.
pub const TABLE_LEN: usize = 9;

/// Returns whether `SplInitializeWinSpoolDrv` succeeds: the DLL loaded and every slot
/// resolved. `resolved[i]` is true when slot `i` got a non-NULL address.
pub fn winspool_table_ready(loaded: bool, resolved: [bool; TABLE_LEN]) -> bool {
    loaded && resolved.iter().all(|&slot| slot)
}
