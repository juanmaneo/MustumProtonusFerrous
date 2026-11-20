//! The wide-character helpers that `query.c` takes from the C runtime: `towupper`, `towlower` and
//! `wcsicmp`. Strings are slices of UTF-16 code units, and the end of a string is the end of its
//! slice. [at] reads past the end as the NUL terminator of a C string would.

/// The code unit at `index`, or NUL past the end of `s`.
pub(crate) fn at(s: &[u16], index: usize) -> u16 {
    s.get(index).copied().unwrap_or(0)
}

/// `s` without its first code unit. An empty slice stays empty.
pub(crate) fn rest(s: &[u16]) -> &[u16] {
    s.get(1..).unwrap_or_default()
}

fn single(mut mapped: impl Iterator<Item = char>, unit: u16) -> u16 {
    match (mapped.next(), mapped.next()) {
        (Some(one), None) => u16::try_from(u32::from(one)).unwrap_or(unit),
        _ => unit,
    }
}

/// Upper case of one code unit. A surrogate, or a character whose mapping has several characters
/// (such as `ß`), keeps its unit.
pub fn towupper(unit: u16) -> u16 {
    match char::from_u32(u32::from(unit)) {
        Some(ch) => single(ch.to_uppercase(), unit),
        None => unit,
    }
}

/// Lower case of one code unit, with the same rules as [towupper].
pub fn towlower(unit: u16) -> u16 {
    match char::from_u32(u32::from(unit)) {
        Some(ch) => single(ch.to_lowercase(), unit),
        None => unit,
    }
}

/// Case-insensitive comparison, as `_wcsicmp_l` in `dlls/msvcrt/wcs.c`: the difference of the first
/// pair of lower-cased units that differ, or of the terminators.
pub fn wcsicmp(a: &[u16], b: &[u16]) -> i32 {
    let mut index = 0;
    loop {
        let c1 = towlower(at(a, index));
        let c2 = towlower(at(b, index));
        if c1 == 0 || c1 != c2 {
            return i32::from(c1) - i32::from(c2);
        }
        index += 1;
    }
}
