//! `GUID` values and the helpers that build them, from `include/guiddef.h`, `include/mfapi.h`,
//! `include/uuids.h`, `include/wmcodecdsp.idl`, `include/dmoreg.h`, `include/d3d9types.h`,
//! `include/unknwn.idl`, `include/mfidl.idl` and `dlls/wmvdecod/video_decoder.c`.

/// A `GUID`, with the field layout of Wine's structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl Guid {
    pub const fn new(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Guid {
        Guid {
            data1,
            data2,
            data3,
            data4,
        }
    }
}

/// `IsEqualGUID`: all 16 bytes must match.
pub fn is_equal_guid(a: &Guid, b: &Guid) -> bool {
    a == b
}

/// `DEFINE_MEDIATYPE_GUID`: a FOURCC in the Media Foundation base GUID.
pub const fn media_type_guid(fourcc: u32) -> Guid {
    Guid::new(
        fourcc,
        0x0000,
        0x0010,
        [0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71],
    )
}

/// A `GUID` in the `524f11ce-...` family used by the DMO video formats and `MEDIASUBTYPE_RGB*`.
pub const fn rgb_guid(data1: u32) -> Guid {
    Guid::new(
        data1,
        0x524f,
        0x11ce,
        [0x9f, 0x53, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70],
    )
}

/// `IClassFactory`, from `include/unknwn.idl`.
pub const IID_ICLASSFACTORY: Guid = Guid::new(
    0x0000_0001,
    0x0000,
    0x0000,
    [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
);

/// `IUnknown`, from `include/unknwn.idl`.
pub const IID_IUNKNOWN: Guid = Guid::new(
    0x0000_0000,
    0x0000,
    0x0000,
    [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
);

/// `CLSID_WMVDecoderMFT`, from `include/mfidl.idl`.
pub const CLSID_WMVDECODER_MFT: Guid = Guid::new(
    0x82d3_53df,
    0x90bd,
    0x4382,
    [0x8b, 0xc2, 0x3f, 0x61, 0x92, 0xb7, 0x6e, 0x34],
);

/// `CLSID_wg_wmv_decoder`, from `dlls/wmvdecod/video_decoder.c`.
pub const CLSID_WG_WMV_DECODER: Guid = Guid::new(
    0x62ee_5ddb,
    0x4f52,
    0x48e2,
    [0x89, 0x28, 0x78, 0x7b, 0x02, 0x53, 0xa0, 0xbc],
);

/// `CLSID_wg_h264_decoder`, from `dlls/wmvdecod/video_decoder.c`.
pub const CLSID_WG_H264_DECODER: Guid = Guid::new(
    0x1f1e_273d,
    0x12c0,
    0x4b3a,
    [0x8e, 0x9b, 0x19, 0x33, 0xc2, 0x49, 0x8a, 0xea],
);

/// `MFT_CATEGORY_VIDEO_DECODER`, from `include/mfapi.h`.
pub const MFT_CATEGORY_VIDEO_DECODER: Guid = Guid::new(
    0xd6c0_2d4b,
    0x6833,
    0x45b4,
    [0x97, 0x1a, 0x05, 0xa4, 0xb0, 0x4b, 0xab, 0x91],
);

/// `DMOCATEGORY_VIDEO_DECODER`, from `include/dmoreg.h`.
pub const DMOCATEGORY_VIDEO_DECODER: Guid = Guid::new(
    0x4a69_b442,
    0x28be,
    0x4991,
    [0x96, 0x9c, 0xb5, 0x00, 0xad, 0xf5, 0xd8, 0xa8],
);

/// `MFMediaType_Video` and `MEDIATYPE_Video`, from `include/mfapi.h` and `include/uuids.h`.
pub const MEDIATYPE_VIDEO: Guid = media_type_guid(0x7364_6976);

/// `MEDIASUBTYPE_WMV1` and `MFVideoFormat_WMV1`. The FOURCC values of the `MFVideoFormat_*` and
/// `MEDIASUBTYPE_*` names used here are equal, so one constant serves both tables.
pub const MEDIASUBTYPE_WMV1: Guid = media_type_guid(0x3156_4d57);
/// `MEDIASUBTYPE_WMV2` and `MFVideoFormat_WMV2`.
pub const MEDIASUBTYPE_WMV2: Guid = media_type_guid(0x3256_4d57);
/// `MEDIASUBTYPE_WMV3` and `MFVideoFormat_WMV3`.
pub const MEDIASUBTYPE_WMV3: Guid = media_type_guid(0x3356_4d57);
/// `MEDIASUBTYPE_WMVA`.
pub const MEDIASUBTYPE_WMVA: Guid = media_type_guid(0x4156_4d57);
/// `MEDIASUBTYPE_WMVP`.
pub const MEDIASUBTYPE_WMVP: Guid = media_type_guid(0x5056_4d57);
/// `MEDIASUBTYPE_WMVR`.
pub const MEDIASUBTYPE_WMVR: Guid = media_type_guid(0x5256_4d57);
/// `MEDIASUBTYPE_WVP2`.
pub const MEDIASUBTYPE_WVP2: Guid = media_type_guid(0x3250_5657);
/// `MEDIASUBTYPE_WVC1` and `MFVideoFormat_WVC1`.
pub const MEDIASUBTYPE_WVC1: Guid = media_type_guid(0x3143_5657);
/// `MEDIASUBTYPE_VC1S`, which `video_decoder.c` defines with `MAKEFOURCC('V','C','1','S')`.
pub const MEDIASUBTYPE_VC1S: Guid = media_type_guid(0x5331_4356);
/// `MEDIASUBTYPE_YV12` and `MFVideoFormat_YV12`.
pub const MEDIASUBTYPE_YV12: Guid = media_type_guid(0x3231_5659);
/// `MEDIASUBTYPE_YUY2` and `MFVideoFormat_YUY2`.
pub const MEDIASUBTYPE_YUY2: Guid = media_type_guid(0x3259_5559);
/// `MEDIASUBTYPE_UYVY` and `MFVideoFormat_UYVY`.
pub const MEDIASUBTYPE_UYVY: Guid = media_type_guid(0x5956_5955);
/// `MEDIASUBTYPE_YVYU` and `MFVideoFormat_YVYU`.
pub const MEDIASUBTYPE_YVYU: Guid = media_type_guid(0x5559_5659);
/// `MEDIASUBTYPE_NV11` and `MFVideoFormat_NV11`.
pub const MEDIASUBTYPE_NV11: Guid = media_type_guid(0x3131_564e);
/// `MEDIASUBTYPE_NV12` and `MFVideoFormat_NV12`.
pub const MEDIASUBTYPE_NV12: Guid = media_type_guid(0x3231_564e);

/// `DMOVideoFormat_RGB32` (`D3DFMT_X8R8G8B8` = 22), from `dlls/wmvdecod/wmvdecod.c`.
pub const DMOVIDEOFORMAT_RGB32: Guid = rgb_guid(22);
/// `DMOVideoFormat_RGB24` (`D3DFMT_R8G8B8` = 20).
pub const DMOVIDEOFORMAT_RGB24: Guid = rgb_guid(20);
/// `DMOVideoFormat_RGB565` (`D3DFMT_R5G6B5` = 23).
pub const DMOVIDEOFORMAT_RGB565: Guid = rgb_guid(23);
/// `DMOVideoFormat_RGB555` (`D3DFMT_X1R5G5B5` = 24).
pub const DMOVIDEOFORMAT_RGB555: Guid = rgb_guid(24);
/// `DMOVideoFormat_RGB8` (`D3DFMT_P8` = 41).
pub const DMOVIDEOFORMAT_RGB8: Guid = rgb_guid(41);

/// `MEDIASUBTYPE_RGB32`, from `include/uuids.h`.
pub const MEDIASUBTYPE_RGB32: Guid = rgb_guid(0xe436_eb7e);
/// `MEDIASUBTYPE_RGB24`, from `include/uuids.h`.
pub const MEDIASUBTYPE_RGB24: Guid = rgb_guid(0xe436_eb7d);
/// `MEDIASUBTYPE_RGB565`, from `include/uuids.h`.
pub const MEDIASUBTYPE_RGB565: Guid = rgb_guid(0xe436_eb7b);
/// `MEDIASUBTYPE_RGB555`, from `include/uuids.h`.
pub const MEDIASUBTYPE_RGB555: Guid = rgb_guid(0xe436_eb7c);
/// `MEDIASUBTYPE_RGB8`, from `include/uuids.h`.
pub const MEDIASUBTYPE_RGB8: Guid = rgb_guid(0xe436_eb7a);
