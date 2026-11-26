//! Control-style logic from `dlls/explorerframe/nstc.c`: the
//! `treeview_style_from_nstcs` flag mapping and the mask checks in
//! `NSTC2_fnSetControlStyle*`. The window writes are left to the caller.

use crate::hresult::{Hresult, E_FAIL};

pub const NSTCS_HASEXPANDOS: u32 = 0x0000_0001;
pub const NSTCS_HASLINES: u32 = 0x0000_0002;
pub const NSTCS_SINGLECLICKEXPAND: u32 = 0x0000_0004;
pub const NSTCS_FULLROWSELECT: u32 = 0x0000_0008;
pub const NSTCS_SPRINGEXPAND: u32 = 0x0000_0010;
pub const NSTCS_HORIZONTALSCROLL: u32 = 0x0000_0020;
pub const NSTCS_ROOTHASEXPANDO: u32 = 0x0000_0040;
pub const NSTCS_SHOWSELECTIONALWAYS: u32 = 0x0000_0080;
pub const NSTCS_NOINFOTIP: u32 = 0x0000_0200;
pub const NSTCS_EVENHEIGHT: u32 = 0x0000_0400;
pub const NSTCS_NOREPLACEOPEN: u32 = 0x0000_0800;
pub const NSTCS_DISABLEDRAGDROP: u32 = 0x0000_1000;
pub const NSTCS_NOORDERSTREAM: u32 = 0x0000_2000;
pub const NSTCS_RICHTOOLTIP: u32 = 0x0000_4000;
pub const NSTCS_BORDER: u32 = 0x0000_8000;
pub const NSTCS_NOEDITLABELS: u32 = 0x0001_0000;
pub const NSTCS_TABSTOP: u32 = 0x0002_0000;
pub const NSTCS_FAVORITESMODE: u32 = 0x0008_0000;
pub const NSTCS_EMPTYTEXT: u32 = 0x0040_0000;
pub const NSTCS_CHECKBOXES: u32 = 0x0080_0000;
pub const NSTCS_NOINDENTCHECKS: u32 = 0x0800_0000;
pub const NSTCS_ALLOWJUNCTIONS: u32 = 0x1000_0000;
pub const NSTCS_SHOWTABSBUTTON: u32 = 0x2000_0000;
pub const NSTCS_SHOWDELETEBUTTON: u32 = 0x4000_0000;
pub const NSTCS_SHOWREFRESHBUTTON: u32 = 0x8000_0000;

pub const NSTCS2_INTERRUPTNOTIFICATIONS: u32 = 0x01;
pub const NSTCS2_SHOWNULLSPACEMENU: u32 = 0x02;
pub const NSTCS2_DISPLAYPADDING: u32 = 0x04;
pub const NSTCS2_DISPLAYPINNEDONLY: u32 = 0x08;
pub const NTSCS2_NOSINGLETONAUTOEXPAND: u32 = 0x10;
pub const NTSCS2_NEVERINSERTNONENUMERATED: u32 = 0x20;

pub const WS_BORDER: u32 = 0x0080_0000;
pub const WS_EX_CONTROLPARENT: u32 = 0x0001_0000;

const WS_CHILD: u32 = 0x4000_0000;
const WS_VISIBLE: u32 = 0x1000_0000;
const WS_CLIPSIBLINGS: u32 = 0x0400_0000;
const WS_CLIPCHILDREN: u32 = 0x0200_0000;
const WS_TABSTOP: u32 = 0x0001_0000;

const TVS_HASBUTTONS: u32 = 0x0001;
const TVS_HASLINES: u32 = 0x0002;
const TVS_LINESATROOT: u32 = 0x0004;
const TVS_EDITLABELS: u32 = 0x0008;
const TVS_DISABLEDRAGDROP: u32 = 0x0010;
const TVS_SHOWSELALWAYS: u32 = 0x0020;
const TVS_CHECKBOXES: u32 = 0x0100;
const TVS_TRACKSELECT: u32 = 0x0200;
const TVS_SINGLEEXPAND: u32 = 0x0400;
const TVS_INFOTIP: u32 = 0x0800;
const TVS_FULLROWSELECT: u32 = 0x1000;
const TVS_NONEVENHEIGHT: u32 = 0x4000;
const TVS_NOHSCROLL: u32 = 0x8000;

/// Tree-view style used when no tree-view window exists yet.
const TREEVIEW_DEFAULT_STYLE: u32 = WS_CHILD
    | WS_VISIBLE
    | WS_CLIPSIBLINGS
    | WS_CLIPCHILDREN
    | WS_TABSTOP
    | TVS_NOHSCROLL
    | TVS_NONEVENHEIGHT
    | TVS_INFOTIP
    | TVS_EDITLABELS
    | TVS_TRACKSELECT;

/// `(NSTCS flag, TVS flag, inverted)`. For inverted entries a cleared NSTCS
/// bit sets the TVS bit, as in `treeview_style_from_nstcs`.
const TREEVIEW_FLAG_MAP: [(u32, u32, bool); 12] = [
    (NSTCS_HASEXPANDOS, TVS_HASBUTTONS, false),
    (NSTCS_HASLINES, TVS_HASLINES, false),
    (NSTCS_FULLROWSELECT, TVS_FULLROWSELECT, false),
    (NSTCS_HORIZONTALSCROLL, TVS_NOHSCROLL, true),
    (NSTCS_ROOTHASEXPANDO, TVS_LINESATROOT, false),
    (NSTCS_SHOWSELECTIONALWAYS, TVS_SHOWSELALWAYS, false),
    (NSTCS_NOINFOTIP, TVS_INFOTIP, true),
    (NSTCS_EVENHEIGHT, TVS_NONEVENHEIGHT, true),
    (NSTCS_DISABLEDRAGDROP, TVS_DISABLEDRAGDROP, false),
    (NSTCS_NOEDITLABELS, TVS_EDITLABELS, true),
    (NSTCS_CHECKBOXES, TVS_CHECKBOXES, false),
    (NSTCS_SINGLECLICKEXPAND, TVS_SINGLEEXPAND, false),
];

const TV_STYLE_FLAGS: u32 = NSTCS_HASEXPANDOS
    | NSTCS_HASLINES
    | NSTCS_FULLROWSELECT
    | NSTCS_HORIZONTALSCROLL
    | NSTCS_ROOTHASEXPANDO
    | NSTCS_SHOWSELECTIONALWAYS
    | NSTCS_NOINFOTIP
    | NSTCS_EVENHEIGHT
    | NSTCS_DISABLEDRAGDROP
    | NSTCS_NOEDITLABELS
    | NSTCS_CHECKBOXES;
const HOST_STYLE_FLAGS: u32 = NSTCS_TABSTOP | NSTCS_BORDER;
const NSTC_STYLE_FLAGS: u32 = NSTCS_SINGLECLICKEXPAND
    | NSTCS_NOREPLACEOPEN
    | NSTCS_NOORDERSTREAM
    | NSTCS_FAVORITESMODE
    | NSTCS_EMPTYTEXT
    | NSTCS_ALLOWJUNCTIONS
    | NSTCS_SHOWTABSBUTTON
    | NSTCS_SHOWDELETEBUTTON
    | NSTCS_SHOWREFRESHBUTTON;

const UNSUPPORTED_STYLES: u32 = NSTCS_NOREPLACEOPEN
    | NSTCS_NOORDERSTREAM
    | NSTCS_FAVORITESMODE
    | NSTCS_EMPTYTEXT
    | NSTCS_ALLOWJUNCTIONS
    | NSTCS_SHOWTABSBUTTON
    | NSTCS_SHOWDELETEBUTTON
    | NSTCS_SHOWREFRESHBUTTON
    | NSTCS_SPRINGEXPAND
    | NSTCS_RICHTOOLTIP
    | NSTCS_NOINDENTCHECKS;
const UNSUPPORTED_STYLES2: u32 = NSTCS2_INTERRUPTNOTIFICATIONS
    | NSTCS2_SHOWNULLSPACEMENU
    | NSTCS2_DISPLAYPADDING
    | NSTCS2_DISPLAYPINNEDONLY
    | NTSCS2_NOSINGLETONAUTOEXPAND
    | NTSCS2_NEVERINSERTNONENUMERATED;

/// Result of `set_control_style`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ControlStyleUpdate {
    /// New `This->style`.
    pub style: u32,
    /// Requested bits the control does not support (the `FIXME` path).
    pub unsupported: u32,
    /// New tree-view window style, when one exists and the mask touches it.
    pub tv_style: Option<u32>,
}

/// `treeview_style_from_nstcs`. `current` is the tree-view window's style,
/// or `None` when the window does not exist and the default style applies.
/// Returns `(new_style, changed_bits)`, where `changed_bits` is `old ^ new`.
pub fn treeview_style_from_nstcs(current: Option<u32>, nstcs: u32, nstcs_mask: u32) -> (u32, u32) {
    let old_style = current.unwrap_or(TREEVIEW_DEFAULT_STYLE);
    let mut tv_mask = 0;
    let mut wanted = 0;
    for &(nstcs_flag, tv_flag, inverted) in &TREEVIEW_FLAG_MAP {
        if nstcs_mask & nstcs_flag != 0 {
            tv_mask |= tv_flag;
        }
        if (nstcs & nstcs_flag != 0) != inverted {
            wanted |= tv_flag;
        }
    }
    let new_style = (old_style & !tv_mask) | (wanted & tv_mask);
    (new_style, old_style ^ new_style)
}

/// `NSTC2_fnSetControlStyle`. Fails with `E_FAIL` when the mask holds a bit
/// that is neither a tree-view, host or NSTC style; in that case nothing changes.
pub fn set_control_style(
    state: u32,
    mask: u32,
    style: u32,
    tv: Option<u32>,
) -> Result<ControlStyleUpdate, Hresult> {
    if mask & !(TV_STYLE_FLAGS | HOST_STYLE_FLAGS | NSTC_STYLE_FLAGS) != 0 {
        return Err(E_FAIL);
    }
    let tv_style = match tv {
        Some(current) if mask & TV_STYLE_FLAGS != 0 => {
            Some(treeview_style_from_nstcs(Some(current), style, mask).0)
        }
        _ => None,
    };
    Ok(ControlStyleUpdate {
        style: (state & !mask) | (style & mask),
        unsupported: style & mask & UNSUPPORTED_STYLES,
        tv_style,
    })
}

/// `NSTC2_fnSetControlStyle2`. Returns `(new_style2, unsupported_bits)`.
pub fn set_control_style2(state: u32, mask: u32, style: u32) -> (u32, u32) {
    (
        (state & !mask) | (style & mask),
        style & mask & UNSUPPORTED_STYLES2,
    )
}

/// The host-window writes made by `NSTC2_fnSetControlStyle` for
/// `NSTCS_BORDER` (`WS_BORDER` in `GWL_STYLE`) and `NSTCS_TABSTOP`
/// (`WS_EX_CONTROLPARENT` in `GWL_EXSTYLE`). Returns `(style, exstyle)`.
pub fn apply_host_style(main_style: u32, main_exstyle: u32, mask: u32, style: u32) -> (u32, u32) {
    let mut main = main_style;
    let mut ex = main_exstyle;
    if mask & NSTCS_BORDER != 0 {
        main = (main & !WS_BORDER)
            | if style & NSTCS_BORDER != 0 {
                WS_BORDER
            } else {
                0
            };
    }
    if mask & NSTCS_TABSTOP != 0 {
        ex = (ex & !WS_EX_CONTROLPARENT)
            | if style & NSTCS_TABSTOP != 0 {
                WS_EX_CONTROLPARENT
            } else {
                0
            };
    }
    (main, ex)
}

/// `NSTC2_fnGetControlStyle` and `NSTC2_fnGetControlStyle2`: the cached style
/// bits selected by the mask.
pub fn get_control_style(state: u32, mask: u32) -> u32 {
    state & mask
}
