//! Deterministic value logic ported from `dlls/oleacc/main.c`.

/// `VT_I4`, the variant type accepted by Wine's `convert_child_id`.
pub const VT_I4: u16 = 3;

pub(crate) const fn is_supported_child_id_type(vt: u16) -> bool {
    vt == VT_I4
}
