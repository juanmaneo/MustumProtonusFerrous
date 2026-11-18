//! Headless port of Wine's `dlls/ir50_32`, the Intel Indeo 5 (IV50) video decoder driver.
//!
//! The component exports one entry point, `IV50_DriverProc` (`ir50_32.spec`), so its
//! export is partially ported: the pure parts of its message handlers are ported and
//! the parts that need a live decoder instance are skipped.
//!
//! Ported:
//! - `DriverProc` constant results per message ([`driver_proc_constant_result`]).
//! - `IV50_Open` fourcc gate ([`open_accepts`]).
//! - `IV50_DecompressQuery` ([`decompress_query`]).
//! - `IV50_DecompressGetFormat` ([`decompress_get_format`]).
//! - `IV50_DecompressBegin` validation and media type values ([`decompress_begin_plan`]).
//! - The output step of `IV50_Decompress` ([`process_output`]).
//! - The frame copy of `IV50_Decompress`, as `MFCopyImage` does it ([`copy_frame`]).
//! - `IV50_GetInfo` ICINFO fields ([`get_info`]).
//! - The `compare_fourcc`, `make_uint64` and `fill_decompressex_params` helpers.
//!
//! Skipped:
//! - `DllMain`: process attach has no portable logic.
//! - `IV50_Open` decoder creation: `winegstreamer_create_video_decoder` makes a COM object.
//! - `IV50_DecompressBegin` media types and `SetInputType`/`SetOutputType`: Media Foundation objects.
//! - `IV50_Decompress` sample and buffer pipeline (`MFCreateSample`, `ProcessInput`, buffer locks).
//! - `IV50_GetInfo` `szName`/`szDescription`: `LoadStringW` reads module resources.
//! - `DRV_CLOSE` decoder release and the `DRV_OPEN` handle return: COM object lifetime.
//! - `DriverProc` dispatch on `dwDriverId` to the decoder: needs the live driver instance.
//! - `ICM_DECOMPRESS_GET_PALETTE`, `ICM_COMPRESS*`, `ICM_CONFIGURE` and unknown messages: FIXME or
//!   no-op in C, routed to `ICERR_UNSUPPORTED` (`ICM_COMPRESS_QUERY` to `ICERR_BADFORMAT`).
//!
//! Divergences from Wine:
//! - [`copy_frame`] refuses a rectangle that falls outside either buffer with `ICERR_BADSIZE`;
//!   `MFCopyImage` writes without checks.
//! - Sizes and heights use wrapping arithmetic where C overflows `LONG` (undefined behaviour), so
//!   `abs(INT_MIN)` stays `INT_MIN`.
//! - [`decompress_begin_plan`] without an output header returns `ICERR_BADPARAM`; Wine dereferences
//!   the NULL pointer.
//!
//! Wine's begin stride ignores bytes per pixel, while the frame copy stride includes them. Both are
//! ported as Wine has them.

/// `LRESULT`, the result type of `DriverProc`.
pub type LResult = isize;

pub const ICERR_OK: LResult = 0;
pub const ICERR_UNSUPPORTED: LResult = -1;
pub const ICERR_BADFORMAT: LResult = -2;
pub const ICERR_INTERNAL: LResult = -4;
pub const ICERR_BADPARAM: LResult = -6;
pub const ICERR_BADSIZE: LResult = -7;

const TRUE: LResult = 1;
const BI_RGB: u32 = 0;
const BI_BITFIELDS: u32 = 3;
const BITMAPINFOHEADER_SIZE: u32 = 40;
const BITMAPINFO_PALETTE_SIZE: usize = BITMAPINFOHEADER_SIZE as usize + 256 * 4;
const ICINFO_SIZE: u32 = 568;
const ICVERSION: u32 = 0x0104;
const RGB565_MASKS: [u32; 3] = [0xf800, 0x07e0, 0x001f];

const DRV_USER: u32 = 0x4000;
const ICM_USER: u32 = DRV_USER;
const ICM_RESERVED: u32 = DRV_USER + 0x1000;

const DRV_LOAD: u32 = 0x0001;
const DRV_ENABLE: u32 = 0x0002;
const DRV_OPEN: u32 = 0x0003;
const DRV_CLOSE: u32 = 0x0004;
const DRV_DISABLE: u32 = 0x0005;
const DRV_FREE: u32 = 0x0006;
const ICM_GETINFO: u32 = ICM_RESERVED + 2;
const ICM_COMPRESS_GET_FORMAT: u32 = ICM_USER + 4;
const ICM_COMPRESS_QUERY: u32 = ICM_USER + 6;
const ICM_COMPRESS: u32 = ICM_USER + 8;
const ICM_COMPRESS_END: u32 = ICM_USER + 9;
const ICM_DECOMPRESS_GET_FORMAT: u32 = ICM_USER + 10;
const ICM_DECOMPRESS_QUERY: u32 = ICM_USER + 11;
const ICM_DECOMPRESS_BEGIN: u32 = ICM_USER + 12;
const ICM_DECOMPRESS: u32 = ICM_USER + 13;
const ICM_DECOMPRESS_END: u32 = ICM_USER + 14;
const ICM_DECOMPRESS_GET_PALETTE: u32 = ICM_USER + 30;
const ICM_DECOMPRESSEX_BEGIN: u32 = ICM_USER + 60;
const ICM_DECOMPRESSEX_QUERY: u32 = ICM_USER + 61;
const ICM_DECOMPRESSEX: u32 = ICM_USER + 62;
const ICM_DECOMPRESSEX_END: u32 = ICM_USER + 63;
const ICM_CONFIGURE: u32 = ICM_RESERVED + 10;

const IV50: u32 = u32::from_le_bytes(*b"IV50");
const ICTYPE_VIDEO: u32 = u32::from_le_bytes(*b"vidc");

const MF_E_TRANSFORM_NEED_MORE_INPUT: i32 = -1_072_861_838;
const MF_E_TRANSFORM_STREAM_CHANGE: i32 = -1_072_861_855;

/// `BITMAPINFOHEADER`, with the same field widths as the Win32 structure.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BitmapInfoHeader {
    pub size: u32,
    pub width: i32,
    pub height: i32,
    pub planes: u16,
    pub bit_count: u16,
    pub compression: u32,
    pub size_image: u32,
    pub x_pels_per_meter: i32,
    pub y_pels_per_meter: i32,
    pub clr_used: u32,
    pub clr_important: u32,
}

/// A `BITMAPINFO` header with the three DWORD masks that `BI_BITFIELDS` reads from
/// `bmiColors[0..3]`. The rest of the palette is not modelled.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BitmapInfo {
    pub header: BitmapInfoHeader,
    pub masks: [u32; 3],
}

/// The offset and size of an `ICDECOMPRESSEX` rectangle.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub dx: i32,
    pub dy: i32,
}

/// The fields of `ICDECOMPRESSEX` that the handlers read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecompressParams {
    pub src: BitmapInfoHeader,
    pub dst: Option<BitmapInfo>,
    pub src_rect: Rect,
    pub dst_rect: Rect,
}

impl DecompressParams {
    /// `fill_decompressex_params`: the rectangles cover whole images, and the output height is
    /// taken as an absolute value.
    pub fn from_headers(src: BitmapInfoHeader, dst: Option<BitmapInfo>) -> Self {
        let src_rect = Rect {
            x: 0,
            y: 0,
            dx: src.width,
            dy: src.height,
        };
        let dst_rect = dst.map_or_else(Rect::default, |output| Rect {
            x: 0,
            y: 0,
            dx: output.header.width,
            dy: output.header.height.wrapping_abs(),
        });
        Self {
            src,
            dst,
            src_rect,
            dst_rect,
        }
    }
}

/// The fields of `ICINFO` that `IV50_GetInfo` sets. The name and description are skipped.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct IcInfo {
    pub dw_size: u32,
    pub fcc_type: u32,
    pub fcc_handler: u32,
    pub flags: u32,
    pub version: u32,
    pub version_icm: u32,
}

/// The result of `IV50_GetInfo`, which returns a size, `0`, or the filled structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InfoResult {
    SizeQuery,
    TooSmall,
    Filled(IcInfo),
}

/// The `ICERR_*` failure codes the handlers can return.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IcError {
    BadFormat,
    BadParam,
    BadSize,
    Internal,
}

impl IcError {
    pub fn code(self) -> LResult {
        match self {
            Self::BadFormat => ICERR_BADFORMAT,
            Self::BadParam => ICERR_BADPARAM,
            Self::BadSize => ICERR_BADSIZE,
            Self::Internal => ICERR_INTERNAL,
        }
    }
}

pub type IcResult<T> = Result<T, IcError>;

/// The result of `IV50_DecompressGetFormat`: either the palette size of the `BITMAPINFO`
/// buffer the caller must allocate, or the output header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatQuery {
    PaletteSize(usize),
    Output(BitmapInfoHeader),
}

/// The RGB output subtype chosen by `IV50_DecompressBegin`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputSubtype {
    Rgb32,
    Rgb24,
    Rgb565,
    Rgb555,
}

/// The values `IV50_DecompressBegin` assigns to its media types, before the media types are
/// handed to the transform.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BeginPlan {
    pub output_subtype: OutputSubtype,
    pub input_frame_size: u64,
    pub output_frame_size: u64,
    pub default_stride: u32,
}

/// What `IV50_Decompress` does after `IMFTransform_ProcessOutput`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputAction {
    /// The output sample holds a frame: copy it with [`copy_frame`].
    Copy,
    /// No frame to copy: return this result.
    Finish(IcResult<()>),
}

/// Zero when the fourccs match, ignoring the case bit of each byte.
pub fn compare_fourcc(a: u32, b: u32) -> u32 {
    (a ^ b) & !0x2020_2020
}

pub fn make_uint64(high: u32, low: u32) -> u64 {
    (u64::from(high) << 32) | u64::from(low)
}

/// `IV50_DecompressQuery`: accepts IV50 input at its full size and, when an output header is given,
/// an RGB output of the same size.
pub fn decompress_query(params: &DecompressParams) -> IcResult<()> {
    let input = &params.src;
    let rect = params.src_rect;
    if compare_fourcc(input.compression, IV50) != 0
        || rect.x != 0
        || rect.y != 0
        || rect.dx != input.width
        || rect.dy != input.height
    {
        return Err(IcError::BadFormat);
    }
    let Some(output) = &params.dst else {
        return Ok(());
    };
    if !output_format_accepted(output) {
        return Err(IcError::BadFormat);
    }
    if input.width != params.dst_rect.dx || input.height != params.dst_rect.dy {
        return Err(IcError::BadParam);
    }
    Ok(())
}

fn output_format_accepted(output: &BitmapInfo) -> bool {
    let header = &output.header;
    match header.compression {
        BI_RGB => matches!(header.bit_count, 32 | 24 | 16),
        BI_BITFIELDS => header.bit_count == 16 && output.masks == RGB565_MASKS,
        _ => false,
    }
}

/// `IV50_DecompressGetFormat`. Without an output request, the answer is the palette size.
pub fn decompress_get_format(input: &BitmapInfoHeader, want_output: bool) -> IcResult<FormatQuery> {
    if compare_fourcc(input.compression, IV50) != 0 {
        return Err(IcError::BadFormat);
    }
    if !want_output {
        return Ok(FormatQuery::PaletteSize(BITMAPINFO_PALETTE_SIZE));
    }
    let width = input.width;
    let height = input.height.wrapping_abs();
    Ok(FormatQuery::Output(BitmapInfoHeader {
        size: BITMAPINFOHEADER_SIZE,
        width,
        height,
        planes: 1,
        bit_count: 24,
        compression: BI_RGB,
        size_image: width.wrapping_mul(height).wrapping_mul(3) as u32,
        ..Default::default()
    }))
}

/// `IV50_DecompressBegin`: validates the pair of headers and computes the values assigned to the
/// input and output media types. `has_decoder` is false when no transform was created.
pub fn decompress_begin_plan(has_decoder: bool, params: &DecompressParams) -> IcResult<BeginPlan> {
    let output = match (has_decoder, &params.dst) {
        (true, Some(output)) => output,
        _ => return Err(IcError::BadParam),
    };
    let header = &output.header;
    let output_subtype = if header.compression == BI_BITFIELDS {
        OutputSubtype::Rgb565
    } else {
        match header.bit_count {
            32 => OutputSubtype::Rgb32,
            24 => OutputSubtype::Rgb24,
            16 => OutputSubtype::Rgb555,
            _ => return Err(IcError::BadFormat),
        }
    };
    let stride = (header.width.wrapping_add(3) & !3) as u32;
    Ok(BeginPlan {
        output_subtype,
        input_frame_size: make_uint64(params.src.width as u32, params.src.height as u32),
        output_frame_size: make_uint64(params.dst_rect.dx as u32, params.dst_rect.dy as u32),
        default_stride: if header.height >= 0 {
            stride.wrapping_neg()
        } else {
            stride
        },
    })
}

/// The output step of `IV50_Decompress`. `process` calls `IMFTransform_ProcessOutput` and returns
/// its HRESULT. A stream change is answered by one more call, and only a success yields a frame.
pub fn process_output(mut process: impl FnMut() -> i32) -> OutputAction {
    let mut hr = process();
    if hr == MF_E_TRANSFORM_STREAM_CHANGE {
        hr = process();
    }
    if hr >= 0 {
        OutputAction::Copy
    } else if hr == MF_E_TRANSFORM_NEED_MORE_INPUT {
        OutputAction::Finish(Ok(()))
    } else {
        OutputAction::Finish(Err(IcError::Internal))
    }
}

/// The copy stage of `IV50_Decompress`: `MFCopyImage` of `rect` from the decoded image into `out`.
/// The decoded image has no offset; the destination starts at the rectangle's offset. Both use the
/// stride `width * bytes_per_pixel`, rounded up to a multiple of 4.
pub fn copy_frame(
    dst: &BitmapInfoHeader,
    rect: Rect,
    decoded: &[u8],
    out: &mut [u8],
) -> IcResult<()> {
    let depth = i128::from(dst.bit_count / 8);
    let stride = (i128::from(dst.width) * depth + 3) & !3;
    let row = i128::from(rect.dx) * depth;
    let lines = i128::from(rect.dy);
    if lines < 0 || row < 0 {
        return Err(IcError::BadSize);
    }
    if lines == 0 || row == 0 {
        return Ok(());
    }
    let start = i128::from(rect.y) * stride + i128::from(rect.x) * depth;
    let span = (lines - 1) * stride + row;
    if stride < 0 || start < 0 || start + span > out.len() as i128 || span > decoded.len() as i128 {
        return Err(IcError::BadSize);
    }
    let (start, stride, row, lines) = (
        start as usize,
        stride as usize,
        row as usize,
        lines as usize,
    );
    for line in 0..lines {
        let from = line * stride;
        let to = start + from;
        out[to..to + row].copy_from_slice(&decoded[from..from + row]);
    }
    Ok(())
}

/// `IV50_GetInfo`. `icinfo_present` is false for the size query, and `size` is the caller's buffer.
pub fn get_info(icinfo_present: bool, size: u32) -> InfoResult {
    if !icinfo_present {
        return InfoResult::SizeQuery;
    }
    if size < ICINFO_SIZE {
        return InfoResult::TooSmall;
    }
    InfoResult::Filled(IcInfo {
        dw_size: ICINFO_SIZE,
        fcc_type: ICTYPE_VIDEO,
        fcc_handler: IV50,
        flags: 0,
        version: ICVERSION,
        version_icm: ICVERSION,
    })
}

/// The `IV50_Open` gate: rejects an `ICINFO` whose type is not `vidc`. A missing `ICINFO` is
/// accepted.
pub fn open_accepts(icinfo: Option<&IcInfo>) -> bool {
    match icinfo {
        None => true,
        Some(info) => compare_fourcc(info.fcc_type, ICTYPE_VIDEO) == 0,
    }
}

/// The `DriverProc` result for messages that do not need the driver instance.
/// `None` means the message is dispatched to one of the handlers above.
pub fn driver_proc_constant_result(msg: u32) -> Option<LResult> {
    match msg {
        DRV_LOAD | DRV_CLOSE => Some(TRUE),
        DRV_OPEN
        | ICM_GETINFO
        | ICM_DECOMPRESS_GET_FORMAT
        | ICM_DECOMPRESS_QUERY
        | ICM_DECOMPRESSEX_QUERY
        | ICM_DECOMPRESS_BEGIN
        | ICM_DECOMPRESSEX_BEGIN
        | ICM_DECOMPRESS
        | ICM_DECOMPRESSEX => None,
        ICM_DECOMPRESS_END | ICM_DECOMPRESSEX_END => Some(ICERR_OK),
        ICM_COMPRESS_QUERY => Some(ICERR_BADFORMAT),
        DRV_ENABLE
        | DRV_DISABLE
        | DRV_FREE
        | ICM_COMPRESS_GET_FORMAT
        | ICM_COMPRESS_END
        | ICM_COMPRESS
        | ICM_DECOMPRESS_GET_PALETTE
        | ICM_CONFIGURE => Some(ICERR_UNSUPPORTED),
        _ => Some(ICERR_UNSUPPORTED),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const YUY2: u32 = u32::from_le_bytes(*b"YUY2");
    const AUDC: u32 = u32::from_le_bytes(*b"audc");

    fn header(width: i32, height: i32, bit_count: u16, compression: u32) -> BitmapInfoHeader {
        BitmapInfoHeader {
            size: BITMAPINFOHEADER_SIZE,
            width,
            height,
            planes: 1,
            bit_count,
            compression,
            ..Default::default()
        }
    }

    fn bitmap_info(header: BitmapInfoHeader) -> BitmapInfo {
        BitmapInfo {
            header,
            masks: [0; 3],
        }
    }

    fn iv50_input() -> BitmapInfoHeader {
        header(320, 240, 12, IV50)
    }

    fn code_of(result: IcResult<()>) -> LResult {
        match result {
            Ok(()) => ICERR_OK,
            Err(error) => error.code(),
        }
    }

    #[test]
    fn query_rejects_non_iv50_input() {
        let params = DecompressParams::from_headers(header(320, 240, 12, YUY2), None);
        assert_eq!(decompress_query(&params), Err(IcError::BadFormat));
    }

    #[test]
    fn query_without_output_accepts_iv50() {
        let params = DecompressParams::from_headers(iv50_input(), None);
        assert_eq!(decompress_query(&params), Ok(()));
    }

    #[test]
    fn query_accepts_rgb32_and_rgb16_output() {
        for bit_count in [32, 16] {
            let output = bitmap_info(header(320, 240, bit_count, BI_RGB));
            let params = DecompressParams::from_headers(iv50_input(), Some(output));
            assert_eq!(decompress_query(&params), Ok(()));
        }
    }

    #[test]
    fn query_rejects_bitfields_with_other_depth() {
        let output = BitmapInfo {
            header: header(320, 240, 24, BI_BITFIELDS),
            masks: RGB565_MASKS,
        };
        let params = DecompressParams::from_headers(iv50_input(), Some(output));
        assert_eq!(decompress_query(&params), Err(IcError::BadFormat));
    }

    #[test]
    fn query_rejects_unknown_output_compression() {
        let output = bitmap_info(header(320, 240, 24, YUY2));
        let params = DecompressParams::from_headers(iv50_input(), Some(output));
        assert_eq!(decompress_query(&params), Err(IcError::BadFormat));
    }

    #[test]
    fn query_rejects_mismatched_output_width() {
        let output = bitmap_info(header(319, 240, 24, BI_RGB));
        let params = DecompressParams::from_headers(iv50_input(), Some(output));
        assert_eq!(decompress_query(&params), Err(IcError::BadParam));
    }

    #[test]
    fn get_format_rejects_non_iv50_input() {
        let input = header(320, 240, 12, YUY2);
        assert_eq!(
            decompress_get_format(&input, false),
            Err(IcError::BadFormat)
        );
        assert_eq!(decompress_get_format(&input, true), Err(IcError::BadFormat));
    }

    #[test]
    fn get_format_keeps_int_min_height() {
        let input = header(320, i32::MIN, 12, IV50);
        assert_eq!(
            decompress_get_format(&input, true),
            Ok(FormatQuery::Output(header(320, i32::MIN, 24, BI_RGB)))
        );
    }

    #[test]
    fn begin_requires_decoder_and_output() {
        let with_output = DecompressParams::from_headers(
            iv50_input(),
            Some(bitmap_info(header(320, 240, 24, BI_RGB))),
        );
        assert_eq!(
            decompress_begin_plan(false, &with_output),
            Err(IcError::BadParam)
        );
        let without_output = DecompressParams::from_headers(iv50_input(), None);
        assert_eq!(
            decompress_begin_plan(true, &without_output),
            Err(IcError::BadParam)
        );
    }

    #[test]
    fn begin_selects_subtype_by_depth_and_compression() {
        let plan = |output: BitmapInfoHeader| {
            let params = DecompressParams::from_headers(iv50_input(), Some(bitmap_info(output)));
            decompress_begin_plan(true, &params).map(|plan| plan.output_subtype)
        };
        assert_eq!(plan(header(320, 240, 32, BI_RGB)), Ok(OutputSubtype::Rgb32));
        assert_eq!(
            plan(header(320, 240, 16, BI_RGB)),
            Ok(OutputSubtype::Rgb555)
        );
        assert_eq!(plan(header(320, 240, 24, YUY2)), Ok(OutputSubtype::Rgb24));
        assert_eq!(
            plan(header(320, 240, 24, BI_BITFIELDS)),
            Ok(OutputSubtype::Rgb565)
        );
        assert_eq!(plan(header(320, 240, 8, BI_RGB)), Err(IcError::BadFormat));
    }

    #[test]
    fn begin_stride_sign_follows_height() {
        let stride_for = |height: i32| {
            let params = DecompressParams::from_headers(
                iv50_input(),
                Some(bitmap_info(header(321, height, 24, BI_RGB))),
            );
            decompress_begin_plan(true, &params).map(|plan| plan.default_stride)
        };
        assert_eq!(stride_for(240), Ok(324u32.wrapping_neg()));
        assert_eq!(stride_for(-240), Ok(324));
    }

    #[test]
    fn process_output_calls_once_on_success() {
        let mut calls = 0;
        let action = process_output(|| {
            calls += 1;
            0
        });
        assert_eq!(action, OutputAction::Copy);
        assert_eq!(calls, 1);
    }

    #[test]
    fn process_output_retries_stream_change_once() {
        let mut replies = [MF_E_TRANSFORM_STREAM_CHANGE, MF_E_TRANSFORM_STREAM_CHANGE].into_iter();
        let mut calls = 0;
        let action = process_output(|| {
            calls += 1;
            replies.next().unwrap()
        });
        assert_eq!(action, OutputAction::Finish(Err(IcError::Internal)));
        assert_eq!(calls, 2);
    }

    #[test]
    fn process_output_need_more_input_finishes_ok() {
        let action = process_output(|| MF_E_TRANSFORM_NEED_MORE_INPUT);
        assert_eq!(action, OutputAction::Finish(Ok(())));
    }

    #[test]
    fn copy_frame_places_rect_at_offset() {
        let dst = header(3, 3, 24, BI_RGB);
        let decoded: Vec<u8> = (1..=6).collect();
        let mut out = vec![0u8; 36];
        let rect = Rect {
            x: 1,
            y: 1,
            dx: 2,
            dy: 1,
        };
        assert_eq!(copy_frame(&dst, rect, &decoded, &mut out), Ok(()));
        assert_eq!(&out[15..21], &[1, 2, 3, 4, 5, 6]);
        assert_eq!(out.iter().filter(|&&b| b != 0).count(), 6);
    }

    #[test]
    fn copy_frame_refuses_bad_rects() {
        let dst = header(3, 3, 24, BI_RGB);
        let decoded = vec![0u8; 36];
        let mut out = vec![0u8; 36];
        let rect = |x, y, dx, dy| Rect { x, y, dx, dy };
        for bad in [
            rect(-1, 0, 3, 1),
            rect(0, -1, 3, 1),
            rect(0, 0, 3, -1),
            rect(0, 0, -1, 1),
        ] {
            assert_eq!(
                copy_frame(&dst, bad, &decoded, &mut out),
                Err(IcError::BadSize)
            );
        }
        let negative_width = header(-3, 3, 24, BI_RGB);
        assert_eq!(
            copy_frame(&negative_width, rect(0, 0, 3, 1), &decoded, &mut out),
            Err(IcError::BadSize)
        );
    }

    #[test]
    fn copy_frame_refuses_short_buffers() {
        let dst = header(3, 2, 24, BI_RGB);
        let full = rect_full(3, 2);
        let mut out = vec![0u8; 24];
        assert_eq!(
            copy_frame(&dst, full, &[0u8; 20], &mut out),
            Err(IcError::BadSize)
        );
        assert_eq!(
            copy_frame(&dst, full, &[0u8; 24], &mut [0u8; 20]),
            Err(IcError::BadSize)
        );
    }

    fn rect_full(dx: i32, dy: i32) -> Rect {
        Rect { x: 0, y: 0, dx, dy }
    }

    #[test]
    fn copy_frame_empty_rect_is_noop() {
        let dst = header(3, 2, 24, BI_RGB);
        let mut out = vec![0xEEu8; 4];
        assert_eq!(copy_frame(&dst, rect_full(3, 0), &[], &mut out), Ok(()));
        assert_eq!(copy_frame(&dst, rect_full(0, 2), &[], &mut out), Ok(()));
        let no_depth = header(3, 2, 0, BI_RGB);
        assert_eq!(
            copy_frame(&no_depth, rect_full(3, 2), &[], &mut out),
            Ok(())
        );
        assert_eq!(out, vec![0xEEu8; 4]);
    }

    #[test]
    fn get_info_refuses_small_buffer() {
        assert_eq!(get_info(true, ICINFO_SIZE - 1), InfoResult::TooSmall);
        assert_eq!(get_info(false, 0), InfoResult::SizeQuery);
    }

    #[test]
    fn open_accepts_missing_icinfo() {
        assert!(open_accepts(None));
        let audc = IcInfo {
            fcc_type: AUDC,
            ..Default::default()
        };
        assert!(!open_accepts(Some(&audc)));
    }

    #[test]
    fn driver_proc_results_per_message() {
        for msg in [
            DRV_OPEN,
            ICM_GETINFO,
            ICM_DECOMPRESS_GET_FORMAT,
            ICM_DECOMPRESS_QUERY,
            ICM_DECOMPRESSEX_QUERY,
            ICM_DECOMPRESS_BEGIN,
            ICM_DECOMPRESSEX_BEGIN,
            ICM_DECOMPRESS,
            ICM_DECOMPRESSEX,
        ] {
            assert_eq!(driver_proc_constant_result(msg), None, "msg {msg:#x}");
        }
        assert_eq!(driver_proc_constant_result(DRV_CLOSE), Some(1));
        assert_eq!(driver_proc_constant_result(ICM_DECOMPRESSEX_END), Some(0));
        for msg in [
            DRV_ENABLE,
            DRV_DISABLE,
            DRV_FREE,
            ICM_COMPRESS_GET_FORMAT,
            ICM_COMPRESS_END,
            ICM_COMPRESS,
            ICM_DECOMPRESS_GET_PALETTE,
            ICM_CONFIGURE,
            0x7777,
        ] {
            assert_eq!(driver_proc_constant_result(msg), Some(ICERR_UNSUPPORTED));
        }
    }

    #[test]
    fn error_codes_match_icerr() {
        assert_eq!(IcError::BadFormat.code(), ICERR_BADFORMAT);
        assert_eq!(IcError::BadParam.code(), ICERR_BADPARAM);
        assert_eq!(IcError::BadSize.code(), ICERR_BADSIZE);
        assert_eq!(IcError::Internal.code(), ICERR_INTERNAL);
        assert_eq!(code_of(Ok(())), ICERR_OK);
        assert_eq!(code_of(Err(IcError::Internal)), ICERR_INTERNAL);
    }
}

#[cfg(test)]
mod oracle_checks;
