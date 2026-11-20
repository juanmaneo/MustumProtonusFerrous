//! `is_ime_hkl` in `dlls/imm32/ime.c`: whether a keyboard layout is an input method editor.

/// `MAKELANGID`: the language ID from a primary language and a sublanguage.
pub const fn makelangid(primary: u16, sub: u16) -> u16 {
    (sub << 10) | primary
}

/// The `(LANG_*, SUBLANG_*)` pairs of the `MAKELANGID` cases in `is_ime_hkl`, in source order.
const IME_LANGIDS: [(u16, u16); 20] = [
    (0x5e, 0x01),
    (0x45, 0x01),
    (0x04, 0x02),
    (0x04, 0x01),
    (0x47, 0x01),
    (0x39, 0x01),
    (0x11, 0x01),
    (0x4b, 0x01),
    (0x12, 0x01),
    (0x4c, 0x01),
    (0x4e, 0x01),
    (0x61, 0x01),
    (0x48, 0x01),
    (0x46, 0x01),
    (0x49, 0x01),
    (0x49, 0x02),
    (0x4a, 0x01),
    (0x73, 0x01),
    (0x2a, 0x01),
    (0x78, 0x01),
];

/// `is_ime_hkl`: the high word of `hkl` is one of the IME language IDs, or it has the `0xe000`
/// bits that a device layout carries.
pub fn is_ime_hkl(hkl: usize) -> bool {
    let langid = (hkl >> 16) as u16;
    IME_LANGIDS
        .iter()
        .any(|&(primary, sub)| makelangid(primary, sub) == langid)
        || (langid & 0xf000) == 0xe000
}
