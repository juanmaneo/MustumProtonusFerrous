//! The decompression entry points of `msvideo1.c`: the query, format and begin messages, and
//! `CRAM_Decompress` / `CRAM_DecompressEx`, which share one path.

use crate::abi::{BitmapInfoHeader, BI_RGB, CRAM_MAGIC, MSVC_MAGIC, RGBQUAD_SIZE, WHAM_MAGIC};
use crate::convert::convert_depth;
use crate::decode::{decode_16bit, decode_8bit};
use crate::driver::Msvideo1Context;
use crate::frame::get_stride;
use crate::{IcError, IcResult};

/// The reply of `CRAM_DecompressGetFormat`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatQuery {
    /// No output header was passed: the bytes that the input format occupies, its `biSize`
    /// plus the palette.
    Size(u32),
    /// An output header was passed: the input format with even dimensions, `BI_RGB`, and the
    /// size of the image it produces.
    Output(BitmapInfoHeader),
}

type DecodeFn = fn(i32, i32, &[u8], &mut [u8], i32) -> IcResult<()>;

impl Msvideo1Context {
    /// `CRAM_DecompressQuery`: whether the input format is decodable and, when `output` is
    /// given, whether the output can take the decoded frame.
    pub fn decompress_query(
        &self,
        input: &BitmapInfoHeader,
        output: Option<&BitmapInfoHeader>,
    ) -> IcResult<()> {
        self.check()?;
        if ![CRAM_MAGIC, MSVC_MAGIC, WHAM_MAGIC].contains(&input.compression) {
            return Err(IcError::BadFormat);
        }
        if ![8, 16].contains(&input.bit_count) {
            return Err(IcError::BadFormat);
        }
        if let Some(out) = output {
            let depth_ok =
                input.bit_count == out.bit_count || (input.bit_count == 16 && out.bit_count == 24);
            if !depth_ok
                || input.planes != out.planes
                || input.height != out.height
                || input.width != out.width
            {
                return Err(IcError::BadFormat);
            }
        }
        Ok(())
    }

    /// `CRAM_DecompressGetFormat`. Without `want_output`, the reply is the input format's size.
    /// With it, the reply is the output format. Its size comes from the input height, as in
    /// Wine.
    pub fn decompress_get_format(
        &self,
        input: &BitmapInfoHeader,
        want_output: bool,
    ) -> IcResult<FormatQuery> {
        self.check()?;
        let mut size = input.size;
        if input.bit_count <= 8 {
            size = size.wrapping_add(input.clr_used.wrapping_mul(RGBQUAD_SIZE));
        }
        if ![8, 16].contains(&input.bit_count) {
            return Err(IcError::BadFormat);
        }
        if !want_output {
            return Ok(FormatQuery::Size(size));
        }
        let mut out = *input;
        out.width &= !1;
        out.height &= !1;
        out.compression = BI_RGB;
        let stride = get_stride(out.width, i32::from(out.bit_count));
        out.size_image = input.height.wrapping_mul(stride) as u32;
        Ok(FormatQuery::Output(out))
    }

    /// `CRAM_DecompressBegin`: chooses the output depth from the bit count of `input`. An
    /// unsupported bit count leaves the depth at 0, and [Msvideo1Context::decompress] then
    /// refuses to decode.
    pub fn decompress_begin(&mut self, input: &BitmapInfoHeader) -> IcResult<()> {
        self.check()?;
        self.depth = match input.bit_count {
            8 => 8,
            16 => 16,
            _ => 0,
        };
        Ok(())
    }

    /// `CRAM_Decompress` and `CRAM_DecompressEx`. Decodes `stream` at the input's size, using the
    /// input's dimensions, into `out` as the output format describes. A 16-bit decode into an
    /// 8-bit output, or the reverse, goes through a scratch buffer sized by the output's
    /// dimensions and then converted.
    pub fn decompress(
        &self,
        input: &BitmapInfoHeader,
        stream: &[u8],
        output: &BitmapInfoHeader,
        out: &mut [u8],
    ) -> IcResult<()> {
        self.check()?;
        let (decode, stride): (DecodeFn, i32) = match self.depth {
            8 => (decode_8bit, get_stride(input.width, 8)),
            16 => (decode_16bit, get_stride(input.width, 16) / 2),
            _ => return Err(IcError::BadFormat),
        };
        let limit = (input.size_image as i32).max(0) as usize;
        let stream = &stream[..limit.min(stream.len())];
        if i32::from(output.bit_count) == self.depth {
            return decode(input.width, input.height, stream, out, stride);
        }
        let mut scratch = vec![0u8; scratch_len(output.width, output.height, self.depth)?];
        decode(input.width, input.height, stream, &mut scratch, stride)?;
        convert_depth(&scratch, self.depth, out, output)
    }
}

/// The scratch size that `CRAM_Decompress` allocates: `width * height * depth / 8` bytes.
/// Overflow wraps as C's `LONG` does. A negative size fails the allocation.
fn scratch_len(width: i32, height: i32, depth: i32) -> IcResult<usize> {
    let bytes = width.wrapping_mul(height).wrapping_mul(depth) / 8;
    usize::try_from(bytes).map_err(|_| IcError::Memory)
}
