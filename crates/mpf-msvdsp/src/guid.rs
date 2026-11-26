//! GUID values for the registration tables. The CLSIDs and categories are the msvdsp.c and
//! msvdsp.idl values; the media types are the Wine header values named on each item.

/// A COM `GUID`, laid out like Wine's `GUID` struct.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// The `0000xxxx-0000-0010-8000-00aa00389b71` family of `MFMediaType_*`, `MFVideoFormat_*` and
/// `MEDIASUBTYPE_*` values, with the FourCC or type code in `data1`.
pub(crate) const fn mf_guid(data1: u32) -> Guid {
    Guid {
        data1,
        data2: 0x0000,
        data3: 0x0010,
        data4: [0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71],
    }
}

/// The `xxxxxxxx-524f-11ce-9f53-0020af0ba770` family of `DMOVideoFormat_*` and `MEDIASUBTYPE_RGB*`.
pub(crate) const fn dmo_guid(data1: u32) -> Guid {
    Guid {
        data1,
        data2: 0x524f,
        data3: 0x11ce,
        data4: [0x9f, 0x53, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70],
    }
}

/// `CLSID_CFrameRateConvertDmo` (msvdsp.idl).
pub const CLSID_C_FRAME_RATE_CONVERT_DMO: Guid = Guid {
    data1: 0x01f3_6ce2,
    data2: 0x0907,
    data3: 0x4d8b,
    data4: [0x97, 0x9d, 0xf1, 0x51, 0xbe, 0x91, 0xc8, 0x83],
};

/// `CLSID_CColorControlDmo` (msvdsp.idl).
pub const CLSID_C_COLOR_CONTROL_DMO: Guid = Guid {
    data1: 0x7980_59f0,
    data2: 0x89ca,
    data3: 0x4160,
    data4: [0xb3, 0x25, 0xae, 0xb4, 0x8e, 0xfe, 0x4f, 0x9a],
};

/// `MFT_CATEGORY_VIDEO_EFFECT` (include/mfapi.h:368).
pub const MFT_CATEGORY_VIDEO_EFFECT: Guid = Guid {
    data1: 0x12e1_7c21,
    data2: 0x532c,
    data3: 0x4a6e,
    data4: [0x8a, 0x1c, 0x40, 0x82, 0x5a, 0x73, 0x63, 0x97],
};

/// `DMOCATEGORY_VIDEO_EFFECT` (include/dmoreg.h:63).
pub const DMOCATEGORY_VIDEO_EFFECT: Guid = Guid {
    data1: 0xd990_ee14,
    data2: 0x776c,
    data3: 0x4723,
    data4: [0xbe, 0x46, 0x3d, 0xa2, 0xf5, 0x6f, 0x10, 0xb9],
};

/// `MFMediaType_Video` (include/mfapi.h:392).
pub const MF_MEDIA_TYPE_VIDEO: Guid = mf_guid(u32::from_le_bytes(*b"vids"));

/// `MEDIATYPE_Video` (include/uuids.h:417), the DirectShow spelling of the same value.
pub const MEDIATYPE_VIDEO: Guid = mf_guid(u32::from_le_bytes(*b"vids"));

/// `MFVideoFormat_*` FourCC subtypes used by the MFT tables (include/mfapi.h:94-132).
pub const MF_VIDEO_FORMAT_YV12: Guid = mf_guid(u32::from_le_bytes(*b"YV12"));
pub const MF_VIDEO_FORMAT_YUY2: Guid = mf_guid(u32::from_le_bytes(*b"YUY2"));
pub const MF_VIDEO_FORMAT_UYVY: Guid = mf_guid(u32::from_le_bytes(*b"UYVY"));
pub const MF_VIDEO_FORMAT_AYUV: Guid = mf_guid(u32::from_le_bytes(*b"AYUV"));
pub const MF_VIDEO_FORMAT_NV12: Guid = mf_guid(u32::from_le_bytes(*b"NV12"));
pub const MF_VIDEO_FORMAT_I420: Guid = mf_guid(u32::from_le_bytes(*b"I420"));
pub const MF_VIDEO_FORMAT_IYUV: Guid = mf_guid(u32::from_le_bytes(*b"IYUV"));
pub const MF_VIDEO_FORMAT_YVYU: Guid = mf_guid(u32::from_le_bytes(*b"YVYU"));
pub const MF_VIDEO_FORMAT_NV11: Guid = mf_guid(u32::from_le_bytes(*b"NV11"));
pub const MF_VIDEO_FORMAT_Y41P: Guid = mf_guid(u32::from_le_bytes(*b"Y41P"));
pub const MF_VIDEO_FORMAT_Y41T: Guid = mf_guid(u32::from_le_bytes(*b"Y41T"));
pub const MF_VIDEO_FORMAT_Y42T: Guid = mf_guid(u32::from_le_bytes(*b"Y42T"));
pub const MF_VIDEO_FORMAT_YVU9: Guid = mf_guid(u32::from_le_bytes(*b"YVU9"));

/// `DMOVideoFormat_RGB*` used by the MFT tables. `data1` is the D3DFMT code (include/d3d9types.h).
pub const DMO_VIDEO_FORMAT_RGB32: Guid = dmo_guid(22);
pub const DMO_VIDEO_FORMAT_RGB565: Guid = dmo_guid(23);
pub const DMO_VIDEO_FORMAT_RGB24: Guid = dmo_guid(20);
pub const DMO_VIDEO_FORMAT_RGB555: Guid = dmo_guid(24);
pub const DMO_VIDEO_FORMAT_RGB8: Guid = dmo_guid(41);

/// `MEDIASUBTYPE_*` used by the DMO tables and, for V216 and V410, the MFT tables. The FourCC
/// values are from include/uuids.h:281-391 and include/wmcodecdsp.idl:43-51.
pub const MEDIASUBTYPE_V216: Guid = mf_guid(u32::from_le_bytes(*b"V216"));
pub const MEDIASUBTYPE_V410: Guid = mf_guid(u32::from_le_bytes(*b"V410"));
pub const MEDIASUBTYPE_YV12: Guid = mf_guid(u32::from_le_bytes(*b"YV12"));
pub const MEDIASUBTYPE_YUY2: Guid = mf_guid(u32::from_le_bytes(*b"YUY2"));
pub const MEDIASUBTYPE_UYVY: Guid = mf_guid(u32::from_le_bytes(*b"UYVY"));
pub const MEDIASUBTYPE_AYUV: Guid = mf_guid(u32::from_le_bytes(*b"AYUV"));
pub const MEDIASUBTYPE_NV12: Guid = mf_guid(u32::from_le_bytes(*b"NV12"));
pub const MEDIASUBTYPE_I420: Guid = mf_guid(u32::from_le_bytes(*b"I420"));
pub const MEDIASUBTYPE_IYUV: Guid = mf_guid(u32::from_le_bytes(*b"IYUV"));
pub const MEDIASUBTYPE_YVYU: Guid = mf_guid(u32::from_le_bytes(*b"YVYU"));
pub const MEDIASUBTYPE_NV11: Guid = mf_guid(u32::from_le_bytes(*b"NV11"));
pub const MEDIASUBTYPE_Y41P: Guid = mf_guid(u32::from_le_bytes(*b"Y41P"));
pub const MEDIASUBTYPE_Y41T: Guid = mf_guid(u32::from_le_bytes(*b"Y41T"));
pub const MEDIASUBTYPE_Y42T: Guid = mf_guid(u32::from_le_bytes(*b"Y42T"));
pub const MEDIASUBTYPE_YVU9: Guid = mf_guid(u32::from_le_bytes(*b"YVU9"));

/// `MEDIASUBTYPE_RGB*` used by the DMO tables (include/uuids.h:361-367).
pub const MEDIASUBTYPE_RGB32: Guid = dmo_guid(0xe436_eb7e);
pub const MEDIASUBTYPE_RGB565: Guid = dmo_guid(0xe436_eb7b);
pub const MEDIASUBTYPE_RGB24: Guid = dmo_guid(0xe436_eb7d);
pub const MEDIASUBTYPE_RGB555: Guid = dmo_guid(0xe436_eb7c);
pub const MEDIASUBTYPE_RGB8: Guid = dmo_guid(0xe436_eb7a);
