//! HSTRING logic from Wine's `dlls/combase`.
//!
//! Ported from `string.c` (HSTRING values and string buffers) and from
//! `usrmarshal.c` (the local wire record of `HSTRING_UserSize`,
//! `HSTRING_UserMarshal` and `HSTRING_UserUnmarshal`, in [`user_size`],
//! [`user_marshal`] and [`user_unmarshal`]). The VARTYPE check of
//! `propvar_validatetype` in `combase.c` is [`validate_vartype`].
//!
//! A null `HSTRING` is `None`, which is also the empty string. A non-null
//! `HSTRING` is never empty, because Wine's constructors turn empty input into
//! null. `Rc` stands in for Wine's reference count, so [`duplicate_string`]
//! shares the allocation as Wine does.
//!
//! Not ported: GUID parsing (`mpf-ole32` has it), `hglobalstream.c`, apartment,
//! RPC, marshal and stub-manager code, COM interface paths, the `Delete` and
//! `Free` paths (`Drop` releases memory), `PropVariantClear` and
//! `PropVariantCopy` memory handling, and the `MSHCTX_INPROC` wire record, which
//! holds a raw `HSTRING` pointer.
//!
//! The wire functions assume a 64-bit Wine build, where `sizeof(HSTRING)` is 8.

use std::rc::Rc;

mod hstring_wire;
mod propvariant;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use hstring_wire::{
    user_marshal, user_size, user_unmarshal, WireError, MSHCTX_DIFFERENTMACHINE, MSHCTX_INPROC,
};
pub use propvariant::validate_vartype;

/// A Windows `HRESULT`.
pub type HRESULT = i32;

pub const E_INVALIDARG: HRESULT = 0x8007_0057u32 as i32;
pub const E_POINTER: HRESULT = 0x8000_4003u32 as i32;
pub const E_OUTOFMEMORY: HRESULT = 0x8007_000Eu32 as i32;
pub const E_BOUNDS: HRESULT = 0x8000_000Bu32 as i32;
pub const STG_E_INVALIDPARAMETER: HRESULT = 0x8003_0057u32 as i32;

/// An `HSTRING`. `None` is the null handle. `Some` always holds at least one unit.
pub type HString = Option<Rc<[u16]>>;

/// Wraps `units` as an `HString`, mapping empty input to null.
fn from_units(units: &[u16]) -> HString {
    if units.is_empty() {
        None
    } else {
        Some(Rc::from(units))
    }
}

/// `WindowsGetStringRawBuffer`: the code units of `s`, without a terminator.
pub fn raw_buffer(s: &HString) -> &[u16] {
    s.as_deref().unwrap_or(&[])
}

/// `WindowsGetStringLen`. Every `HString` holds at most `u32::MAX` units, because
/// [`concat`] refuses longer results.
pub fn get_len(s: &HString) -> u32 {
    raw_buffer(s).len() as u32
}

/// `WindowsIsStringEmpty`.
pub fn is_empty(s: &HString) -> bool {
    raw_buffer(s).is_empty()
}

/// `WindowsStringHasEmbeddedNull`, over the logical length only.
pub fn has_embedded_null(s: &HString) -> bool {
    raw_buffer(s).contains(&0)
}

/// `WindowsCreateString`. `len == 0` gives null, even when `ptr` is `None`. A
/// `None` pointer with a non-zero length is `E_POINTER`. Wine reads `len` units
/// without a check, so a slice shorter than `len` is `E_INVALIDARG` here.
pub fn create_string(ptr: Option<&[u16]>, len: u32) -> Result<HString, HRESULT> {
    if len == 0 {
        return Ok(None);
    }
    let ptr = ptr.ok_or(E_POINTER)?;
    let units = ptr.get(..len as usize).ok_or(E_INVALIDARG)?;
    Ok(from_units(units))
}

/// `WindowsCreateStringReference`, without the caller's `HSTRING_HEADER`. Wine
/// aliases `ptr` and requires it to outlive the handle. This copies it, so the
/// result owns its units. The terminator check runs first, as in Wine. A slice
/// too short to hold the terminator is `E_INVALIDARG`.
pub fn create_string_reference(ptr: Option<&[u16]>, len: u32) -> Result<HString, HRESULT> {
    if let Some(ptr) = ptr {
        if ptr.get(len as usize) != Some(&0) {
            return Err(E_INVALIDARG);
        }
    }
    create_string(ptr, len)
}

/// `WindowsDuplicateString`. Shares the allocation, as Wine's reference count does.
pub fn duplicate_string(s: &HString) -> HString {
    s.clone()
}

/// `WindowsSubstring`: the units from `start` to the end. A `start` equal to the
/// length gives null, and a `start` past it is `E_BOUNDS`.
pub fn substring(s: &HString, start: u32) -> Result<HString, HRESULT> {
    let rest = raw_buffer(s).get(start as usize..).ok_or(E_BOUNDS)?;
    Ok(from_units(rest))
}

/// `WindowsSubstringWithSpecifiedLength`. The end is summed in 64 bits, so a
/// `start + len` that wraps in 32 bits is `E_BOUNDS`, as Wine's overflow check
/// also reports.
pub fn substring_with_length(s: &HString, start: u32, len: u32) -> Result<HString, HRESULT> {
    let units = raw_buffer(s);
    let end = u64::from(start) + u64::from(len);
    if end > units.len() as u64 {
        return Err(E_BOUNDS);
    }
    Ok(from_units(&units[start as usize..end as usize]))
}

/// `WindowsConcatString`. A null operand gives the other operand, shared as
/// `WindowsDuplicateString` does. Wine wraps the length when the result exceeds
/// `u32::MAX` units, which is undefined behaviour in C. That case is
/// `E_OUTOFMEMORY` here.
pub fn concat(a: &HString, b: &HString) -> Result<HString, HRESULT> {
    concat_bounded(a, b, u64::from(u32::MAX))
}

/// [`concat`] with the length cap as a parameter, so the overflow arm can be
/// tested without allocating 4 GiB.
fn concat_bounded(a: &HString, b: &HString, max_len: u64) -> Result<HString, HRESULT> {
    match (a.as_deref(), b.as_deref()) {
        (None, _) => Ok(b.clone()),
        (Some(_), None) => Ok(a.clone()),
        (Some(x), Some(y)) => {
            if x.len() as u64 + y.len() as u64 > max_len {
                return Err(E_OUTOFMEMORY);
            }
            Ok(from_units(&[x, y].concat()))
        }
    }
}

/// `WindowsCompareStringOrdinal`: `-1`, `0` or `1`. Code units compare as
/// unsigned values, then by length, so a null string sorts before any other.
pub fn compare_ordinal(a: &HString, b: &HString) -> i32 {
    raw_buffer(a).cmp(raw_buffer(b)) as i32
}

/// `WindowsTrimStringStart`: drops leading units that occur in `set`. A null
/// `set` is `E_INVALIDARG`, even when `s` is null.
pub fn trim_start(s: &HString, set: &HString) -> Result<HString, HRESULT> {
    let set = set.as_deref().ok_or(E_INVALIDARG)?;
    let units = raw_buffer(s);
    let start = units.iter().take_while(|unit| set.contains(unit)).count();
    Ok(if start == 0 {
        s.clone()
    } else {
        from_units(&units[start..])
    })
}

/// `WindowsTrimStringEnd`: drops trailing units that occur in `set`. A null
/// `set` is `E_INVALIDARG`, even when `s` is null.
pub fn trim_end(s: &HString, set: &HString) -> Result<HString, HRESULT> {
    let set = set.as_deref().ok_or(E_INVALIDARG)?;
    let units = raw_buffer(s);
    let keep = units
        .iter()
        .rposition(|unit| !set.contains(unit))
        .map_or(0, |last| last + 1);
    Ok(if keep == units.len() {
        s.clone()
    } else {
        from_units(&units[..keep])
    })
}

/// The buffer of `WindowsPreallocateStringBuffer`: `len` units followed by a
/// terminator slot, which must hold zero when the buffer is promoted.
pub struct StringBuffer {
    units: Vec<u16>,
}

impl StringBuffer {
    /// `WindowsPreallocateStringBuffer`. Every unit starts at zero. Allocation
    /// failure aborts, as any Rust allocation does, instead of `E_OUTOFMEMORY`.
    pub fn new(len: u32) -> Self {
        Self {
            units: vec![0; len as usize + 1],
        }
    }

    /// The `len` units to fill, followed by the terminator slot.
    pub fn as_mut_slice(&mut self) -> &mut [u16] {
        &mut self.units
    }

    /// `WindowsPromoteStringBuffer`. A non-zero terminator slot is `E_INVALIDARG`.
    /// Wine also rejects a shared or reference buffer, which this type cannot be.
    pub fn promote(mut self) -> Result<HString, HRESULT> {
        match self.units.pop() {
            Some(0) => Ok(from_units(&self.units)),
            _ => Err(E_INVALIDARG),
        }
    }
}
