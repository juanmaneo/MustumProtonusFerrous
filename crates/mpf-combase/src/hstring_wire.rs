//! The local wire record of `HSTRING_UserSize`, `HSTRING_UserMarshal` and
//! `HSTRING_UserUnmarshal` in Wine's `dlls/combase/usrmarshal.c`.
//!
//! A record is a `u32` context (`WDT_INPROC64_CALL` on marshal), a `u32` byte
//! count and the UTF-16LE units. Each record starts on an 8-byte boundary.
//! `MSHCTX_INPROC` records hold a raw `HSTRING` pointer, so marshal and
//! unmarshal refuse them.

use crate::{from_units, raw_buffer, HString};

/// `MSHCTX_DIFFERENTMACHINE`. Wine raises `RPC_S_INVALID_TAG` when marshalling.
pub const MSHCTX_DIFFERENTMACHINE: u32 = 2;
/// `MSHCTX_INPROC`, which selects the in-process record.
pub const MSHCTX_INPROC: u32 = 3;

/// `WDT_INPROC64_CALL`, the context of a 64-bit `HSTRING` record.
const WDT_INPROC64_CALL: u32 = 0x5074_6457;
/// Size of the in-process record on 64-bit: context, padding and pointer.
const INPROC_RECORD: u64 = 16;
/// Size of the local header: context and byte count.
const LOCAL_HEADER: u64 = 8;

/// A failed wire operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WireError {
    /// `MSHCTX_INPROC` record, which holds a raw `HSTRING` pointer.
    InProcess,
    /// `MSHCTX_DIFFERENTMACHINE` marshalling. Wine raises `RPC_S_INVALID_TAG` (1733).
    DifferentMachine,
    /// The record does not fit the buffer, or its size does not fit the `u32` field.
    OutOfBounds,
}

fn is_inproc(flags: u32) -> bool {
    (flags & 0xFFFF) == MSHCTX_INPROC
}

/// Rounds `offset` up to the next 8-byte boundary. It saturates, so a huge offset
/// fails the bounds checks instead of wrapping.
fn align8(offset: usize) -> usize {
    offset.saturating_add(7) & !7
}

/// Where the local record for `units` code units goes when written at `offset`
/// in a buffer of `buf_len` bytes: its aligned start, its end, and its `u32`
/// byte count. It takes the unit count rather than a slice, so the `u32` limit
/// can be tested without a 4 GiB string.
pub(crate) fn local_layout(
    offset: usize,
    units: usize,
    buf_len: usize,
) -> Result<(usize, usize, u32), WireError> {
    let start = align8(offset);
    let bytes = 2 * units as u64;
    let end = (start as u64).saturating_add(LOCAL_HEADER + bytes);
    if bytes > u64::from(u32::MAX) || end > buf_len as u64 {
        return Err(WireError::OutOfBounds);
    }
    Ok((start, end as usize, bytes as u32))
}

/// `HSTRING_UserSize`: the bytes a record needs from `start`, including its
/// alignment padding. Only `MSHCTX_INPROC` selects the in-process size. Every
/// other context, including `MSHCTX_DIFFERENTMACHINE`, sizes the local record.
/// Wine wraps a `ULONG` overflow, which is undefined behaviour in C. Here it is
/// `OutOfBounds`.
pub fn user_size(flags: u32, start: u32, s: &HString) -> Result<u32, WireError> {
    let record = if is_inproc(flags) {
        INPROC_RECORD
    } else {
        LOCAL_HEADER + 2 * raw_buffer(s).len() as u64
    };
    let aligned = (u64::from(start) + 7) & !7;
    u32::try_from(aligned + record).map_err(|_| WireError::OutOfBounds)
}

/// `HSTRING_UserMarshal` for local contexts. Writes the record at `offset`,
/// rounded up to 8 bytes, and returns the offset just past it. Alignment is
/// relative to `buf`. Wine aligns absolute addresses, so `buf` must start on an
/// 8-byte boundary for the two to agree.
pub fn user_marshal(
    flags: u32,
    buf: &mut [u8],
    offset: usize,
    s: &HString,
) -> Result<usize, WireError> {
    match flags & 0xFFFF {
        MSHCTX_INPROC => return Err(WireError::InProcess),
        MSHCTX_DIFFERENTMACHINE => return Err(WireError::DifferentMachine),
        _ => {}
    }
    let units = raw_buffer(s);
    let (start, end, size) = local_layout(offset, units.len(), buf.len())?;
    let record = &mut buf[start..end];
    record[..4].copy_from_slice(&WDT_INPROC64_CALL.to_le_bytes());
    record[4..8].copy_from_slice(&size.to_le_bytes());
    for (dst, unit) in record[8..].as_chunks_mut::<2>().0.iter_mut().zip(units) {
        *dst = unit.to_le_bytes();
    }
    Ok(end)
}

/// `HSTRING_UserUnmarshal` for local contexts. Returns the offset just past the
/// record and its string. Only `MSHCTX_INPROC` is refused, and every other
/// context reads the local record, as Wine does. The context word is not
/// checked. An odd byte count drops its final byte.
pub fn user_unmarshal(
    flags: u32,
    buf: &[u8],
    offset: usize,
) -> Result<(usize, HString), WireError> {
    if is_inproc(flags) {
        return Err(WireError::InProcess);
    }
    let start = align8(offset);
    let header = buf
        .get(start..)
        .and_then(|rest| rest.get(..LOCAL_HEADER as usize))
        .ok_or(WireError::OutOfBounds)?;
    let size = u32::from_le_bytes([header[4], header[5], header[6], header[7]]);
    let data_len = (size & !1) as usize;
    let body = start + LOCAL_HEADER as usize;
    let data = buf
        .get(body..)
        .and_then(|rest| rest.get(..data_len))
        .ok_or(WireError::OutOfBounds)?;
    let units: Vec<u16> = data
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u16::from_le_bytes(*pair))
        .collect();
    Ok((body + data_len, from_units(&units)))
}
