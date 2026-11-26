//! `EnumPrintProcessorDatatypesW`: parameter checks, size query and output layout.

use crate::{PpError, RAW, TEXT};

/// Pointer width of the target. It sets the size of `DATATYPES_INFO_1W`, which holds one `LPWSTR`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arch {
    X86,
    X64,
}

impl Arch {
    pub fn pointer_size(self) -> usize {
        match self {
            Self::X86 => 4,
            Self::X64 => 8,
        }
    }
}

/// `EnumPrintProcessorDatatypesW` for `level` 1.
///
/// On success `datatypes` holds two `DATATYPES_INFO_1W` entries, then the NUL-terminated
/// UTF-16LE strings `RAW` and `TEXT`. Each `pName` is written as its byte offset from the start
/// of `datatypes`; a caller that needs absolute pointers adds the buffer's base address.
/// When given, `datatypes` must be at least `size` bytes long.
///
/// `needed` and `no` are set before `level`, `size` and `datatypes` are validated, as in the C
/// code. Both are left untouched only when one of them is `None`.
pub fn enum_print_processor_datatypes_w(
    arch: Arch,
    level: u32,
    size: u32,
    datatypes: Option<&mut [u8]>,
    needed: Option<&mut u32>,
    no: Option<&mut u32>,
) -> Result<(), PpError> {
    let (Some(needed), Some(no)) = (needed, no) else {
        return Err(PpError::InvalidParameter);
    };
    let ptr = arch.pointer_size();
    let raw_at = 2 * ptr;
    let text_at = raw_at + 2 * (RAW.len() + 1);
    *no = 0;
    *needed = (text_at + 2 * (TEXT.len() + 1)) as u32;
    if level != 1 || (size != 0 && datatypes.is_none()) {
        return Err(PpError::InvalidParameter);
    }
    let buf = match datatypes {
        Some(buf) if size >= *needed => buf,
        _ => return Err(PpError::InsufficientBuffer),
    };
    put_pointer(buf, 0, ptr, raw_at);
    put_pointer(buf, ptr, ptr, text_at);
    put_wide_z(buf, raw_at, RAW);
    put_wide_z(buf, text_at, TEXT);
    *no = 2;
    Ok(())
}

/// Writes `value` as a little-endian pointer of `ptr` bytes at `at`.
fn put_pointer(buf: &mut [u8], at: usize, ptr: usize, value: usize) {
    buf[at..at + ptr].copy_from_slice(&(value as u64).to_le_bytes()[..ptr]);
}

/// Writes `s` and a terminating NUL as UTF-16LE at `at`.
fn put_wide_z(buf: &mut [u8], at: usize, s: &[u16]) {
    for (i, unit) in s.iter().chain(&[0u16]).enumerate() {
        buf[at + 2 * i..at + 2 * i + 2].copy_from_slice(&unit.to_le_bytes());
    }
}
