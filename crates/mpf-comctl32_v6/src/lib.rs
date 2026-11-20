//! Port of the deterministic button style helpers in Wine's `dlls/comctl32_v6/button.c`.
//!
//! Ported:
//! - `get_button_type`: [`button_type`].
//! - `button_centers_text`: [`button_centers_text`].
//! - `BUTTON_BStoDT`: [`bs_to_dt`]. It maps button styles to DrawText flags.
//! - `BUTTON_ILStoBS`: [`il_align_to_bs`]. It maps an image list alignment to button alignment.
//!
//! Skipped: the window procedures, painting, sizing and ideal-size code, the themed paths, and
//! the custom draw and notification code. They need window state, device contexts, theme handles
//! or the OS.
//!
//! Reference quirks that are kept:
//! - `BS_PUSHLIKE` clears the type bits before the centering test, so a push-like radio button
//!   centers its text.
//! - `WS_EX_RIGHT` overrides the horizontal alignment and clears `DT_CENTER`.
//! - `BS_GROUPBOX` gets no vertical alignment flag.
//! - An unknown image list alignment maps to `BS_LEFT | BS_VCENTER`.

const BS_TYPEMASK: u32 = 0x0000_000f;
const BS_DEFPUSHBUTTON: u32 = 0x0000_0001;
const BS_GROUPBOX: u32 = 0x0000_0007;
const BS_SPLITBUTTON: u32 = 0x0000_000c;
const BS_DEFSPLITBUTTON: u32 = 0x0000_000d;
const BS_PUSHLIKE: u32 = 0x0000_1000;
const BS_MULTILINE: u32 = 0x0000_2000;
const BS_LEFT: u32 = 0x0000_0100;
const BS_RIGHT: u32 = 0x0000_0200;
const BS_CENTER: u32 = 0x0000_0300;
const BS_TOP: u32 = 0x0000_0400;
const BS_BOTTOM: u32 = 0x0000_0800;
const BS_VCENTER: u32 = 0x0000_0c00;

const WS_EX_RIGHT: u32 = 0x0000_1000;

const DT_CENTER: u32 = 0x0000_0001;
const DT_RIGHT: u32 = 0x0000_0002;
const DT_VCENTER: u32 = 0x0000_0004;
const DT_BOTTOM: u32 = 0x0000_0008;
const DT_WORDBREAK: u32 = 0x0000_0010;
const DT_SINGLELINE: u32 = 0x0000_0020;
const DT_NOCLIP: u32 = 0x0000_0100;

const BUTTON_IMAGELIST_ALIGN_RIGHT: u32 = 1;
const BUTTON_IMAGELIST_ALIGN_TOP: u32 = 2;
const BUTTON_IMAGELIST_ALIGN_BOTTOM: u32 = 3;
const BUTTON_IMAGELIST_ALIGN_CENTER: u32 = 4;

/// The button type, which is the low four bits of the window style.
pub fn button_type(window_style: i32) -> u32 {
    (window_style as u32) & BS_TYPEMASK
}

/// Whether the text of this button type is centered by default.
pub fn button_centers_text(window_style: i32) -> bool {
    let kind = button_type(window_style);
    kind <= BS_DEFPUSHBUTTON || kind == BS_SPLITBUTTON || kind == BS_DEFSPLITBUTTON
}

/// Converts button styles to DrawText flags.
pub fn bs_to_dt(style: u32, ex_style: u32) -> u32 {
    let mut style = style;
    let mut dt = DT_NOCLIP;

    if style & BS_PUSHLIKE != 0 {
        style &= !BS_TYPEMASK;
    }

    if style & BS_MULTILINE == 0 {
        dt |= DT_SINGLELINE;
    } else {
        dt |= DT_WORDBREAK;
    }

    match style & BS_CENTER {
        BS_LEFT => {}
        BS_RIGHT => dt |= DT_RIGHT,
        BS_CENTER => dt |= DT_CENTER,
        _ => {
            if button_centers_text(style as i32) {
                dt |= DT_CENTER;
            }
        }
    }

    // DT_LEFT is 0, so clearing DT_CENTER is enough to select left alignment.
    if ex_style & WS_EX_RIGHT != 0 {
        dt = DT_RIGHT | (dt & !DT_CENTER);
    }

    if button_type(style as i32) != BS_GROUPBOX {
        match style & BS_VCENTER {
            BS_TOP => {}
            BS_BOTTOM => dt |= DT_BOTTOM,
            _ => dt |= DT_VCENTER,
        }
    }

    dt
}

/// Converts an image list alignment to button alignment flags.
pub fn il_align_to_bs(align: u32) -> u32 {
    match align {
        BUTTON_IMAGELIST_ALIGN_TOP => BS_CENTER | BS_TOP,
        BUTTON_IMAGELIST_ALIGN_BOTTOM => BS_CENTER | BS_BOTTOM,
        BUTTON_IMAGELIST_ALIGN_CENTER => BS_CENTER | BS_VCENTER,
        BUTTON_IMAGELIST_ALIGN_RIGHT => BS_RIGHT | BS_VCENTER,
        // BUTTON_IMAGELIST_ALIGN_LEFT (0) and any unknown value.
        _ => BS_LEFT | BS_VCENTER,
    }
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
