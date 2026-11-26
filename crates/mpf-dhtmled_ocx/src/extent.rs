//! `MulDiv` from kernelbase, the default control extent from `dhtml_edit_create`, and the
//! `OleObject_SetExtent` / `OleObject_GetExtent` aspect checks.

use crate::hresult::{HResult, DV_E_DVASPECT, E_FAIL, S_OK};

pub const DVASPECT_CONTENT: u32 = 1;

/// `SIZEL`, in HIMETRIC units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sizel {
    pub cx: i32,
    pub cy: i32,
}

/// `MulDiv(a, b, c)`: `a * b / c` rounded half away from zero, or -1 on a zero divisor or overflow.
/// The sign of `c` is folded into `a` first. The negation is done in 64 bits, so `a = i32::MIN`
/// is defined here, where the C negation is undefined behaviour.
pub fn mul_div(a: i32, b: i32, c: i32) -> i32 {
    if c == 0 {
        return -1;
    }
    let (a, c) = if c < 0 {
        (-i64::from(a), -i64::from(c))
    } else {
        (i64::from(a), i64::from(c))
    };
    let b = i64::from(b);
    let product = a * b;
    let half = c / 2;
    let ret = if (a < 0 && b < 0) || (a >= 0 && b >= 0) {
        (product + half) / c
    } else {
        (product - half) / c
    };
    if !(-2_147_483_647..=2_147_483_647).contains(&ret) {
        -1
    } else {
        ret as i32
    }
}

/// Extent `dhtml_edit_create` gives a new control: 192 x 192 pixels converted to HIMETRIC.
pub fn default_extent(dpi_x: i32, dpi_y: i32) -> Sizel {
    Sizel {
        cx: mul_div(192, 2540, dpi_x),
        cy: mul_div(192, 2540, dpi_y),
    }
}

/// `OleObject_SetExtent`. Only DVASPECT_CONTENT is accepted; other aspects leave the extent unchanged.
pub fn object_set_extent(extent: &mut Sizel, aspect: u32, size: Sizel) -> HResult {
    if aspect != DVASPECT_CONTENT {
        return DV_E_DVASPECT;
    }
    *extent = size;
    S_OK
}

/// `OleObject_GetExtent`. Only DVASPECT_CONTENT is accepted.
pub fn object_get_extent(extent: &Sizel, aspect: u32) -> Result<Sizel, HResult> {
    if aspect != DVASPECT_CONTENT {
        return Err(E_FAIL);
    }
    Ok(*extent)
}
