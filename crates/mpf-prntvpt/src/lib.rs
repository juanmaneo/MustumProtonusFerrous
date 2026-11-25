//! Port of the portable logic in Wine's `dlls/prntvpt/ticket.c`, with the
//! `MulDiv` helper from `dlls/kernelbase/main.c` that its geometry uses.
//!
//! Ported:
//! - `psk_media` table, `media_to_paper` and `paper_to_media`:
//!   [`media_to_paper`], [`paper_to_media`]
//! - `is_valid_node_name`: [`is_valid_node_name`]
//! - Option-name mapping for PageOutputColor, PageOrientation, DocumentCollate and
//!   PageScaling, read side and write side: [`page_output_color_from_option`],
//!   [`page_orientation_from_option`], [`document_collate_from_option`],
//!   [`page_scaling_from_option`], [`page_output_color_option`],
//!   [`page_orientation_option`], [`document_collate_option`], [`page_scaling_option`]
//! - `ticket_to_devmode` and `devmode_to_ticket`: [`ticket_to_devmode`], [`devmode_to_ticket`]
//! - `MulDiv` and `pixels_to_mm_x1000`: [`mul_div`], [`pixels_to_mm_x1000`]
//! - Orientation swap in `write_PageImageableSize_caps`: [`imageable_area`]
//! - Option list loop of `write_PageMediaSize_caps`: [`paper_size_options`]
//! - Attribute filter of `set_SelectionNamespaces`: [`selection_namespaces`]
//!
//! Skipped:
//! - `main.c`: the DLL entry point, `PT*` and `*Thunk*` exports, provider and handle
//!   management, and memory plumbing.
//! - DOM and stream code: `verify_ticket`, `read_int_value`, `write_int_value`, the
//!   `create_*` helpers, and every `read_*` and `write_*` that walks or builds an
//!   `IXMLDOM*` tree.
//! - Device access: `DeviceCapabilitiesW`, `CreateDCW` and `GetDeviceCaps`. The
//!   `write_PageMediaSize_caps` and `write_PageImageableSize_caps` loops are ported only
//!   as the pure functions above. The seven `write_*_caps` writers that are `FIXME("stub")`
//!   in Wine are not ported.
//! - `initialize_ticket` (printer defaults) and the `PTConvert*`, `PTMerge*` and `PTGet*`
//!   entry points, which combine the above with COM streams.
//! - `dump_fields` and `dump_devmode`: trace output only.
//! - `DEVMODEW.dmSize`: `ticket_to_devmode` leaves it unset because [`Devmode`] holds only
//!   the fields below. The name, form and display fields, and `Ticket.job.nup`, are not
//!   modelled either.

mod caps;
mod media;
mod namespaces;
mod options;

pub use caps::{
    imageable_area, mul_div, paper_size_options, pixels_to_mm_x1000, DeviceMetrics, ImageableArea,
    PaperOption,
};
pub use media::{media_to_paper, paper_to_media};
pub use namespaces::selection_namespaces;
pub use options::{
    document_collate_from_option, document_collate_option, is_valid_node_name,
    page_orientation_from_option, page_orientation_option, page_output_color_from_option,
    page_output_color_option, page_scaling_from_option, page_scaling_option,
};

pub const DMPAPER_LETTER: i32 = 1;
pub const DMPAPER_LEGAL: i32 = 5;
pub const DMPAPER_EXECUTIVE: i32 = 7;
pub const DMPAPER_A3: i32 = 8;
pub const DMPAPER_A4: i32 = 9;
pub const DMPAPER_A5: i32 = 11;
pub const DMPAPER_B4: i32 = 12;
pub const DMPAPER_B5: i32 = 13;

pub const DMORIENT_PORTRAIT: i32 = 1;
pub const DMORIENT_LANDSCAPE: i32 = 2;

pub const DMCOLOR_MONOCHROME: i32 = 1;
pub const DMCOLOR_COLOR: i32 = 2;

pub const DMCOLLATE_FALSE: i32 = 0;
pub const DMCOLLATE_TRUE: i32 = 1;

pub const DMBIN_AUTO: i32 = 7;

pub const DM_ORIENTATION: u32 = 0x0000_0001;
pub const DM_PAPERSIZE: u32 = 0x0000_0002;
pub const DM_PAPERLENGTH: u32 = 0x0000_0004;
pub const DM_PAPERWIDTH: u32 = 0x0000_0008;
pub const DM_SCALE: u32 = 0x0000_0010;
pub const DM_COPIES: u32 = 0x0000_0100;
pub const DM_DEFAULTSOURCE: u32 = 0x0000_0200;
pub const DM_PRINTQUALITY: u32 = 0x0000_0400;
pub const DM_COLOR: u32 = 0x0000_0800;
pub const DM_YRESOLUTION: u32 = 0x0000_2000;
pub const DM_COLLATE: u32 = 0x0000_8000;
pub const DM_LOGPIXELS: u32 = 0x0002_0000;

/// `struct size` from `ticket.c`, in micrometres for media sizes.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

/// `struct media` from `ticket.c`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Media {
    pub paper: i32,
    pub size: Size,
}

/// `struct resolution` from `ticket.c`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Resolution {
    pub x: i32,
    pub y: i32,
}

/// `struct page` from `ticket.c`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Page {
    pub media: Media,
    pub resolution: Resolution,
    pub orientation: i32,
    pub scaling: i32,
    pub color: i32,
}

/// `struct document` from `ticket.c`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Document {
    pub collate: i32,
}

/// `struct job` from `ticket.c`. `nup` is not modelled.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Job {
    pub copies: i32,
    pub input_bin: i32,
}

/// `struct ticket` from `ticket.c`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Ticket {
    pub job: Job,
    pub document: Document,
    pub page: Page,
}

/// The `DEVMODEW` fields that [`ticket_to_devmode`] writes and [`devmode_to_ticket`] reads.
/// Field widths follow `DEVMODEW`: `SHORT` for the printer fields, `WORD` for `dmLogPixels`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Devmode {
    pub fields: u32,
    pub orientation: i16,
    pub paper_size: i16,
    pub paper_length: i16,
    pub paper_width: i16,
    pub scale: i16,
    pub copies: i16,
    pub default_source: i16,
    pub print_quality: i16,
    pub color: i16,
    pub y_resolution: i16,
    pub collate: i16,
    pub log_pixels: u16,
}

fn has_flag(fields: u32, flag: u32) -> bool {
    fields & flag != 0
}

/// `ticket_to_devmode`: fills every field the ticket carries. Media sizes are
/// divided by 100 to get the `DEVMODE` tenths of a millimetre. Values are truncated
/// to 16 bits as the C `SHORT` assignments do.
pub fn ticket_to_devmode(ticket: &Ticket) -> Devmode {
    Devmode {
        fields: DM_ORIENTATION
            | DM_PAPERSIZE
            | DM_PAPERLENGTH
            | DM_PAPERWIDTH
            | DM_SCALE
            | DM_COPIES
            | DM_DEFAULTSOURCE
            | DM_COLOR
            | DM_PRINTQUALITY
            | DM_YRESOLUTION
            | DM_COLLATE,
        orientation: ticket.page.orientation as i16,
        paper_size: ticket.page.media.paper as i16,
        paper_length: (ticket.page.media.size.height / 100) as i16,
        paper_width: (ticket.page.media.size.width / 100) as i16,
        scale: ticket.page.scaling as i16,
        copies: ticket.job.copies as i16,
        default_source: ticket.job.input_bin as i16,
        color: ticket.page.color as i16,
        print_quality: ticket.page.resolution.x as i16,
        y_resolution: ticket.page.resolution.y as i16,
        collate: ticket.document.collate as i16,
        ..Devmode::default()
    }
}

/// `devmode_to_ticket`: copies each field whose `DM_*` flag is set. The order matters:
/// `DM_PRINTQUALITY` sets both resolution axes, `DM_YRESOLUTION` then overrides y, and
/// `DM_LOGPIXELS` overrides both.
pub fn devmode_to_ticket(dm: &Devmode, ticket: &mut Ticket) {
    if has_flag(dm.fields, DM_ORIENTATION) {
        ticket.page.orientation = i32::from(dm.orientation);
    }
    if has_flag(dm.fields, DM_PAPERSIZE) {
        ticket.page.media.paper = i32::from(dm.paper_size);
    }
    if has_flag(dm.fields, DM_PAPERWIDTH) {
        ticket.page.media.size.width = i32::from(dm.paper_width) * 100;
    }
    if has_flag(dm.fields, DM_PAPERLENGTH) {
        ticket.page.media.size.height = i32::from(dm.paper_length) * 100;
    }
    if has_flag(dm.fields, DM_SCALE) {
        ticket.page.scaling = i32::from(dm.scale);
    }
    if has_flag(dm.fields, DM_COPIES) {
        ticket.job.copies = i32::from(dm.copies);
    }
    if has_flag(dm.fields, DM_DEFAULTSOURCE) {
        ticket.job.input_bin = i32::from(dm.default_source);
    }
    if has_flag(dm.fields, DM_COLOR) {
        ticket.page.color = i32::from(dm.color);
    }
    if has_flag(dm.fields, DM_PRINTQUALITY) {
        ticket.page.resolution.x = i32::from(dm.print_quality);
        ticket.page.resolution.y = i32::from(dm.print_quality);
    }
    if has_flag(dm.fields, DM_YRESOLUTION) {
        ticket.page.resolution.y = i32::from(dm.y_resolution);
    }
    if has_flag(dm.fields, DM_LOGPIXELS) {
        ticket.page.resolution.x = i32::from(dm.log_pixels);
        ticket.page.resolution.y = i32::from(dm.log_pixels);
    }
    if has_flag(dm.fields, DM_COLLATE) {
        ticket.document.collate = i32::from(dm.collate);
    }
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
