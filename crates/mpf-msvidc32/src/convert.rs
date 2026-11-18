//! `convert_depth` of `msvideo1.c`: expands 16-bit pixels to 24-bit, with 5 bits per channel
//! scaled to 8 bits.

use crate::abi::BitmapInfoHeader;
use crate::frame::{check_rows, get_stride};
use crate::IcResult;

const CONVERT_5TO8: [u8; 32] = [
    0x00, 0x08, 0x10, 0x19, 0x21, 0x29, 0x31, 0x3a, 0x42, 0x4a, 0x52, 0x5a, 0x63, 0x6b, 0x73, 0x7b,
    0x84, 0x8c, 0x94, 0x9c, 0xa5, 0xad, 0xb5, 0xbd, 0xc5, 0xce, 0xd6, 0xde, 0xe6, 0xef, 0xf7, 0xff,
];

/// `convert_depth`: converts the `depth_in`-bit rows of `input` into the frame that `out_hdr`
/// describes in `output`. Only 16 to 24 bits converts. For any other pair, and for a frame
/// with no pixels, nothing is written and the call returns `Ok`, as in Wine.
pub fn convert_depth(
    input: &[u8],
    depth_in: i32,
    output: &mut [u8],
    out_hdr: &BitmapInfoHeader,
) -> IcResult<()> {
    if depth_in != 16 || out_hdr.bit_count != 24 || out_hdr.width <= 0 || out_hdr.height <= 0 {
        return Ok(());
    }
    let width = out_hdr.width as usize;
    let height = out_hdr.height as usize;
    let stride_in = check_rows(
        height,
        get_stride(out_hdr.width, 16),
        2 * width,
        input.len(),
    )?;
    let stride_out = check_rows(
        height,
        get_stride(out_hdr.width, 24),
        3 * width,
        output.len(),
    )?;
    for y in 0..height {
        let (src, _) = input[y * stride_in..][..2 * width].as_chunks::<2>();
        let (dst, _) = output[y * stride_out..][..3 * width].as_chunks_mut::<3>();
        for (pixel, out) in src.iter().zip(dst) {
            let value = u16::from_le_bytes(*pixel);
            out[0] = CONVERT_5TO8[usize::from((value & 0x7c00) >> 10)];
            out[1] = CONVERT_5TO8[usize::from((value & 0x03e0) >> 5)];
            out[2] = CONVERT_5TO8[usize::from(value & 0x001f)];
        }
    }
    Ok(())
}
