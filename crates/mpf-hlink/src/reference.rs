use crate::{c_str, non_empty, HResult, E_INVALIDARG};

const HLINKSETF_TARGET: u32 = 1;
const HLINKSETF_LOCATION: u32 = 2;
const HLINKSETF_ALL: u32 = HLINKSETF_TARGET | HLINKSETF_LOCATION;

const HLINKGETREF_DEFAULT: u32 = 0;
const HLINKGETREF_ABSOLUTE: u32 = 1;
const HLINKGETREF_RELATIVE: u32 = 2;
const HLINKGETREF_KNOWN: u32 = HLINKGETREF_ABSOLUTE | HLINKGETREF_RELATIVE;

const COLON: u16 = b':' as u16;

/// A target is a URL when its first colon comes after at least two units. A drive
/// letter such as `C:` is a file.
pub fn fallback_is_url(target: &[u16]) -> bool {
    c_str(target)
        .iter()
        .position(|&unit| unit == COLON)
        .is_some_and(|colon| colon > 1)
}

/// `GetMoniker` treats the default reference type as relative.
pub fn resolve_ref_type(ref_type: u32) -> u32 {
    if ref_type == HLINKGETREF_DEFAULT {
        HLINKGETREF_RELATIVE
    } else {
        ref_type
    }
}

/// `GetStringReference` accepts the all-ones sentinel and any combination of the known
/// reference bits.
pub fn check_get_string_reference(which: u32) -> Result<(), HResult> {
    if which != u32::MAX && (which & !HLINKGETREF_KNOWN) != 0 {
        Err(E_INVALIDARG)
    } else {
        Ok(())
    }
}

/// What `SetMonikerReference` changes.
#[derive(Debug, PartialEq, Eq)]
pub struct MonikerPlan {
    pub target: bool,
    pub location: bool,
}

/// Checks the flags of `SetMonikerReference`. Zero is `E_INVALIDARG`. Flags with neither
/// bit set are returned as the HRESULT, as in C.
pub fn set_moniker_reference_plan(flags: u32) -> Result<MonikerPlan, HResult> {
    if flags == 0 {
        return Err(E_INVALIDARG);
    }
    if (flags & HLINKSETF_ALL) == 0 {
        return Err(flags as HResult);
    }
    Ok(MonikerPlan {
        target: (flags & HLINKSETF_TARGET) != 0,
        location: (flags & HLINKSETF_LOCATION) != 0,
    })
}

/// What `SetStringReference` changes. A target or location is set only when the caller
/// supplies a non-empty string for it.
#[derive(Debug, PartialEq, Eq)]
pub struct StringPlan<'a> {
    pub clear_target: bool,
    pub target: Option<&'a [u16]>,
    pub replace_location: bool,
    pub location: Option<&'a [u16]>,
}

/// Checks the flags of `SetStringReference`. Flags from 4 to 0xFFFF_FFFC are returned as
/// the HRESULT, as in C. Flags outside that range take the normal path.
pub fn set_string_reference_plan<'a>(
    flags: u32,
    target: Option<&'a [u16]>,
    location: Option<&'a [u16]>,
) -> Result<StringPlan<'a>, HResult> {
    if flags > HLINKSETF_ALL && flags < HLINKSETF_ALL.wrapping_neg() {
        return Err(flags as HResult);
    }
    let clear_target = (flags & HLINKSETF_TARGET) != 0;
    let replace_location = (flags & HLINKSETF_LOCATION) != 0;
    Ok(StringPlan {
        clear_target,
        target: if clear_target {
            non_empty(target)
        } else {
            None
        },
        replace_location,
        location: if replace_location {
            non_empty(location)
        } else {
            None
        },
    })
}
