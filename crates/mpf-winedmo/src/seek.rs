//! The read and seek callbacks of `dlls/winedmo/unixlib.c`, which serve FFmpeg's I/O context from
//! a stream that is read in fixed-capacity chunks.

pub const AVSEEK_SIZE: i32 = 0x10000;
pub const AVERROR_EINVAL: i32 = -22;
pub const AVERROR_EOF: i32 = -541_478_725;

const SEEK_CUR: i32 = 1;
const SEEK_END: i32 = 2;

/// `struct stream_context` from `dlls/winedmo/unixlib.h`. `buffer` holds `capacity` bytes, which
/// the read dispatch fills.
pub struct StreamContext {
    pub length: u64,
    pub position: u64,
    pub capacity: u32,
    pub size: u32,
    pub buffer: Vec<u8>,
}

/// `unix_seek_callback`. `dispatch` is the `KeUserDispatchCallback` seek request: it takes a chunk
/// aligned offset and returns the offset the client seeked to, or `None` on a failed or malformed
/// reply. A `None` makes the callback return `AVERROR_EINVAL` and leave the context untouched.
/// `capacity` must be non-zero, since the offsets are computed by dividing by it.
pub fn seek_callback(
    ctx: &mut StreamContext,
    offset: i64,
    whence: i32,
    dispatch: &mut dyn FnMut(u64) -> Option<u64>,
) -> i64 {
    if whence == AVSEEK_SIZE {
        return ctx.length as i64;
    }
    let offset = match whence {
        SEEK_END => offset.wrapping_add(ctx.length as i64),
        SEEK_CUR => offset.wrapping_add(ctx.position as i64),
        // SEEK_SET, and any other whence, keeps the offset as it is.
        _ => offset,
    };
    let mut offset = if offset as u64 > ctx.length {
        ctx.length
    } else {
        offset as u64
    };

    let capacity = u64::from(ctx.capacity);
    if offset / capacity != ctx.position / capacity {
        loop {
            let chunk = offset / capacity * capacity;
            let Some(reply) = dispatch(chunk) else {
                return i64::from(AVERROR_EINVAL);
            };
            if reply == chunk {
                break;
            }
            offset = reply;
        }
        ctx.size = 0;
    }
    ctx.position = offset;
    offset as i64
}

/// `unix_read_callback`. `out` gives the requested size. `dispatch` is the `stream_context_read`
/// request for `capacity` bytes, and returns the number of bytes the client put in the buffer, or
/// `None` on a failed or malformed reply. The data itself is already in `ctx.buffer`.
///
/// The offset into the buffer is subtracted from the buffer size with C unsigned wrap-around, so
/// a stale `size` below the offset copies the requested bytes. A stale size with a large enough
/// reply can read past `capacity`; the slice index then panics where the C code reads past the
/// allocation.
pub fn read_callback(
    ctx: &mut StreamContext,
    out: &mut [u8],
    dispatch: &mut dyn FnMut(u32) -> Option<u32>,
) -> i32 {
    let remaining = ctx.length.wrapping_sub(ctx.position);
    let mut size = remaining.min(out.len() as u64) as usize;
    if size == 0 {
        return AVERROR_EOF;
    }

    let capacity = u64::from(ctx.capacity);
    let mut total = 0;
    while size > 0 {
        let buffer_offset = (ctx.position % capacity) as usize;
        if ctx.size == 0 {
            let Some(reply) = dispatch(ctx.capacity) else {
                return AVERROR_EINVAL;
            };
            ctx.size = reply;
        }
        let step = size.min((ctx.size as usize).wrapping_sub(buffer_offset));
        if step == 0 {
            break;
        }
        out[total..total + step].copy_from_slice(&ctx.buffer[buffer_offset..buffer_offset + step]);
        total += step;
        size -= step;
        ctx.position += step as u64;
        if ctx.position.is_multiple_of(capacity) {
            ctx.size = 0;
        }
    }

    if total == 0 {
        return AVERROR_EOF;
    }
    total as i32
}
