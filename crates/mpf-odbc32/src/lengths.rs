//! Copies between the driver's packed length buffer and the application's `SQLLEN` array (`len_to_user` and `len_from_user`).

/// Size of one length slot. The port uses the Win64 `SQLLEN`, which is `INT64`.
const SLOT: usize = 8;

/// The caller's buffers are too short for the requested number of rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LengthError {
    BufferTooShort,
}

/// Bytes that `row_count` slots placed `width` bytes apart read or write. Zero rows touch nothing.
fn span(row_count: usize, width: usize) -> usize {
    match row_count {
        0 => 0,
        rows => (rows - 1).saturating_mul(width).saturating_add(SLOT),
    }
}

/// Copies `row_count` native-endian lengths from `lens` into `dst`. `None` skips the copy, as a NULL pointer does in C.
///
/// Both buffers are checked before anything is written. The C code does not check them.
pub fn len_to_user(
    dst: Option<&mut [i64]>,
    lens: &[u8],
    row_count: usize,
    width: usize,
) -> Result<(), LengthError> {
    let Some(dst) = dst else {
        return Ok(());
    };
    if dst.len() < row_count || lens.len() < span(row_count, width) {
        return Err(LengthError::BufferTooShort);
    }
    for (row, value) in dst[..row_count].iter_mut().enumerate() {
        let at = row * width;
        *value = i64::from_ne_bytes(lens[at..at + SLOT].try_into().expect("slot is SLOT bytes"));
    }
    Ok(())
}

/// Stores `row_count` lengths from `src` into `lens`, each `width` bytes after the last. `None` skips the copy.
///
/// Both buffers are checked before anything is written. The C code does not check them.
pub fn len_from_user(
    lens: &mut [u8],
    src: Option<&[i64]>,
    row_count: usize,
    width: usize,
) -> Result<(), LengthError> {
    let Some(src) = src else {
        return Ok(());
    };
    if src.len() < row_count || lens.len() < span(row_count, width) {
        return Err(LengthError::BufferTooShort);
    }
    for (row, value) in src[..row_count].iter().enumerate() {
        let at = row * width;
        lens[at..at + SLOT].copy_from_slice(&value.to_ne_bytes());
    }
    Ok(())
}
