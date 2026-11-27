//! `mmioStringToFOURCCA` and `mmioStringToFOURCCW` from `dlls/winmm/mmio.c`, with `mmioFOURCC` from
//! `include/mmsystem.h` and `RtlUpperChar` from `dlls/ntdll/rtlstr.c`.

use crate::acp::wide_to_acp_byte;

/// `MMIO_TOUPPER`, from `include/mmsystem.h`.
pub const MMIO_TOUPPER: u32 = 0x0010;

/// `mmioFOURCC`: packs four bytes with the first in the low byte.
pub fn mmio_fourcc(c0: u8, c1: u8, c2: u8, c3: u8) -> u32 {
    u32::from_le_bytes([c0, c1, c2, c3])
}

/// `RtlUpperChar` for the byte range the port uses: only `a` to `z` change.
pub fn rtl_upper_char(ch: u8) -> u8 {
    if ch.is_ascii_lowercase() {
        ch.to_ascii_uppercase()
    } else {
        ch
    }
}

/// `mmioStringToFOURCCA`. Reads at most four bytes and stops at a NUL. A byte past the end of `sz`
/// reads as NUL, and the rest of the four bytes stay spaces.
pub fn mmio_string_to_fourcc_a(sz: &[u8], flags: u32) -> u32 {
    let mut cc = [b' '; 4];
    for (out, &ch) in cc.iter_mut().zip(sz) {
        if ch == 0 {
            break;
        }
        *out = if (flags & MMIO_TOUPPER) != 0 {
            rtl_upper_char(ch)
        } else {
            ch
        };
    }
    mmio_fourcc(cc[0], cc[1], cc[2], cc[3])
}

/// `mmioStringToFOURCCW`. Converts the four units with `CP_ACP` and then reads them as
/// [`mmio_string_to_fourcc_a`] does.
pub fn mmio_string_to_fourcc_w(sz: &[u16; 4], flags: u32) -> u32 {
    mmio_string_to_fourcc_a(&sz.map(wide_to_acp_byte), flags)
}
