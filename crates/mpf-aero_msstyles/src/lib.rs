//! Port of the resource data in Wine's `dlls/aero.msstyles/aero.rc`, `resources.h` and
//! `Makefile.in`, and of the `PACKTHEM_VERSION` check in `dlls/uxtheme/msstyles.c`.
//!
//! The DLL is data-only (`EXTRADLLFLAGS = -Wb,--data-only`). It has no code, no `.spec` file and
//! no export table, so there are no `@ stub` exports to model.
//!
//! Ported:
//! - `resources.h` IDs and the English `STRINGTABLE`: [`IDS_COLOR_DISPLAY_NAME_BLUE`],
//!   [`IDS_COLOR_TOOLTIP_BLUE`], [`IDS_SIZE_DISPLAY_NAME_NORMAL`], [`IDS_SIZE_TOOLTIP_NORMAL`],
//!   and [`string_table_entry`]
//! - `PACKTHEM_VERSION` and its check: [`PACKTHEM_VERSION`], [`is_supported_version`]
//! - the `Makefile.in` version strings: [`VER_FILEDESCRIPTION_STR`], [`VER_PRODUCTVERSION`]
//! - the `COLORNAMES`, `SIZENAMES` and `FILERESNAMES` lists: [`COLOR_NAMES`], [`SIZE_NAMES`],
//!   [`FILE_RES_NAMES`], and the list walk [`multi_sz_entries`]
//! - the `TEXTFILE` resources `THEMES_INI` and `BLUE_INI`: [`THEMES_INI`], [`BLUE_INI`], [`textfile`]
//! - the `BITMAP` name-to-file table: [`BITMAPS`] and [`bitmap_file`]
//!
//! Skipped:
//! - The bitmap payloads and the `.svg` sources: binary assets, not logic.
//! - The resource compiler's work (`wrc -u`), `#pragma makedep`, `LANGUAGE` statements and comments.
//! - `FindResourceW`, `LoadResource` and `SizeofResource`, and the colour and size index selection
//!   in `MSSTYLES_GetThemeFile`, which belong to `dlls/uxtheme`.
//! - Parsing the INI text, which `mpf-uxtheme` ports.
//!
//! Deviations from Wine:
//! - The text is `&str`. [`multi_sz_entries`] takes the UTF-16 units that `str::encode_utf16` yields.
//! - [`textfile`] matches names exactly, as Wine's `find_entry_by_name` does with `wcsncmp`.
//! - A list with no final empty name ends at the end of the slice.

mod bitmaps;
mod names;
mod strings;
mod textfiles;
mod theme;

pub use bitmaps::{bitmap_file, BITMAPS};
pub use names::{multi_sz_entries, COLOR_NAMES, FILE_RES_NAMES, SIZE_NAMES};
pub use strings::{
    string_table_entry, IDS_COLOR_DISPLAY_NAME_BLUE, IDS_COLOR_TOOLTIP_BLUE,
    IDS_SIZE_DISPLAY_NAME_NORMAL, IDS_SIZE_TOOLTIP_NORMAL,
};
pub use textfiles::{textfile, BLUE_INI, THEMES_INI};
pub use theme::{
    is_supported_version, PACKTHEM_VERSION, VER_FILEDESCRIPTION_STR, VER_PRODUCTVERSION,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
