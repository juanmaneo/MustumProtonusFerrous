//! Headless port of Wine's `dlls/iccvid`, the Cinepak (`cvid`) video decoder.
//!
//! The C reference is a `DriverProc` driver: its messages arrive through ICM,
//! run on driver instances and write to registry state. Here the Cinepak
//! decoder runs on plain byte slices and mirrors the C control flow step for
//! step, so its output matches the reference byte for byte.
//!
//! The component exports one entry point, `DriverProc` (`iccvid.spec`), which
//! is skipped (see below). Everything the decoder does is ported.
//!
//! Ported (names in `dlls/iccvid/iccvid.c`):
//! - `decode_cinepak_init`: [`Decoder::new`]. The clip table is not needed: colour
//!   conversion clamps directly.
//! - `free_cvinfo`: implicit, the strip codebooks are owned `Vec`s.
//! - `read_codebook`: private `read_codebook`, the YUV-to-RGB entry reader.
//! - `get_addr`: private `get_addr`, the top-down or bottom-up block address.
//! - `cvid_v1_15`, `cvid_v1_16`, `cvid_v1_24`, `cvid_v1_32`: [`write_v1`] with a [`Depth`].
//! - `cvid_v4_15`, `cvid_v4_16`, `cvid_v4_24`, `cvid_v4_32`: [`write_v4`] with a [`Depth`].
//! - `decode_cinepak`: [`Decoder::decode_frame`].
//!
//! Skipped:
//! - `DllMain`: process attach has no portable logic.
//! - `ICCVID_DriverProc` and its handlers (`ICCVID_Close`, `ICCVID_GetInfo`,
//!   `decompress_query`, `ICCVID_DecompressGetFormat`, `decompress_begin`,
//!   `decompress`, `ICCVID_DecompressEnd`): ICM message dispatch that needs
//!   driver instances and handles.
//! - `fill_decompressex_params`, `ICCVID_CheckMask`, `get_stride`, and the bitmap
//!   format checks: ICM format negotiation on `BITMAPINFO` headers.
//! - `ICCVID_dump_BITMAPINFO`, and the `ERR`/`WARN`/`TRACE` diagnostics: logging.
//! - The `frame.length` check and the END INFO trailer: they only log a size
//!   mismatch. The length field is read and ignored, so output is unaffected.
//!
//! Deviations, where Wine has undefined behaviour and the port returns an
//! [`Error`] instead: a frame that runs past its bytes ([`Error::Truncated`]),
//! more codebook entries than a strip table holds ([`Error::CodebookOverflow`]),
//! a block that would be written past the output ([`Error::OutOfBounds`]), and
//! a frame with 32 or more strips, which Wine drops silently
//! ([`Error::TooManyStrips`]). Chunk skips past the end of the frame stop at the
//! end, and a negative codebook count is treated as zero.

#[cfg(test)]
mod oracle;
#[cfg(test)]
mod tests;

/// Strips a frame may carry. Wine drops a frame with this many or more (`MAX_STRIPS`).
const MAX_STRIPS: usize = 32;
/// Codebook entries per strip and table. Wine allocates 260 of each.
const CODEBOOK_ENTRIES: usize = 260;

/// Why a frame could not be decoded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// The frame ends inside a header, a chunk, or a codebook entry.
    Truncated,
    /// The frame has 32 or more strips.
    TooManyStrips,
    /// A codebook chunk holds more entries than a table has, or a flagged chunk
    /// names an entry past the table.
    CodebookOverflow,
    /// A block would be written past the end of the output.
    OutOfBounds,
}

/// Output pixel format, chosen from the `bit_per_pixel` the caller passes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Depth {
    /// 15-bit RGB in a 16-bit word.
    Bits15,
    /// 16-bit RGB (5-6-5).
    Bits16,
    /// 24-bit BGR bytes.
    Bits24,
    /// 32-bit pixels, BGR bytes and a zero pad byte.
    Bits32,
}

impl Depth {
    /// Maps a bit depth to its format. As in Wine, any depth other than 15, 16
    /// or 32 is written as 24-bit.
    pub fn from_bits(bits: u32) -> Self {
        match bits {
            15 => Self::Bits15,
            16 => Self::Bits16,
            32 => Self::Bits32,
            _ => Self::Bits24,
        }
    }

    fn bytes_per_pixel(self) -> usize {
        match self {
            Self::Bits15 | Self::Bits16 => 2,
            Self::Bits24 => 3,
            Self::Bits32 => 4,
        }
    }

    /// Bytes between block rows. Wine rounds the stride down to whole pixels for
    /// 16-bit and 32-bit output, and uses it unchanged for 24-bit.
    fn row_step(self, stride: usize) -> usize {
        match self {
            Self::Bits15 | Self::Bits16 => stride / 2 * 2,
            Self::Bits24 => stride,
            Self::Bits32 => stride / 4 * 4,
        }
    }

    /// The pixel's bytes in memory order. Only the first `bytes_per_pixel` are used.
    fn pack(self, r: u8, g: u8, b: u8) -> [u8; 4] {
        let word = match self {
            Self::Bits15 => {
                (u16::from(r >> 3) << 10) | (u16::from(g >> 3) << 5) | u16::from(b >> 3)
            }
            Self::Bits16 => {
                (u16::from(r >> 3) << 11) | (u16::from(g >> 2) << 5) | u16::from(b >> 3)
            }
            Self::Bits24 | Self::Bits32 => return [b, g, r, 0],
        };
        let [lo, hi] = word.to_le_bytes();
        [lo, hi, 0, 0]
    }
}

/// The caller's output frame: `pixels` holds `height` rows of `stride` bytes.
pub struct Target<'a> {
    /// Output buffer, written in place.
    pub pixels: &'a mut [u8],
    /// Frame width in pixels. Blocks wrap to the next row of blocks at this width.
    pub width: usize,
    /// Frame height in pixels.
    pub height: usize,
    /// Bytes per row.
    pub stride: usize,
    /// Pixel format.
    pub depth: Depth,
    /// `true` writes rows top-down. `false` writes bottom-up, as Cinepak stores them.
    pub inverted: bool,
}

impl Target<'_> {
    /// Address of the block at `(x, y)` in cinepak coordinates (`get_addr`).
    fn addr(&self, x: usize, y: usize) -> i64 {
        get_addr(
            self.inverted,
            x,
            y,
            self.stride,
            self.depth.bytes_per_pixel(),
            self.height,
        )
    }

    fn put_v1(&mut self, x: usize, y: usize, cb: &Codebook) -> Result<(), Error> {
        let addr = self.addr(x, y);
        write_v1(
            self.pixels,
            addr,
            self.stride,
            self.inverted,
            self.depth,
            cb,
        )
    }

    fn put_v4(&mut self, x: usize, y: usize, cbs: [&Codebook; 4]) -> Result<(), Error> {
        let addr = self.addr(x, y);
        write_v4(
            self.pixels,
            addr,
            self.stride,
            self.inverted,
            self.depth,
            cbs,
        )
    }

    /// Moves to the next block: right by 4, and down 4 when the row is full.
    fn advance(&self, x: &mut usize, y: &mut usize) {
        *x += 4;
        if *x >= self.width {
            *x = 0;
            *y += 4;
        }
    }
}

/// Decoder state that persists across the frames of one stream (`cinepak_info`).
#[derive(Default)]
pub struct Decoder {
    strips: Vec<Strip>,
}

impl Decoder {
    /// Creates the context for a stream (`decode_cinepak_init`).
    pub fn new() -> Self {
        Self::default()
    }

    /// Decodes one Cinepak frame into `target` (`decode_cinepak`).
    ///
    /// Codebooks carry over from earlier frames unless the frame replaces them.
    /// On error, `target` may already hold partial output, as in the C code.
    pub fn decode_frame(&mut self, frame: &[u8], mut target: Target<'_>) -> Result<(), Error> {
        let mut r = Reader::new(frame);
        let flags = r.byte()?;
        // Frame length, then width and height. Wine only logs a mismatch with the
        // buffer size, and the output size comes from the caller.
        r.take::<3>()?;
        r.take::<4>()?;
        let strips = usize::from(r.word()?);
        if strips >= MAX_STRIPS {
            return Err(Error::TooManyStrips);
        }
        self.strips.resize_with(strips, Strip::new);

        let mut y = 0;
        let mut y_bottom = 0;
        for cur in 0..strips {
            if cur > 0 && flags & 1 == 0 {
                let (done, rest) = self.strips.split_at_mut(cur);
                rest[0].v4.copy_from_slice(&done[cur - 1].v4);
                rest[0].v1.copy_from_slice(&done[cur - 1].v1);
            }
            let strip = &mut self.strips[cur];

            // Strip kind (0x1000 key, 0x1100 iterated), origin, and width: decoded alike.
            r.word()?;
            let mut top_size = i64::from(r.word()?) - 12;
            r.take::<4>()?;
            y_bottom += usize::from(r.word()?);
            r.word()?;

            let mut x = 0;
            while top_size > 0 {
                let chunk_id = r.word()?;
                let mut chunk_size = i64::from(r.word()?);
                top_size -= chunk_size;
                chunk_size -= 4;

                let v1 = matches!(chunk_id, 0x2200 | 0x2300 | 0x2600 | 0x2700);
                let mode = if matches!(chunk_id, 0x2400 | 0x2500 | 0x2600 | 0x2700) {
                    Mode::Mono
                } else {
                    Mode::Colour
                };
                let book = if v1 { &mut strip.v1 } else { &mut strip.v4 };
                match chunk_id {
                    0x2000 | 0x2200 | 0x2400 | 0x2600 => {
                        read_plain_codebooks(&mut r, book, chunk_size, mode)?;
                    }
                    0x2100 | 0x2300 | 0x2500 | 0x2700 => {
                        read_masked_codebooks(&mut r, book, chunk_size, mode)?;
                    }
                    0x3000 => {
                        while chunk_size > 0 && y < y_bottom {
                            let mut flag = r.long()?;
                            chunk_size -= 4;
                            for _ in 0..32 {
                                if y >= y_bottom {
                                    break;
                                }
                                if flag & 0x8000_0000 != 0 {
                                    let d = r.take::<4>()?;
                                    chunk_size -= 4;
                                    target.put_v4(x, y, quad(&strip.v4, d))?;
                                } else {
                                    let d = r.byte()?;
                                    chunk_size -= 1;
                                    target.put_v1(x, y, &strip.v1[usize::from(d)])?;
                                }
                                target.advance(&mut x, &mut y);
                                flag <<= 1;
                            }
                        }
                        r.skip(chunk_size);
                    }
                    0x3100 => {
                        while chunk_size > 0 && y < y_bottom {
                            let mut flag = r.long()?;
                            chunk_size -= 4;
                            let mut mask = 0x8000_0000u32;
                            while mask != 0 && y < y_bottom {
                                if flag & mask != 0 {
                                    if mask == 1 {
                                        if chunk_size < 0 {
                                            break;
                                        }
                                        flag = r.long()?;
                                        chunk_size -= 4;
                                        mask = 0x8000_0000;
                                    } else {
                                        mask >>= 1;
                                    }
                                    if flag & mask != 0 {
                                        let d = r.take::<4>()?;
                                        chunk_size -= 4;
                                        target.put_v4(x, y, quad(&strip.v4, d))?;
                                    } else {
                                        chunk_size -= 1;
                                        let d = r.byte()?;
                                        target.put_v1(x, y, &strip.v1[usize::from(d)])?;
                                    }
                                }
                                mask >>= 1;
                                target.advance(&mut x, &mut y);
                            }
                        }
                        r.skip(chunk_size);
                    }
                    0x3200 => {
                        while chunk_size > 0 && y < y_bottom {
                            let d = r.byte()?;
                            chunk_size -= 1;
                            target.put_v1(x, y, &strip.v1[usize::from(d)])?;
                            target.advance(&mut x, &mut y);
                        }
                        r.skip(chunk_size);
                    }
                    _ => r.skip(chunk_size),
                }
            }
        }
        Ok(())
    }
}

/// A strip's two codebook tables. Each table holds `CODEBOOK_ENTRIES` entries.
struct Strip {
    v4: Vec<Codebook>,
    v1: Vec<Codebook>,
}

impl Strip {
    fn new() -> Self {
        Self {
            v4: vec![Codebook::default(); CODEBOOK_ENTRIES],
            v1: vec![Codebook::default(); CODEBOOK_ENTRIES],
        }
    }
}

/// One codebook entry: the colours of a 2x2 block of pixels, in row-major order.
#[derive(Clone, Copy, Default)]
struct Codebook {
    r: [u8; 4],
    g: [u8; 4],
    b: [u8; 4],
}

/// Codebook entry size: `Colour` entries are YUV (6 bytes), `Mono` entries are luma only (4 bytes).
#[derive(Clone, Copy)]
enum Mode {
    Colour,
    Mono,
}

impl Mode {
    fn size(self) -> i64 {
        match self {
            Self::Colour => 6,
            Self::Mono => 4,
        }
    }
}

/// Reads one codebook entry (`read_codebook`), converting YUV to RGB with clamping.
fn read_codebook(r: &mut Reader<'_>, mode: Mode) -> Result<Codebook, Error> {
    match mode {
        Mode::Mono => {
            let y = r.take::<4>()?;
            Ok(Codebook { r: y, g: y, b: y })
        }
        Mode::Colour => {
            let [y0, y1, y2, y3, u, v] = r.take::<6>()?;
            let (u, v) = (i32::from(u as i8), i32::from(v as i8));
            let luma = [y0, y1, y2, y3];
            let (uv_r, uv_g, uv_b) = (v << 1, -((u + 1) >> 1) - v, u << 1);
            let mix = |uv: i32| luma.map(|y| (i32::from(y) + uv).clamp(0, 255) as u8);
            Ok(Codebook {
                r: mix(uv_r),
                g: mix(uv_g),
                b: mix(uv_b),
            })
        }
    }
}

/// Reads a codebook chunk without masks (0x2000, 0x2200, 0x2400, 0x2600).
/// Bytes past the last whole entry are left unread, as in Wine.
fn read_plain_codebooks(
    r: &mut Reader<'_>,
    book: &mut [Codebook],
    chunk_size: i64,
    mode: Mode,
) -> Result<(), Error> {
    let count = (chunk_size.max(0) / mode.size()) as usize;
    if count > CODEBOOK_ENTRIES {
        return Err(Error::CodebookOverflow);
    }
    for entry in &mut book[..count] {
        *entry = read_codebook(r, mode)?;
    }
    Ok(())
}

/// Reads a codebook chunk with a 32-bit mask before each group of 32 entries
/// (0x2100, 0x2300, 0x2500, 0x2700). A set bit means the entry is present.
fn read_masked_codebooks(
    r: &mut Reader<'_>,
    book: &mut [Codebook],
    mut chunk_size: i64,
    mode: Mode,
) -> Result<(), Error> {
    let mut ci = 0;
    while chunk_size > 0 {
        let mut flag = r.long()?;
        chunk_size -= 4;
        for _ in 0..32 {
            if flag & 0x8000_0000 != 0 {
                chunk_size -= mode.size();
                let entry = book.get_mut(ci).ok_or(Error::CodebookOverflow)?;
                *entry = read_codebook(r, mode)?;
            }
            ci += 1;
            flag <<= 1;
        }
    }
    r.skip(chunk_size);
    Ok(())
}

/// Address of the block at `(x, y)` (`get_addr`). Bottom-up frames count rows
/// from the bottom, so `y` past the height gives a negative address.
fn get_addr(inverted: bool, x: usize, y: usize, stride: usize, bpp: usize, height: usize) -> i64 {
    let row = if inverted {
        y as i64
    } else {
        height as i64 - 1 - y as i64
    };
    row * stride as i64 + (x * bpp) as i64
}

/// Writes one 4x4 block. Each block row is checked as it starts: a row that begins
/// before the buffer stops the block, keeping the rows already written, as in C.
fn write_block(
    pixels: &mut [u8],
    addr: i64,
    stride: usize,
    inverted: bool,
    depth: Depth,
    colour: impl Fn(usize, usize) -> (u8, u8, u8),
) -> Result<(), Error> {
    let bpp = depth.bytes_per_pixel();
    let step = depth.row_step(stride) as i64;
    let row_inc = if inverted { step } else { -step };
    for y in 0..4 {
        let row = addr + y as i64 * row_inc;
        if row < 0 {
            return Ok(());
        }
        let row = row as usize;
        for x in 0..4 {
            let (r, g, b) = colour(x, y);
            let at = row + x * bpp;
            pixels
                .get_mut(at..at + bpp)
                .ok_or(Error::OutOfBounds)?
                .copy_from_slice(&depth.pack(r, g, b)[..bpp]);
        }
    }
    Ok(())
}

/// Fills a block from one codebook: each 2x2 quadrant takes one of its entries (`cvid_v1_*`).
fn write_v1(
    pixels: &mut [u8],
    addr: i64,
    stride: usize,
    inverted: bool,
    depth: Depth,
    cb: &Codebook,
) -> Result<(), Error> {
    write_block(pixels, addr, stride, inverted, depth, |x, y| {
        let q = x / 2 + (y / 2) * 2;
        (cb.r[q], cb.g[q], cb.b[q])
    })
}

/// Fills a block from four codebooks, one per 2x2 quadrant (`cvid_v4_*`).
fn write_v4(
    pixels: &mut [u8],
    addr: i64,
    stride: usize,
    inverted: bool,
    depth: Depth,
    cbs: [&Codebook; 4],
) -> Result<(), Error> {
    write_block(pixels, addr, stride, inverted, depth, |x, y| {
        let cb = cbs[x / 2 + (y / 2) * 2];
        let q = x % 2 + (y % 2) * 2;
        (cb.r[q], cb.g[q], cb.b[q])
    })
}

/// The four codebook entries a V4 block names, in quadrant order.
fn quad(book: &[Codebook], d: [u8; 4]) -> [&Codebook; 4] {
    d.map(|i| &book[usize::from(i)])
}

/// Big-endian reads from a frame. Reads past the end fail with [`Error::Truncated`].
struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    fn take<const N: usize>(&mut self) -> Result<[u8; N], Error> {
        let bytes = self
            .data
            .get(self.pos..self.pos + N)
            .ok_or(Error::Truncated)?;
        self.pos += N;
        let mut out = [0; N];
        out.copy_from_slice(bytes);
        Ok(out)
    }

    fn byte(&mut self) -> Result<u8, Error> {
        Ok(self.take::<1>()?[0])
    }

    fn word(&mut self) -> Result<u16, Error> {
        Ok(u16::from_be_bytes(self.take()?))
    }

    fn long(&mut self) -> Result<u32, Error> {
        Ok(u32::from_be_bytes(self.take()?))
    }

    /// Skips `n` bytes, stopping at the end. Wine skips chunk leftovers without reading them.
    fn skip(&mut self, n: i64) {
        self.pos = (self.pos + n.max(0) as usize).min(self.data.len());
    }
}
