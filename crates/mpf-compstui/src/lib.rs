//! Headless subset of Wine's `dlls/compstui/compstui_main.c` (Common Property Sheet UI).
//!
//! Ported, checked against the verbatim C source by `oracle_checks.rs`:
//! - [`build_sheet`]: the header resolution of `create_property_sheetW`: the `PSH_*` flags, the icon choice, the title composition and the no-page guard.
//! - [`sheet_result`]: the mapping of the `PropertySheetW` return value that `create_property_sheetW` performs.
//! - [`handles::HandleTable`]: `alloc_handle`, `free_handle` and `get_handle_data`.
//! - [`page::page_copy_layout`] and [`page::strip_page_info`]: the `PSPINFO` sizes used by `add_propsheetpageA`/`W` and by the `WM_INITDIALOG` handler.
//!
//! Skipped:
//! - `CommonPropertySheetUIA`, `GetCPSUIUserData` and `SetCPSUIUserData`: `FIXME` stubs with no logic to port.
//! - `DllMain`: stores the module handle only.
//! - `cps_callback`: routes each `CPSFUNC_*` code to the add functions; the other codes are `FIXME`s that return 0. The add functions take live window and callback objects.
//! - `CommonPropertySheetUIW` (`create_prop_dlg`): runs client callbacks, `PropertySheetW` and the handle table in sequence against live objects.
//!
//! Resource strings (`IDS_CPSUI_OPTIONS`, `IDS_CPSUI_DEFAULT`, `IDS_CPSUI_PROPERTIES`) are supplied by the caller through [`SheetStrings`].

pub mod handles;
pub mod page;

#[cfg(test)]
mod oracle_checks;

/// `CPSUI_OK`: the default result of `create_property_sheetW`.
pub const CPSUI_OK: i32 = 1;
/// `CPSUI_RESTARTWINDOWS`.
pub const CPSUI_RESTARTWINDOWS: i32 = 2;
/// `CPSUI_REBOOTSYSTEM`.
pub const CPSUI_REBOOTSYSTEM: i32 = 3;
/// `ERR_CPSUI_GETLASTERROR`: `PropertySheetW` failed.
pub const ERR_CPSUI_GETLASTERROR: i32 = -1;
/// `ERR_CPSUI_NO_PROPSHEETPAGE`: the property sheet has no pages.
pub const ERR_CPSUI_NO_PROPSHEETPAGE: i32 = -8;

/// `PSUIHDRF_NOAPPLYNOW`.
pub const PSUIHDRF_NOAPPLYNOW: u16 = 0x02;
/// `PSUIHDRF_PROPTITLE`.
pub const PSUIHDRF_PROPTITLE: u16 = 0x04;
/// `PSUIHDRF_USEHICON`.
pub const PSUIHDRF_USEHICON: u16 = 0x08;
/// `PSUIHDRF_DEFTITLE`.
pub const PSUIHDRF_DEFTITLE: u16 = 0x10;
/// `PSUIHDRF_EXACT_PTITLE`.
pub const PSUIHDRF_EXACT_PTITLE: u16 = 0x20;

/// `PSH_USEHICON`.
pub const PSH_USEHICON: u32 = 0x0002;
/// `PSH_USEICONID`.
pub const PSH_USEICONID: u32 = 0x0004;
/// `PSH_NOAPPLYNOW`.
pub const PSH_NOAPPLYNOW: u32 = 0x0080;

/// `ID_PSRESTARTWINDOWS`: the `PropertySheetW` return value for a restart request.
pub const ID_PSRESTARTWINDOWS: isize = 0x2;
/// `ID_PSREBOOTSYSTEM`: the `PropertySheetW` return value for a reboot request.
pub const ID_PSREBOOTSYSTEM: isize = 0x3;

/// `IDI_CPSUI_OPTION`: the icon used when the header gives none.
pub const IDI_CPSUI_OPTION: u64 = 64066;
/// `IDI_CPSUI_ICONID_FIRST`.
pub const IDI_CPSUI_ICONID_FIRST: u64 = 64000;
/// `IDI_CPSUI_ICONID_LAST`.
pub const IDI_CPSUI_ICONID_LAST: u64 = 64111;

/// Length of the title buffer in `create_property_sheetW`, including the terminator.
const TITLE_LEN: usize = 256;

/// The `PROPSHEETUI_INFO_HEADERAW` fields that `create_property_sheetW` reads.
#[derive(Clone, Copy, Debug)]
pub struct SheetHeader<'a> {
    /// The `PSUIHDRF_*` bits.
    pub flags: u16,
    /// `titleW`, where `None` is the NULL pointer.
    pub title: Option<&'a [u16]>,
    /// `hicon` when `PSUIHDRF_USEHICON` is set, otherwise `icon_id`.
    pub icon: u64,
}

/// The resource strings that `create_property_sheetW` loads.
#[derive(Clone, Copy, Debug)]
pub struct SheetStrings<'a> {
    /// `IDS_CPSUI_OPTIONS`, the title when the header gives none.
    pub options: &'a [u16],
    /// `IDS_CPSUI_DEFAULT`, appended for `PSUIHDRF_DEFTITLE`.
    pub default: &'a [u16],
    /// `IDS_CPSUI_PROPERTIES`, appended for `PSUIHDRF_PROPTITLE`.
    pub properties: &'a [u16],
}

/// The icon that `create_property_sheetW` gives `PROPSHEETHEADERW`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SheetIcon {
    /// No icon: an icon id in the `IDI_CPSUI_ICONID_*` range, which the C code reports as unimplemented.
    None,
    /// `hIcon` with `PSH_USEHICON`.
    Handle(u64),
    /// `pszIcon` as an icon id with `PSH_USEICONID`.
    Id(u64),
}

/// The `PROPSHEETHEADERW` fields that `create_property_sheetW` sets.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SheetRequest {
    /// `dwFlags`.
    pub flags: u32,
    /// `hIcon` or `pszIcon`.
    pub icon: SheetIcon,
    /// `pszCaption`, without the terminator.
    pub title: Vec<u16>,
}

/// Resolves a header the way `create_property_sheetW` does, before `PropertySheetW` is called.
///
/// Fails with [`ERR_CPSUI_NO_PROPSHEETPAGE`] when the sheet has no pages.
pub fn build_sheet(
    header: &SheetHeader,
    strings: &SheetStrings,
    page_count: usize,
) -> Result<SheetRequest, i32> {
    if page_count == 0 {
        return Err(ERR_CPSUI_NO_PROPSHEETPAGE);
    }

    let mut flags = 0;
    if header.flags & PSUIHDRF_NOAPPLYNOW != 0 {
        flags |= PSH_NOAPPLYNOW;
    }

    let icon = if header.flags & PSUIHDRF_USEHICON != 0 {
        flags |= PSH_USEHICON;
        SheetIcon::Handle(header.icon)
    } else {
        let icon_id = if header.icon == 0 {
            IDI_CPSUI_OPTION
        } else {
            header.icon
        };
        if (IDI_CPSUI_ICONID_FIRST..=IDI_CPSUI_ICONID_LAST).contains(&icon_id) {
            SheetIcon::None
        } else {
            flags |= PSH_USEICONID;
            SheetIcon::Id(icon_id)
        }
    };

    let mut title = [0u16; TITLE_LEN];
    match header.title {
        Some(text) => copy_title(&mut title, text),
        None => load_string(&mut title, strings.options),
    }
    let relabel = header.title.is_none() || header.flags & PSUIHDRF_EXACT_PTITLE == 0;
    if header.flags & PSUIHDRF_DEFTITLE != 0 && relabel {
        append_string(&mut title, strings.default);
    }
    if header.flags & PSUIHDRF_PROPTITLE != 0 && relabel {
        append_string(&mut title, strings.properties);
    }

    Ok(SheetRequest {
        flags,
        icon,
        title: title[..wstr_len(&title)].to_vec(),
    })
}

/// Maps the `PropertySheetW` return value to the `CPSUI_*` result of `create_property_sheetW`.
pub fn sheet_result(ret: isize) -> i32 {
    match ret {
        -1 => ERR_CPSUI_GETLASTERROR,
        ID_PSREBOOTSYSTEM => CPSUI_REBOOTSYSTEM,
        ID_PSRESTARTWINDOWS => CPSUI_RESTARTWINDOWS,
        _ => CPSUI_OK,
    }
}

/// Length of a NUL-terminated string, or of the whole slice when it has no NUL (`wcslen`).
fn wstr_len(text: &[u16]) -> usize {
    text.iter().position(|&c| c == 0).unwrap_or(text.len())
}

/// `LoadStringW`: copies at most `buf.len() - 1` units and NUL-terminates.
fn load_string(buf: &mut [u16], text: &[u16]) {
    let n = wstr_len(text).min(buf.len() - 1);
    buf[..n].copy_from_slice(&text[..n]);
    buf[n] = 0;
}

/// `wcscpy_s` into the title buffer: when the text does not fit, the buffer becomes empty.
fn copy_title(buf: &mut [u16], text: &[u16]) {
    let n = wstr_len(text);
    if n < buf.len() {
        buf[..n].copy_from_slice(&text[..n]);
        buf[n] = 0;
    } else {
        buf[0] = 0;
    }
}

/// Appends a space and a resource string while the title has room, as `create_property_sheetW` does.
fn append_string(title: &mut [u16], text: &[u16]) {
    let len = wstr_len(title);
    if len < TITLE_LEN - 1 {
        title[len] = u16::from(b' ');
        load_string(&mut title[len + 1..], text);
    }
}
