use crate::proxy::ArgSplit;
use crate::{succeeded, DISP_E_BADCALLEE, DISP_E_EXCEPTION, NULL_EXCEPINFO, NULL_RESULT, VT_EMPTY};

/// Decoded `dwFlags` of `IDispatchEx_InvokeEx_Stub`.
#[derive(Debug, PartialEq, Eq)]
pub struct StubFlags {
    /// `dwFlags & 0xffff`, the `WORD` passed to `IDispatchEx_InvokeEx`.
    pub invoke_flags: u16,
    /// `dwFlags & NULL_RESULT`: the result pointer is passed as NULL.
    pub result_null: bool,
    /// `dwFlags & NULL_EXCEPINFO`: the exception info pointer is passed as NULL.
    pub excep_null: bool,
}

/// Decodes `dwFlags` as the stub does. The 16-bit truncation drops
/// `NULL_RESULT` and `NULL_EXCEPINFO`, which are only read from the full value.
pub fn stub_flags(dw_flags: u32) -> StubFlags {
    StubFlags {
        invoke_flags: (dw_flags & 0xffff) as u16,
        result_null: (dw_flags & NULL_RESULT) != 0,
        excep_null: (dw_flags & NULL_EXCEPINFO) != 0,
    }
}

/// Stub step `pdp->rgvarg[ref_idx[i]] = ref_arg[i]`: the argument array the callee sees.
pub fn seed_ref_args(split: &ArgSplit) -> Vec<u16> {
    let mut vts = split.copy_vts.clone();
    for (&idx, &vt) in split.ref_idx.iter().zip(&split.ref_vts) {
        vts[idx as usize] = vt;
    }
    vts
}

/// Stub step after the call: `VariantInit(pdp->rgvarg + ref_idx[i])`.
pub fn clear_ref_slots(vts: &mut [u16], ref_idx: &[u32]) {
    for &idx in ref_idx {
        vts[idx as usize] = VT_EMPTY;
    }
}

/// Stub: when the call succeeded and there are by-reference args, a changed VT in any
/// slot turns the result into `DISP_E_BADCALLEE`. `before` is the array at call time.
pub fn callee_result(hr: u32, before: &[u16], after: &[u16], byref_args: usize) -> u32 {
    if succeeded(hr) && byref_args > 0 && before != after {
        DISP_E_BADCALLEE
    } else {
        hr
    }
}

/// Stub: `DISP_E_EXCEPTION` with a non-NULL `pei` that has a deferred fill-in
/// calls it, so the fill-in runs only under all three conditions.
pub fn deferred_fill_in_runs(hr: u32, pei_present: bool, fill_in_present: bool) -> bool {
    hr == DISP_E_EXCEPTION && pei_present && fill_in_present
}
