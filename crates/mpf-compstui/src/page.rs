//! Page copy layout used by `add_propsheetpageA`/`W` and by the `WM_INITDIALOG` handler.
//!
//! The sizes are the LP64 values that the oracle runs on, from `sizeof(PSPINFO)` and
//! `sizeof(PROPSHEETPAGE{A,W}_V1)`.
//! Only the size arithmetic is ported. The DWORD wrap of `psp_copy->dwSize` is not modelled.

/// `sizeof(PSPINFO)`: the info block appended to each page copy.
pub const PSPINFO_SIZE: u32 = 32;
/// `PROPSHEETPAGEA_V1_SIZE`: the smallest `dwSize` that `add_propsheetpageA` accepts.
pub const PROPSHEETPAGEA_V1_SIZE: u32 = 72;
/// `PROPSHEETPAGEW_V1_SIZE`: the smallest `dwSize` that `add_propsheetpageW` accepts.
pub const PROPSHEETPAGEW_V1_SIZE: u32 = 72;

/// The copy that `add_propsheetpageA`/`W` makes of a page.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PageCopy {
    /// Bytes allocated for the copy: `dwSize` plus the info block.
    pub size: u64,
    /// Offset of the info block inside the copy, which equals the original `dwSize`.
    pub info_offset: u32,
}

/// Lays out the copy of a page, or returns `None` when `dw_size` is below `v1_size`.
pub fn page_copy_layout(dw_size: u32, v1_size: u32, info_size: u32) -> Option<PageCopy> {
    if dw_size < v1_size {
        return None;
    }
    Some(PageCopy {
        size: u64::from(dw_size) + u64::from(info_size),
        info_offset: dw_size,
    })
}

/// The `dwSize` that the `WM_INITDIALOG` handler leaves on the copy, or `None` when the copy is smaller than the info block.
pub fn strip_page_info(copy_size: u32, info_size: u32) -> Option<u32> {
    copy_size.checked_sub(info_size)
}
