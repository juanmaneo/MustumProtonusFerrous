//! `NSTC2_fnGetItemState` flag mapping from `dlls/explorerframe/nstc.c`.
//! The tree-view lookup and `TVM_GETITEMW` are left to the caller, who passes
//! the item's `TVIS_*` state bits and `TVIS_EX_*` bits.

use crate::hresult::{Hresult, E_INVALIDARG};

pub const NSTCIS_SELECTED: u32 = 0x0001;
pub const NSTCIS_EXPANDED: u32 = 0x0002;
pub const NSTCIS_BOLD: u32 = 0x0004;
pub const NSTCIS_DISABLED: u32 = 0x0008;

const TVIS_SELECTED: u32 = 0x0002;
const TVIS_BOLD: u32 = 0x0010;
const TVIS_EXPANDED: u32 = 0x0020;
const TVIS_EX_DISABLED: u32 = 0x0002;

/// `NSTC2_fnGetItemState`. `found` is `treeitem_from_shellitem(...) != NULL`.
/// Returns the `NSTCIS_*` flags restricted to `mask`.
pub fn get_item_state(
    found: bool,
    tv_state: u32,
    tv_state_ex: u32,
    mask: u32,
) -> Result<u32, Hresult> {
    if !found {
        return Err(E_INVALIDARG);
    }
    let mut flags = 0;
    if tv_state & TVIS_SELECTED != 0 {
        flags |= NSTCIS_SELECTED;
    }
    if tv_state & TVIS_EXPANDED != 0 {
        flags |= NSTCIS_EXPANDED;
    }
    if tv_state & TVIS_BOLD != 0 {
        flags |= NSTCIS_BOLD;
    }
    if tv_state_ex & TVIS_EX_DISABLED != 0 {
        flags |= NSTCIS_DISABLED;
    }
    Ok(flags & mask)
}
