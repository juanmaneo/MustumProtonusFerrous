//! Headless port of Wine's `dlls/iyuv_32`, the IYUV video decoder driver.
//!
//! The component exports one entry point, `DriverProc` (`iyuv_32.spec`), so its
//! export is partially ported: the pure parts of its message handlers are ported
//! and the parts that need a live driver instance are skipped.
//!
//! Ported:
//! - `DriverProc` constant results per message ([`driver_proc_constant_result`]).
//! - `IYUV_Open` fourcc gate ([`open_accepts`]).
//! - `IYUV_DecompressQuery` ([`decompress_query`]).
//! - `IYUV_DecompressGetFormat` ([`decompress_get_format`]).
//! - `IYUV_DecompressBegin` validation and output parameters ([`decompress_begin_plan`]).
//! - `IYUV_Decompress` row copy of the converted image, as `MFCopyImage` does ([`decompress_frame`]).
//! - `IYUV_GetInfo` ICINFO fields ([`get_info`]).
//! - The `compare_fourcc` and `make_uint64` helpers.
//!
//! Skipped:
//! - `DllMain`: process attach has no portable logic.
//! - `IYUV_Open` transform creation: needs `IClassFactory_CreateInstance`, a COM object.
//! - `IYUV_DecompressBegin` media types and `SetInputType`/`SetOutputType`: Media Foundation objects.
//! - `IYUV_Decompress` sample and buffer pipeline and I420 to RGB conversion: Media Foundation transform.
//! - `IYUV_GetInfo` `szName`/`szDescription`: `LoadStringW` reads module resources.
//! - `DRV_CLOSE` transform release and `DRV_OPEN` handle return: COM object lifetime.
//! - `DriverProc` dispatch of `driver_id` to the transform: needs the live driver instance.
//! - `ICM_DECOMPRESS_GET_PALETTE`, `ICM_DECOMPRESSEX*`, `ICM_COMPRESS*`, `ICM_CONFIGURE`: FIXME or no-op in C, routed to `ICERR_UNSUPPORTED`.

/// `LRESULT`, the result type of `DriverProc`.
pub type LResult = isize;

pub const ICERR_OK: LResult = 0;
pub const ICERR_UNSUPPORTED: LResult = -1;
pub const ICERR_BADFORMAT: LResult = -2;
pub const ICERR_BADPARAM: LResult = -6;
pub const ICERR_BADSIZE: LResult = -7;

const TRUE: LResult = 1;
const BI_RGB: u32 = 0;
const BITMAPINFOHEADER_SIZE: u32 = 40;
const ICINFO_SIZE: u32 = 568;
const ICVERSION: u32 = 0x0104;

const DRV_USER: u32 = 0x4000;
const ICM_USER: u32 = DRV_USER;
const ICM_RESERVED: u32 = DRV_USER + 0x1000;

const DRV_LOAD: u32 = 0x0001;
const DRV_OPEN: u32 = 0x0003;
const DRV_CLOSE: u32 = 0x0004;
const ICM_GETINFO: u32 = ICM_RESERVED + 2;
const ICM_COMPRESS_QUERY: u32 = ICM_USER + 6;
const ICM_DECOMPRESS_GET_FORMAT: u32 = ICM_USER + 10;
const ICM_DECOMPRESS_QUERY: u32 = ICM_USER + 11;
const ICM_DECOMPRESS_BEGIN: u32 = ICM_USER + 12;
const ICM_DECOMPRESS: u32 = ICM_USER + 13;
const ICM_DECOMPRESS_END: u32 = ICM_USER + 14;

const FOURCC_I420: u32 = u32::from_le_bytes(*b"I420");
const FOURCC_IYUV: u32 = u32::from_le_bytes(*b"IYUV");
const ICTYPE_VIDEO: u32 = u32::from_le_bytes(*b"vidc");

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

/// The fields of `ICINFO` that `IYUV_GetInfo` sets. The name and description are skipped.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct IcInfo {
    pub dw_size: u32,
    pub fcc_type: u32,
    pub fcc_handler: u32,
    pub flags: u32,
    pub version: u32,
    pub version_icm: u32,
}

/// The `ICERR_*` failure codes the handlers can return.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IcError {
    BadFormat,
    BadParam,
    BadSize,
}

impl IcError {
    pub fn code(self) -> LResult {
        match self {
            Self::BadFormat => ICERR_BADFORMAT,
            Self::BadParam => ICERR_BADPARAM,
            Self::BadSize => ICERR_BADSIZE,
        }
    }
}

pub type IcResult<T> = Result<T, IcError>;

/// The RGB output subtype selected by the bit depth of the output header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputSubtype {
    Rgb24,
    Rgb555,
    Rgb8,
}

/// The values `IYUV_DecompressBegin` assigns to its media types, before the
/// media types are handed to the transform.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BeginPlan {
    pub output_subtype: OutputSubtype,
    pub input_frame_size: u64,
    pub output_frame_size: u64,
    pub output_stride: i32,
}

/// Zero when the fourccs match, ignoring the case bit of each byte.
pub fn compare_fourcc(a: u32, b: u32) -> u32 {
    (a ^ b) & !0x2020_2020
}

pub fn make_uint64(high: u32, low: u32) -> u64 {
    (u64::from(high) << 32) | u64::from(low)
}

/// `IYUV_DecompressQuery`: accepts I420 or IYUV input and, when an output header is given,
/// an RGB output of the same size.
pub fn decompress_query(
    input: &BitmapInfoHeader,
    output: Option<&BitmapInfoHeader>,
) -> IcResult<()> {
    let format_ok = compare_fourcc(input.compression, FOURCC_I420) == 0
        || compare_fourcc(input.compression, FOURCC_IYUV) == 0;
    let output_ok = match output {
        None => true,
        Some(out) => {
            out.compression == BI_RGB
                && matches!(out.bit_count, 24 | 16 | 8)
                && out.width == input.width
                && out.height == input.height
        }
    };
    if format_ok && input.width != 0 && input.height != 0 && output_ok {
        Ok(())
    } else {
        Err(IcError::BadFormat)
    }
}

/// `IYUV_DecompressGetFormat`. `Ok(None)` is the size query, where the caller returns
/// `sizeof(BITMAPINFOHEADER)`.
pub fn decompress_get_format(
    input: &BitmapInfoHeader,
    want_output: bool,
) -> IcResult<Option<BitmapInfoHeader>> {
    if compare_fourcc(input.compression, FOURCC_I420) != 0
        && compare_fourcc(input.compression, FOURCC_IYUV) != 0
    {
        return Err(IcError::BadFormat);
    }
    if !want_output {
        return Ok(None);
    }
    let width = input.width;
    let height = input.height.wrapping_abs();
    Ok(Some(BitmapInfoHeader {
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

/// `IYUV_DecompressBegin`: validates the pair of headers and computes the values
/// assigned to the input and output media types.
pub fn decompress_begin_plan(
    has_transform: bool,
    input: &BitmapInfoHeader,
    output: &BitmapInfoHeader,
) -> IcResult<BeginPlan> {
    if !has_transform {
        return Err(IcError::BadParam);
    }
    let subtype = match output.bit_count {
        24 => Some(OutputSubtype::Rgb24),
        16 => Some(OutputSubtype::Rgb555),
        8 => Some(OutputSubtype::Rgb8),
        _ => None,
    };
    let same_size = input.width == output.width && input.height == output.height;
    let output_subtype = match subtype {
        Some(subtype) if output.compression == BI_RGB && same_size => subtype,
        _ => return Err(IcError::BadFormat),
    };
    let depth = i32::from(output.bit_count / 8);
    let stride = output.width.wrapping_mul(depth).wrapping_add(3) & !3;
    Ok(BeginPlan {
        output_subtype,
        input_frame_size: make_uint64(input.width as u32, input.height as u32),
        output_frame_size: make_uint64(output.width as u32, output.height as u32),
        output_stride: stride.wrapping_neg(),
    })
}

/// The copy stage of `IYUV_Decompress`: `src` holds the converted image, which
/// is copied row by row into `dst` with the stride `MFCopyImage` uses.
pub fn decompress_frame(output: &BitmapInfoHeader, src: &[u8], dst: &mut [u8]) -> IcResult<()> {
    let (Ok(width), Ok(lines)) = (
        usize::try_from(output.width),
        usize::try_from(output.height),
    ) else {
        return Err(IcError::BadSize);
    };
    let row = width.saturating_mul(usize::from(output.bit_count / 8));
    let stride = row.saturating_add(3) & !3;
    copy_image(dst, src, stride, row, lines)
}

fn copy_image(
    dst: &mut [u8],
    src: &[u8],
    stride: usize,
    width: usize,
    lines: usize,
) -> IcResult<()> {
    let needed = extent(stride, width, lines);
    if needed > dst.len() || needed > src.len() {
        return Err(IcError::BadSize);
    }
    for line in 0..lines {
        let start = line * stride;
        dst[start..start + width].copy_from_slice(&src[start..start + width]);
    }
    Ok(())
}

/// Bytes spanned by `lines` rows of `width` bytes, `stride` apart.
fn extent(stride: usize, width: usize, lines: usize) -> usize {
    if lines == 0 {
        0
    } else {
        (lines - 1).saturating_mul(stride).saturating_add(width)
    }
}

/// `IYUV_GetInfo` for a caller buffer of `size` bytes. `None` is the `0` return
/// for a buffer that is too small.
pub fn get_info(size: u32) -> Option<IcInfo> {
    if size < ICINFO_SIZE {
        return None;
    }
    Some(IcInfo {
        dw_size: ICINFO_SIZE,
        fcc_type: ICTYPE_VIDEO,
        fcc_handler: FOURCC_IYUV,
        flags: 0,
        version: 0,
        version_icm: ICVERSION,
    })
}

/// The `IYUV_Open` gate: rejects an `ICINFO` whose type is not `vidc`. A missing
/// `ICINFO` is accepted.
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
        | ICM_DECOMPRESS_BEGIN
        | ICM_DECOMPRESS => None,
        ICM_DECOMPRESS_END => Some(ICERR_OK),
        ICM_COMPRESS_QUERY => Some(ICERR_BADFORMAT),
        _ => Some(ICERR_UNSUPPORTED),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const YUY2: u32 = u32::from_le_bytes(*b"YUY2");
    const AUDC: u32 = u32::from_le_bytes(*b"audc");
    const DRV_ENABLE: u32 = 0x0002;
    const DRV_FREE: u32 = 0x0006;
    const ICM_COMPRESS: u32 = ICM_USER + 8;
    const ICM_DECOMPRESSEX_QUERY: u32 = ICM_USER + 61;

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

    fn info(fcc_type: u32) -> IcInfo {
        IcInfo {
            dw_size: ICINFO_SIZE,
            fcc_type,
            ..Default::default()
        }
    }

    fn code_of(result: IcResult<()>) -> LResult {
        match result {
            Ok(()) => ICERR_OK,
            Err(error) => error.code(),
        }
    }

    #[test]
    fn oracle_fourcc_and_uint64() {
        let fourcc_cases = [("V01", u32::from_le_bytes(*b"i420"), FOURCC_I420, 0)];
        for (id, a, b, expected) in fourcc_cases {
            assert_eq!(compare_fourcc(a, b), expected, "{id}");
        }
        let uint64_cases = [("V02", 640, (-480i32) as u32, 0x0000_0280_FFFF_FE20)];
        for (id, high, low, expected) in uint64_cases {
            assert_eq!(make_uint64(high, low), expected, "{id}");
        }
    }

    #[test]
    fn oracle_decompress_query() {
        let cases = [
            ("Q1", header(320, 240, 12, FOURCC_I420), None, ICERR_OK),
            (
                "Q2",
                header(320, 240, 12, FOURCC_IYUV),
                Some(header(320, 240, 24, BI_RGB)),
                ICERR_OK,
            ),
            ("Q3", header(320, 240, 12, YUY2), None, ICERR_BADFORMAT),
            (
                "Q4",
                header(320, 240, 12, FOURCC_I420),
                Some(header(320, 120, 24, BI_RGB)),
                ICERR_BADFORMAT,
            ),
        ];
        for (id, input, output, expected) in cases {
            assert_eq!(
                code_of(decompress_query(&input, output.as_ref())),
                expected,
                "{id}"
            );
        }
    }

    #[test]
    fn oracle_decompress_get_format() {
        let expected_rgb = BitmapInfoHeader {
            size: 40,
            width: 320,
            height: 240,
            planes: 1,
            bit_count: 24,
            compression: BI_RGB,
            size_image: 230400,
            ..Default::default()
        };
        let cases = [
            ("F1", header(320, -240, 12, FOURCC_I420), false, Ok(None)),
            (
                "F2",
                header(320, -240, 12, FOURCC_I420),
                true,
                Ok(Some(expected_rgb)),
            ),
            (
                "F3",
                header(320, 240, 12, YUY2),
                true,
                Err(IcError::BadFormat),
            ),
        ];
        for (id, input, want_output, expected) in cases {
            assert_eq!(decompress_get_format(&input, want_output), expected, "{id}");
        }
    }

    #[test]
    fn oracle_decompress_begin() {
        let i420 = header(320, 240, 12, FOURCC_I420);
        let cases = [
            (
                "B1",
                true,
                i420,
                header(320, 240, 16, BI_RGB),
                Ok(BeginPlan {
                    output_subtype: OutputSubtype::Rgb555,
                    input_frame_size: 0x0000_0140_0000_00F0,
                    output_frame_size: 0x0000_0140_0000_00F0,
                    output_stride: -640,
                }),
            ),
            (
                "B2",
                true,
                header(320, -240, 12, FOURCC_I420),
                header(320, -240, 8, BI_RGB),
                Ok(BeginPlan {
                    output_subtype: OutputSubtype::Rgb8,
                    input_frame_size: 0x0000_0140_FFFF_FF10,
                    output_frame_size: 0x0000_0140_FFFF_FF10,
                    output_stride: -320,
                }),
            ),
            (
                "B3",
                false,
                i420,
                header(320, 240, 24, BI_RGB),
                Err(ICERR_BADPARAM),
            ),
            (
                "B4",
                true,
                i420,
                header(320, 240, 32, BI_RGB),
                Err(ICERR_BADFORMAT),
            ),
        ];
        for (id, has_transform, input, output, expected) in cases {
            assert_eq!(
                decompress_begin_plan(has_transform, &input, &output).map_err(IcError::code),
                expected,
                "{id}"
            );
        }
    }

    #[test]
    fn oracle_decompress_frame() {
        let src: Vec<u8> = (0..24).collect();
        let mut dst = vec![0u8; 24];
        assert_eq!(
            decompress_frame(&header(3, 2, 24, BI_RGB), &src, &mut dst),
            Ok(()),
            "D1"
        );
        let expected: [u8; 24] = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 12, 13, 14, 15, 16, 17, 18, 19, 20, 0, 0, 0,
        ];
        assert_eq!(dst, expected, "D1");

        let mut tiny = [0x5Au8];
        assert_eq!(
            decompress_frame(&header(3, 0, 24, BI_RGB), &[0u8], &mut tiny),
            Ok(()),
            "D2"
        );
        assert_eq!(tiny, [0x5A], "D2");
    }

    #[test]
    fn oracle_get_info() {
        let expected = IcInfo {
            dw_size: 568,
            fcc_type: ICTYPE_VIDEO,
            fcc_handler: FOURCC_IYUV,
            flags: 0,
            version: 0,
            version_icm: ICVERSION,
        };
        let cases = [("I1", 568, Some(expected)), ("I2", 567, None)];
        for (id, size, expected) in cases {
            assert_eq!(get_info(size), expected, "{id}");
        }
    }

    #[test]
    fn oracle_open_gate() {
        let cases = [
            ("O1", None, true),
            ("O2", Some(info(ICTYPE_VIDEO)), true),
            ("O3", Some(info(AUDC)), false),
        ];
        for (id, icinfo, expected) in cases {
            assert_eq!(open_accepts(icinfo.as_ref()), expected, "{id}");
        }
    }

    #[test]
    fn oracle_driver_proc_constant_results() {
        let cases = [
            ("R1", DRV_LOAD, Some(1)),
            ("R2", ICM_DECOMPRESS_END, Some(0)),
            ("R3", ICM_COMPRESS_QUERY, Some(-2)),
            // C returns 568 for a NULL buffer; the port routes ICM_GETINFO to get_info.
            ("R4", ICM_GETINFO, None),
            ("R5", DRV_FREE, Some(-1)),
        ];
        for (id, msg, expected) in cases {
            assert_eq!(driver_proc_constant_result(msg), expected, "{id}");
        }
    }

    #[test]
    fn query_rejects_invalid_headers() {
        let cases = [
            ("zero input height", header(320, 0, 12, FOURCC_I420), None),
            ("zero input width", header(0, 240, 12, FOURCC_I420), None),
            (
                "output bit depth 32",
                header(320, 240, 12, FOURCC_I420),
                Some(header(320, 240, 32, BI_RGB)),
            ),
            (
                "output compression",
                header(320, 240, 12, FOURCC_I420),
                Some(header(320, 240, 24, YUY2)),
            ),
            (
                "output width",
                header(320, 240, 12, FOURCC_I420),
                Some(header(321, 240, 24, BI_RGB)),
            ),
        ];
        for (name, input, output) in cases {
            assert_eq!(
                decompress_query(&input, output.as_ref()),
                Err(IcError::BadFormat),
                "{name}"
            );
        }
    }

    #[test]
    fn begin_rejects_mismatched_output() {
        let input = header(320, 240, 12, FOURCC_I420);
        let cases = [
            ("output compression", header(320, 240, 16, YUY2)),
            ("output width", header(321, 240, 16, BI_RGB)),
            ("output height", header(320, 239, 16, BI_RGB)),
        ];
        for (name, output) in cases {
            assert_eq!(
                decompress_begin_plan(true, &input, &output),
                Err(IcError::BadFormat),
                "{name}"
            );
        }
    }

    #[test]
    fn begin_accepts_rgb24_output() {
        // Oracle B5 (checked outside the 25-row table): RGB24, stride -960.
        let input = header(320, 240, 12, FOURCC_I420);
        let output = header(320, 240, 24, BI_RGB);
        assert_eq!(
            decompress_begin_plan(true, &input, &output),
            Ok(BeginPlan {
                output_subtype: OutputSubtype::Rgb24,
                input_frame_size: 0x0000_0140_0000_00F0,
                output_frame_size: 0x0000_0140_0000_00F0,
                output_stride: -960,
            })
        );
    }

    #[test]
    fn decompress_frame_checks_bounds() {
        let src = [0u8; 24];
        let cases = [
            ("negative width", header(-3, 2, 24, BI_RGB), 24, 24),
            ("negative height", header(3, -2, 24, BI_RGB), 24, 24),
            ("dst too small", header(3, 2, 24, BI_RGB), 24, 20),
            ("src too short", header(3, 2, 24, BI_RGB), 20, 24),
        ];
        for (name, output, src_len, dst_len) in cases {
            let mut dst = vec![0u8; dst_len];
            assert_eq!(
                code_of(decompress_frame(&output, &src[..src_len], &mut dst)),
                ICERR_BADSIZE,
                "{name}"
            );
        }

        // The last row is not padded, so 3 * 3 bytes per row over 2 rows is enough.
        let mut dst = vec![0u8; 21];
        assert_eq!(
            decompress_frame(&header(3, 2, 24, BI_RGB), &src[..21], &mut dst),
            Ok(()),
        );
    }

    #[test]
    fn driver_proc_routes_other_messages() {
        let cases = [
            ("DRV_CLOSE", DRV_CLOSE, Some(TRUE)),
            ("DRV_OPEN", DRV_OPEN, None),
            ("ICM_DECOMPRESS_GET_FORMAT", ICM_DECOMPRESS_GET_FORMAT, None),
            ("ICM_DECOMPRESS_QUERY", ICM_DECOMPRESS_QUERY, None),
            ("ICM_DECOMPRESS_BEGIN", ICM_DECOMPRESS_BEGIN, None),
            ("ICM_DECOMPRESS", ICM_DECOMPRESS, None),
            ("DRV_ENABLE", DRV_ENABLE, Some(ICERR_UNSUPPORTED)),
            ("ICM_COMPRESS", ICM_COMPRESS, Some(ICERR_UNSUPPORTED)),
            (
                "ICM_DECOMPRESSEX_QUERY",
                ICM_DECOMPRESSEX_QUERY,
                Some(ICERR_UNSUPPORTED),
            ),
        ];
        for (name, msg, expected) in cases {
            assert_eq!(driver_proc_constant_result(msg), expected, "{name}");
        }
    }
}
