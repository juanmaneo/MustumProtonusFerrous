//! `MFConvertColorInfoToDXVA` from `dlls/mfplat/mediatype.c`: packs the colour fields of a video
//! format into the DXVA2 `DWORD` that a decoder reads.

/// The `MFVideoInfo` fields that `MFConvertColorInfoToDXVA` reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VideoColorInfo {
    pub interlace_mode: u32,
    pub source_chroma_subsampling: u32,
    pub nominal_range: u32,
    pub transfer_matrix: u32,
    pub source_lighting: u32,
    pub color_primaries: u32,
    pub transfer_function: u32,
}

/// `MFVideoInterlace_MixedInterlaceOrProgressive`.
const MIXED_INTERLACE_OR_PROGRESSIVE: u32 = 7;

/// `DXVA2_SampleFieldInterleavedEvenFirst`, the sample format that a mixed interlace maps to.
const SAMPLE_FIELD_INTERLEAVED_EVEN_FIRST: u32 = 3;

/// `MFConvertColorInfoToDXVA`: packs the fields LSB first, as 8, 4, 3, 3, 4, 5 and 5 bits.
///
/// Each value keeps only its low bits, as a store to a C bit-field does. The sample format is
/// `interlace_mode`, except that a mixed interlace becomes an even-first interleave.
pub fn convert_color_info_to_dxva(info: &VideoColorInfo) -> u32 {
    let sample_format = if info.interlace_mode == MIXED_INTERLACE_OR_PROGRESSIVE {
        SAMPLE_FIELD_INTERLEAVED_EVEN_FIRST
    } else {
        info.interlace_mode
    };
    field(sample_format, 8, 0)
        | field(info.source_chroma_subsampling, 4, 8)
        | field(info.nominal_range, 3, 12)
        | field(info.transfer_matrix, 3, 15)
        | field(info.source_lighting, 4, 18)
        | field(info.color_primaries, 5, 22)
        | field(info.transfer_function, 5, 27)
}

/// `value` truncated to `bits` bits and placed at bit `shift`.
fn field(value: u32, bits: u32, shift: u32) -> u32 {
    (value & ((1 << bits) - 1)) << shift
}
