//! msrle32 RLE codec: the headless logic of Wine's `dlls/msrle32`.
//!
//! The reference C works on Windows DIB headers, ICM handles and driver
//! state. Here every routine takes plain slices and returns owned values,
//! and mirrors the C control flow step for step so the output matches the
//! reference byte for byte.
//!
//! Ported (names in `dlls/msrle32/msrle32.c`):
//! - `ColorCmp`: [`color_cmp`], squared 16-bit colour difference (wraps as in C).
//! - `Intensity`: [`intensity`], luminance used to build the internal frame.
//! - `GetRawPixel`: source pixel index read, private. The 1-bpp form keeps
//!   Wine's off-by-one (pixel `x` reads bit `8 - x % 8`), so pixel 0 reads 0.
//! - `MSRLE32_GetNearestPaletteIndex`: [`nearest_palette_index`], first minimum wins.
//! - `computeInternalFrame`: [`compute_internal_frame`]. A 1-bpp source fills
//!   only 7 of every 8 pixels; the 8th stays 0, as in C.
//! - `countDiffRLE4`: private. Its previous-frame branch is a no-op in C
//!   (`count2` is always 0), so only the current row is read.
//! - `countDiffRLE8`: private `count_diff_rle8`, with the previous frame.
//! - `MSRLE32_CompressRLE4Line`: private `rle4_line`.
//! - `MSRLE32_CompressRLE8Line`: private `rle8_line`.
//! - `MSRLE32_CompressRLE4`: [`compress_rle4`]. The returned length is the
//!   `biSizeImage` the C code accumulates.
//! - `MSRLE32_CompressRLE8`: [`compress_rle8`].
//! - `MSRLE32_DecompressRLE4`: [`decompress_rle4`]. Returns `Err` where C
//!   would overrun its buffers.
//! - `MSRLE32_DecompressRLE8`: [`decompress_rle8`].
//! - Palette map built in `CompressBegin`: [`compress_palette_map`].
//! - Palette map built in `DecompressBegin`: [`decompress_palette_map`].
//!
//! Skipped:
//! - `MSRLE32_DriverProc` (the only spec export, `DriverProc`): ICM message
//!   dispatcher. It needs driver handles and ICM structs.
//! - `DllMain`: module lifecycle.
//! - `Open`, `Close`, `GetInfo`, `Configure`, `About`: driver instance,
//!   registry and dialog plumbing.
//! - `isSupportedMRLE`, `isSupportedDIB`: header negotiation for ICM queries.
//! - `MSRLE32_GetMaxCompressedSize`: buffer sizing for ICM `CompressGetSize`.
//! - `CompressQuery`, `CompressGetFormat`, `CompressGetSize`, `CompressBegin`,
//!   `CompressEnd`, `Compress`: ICM compress handlers. They own the
//!   `CodecInfo` allocation and drive the ICM structs.
//! - `DecompressQuery`, `DecompressGetFormat`, `DecompressBegin`,
//!   `DecompressEnd`, `Decompress`, `DecompressGetPalette`: the ICM
//!   decompress handlers, same reasons.
//!
//! Divergences from C, all deliberate:
//! - Row strides are computed without Wine's 16-bit `WORD` truncation, so
//!   widths above 2730 pixels at 24 bpp differ from Wine.
//! - Reads past the end of an input, and writes past the end of the output,
//!   return an [`Error`] instead of undefined behaviour.

/// A `RGBQUAD` palette entry. Fields follow the DIB byte order: blue, green,
/// red, reserved.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RgbQuad {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub reserved: u8,
}

impl RgbQuad {
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            blue,
            green,
            red,
            reserved: 0,
        }
    }
}

/// Squared difference of two 16-bit colour values. The C code computes the
/// square in `UINT` and returns it as `WORD`, so the result wraps modulo
/// 2^16. Two values therefore compare equal whenever their difference is a
/// multiple of 256.
pub fn color_cmp(a: u16, b: u16) -> u16 {
    let d = u32::from(a).wrapping_sub(u32::from(b));
    d.wrapping_mul(d) as u16
}

/// Equality as the encoder sees it: `ColorCmp(a, b) <= 0`, since the encoder
/// always passes a distance of 0.
fn same(a: u16, b: u16) -> bool {
    color_cmp(a, b) == 0
}

/// Luminance `(30 R + 59 G + 11 B) / 4`.
pub fn intensity(clr: RgbQuad) -> u16 {
    ((30 * u32::from(clr.red) + 59 * u32::from(clr.green) + 11 * u32::from(clr.blue)) / 4) as u16
}

fn squared(a: u8, b: u8) -> i32 {
    let d = i32::from(a) - i32::from(b);
    d * d
}

/// Index of the palette entry closest to `clr` by squared RGB distance. The
/// first of several equally close entries wins; an empty palette gives 0.
pub fn nearest_palette_index(palette: &[RgbQuad], clr: RgbQuad) -> u8 {
    palette
        .iter()
        .enumerate()
        .min_by_key(|(_, c)| {
            squared(c.red, clr.red) + squared(c.green, clr.green) + squared(c.blue, clr.blue)
        })
        .map_or(0, |(i, _)| i as u8)
}

/// Maps each input palette index to its nearest entry in `output`. Only the
/// first 256 input entries are used, the largest palette a DIB can hold.
pub fn compress_palette_map(input: &[RgbQuad], output: &[RgbQuad]) -> [u8; 256] {
    let mut map = [0u8; 256];
    for (slot, &clr) in map.iter_mut().zip(input) {
        *slot = nearest_palette_index(output, clr);
    }
    map
}

/// The palette map used while decoding, indexed by stream value. Each entry
/// is four bytes wide so one layout serves every output depth.
pub type DecodeMap = [u8; 1024];

/// Builds the decode map for `out_bit_count`, as `DecompressBegin` does.
///
/// For 4 and 8 bpp each input entry maps to its nearest `output` entry,
/// searching only the first `min(input, output)` entries. For 15 and 16 bpp
/// each entry is a little-endian 5-5-5 or 5-6-5 word. For 24 and 32 bpp each
/// entry is the raw `RGBQUAD`. Other depths leave the map zeroed.
pub fn decompress_palette_map(
    out_bit_count: u16,
    input: &[RgbQuad],
    output: &[RgbQuad],
) -> DecodeMap {
    let input = &input[..input.len().min(256)];
    let mut map = [0u8; 1024];
    match out_bit_count {
        4 | 8 => {
            let palette = &output[..output.len().min(input.len())];
            for (slot, &clr) in map.iter_mut().zip(input) {
                *slot = nearest_palette_index(palette, clr);
            }
        }
        15 | 16 => {
            let red_shift = if out_bit_count == 15 { 10 } else { 11 };
            for (i, clr) in input.iter().enumerate() {
                let color = ((u16::from(clr.red >> 3)) << red_shift)
                    | ((u16::from(clr.green >> 3)) << 5)
                    | u16::from(clr.blue >> 3);
                map[2 * i] = color as u8;
                map[2 * i + 1] = (color >> 8) as u8;
            }
        }
        24 | 32 => {
            for (i, clr) in input.iter().enumerate() {
                map[4 * i..4 * i + 4].copy_from_slice(&[
                    clr.blue,
                    clr.green,
                    clr.red,
                    clr.reserved,
                ]);
            }
        }
        _ => {}
    }
    map
}

/// Row stride in bytes of a DIB: `DIBWIDTHBYTES`, a multiple of 4.
fn dib_stride(width: usize, bit_count: u16) -> usize {
    (width * usize::from(bit_count)).div_ceil(32) * 4
}

/// Reads pixel `x` of one source row. 4-bpp and 1-bpp rows are unpacked; any
/// other depth is read one byte per pixel. The 1-bpp shift matches C's
/// `GetRawPixel`, which is off by one.
fn raw_pixel(row: &[u8], x: usize, bit_count: u16) -> u8 {
    match bit_count {
        1 => ((u32::from(row[x / 8]) >> (8 - x % 8)) & 1) as u8,
        4 => ((u32::from(row[x / 2]) >> (4 * (1 - x % 2))) & 15) as u8,
        _ => row[x],
    }
}

/// A source DIB. `pixels` holds `height` rows of `dib_stride(width, bit_count)`
/// bytes, top row first. It must be at least that long, or the accessors panic.
pub struct Source<'a> {
    pub width: usize,
    pub height: usize,
    pub bit_count: u16,
    pub pixels: &'a [u8],
}

impl Source<'_> {
    fn row(&self, y: usize) -> &[u8] {
        let stride = dib_stride(self.width, self.bit_count);
        &self.pixels[y * stride..(y + 1) * stride]
    }
}

/// The internal frame: `width * height` intensities, row-major and compact.
pub struct Frame {
    width: usize,
    pixels: Vec<u16>,
}

impl Frame {
    pub fn pixels(&self) -> &[u16] {
        &self.pixels
    }

    fn row(&self, y: usize) -> &[u16] {
        &self.pixels[y * self.width..(y + 1) * self.width]
    }
}

/// Converts a source DIB to intensities using `palette` (`computeInternalFrame`).
pub fn compute_internal_frame(src: &Source, palette: &[RgbQuad]) -> Frame {
    let mut table = [0u16; 256];
    for (slot, &clr) in table.iter_mut().zip(palette) {
        *slot = intensity(clr);
    }
    let (w, h) = (src.width, src.height);
    let mut pixels = vec![0u16; w * h];
    for y in 0..h {
        let row = src.row(y);
        let out = &mut pixels[y * w..(y + 1) * w];
        match src.bit_count {
            1 => {
                for (group, &byte) in out.as_chunks_mut::<8>().0.iter_mut().zip(row) {
                    for (i, px) in group.iter_mut().take(7).enumerate() {
                        *px = table[usize::from((byte >> (7 - i)) & 1)];
                    }
                }
            }
            4 => {
                for (pair, &byte) in out.as_chunks_mut::<2>().0.iter_mut().zip(row) {
                    pair[0] = table[usize::from(byte >> 4)];
                    pair[1] = table[usize::from(byte & 15)];
                }
            }
            8 => {
                for (px, &index) in out.iter_mut().zip(row) {
                    *px = table[usize::from(index)];
                }
            }
            _ => {}
        }
    }
    Frame { width: w, pixels }
}

/// Number of pixels from `start` that the previous-frame match can encode,
/// as a signed count. Negative results are the C look-ahead adjustments.
fn count_diff_rle4(cur: &[u16], start: usize) -> isize {
    let w = cur.len();
    if start + 1 == w {
        return 1;
    }
    let mut clr1 = cur[start];
    let mut clr2 = cur[start + 1];
    let mut pos = start + 1;
    let mut count: isize = 2;
    while pos + 1 < w {
        pos += 1;
        let clr3 = cur[pos];
        if pos + 1 >= w {
            return count + 1;
        }
        pos += 1;
        let clr4 = cur[pos];
        if same(clr1, clr3)
            && same(clr2, clr4)
            && pos + 2 < w
            && same(clr1, cur[pos + 1])
            && same(clr2, cur[pos + 2])
        {
            if pos + 4 < w && same(cur[pos + 1], cur[pos + 3]) && same(cur[pos + 2], cur[pos + 4]) {
                return count - 3;
            }
            return count - 2;
        }
        count += 2;
        clr1 = clr3;
        clr2 = clr4;
    }
    count
}

/// The pixel left of `pos` in the current row, which C reads as `lpA[pos]`.
/// For `pos == 0` C reads outside the row. That only happens for a 1-pixel
/// row, where the result does not depend on this value, so 0 is used.
fn prev_pixel(cur: &[u16], pos: usize) -> u16 {
    if pos == 0 {
        0
    } else {
        cur[pos - 1]
    }
}

/// The run-length counterpart of [`count_diff_rle4`]: counts pixels from
/// `start` that are worth encoding as absolute data.
fn count_diff_rle8(prev: Option<&[u16]>, cur: &[u16], start: usize) -> isize {
    let w = cur.len();
    let mut count: isize = 0;
    let mut pos = start;
    while pos < w {
        if same(prev_pixel(cur, pos), cur[pos]) {
            if pos + 1 < w && same(cur[pos], cur[pos + 1]) {
                return count - 1;
            }
            if pos + 2 < w && same(cur[pos + 1], cur[pos + 2]) {
                return count - 1;
            }
        } else if let Some(p) = prev.filter(|p| same(p[pos], cur[pos])) {
            let mut count2 = 0;
            pos += 1;
            while pos < w && count2 <= 5 {
                if !same(p[pos], cur[pos]) {
                    break;
                }
                pos += 1;
                count2 += 1;
            }
            if count2 > 4 {
                return count;
            }
            pos -= count2;
        }
        pos += 1;
        count += 1;
    }
    count
}

/// Writes two 4-bit indices per byte, the last byte padded with 0 if the
/// slice has odd length.
fn pack_nibbles(px: &[u8], out: &mut Vec<u8>) {
    for pair in px.chunks(2) {
        out.push((pair[0] << 4) | pair.get(1).copied().unwrap_or(0));
    }
}

/// Encodes one RLE4 segment of row `cur` starting at `x0`, emitting into
/// `out` and returning the next unprocessed x. `mapped` holds the row's
/// palette indices (`CompressRLE4Line`).
fn rle4_line(mapped: &[u8], cur: &[u16], x0: usize, out: &mut Vec<u8>) -> usize {
    let w = cur.len();
    let mut x = x0;
    let clr1 = cur[x0];
    let mut pos = x0 + 1;
    let mut count: usize = 1;
    if pos < w {
        let clr2 = cur[pos];
        count = 2;
        while pos + 1 < w {
            pos += 1;
            if !same(clr1, cur[pos]) {
                break;
            }
            count += 1;
            if pos + 1 >= w {
                break;
            }
            pos += 1;
            if !same(clr2, cur[pos]) {
                break;
            }
            count += 1;
        }
    }

    if count < 4 {
        count = (count as isize + count_diff_rle4(cur, pos - 1)) as usize;
        count = count.min(w - x);
        while count > 2 {
            let size = count.min(254);
            let bytes = size.div_ceil(2);
            count -= size;
            out.push(0);
            out.push(size as u8);
            pack_nibbles(&mapped[x..x + size], out);
            x += size;
            if bytes % 2 == 1 {
                out.push(0);
            }
        }
        if count > 0 {
            out.push(count as u8);
            pack_nibbles(&mapped[x..x + count], out);
            x += count;
        }
    } else {
        let clr = (mapped[x] << 4) | mapped[x + 1];
        x += count;
        while count > 0 {
            let size = count.min(254);
            count -= size;
            out.push(size as u8);
            out.push(clr);
        }
    }
    x
}

/// Encodes one RLE8 segment of row `cur` starting at `x0` and returns the
/// next unprocessed x (`CompressRLE8Line`). `prev` is the previous frame's
/// row when encoding a delta.
fn rle8_line(
    mapped: &[u8],
    cur: &[u16],
    prev: Option<&[u16]>,
    x0: usize,
    out: &mut Vec<u8>,
) -> usize {
    let w = cur.len();
    let mut x = x0;
    let clr = cur[x0];
    let mut pos = x0 + 1;
    let mut count: usize = 1;
    while pos < w {
        let equal = same(clr, cur[pos]);
        pos += 1;
        if !equal {
            break;
        }
        count += 1;
    }

    if count < 2 {
        count = (count as isize + count_diff_rle8(prev, cur, pos - 1)) as usize;
        count = count.min(w - x);
        while count > 2 {
            let size = count.min(255);
            count -= size;
            out.push(0);
            out.push(size as u8);
            out.extend_from_slice(&mapped[x..x + size]);
            x += size;
            if size % 2 == 1 {
                out.push(0);
            }
        }
        for _ in 0..count {
            out.push(1);
            out.push(mapped[x]);
            x += 1;
        }
    } else {
        let clr = mapped[x];
        if x + count + 1 == w {
            count += 1;
        }
        x += count;
        while count > 0 {
            let size = count.min(255);
            count -= size;
            out.push(size as u8);
            out.push(clr);
        }
    }
    x
}

/// Palette indices for every pixel, row-major, after [`raw_pixel`] and `map`.
fn map_pixels(src: &Source, map: &[u8; 256]) -> Vec<u8> {
    let mut mapped = Vec::with_capacity(src.width * src.height);
    for y in 0..src.height {
        let row = src.row(y);
        for x in 0..src.width {
            mapped.push(map[usize::from(raw_pixel(row, x, src.bit_count))]);
        }
    }
    mapped
}

/// RLE4 encoding of `cur` (`MSRLE32_CompressRLE4`). A keyframe has `prev`
/// `None`; a delta encodes only what changed from `prev`. `palette_map` comes
/// from [`compress_palette_map`].
pub fn compress_rle4(
    src: &Source,
    palette_map: &[u8; 256],
    cur: &Frame,
    prev: Option<&Frame>,
) -> Vec<u8> {
    let (w, h) = (src.width, src.height);
    let mapped = map_pixels(src, palette_map);
    let mut out = Vec::new();
    match prev {
        None => {
            for y in 0..h {
                let (m, c) = (&mapped[y * w..(y + 1) * w], cur.row(y));
                let mut x = 0;
                while x < w {
                    x = rle4_line(m, c, x, &mut out);
                }
                out.extend_from_slice(&[0, 0]);
            }
        }
        Some(prev) => {
            let mut jumpx: isize = -1;
            let mut jumpy: isize = 0;
            for y in 0..h {
                let (m, c, p) = (&mapped[y * w..(y + 1) * w], cur.row(y), prev.row(y));
                let mut x = 0;
                loop {
                    if jumpx == -1 {
                        jumpx = x as isize;
                    }
                    let mut pos = x;
                    while pos < w && same(p[pos], c[pos]) {
                        pos += 1;
                    }
                    if pos == w && pos - x > 8 {
                        jumpy += 1;
                        break;
                    }
                    if jumpy != 0 || jumpx != pos as isize {
                        if (pos as isize) < jumpx {
                            let mut rest = w - jumpx as usize;
                            jumpx = 0;
                            jumpy -= 1;
                            while rest > 0 {
                                let n = rest.min(255);
                                out.extend_from_slice(&[0, 2, n as u8, 0]);
                                rest -= n;
                            }
                            out.extend_from_slice(&[0, 0]);
                        }
                        while jumpy != 0 || pos as isize != jumpx {
                            let dx = (pos as isize - jumpx).min(255);
                            let dy = jumpy.min(255);
                            out.extend_from_slice(&[0, 2, dx as u8, dy as u8]);
                            x += dx as usize;
                            jumpx += dx;
                            jumpy -= dy;
                        }
                        jumpy = 0;
                    }
                    jumpx = -1;
                    if x < w {
                        x = rle4_line(m, c, x, &mut out);
                    }
                    if x >= w {
                        break;
                    }
                }
                if jumpy == 0 {
                    out.extend_from_slice(&[0, 0]);
                }
            }
            out.extend_from_slice(&[0, 0]);
        }
    }
    if let Some(last) = out.last_mut() {
        *last = 1;
    }
    out
}

/// RLE8 encoding of `cur` (`MSRLE32_CompressRLE8`). A keyframe has `prev`
/// `None`; a delta encodes only what changed from `prev`. `palette_map` comes
/// from [`compress_palette_map`].
pub fn compress_rle8(
    src: &Source,
    palette_map: &[u8; 256],
    cur: &Frame,
    prev: Option<&Frame>,
) -> Vec<u8> {
    let (w, h) = (src.width, src.height);
    let mapped = map_pixels(src, palette_map);
    let mut out = Vec::new();
    match prev {
        None => {
            for y in 0..h {
                let (m, c) = (&mapped[y * w..(y + 1) * w], cur.row(y));
                let mut x = 0;
                while x < w {
                    x = rle8_line(m, c, None, x, &mut out);
                }
                out.extend_from_slice(&[0, 0]);
            }
        }
        Some(prev) => {
            let mut jumpx: isize = -1;
            let mut jumpy: isize = 0;
            for y in 0..h {
                let (m, c, p) = (&mapped[y * w..(y + 1) * w], cur.row(y), prev.row(y));
                let mut x = 0;
                loop {
                    if jumpx == -1 {
                        jumpx = x as isize;
                    }
                    let mut pos = x;
                    while pos < w && same(p[pos], c[pos]) {
                        pos += 1;
                    }
                    if pos == w && pos - x > 4 {
                        jumpy += 1;
                        break;
                    }
                    if jumpy != 0 || jumpx != pos as isize {
                        if (pos as isize) < jumpx {
                            jumpx = 0;
                            jumpy -= 1;
                            out.extend_from_slice(&[0, 0]);
                        }
                        while jumpy != 0 || pos as isize != jumpx {
                            let dx = (pos as isize - jumpx).min(255);
                            let dy = jumpy.min(255);
                            out.extend_from_slice(&[0, 2, dx as u8, dy as u8]);
                            jumpx += dx;
                            jumpy -= dy;
                        }
                        x = pos;
                        jumpy = 0;
                    }
                    jumpx = -1;
                    if x < w {
                        x = rle8_line(m, c, Some(p), x, &mut out);
                    }
                    if x >= w {
                        break;
                    }
                }
                if jumpy == 0 {
                    out.extend_from_slice(&[0, 0]);
                }
            }
            out.extend_from_slice(&[0, 0]);
        }
    }
    out.extend_from_slice(&[0, 1]);
    out
}

/// Pixel-writing state of the decoder: a flat output buffer and a cursor.
struct Canvas<'m> {
    pixels: Vec<u8>,
    map: &'m DecodeMap,
    stride: usize,
    bpp: usize,
    width: usize,
    row: usize,
    px: usize,
}

impl Canvas<'_> {
    /// Writes the pixel for stream value `index` and advances the cursor.
    fn put(&mut self, index: u8) -> Result<(), Error> {
        let entry = usize::from(index) * self.bpp.next_power_of_two();
        let base = self.row * self.stride + self.px * self.bpp;
        for i in 0..self.bpp.min(3) {
            let slot = self.pixels.get_mut(base + i).ok_or(Error::OutputOverflow)?;
            *slot = self.map[entry + i];
        }
        self.px += 1;
        Ok(())
    }

    /// Fails unless `n` more pixels fit on the current row.
    fn check_run(&self, n: usize) -> Result<(), Error> {
        if self.px + n > self.width {
            Err(Error::RunTooLong)
        } else {
            Ok(())
        }
    }

    fn end_line(&mut self) {
        self.px = 0;
        self.row += 1;
    }

    /// Moves right by `dx` and down by `dy`. Passing the right edge wraps to
    /// the next row, as the C decoder does.
    fn skip(&mut self, dx: usize, dy: usize) {
        self.px += dx;
        self.row += dy;
        if self.px >= self.width {
            self.px = 0;
            self.row += 1;
        }
    }
}

/// Cursor over the compressed stream.
struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8], Error> {
        if n > self.data.len() - self.pos {
            return Err(Error::Truncated);
        }
        let bytes = &self.data[self.pos..self.pos + n];
        self.pos += n;
        Ok(bytes)
    }

    fn byte(&mut self) -> Result<u8, Error> {
        Ok(self.take(1)?[0])
    }
}

/// The high nibble of `byte` when `k` is even, the low nibble when odd.
fn nibble(byte: u8, k: usize) -> u8 {
    if k.is_multiple_of(2) {
        byte >> 4
    } else {
        byte & 15
    }
}

/// Decoding failures. The C decoder has no error results; these replace its
/// undefined behaviour.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// The stream ended inside a command.
    Truncated,
    /// A run or absolute block extends past the end of its row. The C
    /// decoder returns `ICERR_ERROR` here.
    RunTooLong,
    /// A write lands past the end of the output buffer.
    OutputOverflow,
    /// The output bit count has no decoder, so no decode map exists for it.
    UnsupportedBitCount(u16),
}

/// RLE4 decoding (`MSRLE32_DecompressRLE4`) into a zeroed `width x height`
/// DIB of `out_bit_count` bits. `map` comes from [`decompress_palette_map`].
pub fn decompress_rle4(
    map: &DecodeMap,
    out_bit_count: u16,
    width: usize,
    height: usize,
    stream: &[u8],
) -> Result<Vec<u8>, Error> {
    decode(map, out_bit_count, width, height, stream, true)
}

/// RLE8 decoding (`MSRLE32_DecompressRLE8`) into a zeroed `width x height`
/// DIB of `out_bit_count` bits. `map` comes from [`decompress_palette_map`].
pub fn decompress_rle8(
    map: &DecodeMap,
    out_bit_count: u16,
    width: usize,
    height: usize,
    stream: &[u8],
) -> Result<Vec<u8>, Error> {
    decode(map, out_bit_count, width, height, stream, false)
}

fn decode(
    map: &DecodeMap,
    out_bit_count: u16,
    width: usize,
    height: usize,
    stream: &[u8],
    four_bit: bool,
) -> Result<Vec<u8>, Error> {
    let bpp = match out_bit_count {
        8 => 1,
        15 | 16 => 2,
        24 => 3,
        32 => 4,
        _ => return Err(Error::UnsupportedBitCount(out_bit_count)),
    };
    let stride = dib_stride(width, out_bit_count);
    let mut canvas = Canvas {
        pixels: vec![0; stride * height],
        map,
        stride,
        bpp,
        width,
        row: 0,
        px: 0,
    };
    let mut reader = Reader {
        data: stream,
        pos: 0,
    };
    loop {
        let code0 = reader.byte()?;
        let code1 = reader.byte()?;
        if code0 != 0 {
            let n = usize::from(code0);
            canvas.check_run(n)?;
            for k in 0..n {
                if four_bit {
                    canvas.put(nibble(code1, k))?;
                } else {
                    canvas.put(code1)?;
                }
            }
            continue;
        }
        match code1 {
            0 => canvas.end_line(),
            1 => break,
            2 => {
                let dx = reader.byte()?;
                let dy = reader.byte()?;
                canvas.skip(usize::from(dx), usize::from(dy));
            }
            _ => {
                let n = usize::from(code1);
                canvas.check_run(n)?;
                if four_bit {
                    let packed = reader.take(n.div_ceil(2))?;
                    for k in 0..n {
                        canvas.put(nibble(packed[k / 2], k))?;
                    }
                    if packed.len() % 2 == 1 {
                        reader.take(1)?;
                    }
                } else {
                    for &index in reader.take(n)? {
                        canvas.put(index)?;
                    }
                    if n % 2 == 1 {
                        reader.take(1)?;
                    }
                }
            }
        }
    }
    Ok(canvas.pixels)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unhex(s: &str) -> Vec<u8> {
        if s == "-" {
            return Vec::new();
        }
        (0..s.len() / 2)
            .map(|i| u8::from_str_radix(&s[2 * i..2 * i + 2], 16).unwrap())
            .collect()
    }

    fn hex(bytes: &[u8]) -> String {
        if bytes.is_empty() {
            "-".to_string()
        } else {
            bytes.iter().map(|byte| format!("{byte:02x}")).collect()
        }
    }

    fn palette(s: &str) -> Vec<RgbQuad> {
        let bytes = unhex(s);
        bytes
            .as_chunks::<4>()
            .0
            .iter()
            .map(|c| RgbQuad {
                blue: c[0],
                green: c[1],
                red: c[2],
                reserved: c[3],
            })
            .collect()
    }

    fn number<T: std::str::FromStr>(s: &str) -> T
    where
        T::Err: std::fmt::Debug,
    {
        s.parse().unwrap()
    }

    #[allow(clippy::too_many_arguments)]
    fn encode(
        out_bc: u16,
        bc: u16,
        w: usize,
        h: usize,
        pin: &[RgbQuad],
        pout: &[RgbQuad],
        pixels: &[u8],
        previous: Option<&[u8]>,
    ) -> String {
        let src = Source {
            width: w,
            height: h,
            bit_count: bc,
            pixels,
        };
        let map = compress_palette_map(pin, pout);
        let current = compute_internal_frame(&src, pin);
        let previous_frame = previous.map(|bytes| {
            let prev_src = Source {
                width: w,
                height: h,
                bit_count: bc,
                pixels: bytes,
            };
            compute_internal_frame(&prev_src, pin)
        });
        let output = if out_bc == 4 {
            compress_rle4(&src, &map, &current, previous_frame.as_ref())
        } else {
            compress_rle8(&src, &map, &current, previous_frame.as_ref())
        };
        format!("{} {}", output.len(), hex(&output))
    }

    fn run(command: &str) -> String {
        let t: Vec<&str> = command.split_whitespace().collect();
        match t[0] {
            "cmp" => color_cmp(number(t[1]), number(t[2])).to_string(),
            "intensity" => {
                intensity(RgbQuad::new(number(t[1]), number(t[2]), number(t[3]))).to_string()
            }
            "nearest" => nearest_palette_index(&palette(t[1]), palette(t[2])[0]).to_string(),
            "cpal" => {
                let bc_out: u16 = number(t[2]);
                let pin = palette(t[3]);
                let pout = palette(t[4]);
                if bc_out == 4 || bc_out == 8 {
                    hex(&compress_palette_map(&pin, &pout)[..pin.len()])
                } else {
                    "-".to_string()
                }
            }
            "dpal" => {
                let bc_out: u16 = number(t[2]);
                let pin = palette(t[3]);
                let pout = palette(t[4]);
                let mult = match bc_out {
                    4 | 8 => 1,
                    15 | 16 => 2,
                    24 | 32 => 4,
                    _ => 0,
                };
                if mult == 0 {
                    "-".to_string()
                } else {
                    hex(&decompress_palette_map(bc_out, &pin, &pout)[..pin.len() * mult])
                }
            }
            "iframe" => {
                let bc: u16 = number(t[1]);
                let (w, h): (usize, usize) = (number(t[2]), number(t[3]));
                let colors = palette(t[4]);
                let pixels = unhex(t[5]);
                let src = Source {
                    width: w,
                    height: h,
                    bit_count: bc,
                    pixels: &pixels,
                };
                compute_internal_frame(&src, &colors)
                    .pixels()
                    .iter()
                    .map(u16::to_string)
                    .collect::<Vec<_>>()
                    .join(" ")
            }
            "enc" => {
                let out_bc: u16 = number(t[1]);
                let bc: u16 = number(t[2]);
                let (w, h): (usize, usize) = (number(t[3]), number(t[4]));
                let pin = palette(t[5]);
                let pout = palette(t[6]);
                let current = unhex(t[7]);
                let previous = if t[8] == "-" { None } else { Some(unhex(t[8])) };
                encode(out_bc, bc, w, h, &pin, &pout, &current, previous.as_deref())
            }
            "dec" => {
                let in_bc: u16 = number(t[1]);
                let out_bc: u16 = number(t[2]);
                let (w, h): (usize, usize) = (number(t[3]), number(t[4]));
                let pin = palette(t[5]);
                let pout = palette(t[6]);
                let stream = unhex(t[7]);
                let map = decompress_palette_map(out_bc, &pin, &pout);
                let result = if in_bc == 4 {
                    decompress_rle4(&map, out_bc, w, h, &stream)
                } else {
                    decompress_rle8(&map, out_bc, w, h, &stream)
                };
                match result {
                    Ok(bytes) => format!("0 {}", hex(&bytes)),
                    Err(error) => format!("err {error:?}"),
                }
            }
            other => panic!("unknown oracle command {other}"),
        }
    }

    // BEGIN ORACLE VECTORS
    const ORACLE_VECTORS: &[(&str, &str)] = &[
    ("cmp 1681 23", "61988"),
    ("cmp 0 256", "0"),
    ("intensity 18 89 7", "1467"),
    ("intensity 255 255 255", "6375"),
    ("nearest 000000000000ff00ffffff00 0a0ac800", "1"),
    ("cpal 8 4 000000000000ff00ffffff00 ff00000000ff0000", "000000"),
    ("cpal 8 8 000000000000ff00ffffff00 ff00000000ff000000ff00ffffff00", "000000"),
    ("dpal 8 8 000000000000ff00ffffff00 ff00000000ff000000ff00ffffff00", "000000"),
    ("dpal 8 15 000000000000ff00ffffff00 ff00000000ff000000ff00ffffff00", "0000007cff7f"),
    ("dpal 8 32 000000000000ff00ffffff00 ff00000000ff0000", "000000000000ff00ffffff00"),
    ("dpal 8 1 000000000000ff00ffffff00 ff00000000ff0000", "-"),
    ("iframe 1 9 2 61e7ee007894b100e4305f0077201e00e7ca1500200750003ada3e00a7ed0f00967764008eea2b00a1822a00230f930037cd37000822c5006b6d0000c0f01a0025d6cb00ac8a6500145e7300cf628d007e443c00ee1e0500e5605a00c4d643000ced90009b8a00005f6b0a004a15330097bd55008a6dd500 a897c525262e6a7c", "3840 5459 3840 5459 3840 5459 5459 0 0 5459 5459 3840 5459 5459 3840 3840 0 0"),
    ("iframe 4 5 2 0562b50005c34900eb8cf7004a007400ad53bc0026661e002ccfac00d46a1500d82e7200 7668800088008000", "2304 4464 4464 2127 0 2127 2127 2816 2816 0"),
    ("iframe 8 6 2 669af0004b765f0024425f000f966d004b8d0700b27256008347df00dbf977003ca0dc001030cf0045e58600965ae300ae43b40086ebbe00fb42ca00bb6378009fcf950001c7ed00 0d07070f0f0f000008080d0d0d0d0000", "5259 5167 5167 2874 2874 2874 4175 4175 5259 5259 5259 5259"),
    ("enc 4 8 6 2 4df8cd002b590e00652678005b39e600f33e2400ae66c600ba95d100f8a2c500512f0d00fb01fd0010a9d00039a37100697d39009cb1aa0079c0380060d53a00 d07b1200b10753007d7ec000347616001df7d00094841a003c56520008d51600f54d4a004e992e006c0ef500bc4ebb00f4f8da0022a1b700 08080000000000000408080808080000 -", "16 00063344440000000006833333000001"),
    ("enc 4 4 9 3 ad3209003d5ee4007628b000c13b12000bded0005d4dbe000a18dc0061d79f00f0e82400aba71f008e0889007eade4005e8a8f00d610af00cbd19100bf9d5e00 dadd98009bab99001dcedd00cd7a640081239d00c00f5700b1de710012e361005f0c2c001a336700ec9126003d16eb00f2914500 fb800ba55000000088166666300000008888888880000000 -", "20 0130081a00000009aabbbbbb5000000009aa0001"),
    ("enc 4 8 12 3 9d082a006242b7002db405000dc49c00ff2b36001db7dc0088250e00b96e5800a090bc00f75af2005d53e5003a64a5008427940027f15c0037a98f00e89c9300364c840079fc3d007ff82e0055c43300d51c1100067353002fe31d007e5384008726120097eb640053f62000817a5600ea663700d4afe3007a23d30099d1d90037911a004e992e0041a8c000547fdc003aac2800a20ac200028be600ad32090068cd4800e6580f008dd7290087985a001f732c0009c1fe00a395d200ae8c9a00344510008e82f400876fc40038bd4a00e28e3d0032350e0066ab24004f93c20028d01500240f8000ca5c2e002896b8003d14560045095d00f0964700161897000fa73400bf581e000c31460084279400b9df7100e99f7000c2b64200fb149e006b526b0039b0910020fa6f00d452f1009711a100bccc58005dda080020dc6100c3c801002ed30c00568bb300a8c91500223eed003c9e4a00b1656a0091899f000bad97005a15e90047204e00809e39000d78cb00d1e11b0012f4110003d78b00432465003d16470000854800d717c5004a47090004d4b500a6e92900664ffb009d6a550050493a007bddcb0064db3800882069003a4132006920b70032abc100d9bd4b00ab7e51007697ce004e423100dcf082001be41200e89b5800a63844002f99ac00ce35f50008c0c100d16ae300a8120c00c8ab360098d1490084bd5c00895bcd00054ead00f829990028d31b0011dd2500be86d500ccf48000852f7d0083b94200fb4b5800bdadd100c01ada001f213b007e433d007e6c970070732800aaba3300c9d59c0090cb6900135312004016ca0075e0b900136248007155c900eac46d0083890100de58f600693b66003dd00000885c8a007a6f7300669af000d2cd420062d25a00f2e10f000a246c007b9a82008dedc70096048e007228f7005edd2b0043511500029f1c00e12c470017f9fe00a4035700cd00f300b477d4005ba9100092682c00969fa1007bd77200261c1100d0c0cb00512f70004a9b92001ae52e0030c62600ae1073001fc45a00685484004e992e00f5be50006dd7fe00db621700bfce47000feb4b007360f60026e6e6005afc78002badd6009852a3005f6a4000b9d5c400667c9500b1bfd000b56012007228bf003b821e00f136b500b6654c00c61ff200e10e81004f93c2002bac9e00ae336d00f9a13600dd5c79005b638900d0c9cd007f323c002f3ff20098012d00dd0a5e000be2a600b09fc7006bb38300a2d74c00d13b4300cba07b0027d2c400e90b52008e264e00b99a4c004e29c600605a5f00d478fe00ed61020074292700b5254900883a16006b3b0800f064b40040afa6007705bf00e11a030008527200cd42e40023e09700c90fcc00c13b1200e9c13e007ff82e00745b9a0055e2ad008d2bab0006331f00bafa8600 13b3710024e5c800b6bad800eb2a1700b280c300f0994a00832a0600be08110009b1b600769201001b24aa0079762000 7f7fd1cccccccc8ee8e8e8e81a754c4c4c4c4c9c9c9c9c9cec55a5a5a5a5e74646464646 7f7f7f7fcccccccce8e8e8e817754c4c4c4c4c9c9c9c9c9c7aa5a5a5a5a5e74646464646", "16 00020002000c60222255555500000001"),
    ("enc 8 8 7 2 556f5d00e2baea0006914d00560e3b00ff9be30050822b00bb269c00a16d110008d51600a620510091fa4e00a4b75c004efd2f0065f5a60084f343002b7c730075a0850022ce9c005e80a6007faede00e61d11002de5e000cfc77100d1fec0009b90710020ac4a0007860800c9ccd600c3e6880004a3be00ecf609004076560044e46d00490c120096ec1b00ad3209005d19fa005d359e0089fbad00829cfe00 4fd2c100b9373f00e1435c0045769000928f6d00ec8a79000c314600e9aef30058174f003ada3e0077067300ccbfa40087a87a00391ca300b07b0f00ba70960026aac9009bab9900e5b6e7001179d3009ef4b3004f236b0064b8230048b66d00bd816d009ef4b3009153930008c34800f77b2600f8f87e00ca19b8003931b300e347ff003e8ed200af6774006551910074c45400ef1fc300f0659d0001547d007d25460062a1e700927382004116f300d88eaf00de763f00f4669700c2e759008d3b7600262b04009a67fd000148d30094d25f006a2584004b9add00cf7731003afe5200d58d30009032f000 17171717171717001919191919191900 -", "10 07140000071b00000001"),
    ("enc 8 8 12 3 9c6d0700118215004a9b92002c5d28005ca164003235f000b4d4b600b7703600ce242500 58144100882dcb004a9b9200fcdfe4001de2f7005690fd0043148d0009eac000b3db7000 010101030402020202020202030304080800020802080808050505050105050505050501 010101030302020202020202030308080808080808080808050505050505050505050505", "46 00020400010200000002020000070200000002000200000203000000000204000100000206000100000000000001"),
    ("enc 8 1 9 3 1ddd6900eb96a50098e8ef007cdb57000e67d100424971000be2a600 960f0f0003163f00d8a3a000801c6600fbbed9001e4fdb0070732800d01816007c84cf0035a72e00ead7170031b414008d4429006441d300f0b1df004d16100011f33900db091100e4d4940008cf6600475c200089c20c008d0bd8000ecaa400a63844004a779c00a1cdc500164f2100c1788900d1361600221771007aff6f00f7aefa0017fc0800e37320005cb7ff002105d9001c545400a029a800af69d500e0111500d5859b007a5eb40083a6fc00f2958c001dcf8600975f9d0015514800a30ed6003097ef00 0632001e2444c22ab53cf7ca fd61d5808533fb23bcef7188", "34 05130002010003020000021301130002010100030213020000020200000000000001"),
    ("dec 4 8 10 3 f670a900375676000e69c600306da100975a9c008fdb3f000bded00091cc3000e83a640051dd7400893c1800ce38e000f33d4000 6983fa001df34d00fbbe8e00f83913009733fe00820cfe0079b2f000bc028400175ae400b280c30062691900e85e98006374cd00ef9dec007b9a8200299d7800 0007b69c20a000020300033600046b480003b00000000548033600020100013500000001", "0 04060103080b0a00000000000c060c0604090b040b0b0000090b090b090c060c000c0000"),
    ("dec 4 24 9 3 799b4a00a6331300bdf01d00 f804400067936700123dab00 0003201000020100000301100202000000081222002101000000000911201001000000000001", "0 bdf01d799b4aa63313000000799b4aa63313a63313799b4abdf01d00a63313bdf01dbdf01dbdf01d799b4a799b4abdf01da63313799b4a00a63313a63313bdf01d799b4aa63313799b4a799b4aa63313799b4a00"),
    ("dec 8 32 9 3 f57c4c005ff58d00e3f6a400b625ec009ed39a008fda1a001c3c7d00881d62003fbb2100a38b7c00b299b4005ec7c9003fba10003b821e0078fb6f00cf628d006ef14200f2eac600f89de8006e56a6007c38b900a131ba00fd53a1002c11f7008fec9000f185d0000ac6f60055a6a20052a23d005892a300dedbb50011bcab00f674060084445b00653c280021246500bede05003f423400 63011e0014e9460039746b0030fe1d000f9eff00fe03080041a7a800c6ad3200678ae6008441670070508f00edf646000b037800bc09320036977f00049c79008990c40034451000cd1f89003b2ab100a9c46000111085008d7d4e00c552420060fe27009091dc0078de9f00e3e8c100e1bc3200bfce47005f533500d959f4008e264e00ff792c002e4bd400e230cb00fce23100600d6e00879eba00d478ea0019ec6b00a617a7002db405009c639b00 041000051716070c01000000042300052515101601000000020b00040112031a0003061b200000000001", "0 6ef142006ef142006ef142006ef142002c11f700fd53a100881d62003fba10005ff58d00212465002124650021246500212465003f423400a131ba006ef14200fd53a1005ff58d005ec7c9005ec7c9005ff58d00f89de800b625ec000ac6f6001c3c7d0055a6a200f6740600"),
    ("dec 8 16 7 2 0f750e00d921ba004f30ae00299d78001f2c4f0055c61e00c3959e00a2164400c8435b00397f78008fe32600596590002ee6af00ecd7f200478ce3008b816900172e6a0074d09800e9b59e005cb7ff00cf628d0009f53d00baf6af001d850100adf4250049ea2b00dbdaf2005cef4e004bc0bc00fd797100e7884200d00bec0056c8fc00fe7e8900e8549c00 669cca0088e39f004d783d008f389d00cd4026009afff80031f7b300aeaf370012b498006b8e39000412ed00e37320008531ca005512de00ec31c800165f9a005fa8b5006eb1d000dd3e2400725fc0001a712500b6860e00438db00070f9980088e89b00efb4a1005bc71300b6774f006b39d700448057000b9ba900c7375c00c48048000b9ba9003f2f8c005a396e009bb62500be51f9008d9a0f00681e80004a684d0087cc4800 021400020401010700000001", "0 9989998900000000000000000000000000000000000000000000000054400000"),
];
    // END ORACLE VECTORS

    const ORACLE_EDGE: &[(&str, &str)] = &[
        (
            "dpal 8 16 000000000000ff00ffffff00 ff00000000ff000000ff00ffffff00",
            "000000f8fffb",
        ),
        (
            "iframe 16 3 2 000000000000ff00ffffff00 000102030405060708090a0b0c0d0e0f",
            "0 0 0 0 0 0",
        ),
        (
            "dec 8 8 4 2 00000000ffffff00 00000000ffffff00 0002040002010001",
            "0 0000000001010000",
        ),
        (
            "dec 4 8 5 1 00000000ffffff00 00000000ffffff00 0002030002010001",
            "0 0000000001000000",
        ),
    ];

    #[test]
    fn oracle_vectors_match() {
        for &(command, expected) in ORACLE_VECTORS {
            assert_eq!(run(command), expected, "{command}");
        }
    }

    #[test]
    fn oracle_edge_vectors_match() {
        let pin = palette("00000000ffffff00");
        let cur4 = vec![0x11; 152];
        let cur8 = vec![1; 300]
            .into_iter()
            .chain(vec![0; 4])
            .collect::<Vec<_>>();
        let stride4 = 12;
        let stride8 = 8;
        let prev4 = vec![0; stride4 * 300];
        let mut cur_delta4 = prev4.clone();
        cur_delta4[(299 * stride4) + 3] = 1;
        let prev8 = vec![0; stride8 * 300];
        let mut cur_delta8 = prev8.clone();
        cur_delta8[(299 * stride8) + 2] = 1;
        let constructed = [
            encode(4, 4, 300, 1, &pin, &pin, &cur4, None),
            encode(8, 8, 300, 1, &pin, &pin, &cur8, None),
            encode(4, 4, 4, 0, &pin, &pin, &[], None),
            encode(8, 8, 4, 0, &pin, &pin, &[], None),
            encode(4, 8, 10, 300, &pin, &pin, &cur_delta4, Some(&prev4)),
            encode(8, 8, 5, 300, &pin, &pin, &cur_delta8, Some(&prev8)),
            encode(
                8,
                8,
                1,
                2,
                &pin,
                &pin,
                &[1, 0, 0, 0, 0, 0, 0, 0],
                Some(&[0, 0, 0, 0, 0, 0, 0, 0]),
            ),
            encode(
                8,
                8,
                1,
                2,
                &pin,
                &pin,
                &[0, 1, 0, 0, 0, 0, 0, 0],
                Some(&[0, 0, 0, 0, 0, 0, 0, 0]),
            ),
        ];
        let expected = [
            "6 fe112e110001",
            "8 ff012d0100000001",
            "0 -",
            "2 0001",
            "18 000203ff0002002c02100002050000000001",
            "22 000202ff0002002c0101010000020100000000000001",
            "14 0101000000020100000000000001",
            "16 00020100000000020100000000000001",
        ];
        for (got, want) in constructed.iter().zip(expected) {
            assert_eq!(got, want);
        }
        for &(command, expected) in ORACLE_EDGE {
            assert_eq!(run(command), expected, "{command}");
        }
    }

    #[test]
    #[should_panic(expected = "unknown oracle command")]
    fn oracle_protocol_rejects_unknown_commands() {
        run("unknown");
    }

    #[test]
    fn miscellaneous_edges_and_decode_padding() {
        let black = RgbQuad::new(0, 0, 0);
        let white = RgbQuad::new(255, 255, 255);
        assert_eq!(nearest_palette_index(&[], white), 0);
        assert_eq!(
            decompress_palette_map(16, &[white], &[black])[..2],
            [255, 251]
        );
        assert_eq!(decompress_palette_map(1, &[white], &[black]), [0; 1024]);
        let unsupported = Source {
            width: 1,
            height: 1,
            bit_count: 15,
            pixels: &[0; 4],
        };
        assert_eq!(
            compute_internal_frame(&unsupported, &[black]).pixels(),
            &[0]
        );
        assert_eq!(run("cpal 8 15 00000000 ff000000"), "-");
        assert_eq!(run("enc 4 4 1 0 00000000 00000000 - -"), "0 -");
        assert_eq!(run("dec 8 8 8 1 00000000 00000000 -"), "err Truncated");
        let map = decompress_palette_map(8, &[], &[]);
        assert_eq!(
            decompress_rle8(&map, 8, 8, 1, &[0, 3, 1, 2, 3, 0, 0, 1]),
            Ok(vec![0; 8])
        );
    }

    #[test]
    fn encoding_small_rows_cover_boundaries() {
        let palette = [RgbQuad::new(0, 0, 0), RgbQuad::new(255, 255, 255)];
        let map = compress_palette_map(&palette, &palette);
        for width in 1..=6 {
            let stride = dib_stride(width, 8);
            for cur_mask in 0..(1usize << width) {
                for prev_mask in 0..(1usize << width) {
                    let current_pixels = (0..stride)
                        .map(|x| ((cur_mask >> x) & 1) as u8)
                        .collect::<Vec<_>>();
                    let previous_pixels = (0..stride)
                        .map(|x| ((prev_mask >> x) & 1) as u8)
                        .collect::<Vec<_>>();
                    let current_source = Source {
                        width,
                        height: 1,
                        bit_count: 8,
                        pixels: &current_pixels,
                    };
                    let previous_source = Source {
                        width,
                        height: 1,
                        bit_count: 8,
                        pixels: &previous_pixels,
                    };
                    let current = compute_internal_frame(&current_source, &palette);
                    let previous = compute_internal_frame(&previous_source, &palette);
                    assert!(
                        !compress_rle4(&current_source, &map, &current, Some(&previous)).is_empty()
                    );
                    assert!(
                        !compress_rle8(&current_source, &map, &current, Some(&previous)).is_empty()
                    );
                }
            }
        }
        let current_pixels = [1, 0, 0, 0, 0, 0, 0, 0];
        let previous_pixels = current_pixels;
        let current_source = Source {
            width: 8,
            height: 1,
            bit_count: 8,
            pixels: &current_pixels,
        };
        let previous_source = Source {
            width: 8,
            height: 1,
            bit_count: 8,
            pixels: &previous_pixels,
        };
        let current = compute_internal_frame(&current_source, &palette);
        let previous = compute_internal_frame(&previous_source, &palette);
        assert!(!compress_rle8(&current_source, &map, &current, Some(&previous)).is_empty());
        let two_pixel_source = Source {
            width: 2,
            height: 1,
            bit_count: 8,
            pixels: &[1, 0, 0, 0],
        };
        let two_pixel_frame = compute_internal_frame(&two_pixel_source, &palette);
        assert!(!compress_rle4(&two_pixel_source, &map, &two_pixel_frame, None).is_empty());
        let width = 24;
        let previous_pixels = vec![0; width * 2];
        let mut current_pixels = vec![0; width * 2];
        for first_change in 0..width {
            for second_change in 0..width {
                current_pixels.fill(0);
                current_pixels[first_change] = 1;
                current_pixels[width + second_change] = 1;
                let current_source = Source {
                    width,
                    height: 2,
                    bit_count: 8,
                    pixels: &current_pixels,
                };
                let previous_source = Source {
                    width,
                    height: 2,
                    bit_count: 8,
                    pixels: &previous_pixels,
                };
                let current = compute_internal_frame(&current_source, &palette);
                let previous = compute_internal_frame(&previous_source, &palette);
                assert!(
                    !compress_rle4(&current_source, &map, &current, Some(&previous)).is_empty()
                );
            }
        }
    }

    #[test]
    fn decoder_errors_are_reported() {
        let map = decompress_palette_map(8, &[], &[]);
        for four_bit in [true, false] {
            let decode = |stream: &[u8], width, height| {
                if four_bit {
                    decompress_rle4(&map, 8, width, height, stream)
                } else {
                    decompress_rle8(&map, 8, width, height, stream)
                }
            };
            for stream in [&[][..], &[5][..], &[0, 2, 1][..], &[0, 2][..]] {
                assert_eq!(decode(stream, 8, 1), Err(Error::Truncated));
            }
        }
        assert_eq!(
            decompress_rle4(&map, 8, 8, 1, &[0, 4, 0x12]),
            Err(Error::Truncated)
        );
        assert_eq!(
            decompress_rle4(&map, 8, 8, 1, &[0, 5, 0x12, 0x34, 0x50]),
            Err(Error::Truncated)
        );
        assert_eq!(
            decompress_rle8(&map, 8, 8, 1, &[0, 3, 1, 2]),
            Err(Error::Truncated)
        );
        assert_eq!(
            decompress_rle8(&map, 8, 8, 1, &[0, 3, 1, 2, 3]),
            Err(Error::Truncated)
        );
        assert_eq!(
            decompress_rle8(&map, 8, 1, 1, &[0, 3, 1, 2, 3]),
            Err(Error::RunTooLong)
        );
        assert_eq!(
            decompress_rle4(&map, 8, 8, 1, &[9, 0]),
            Err(Error::RunTooLong)
        );
        assert_eq!(
            decompress_rle8(&map, 8, 8, 1, &[9, 0]),
            Err(Error::RunTooLong)
        );
        assert_eq!(
            decompress_rle4(&map, 8, 2, 1, &[0, 0, 1, 0x11]),
            Err(Error::OutputOverflow)
        );
        assert_eq!(
            decompress_rle4(&map, 8, 3, 1, &[0, 0, 0, 3, 0x12, 0x30]),
            Err(Error::OutputOverflow)
        );
        assert_eq!(
            decompress_rle8(&map, 8, 2, 1, &[0, 0, 1, 5]),
            Err(Error::OutputOverflow)
        );
        assert_eq!(
            decompress_rle8(&map, 8, 4, 1, &[0, 0, 0, 3, 5, 6, 7, 0]),
            Err(Error::OutputOverflow)
        );
        assert_eq!(
            decompress_rle8(&map, 4, 8, 1, &[0, 1]),
            Err(Error::UnsupportedBitCount(4))
        );
        assert_eq!(
            decompress_rle8(&map, 1, 8, 1, &[0, 1]),
            Err(Error::UnsupportedBitCount(1))
        );
    }
}
