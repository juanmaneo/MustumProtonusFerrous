//! Port of the `fmtflags` members of `ios_base` from Wine's `dlls/msvcp60/ios.c`.
//!
//! The `ios_base` object is reduced to its `fmtfl` field, which each function takes by mutable
//! reference. `IOSB_fmtflags` is a 32-bit enum, so the flags are `u32`.

/// `FMTFLAG_mask`: the bits that `flags_set`, `setf` and `setf_mask` keep. `unsetf` does not mask.
pub const FMTFLAG_MASK: u32 = 0xffff;

/// `flags(fmtflags)`: replaces the flags and returns the old ones. Bits above [`FMTFLAG_MASK`]
/// are dropped.
pub fn flags_set(fmtfl: &mut u32, flags: u32) -> u32 {
    std::mem::replace(fmtfl, flags & FMTFLAG_MASK)
}

/// `flags()`: returns the current flags.
pub fn flags_get(fmtfl: &u32) -> u32 {
    *fmtfl
}

/// `setf(fmtflags)`: sets the flags and returns the old ones.
pub fn setf(fmtfl: &mut u32, flags: u32) -> u32 {
    let old = *fmtfl;
    *fmtfl |= flags & FMTFLAG_MASK;
    old
}

/// `setf(fmtflags, mask)`: replaces the bits in `mask` with `flags & mask`, and returns the old
/// flags.
pub fn setf_mask(fmtfl: &mut u32, flags: u32, mask: u32) -> u32 {
    let old = *fmtfl;
    *fmtfl = (*fmtfl & !mask) | (flags & mask & FMTFLAG_MASK);
    old
}

/// `unsetf(fmtflags)`: clears the flags. Wine does not mask them, so bits above [`FMTFLAG_MASK`]
/// are cleared too.
pub fn unsetf(fmtfl: &mut u32, flags: u32) {
    *fmtfl &= !flags;
}
