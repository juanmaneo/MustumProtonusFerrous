//! `ACMDM_STREAM_CONVERT` for the MPEG Layer-3 decoder: the feed and read loop
//! of Wine's `mp3_horse`, run over a libmpg123-style [`Decoder`].

pub const ACM_STREAMCONVERTF_START: u32 = 0x0000_0010;

pub const MPG123_OK: i32 = 0;
pub const MPG123_ERR: i32 = -1;
pub const MPG123_NEED_MORE: i32 = -10;

/// The libmpg123 calls that the convert loop makes. `feed` and `read` return
/// the mpg123 result code, and `read` also returns the bytes it wrote.
pub trait Decoder {
    fn seek_to_start(&mut self);
    fn close(&mut self);
    fn open_feed(&mut self);
    fn feed(&mut self, data: &[u8]) -> i32;
    fn read(&mut self, out: &mut [u8]) -> (i32, usize);
}

/// The bytes that a conversion used from the source and wrote to the destination.
#[derive(Debug, PartialEq, Eq)]
pub struct Usage {
    pub src_used: usize,
    pub dst_used: usize,
}

/// `MPEG3_StreamConvert`: decodes `src` into `dst`. The `START` flag rewinds
/// the decoder first. Block alignment and end of stream need no work, and
/// other flags are ignored.
pub fn stream_convert<D: Decoder>(
    decoder: &mut D,
    fdw_convert: u32,
    src: &[u8],
    dst: &mut [u8],
) -> Usage {
    if (fdw_convert & ACM_STREAMCONVERTF_START) != 0 {
        decoder.seek_to_start();
        decoder.close();
        decoder.open_feed();
    }
    decode(decoder, src, dst)
}

/// `mp3_horse`: feeds `src`, then reads into `dst` until it is full or the
/// decoder needs more input. An error from the feed or a read uses nothing.
fn decode<D: Decoder>(decoder: &mut D, src: &[u8], dst: &mut [u8]) -> Usage {
    if !src.is_empty() && decoder.feed(src) != MPG123_OK {
        return Usage {
            src_used: 0,
            dst_used: 0,
        };
    }
    let mut dpos = 0;
    loop {
        let (ret, size) = decoder.read(&mut dst[dpos..]);
        if ret == MPG123_ERR {
            return Usage {
                src_used: 0,
                dst_used: 0,
            };
        }
        dpos = dpos.saturating_add(size);
        if dpos >= dst.len() || ret == MPG123_NEED_MORE {
            break;
        }
    }
    Usage {
        src_used: src.len(),
        dst_used: dpos,
    }
}
