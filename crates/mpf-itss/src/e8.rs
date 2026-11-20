//! The E8 (x86 CALL) translation that the LZX encoder applies, undone after each frame.

/// Undoes the E8 translation in `out`, one frame of output. `curpos` is the position of `out[0]`
/// in the translated stream and advances by the frame length. Arithmetic wraps as in lzx.c.
pub(crate) fn undo_e8(out: &mut [u8], curpos: &mut i32, filesize: i32, started: bool) {
    let len = out.len();
    if len <= 6 || !started {
        *curpos = curpos.wrapping_add(len as i32);
        return;
    }
    let mut cur = *curpos;
    *curpos = cur.wrapping_add(len as i32);
    let mut i = 0;
    // An E8 byte in the last 10 bytes is left alone, so its operand stays clear of the last 6 bytes.
    while i + 10 < len {
        let b = out[i];
        i += 1;
        if b != 0xE8 {
            cur = cur.wrapping_add(1);
            continue;
        }
        let abs = i32::from_le_bytes([out[i], out[i + 1], out[i + 2], out[i + 3]]);
        if abs >= cur.wrapping_neg() && abs < filesize {
            let rel = if abs >= 0 {
                abs.wrapping_sub(cur)
            } else {
                abs.wrapping_add(filesize)
            };
            out[i..i + 4].copy_from_slice(&rel.to_le_bytes());
        }
        i += 4;
        cur = cur.wrapping_add(5);
    }
}
