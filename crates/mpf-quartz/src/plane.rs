//! `copy_plane` in `dlls/quartz/vmr9.c`: copies pixel rows between two buffers that have their own
//! pitches. A negative source height flips the rows.

/// How far `copy_plane` moves the source and destination cursors, in bytes.
pub struct Advance {
    pub src: usize,
    pub dst: usize,
}

/// `copy_plane`. Each row copies the smaller of the two pitches. `None` means the copy does not
/// fit the buffers, and nothing is written.
pub fn copy_plane(
    dst: &mut [u8],
    dst_pitch: usize,
    dst_height: usize,
    src: &[u8],
    src_pitch: usize,
    src_height: i32,
) -> Option<Advance> {
    let rows = src_height.unsigned_abs() as usize;
    let src_len = src_pitch.checked_mul(rows)?;
    let dst_len = dst_pitch.checked_mul(dst_height)?;
    if rows > dst_height || src.len() < src_len || dst.len() < dst_len {
        return None;
    }

    let copy_size = src_pitch.min(dst_pitch);
    for i in 0..rows {
        let row = if src_height < 0 { rows - 1 - i } else { i };
        dst[i * dst_pitch..][..copy_size].copy_from_slice(&src[row * src_pitch..][..copy_size]);
    }
    Some(Advance {
        src: src_len,
        dst: dst_len,
    })
}
