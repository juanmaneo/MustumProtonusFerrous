use crate::{is_byref, NULL_EXCEPINFO, NULL_RESULT, VT_EMPTY};

/// `dword_flags` in `IDispatchEx_InvokeEx_Proxy`: the low nibble of `wFlags`,
/// plus `NULL_RESULT` / `NULL_EXCEPINFO` when the caller passed no output pointer.
pub fn proxy_flags(w_flags: u16, result_given: bool, excep_given: bool) -> u32 {
    let mut flags = u32::from(w_flags & 0xf);
    if !result_given {
        flags |= NULL_RESULT;
    }
    if !excep_given {
        flags |= NULL_EXCEPINFO;
    }
    flags
}

/// The proxy's argument arrays, keyed by VT only.
#[derive(Debug, PartialEq, Eq)]
pub struct ArgSplit {
    /// `copy_arg[]`: by-value VTs in place; by-reference slots are `VT_EMPTY`.
    pub copy_vts: Vec<u16>,
    /// `ref_arg[]`: VTs of the by-reference arguments, in argument order.
    pub ref_vts: Vec<u16>,
    /// `ref_idx[]`: original argument index of each by-reference argument.
    pub ref_idx: Vec<u32>,
}

/// Splits `rgvarg` into by-value and by-reference parts. Returns `None` when the
/// copy block is needed and its allocation fails (`E_OUTOFMEMORY`). With no
/// by-reference argument nothing is allocated, so `alloc_succeeds` is ignored.
pub fn split_args(vts: &[u16], alloc_succeeds: bool) -> Option<ArgSplit> {
    let byref_count = vts.iter().filter(|&&vt| is_byref(vt)).count();
    if byref_count > 0 && !alloc_succeeds {
        return None;
    }
    let mut split = ArgSplit {
        copy_vts: Vec::with_capacity(vts.len()),
        ref_vts: Vec::with_capacity(byref_count),
        ref_idx: Vec::with_capacity(byref_count),
    };
    for (arg, &vt) in vts.iter().enumerate() {
        if is_byref(vt) {
            split.copy_vts.push(VT_EMPTY);
            split.ref_vts.push(vt);
            split.ref_idx.push(arg as u32);
        } else {
            split.copy_vts.push(vt);
        }
    }
    Some(split)
}

/// `size` passed to `CoTaskMemAlloc`: `cArgs * sizeof(VARIANT) + byref * (sizeof(VARIANT) + sizeof(UINT))`.
/// The C expression is wide and then truncated to `DWORD`, so the result wraps modulo 2^32.
pub fn copy_block_size(c_args: u32, byref_args: u32, variant_size: u32, uint_size: u32) -> u32 {
    let variant = u64::from(variant_size);
    let total =
        u64::from(c_args) * variant + u64::from(byref_args) * (variant + u64::from(uint_size));
    total as u32
}
