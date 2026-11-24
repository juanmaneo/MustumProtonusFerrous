//! The module collection's growth rule and index check in `ScriptModuleCollection`.

use crate::HResult;

/// `0x800a0009`, the literal `ScriptModuleCollection_get_Item` returns for an index out of range
/// (VB error 9, subscript out of range).
pub const CTL_E_OUT_OF_RANGE: HResult = 0x800A_0009;

/// The `is_power_of_2` test in `ScriptModuleCollection_Add`. It is also true for 0, where the
/// reference calls `realloc` with a zero size.
pub fn needs_growth(module_count: u32) -> bool {
    (module_count & module_count.wrapping_sub(1)) == 0
}

/// The capacity after growth: `module_count * 2`, in the reference's unsigned arithmetic.
pub fn grown_capacity(module_count: u32) -> u32 {
    module_count.wrapping_mul(2)
}

/// Converts a one-based VB index into a zero-based slot. An index is valid when it is in
/// `1..=module_count`. The reference computes `(unsigned)(index - 1)`, so 0 and negative indices
/// fall out of range.
pub fn module_index(index: i32, module_count: u32) -> Result<usize, HResult> {
    let slot = index.wrapping_sub(1) as u32;
    if slot >= module_count {
        Err(CTL_E_OUT_OF_RANGE)
    } else {
        Ok(slot as usize)
    }
}
