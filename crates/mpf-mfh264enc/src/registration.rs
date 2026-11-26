//! `DllRegisterServer` and `DllUnregisterServer` from `dlls/mfh264enc/mfh264enc.c`, and the
//! `MFTRegister` table of the H.264 encoder MFT.

use crate::guid::{Guid, CLSID_MSH264_ENCODER_MFT};
use crate::hresult::{failed, Hresult, S_OK};

/// `MFT_ENUM_FLAG_SYNCMFT`, from `include/mfapi.h`.
pub const MFT_ENUM_FLAG_SYNCMFT: u32 = 0x0000_0001;

/// `MFT_CATEGORY_VIDEO_ENCODER`, from `include/mfapi.h`.
pub const MFT_CATEGORY_VIDEO_ENCODER: Guid = Guid {
    data1: 0xf79e_ac7d,
    data2: 0xe545,
    data3: 0x4387,
    data4: [0xBD, 0xEE, 0xD6, 0x47, 0xD7, 0xBD, 0xE4, 0x2A],
};

/// `MFMediaType_Video`, from `include/mfapi.h`.
pub const MF_MEDIA_TYPE_VIDEO: Guid = Guid {
    data1: 0x7364_6976,
    data2: 0x0000,
    data3: 0x0010,
    data4: [0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B, 0x71],
};

/// `MFVideoFormat_IYUV`, a `MAKEFOURCC('I', 'Y', 'U', 'V')` GUID from `include/mfapi.h`.
pub const MF_VIDEO_FORMAT_IYUV: Guid = Guid {
    data1: u32::from_le_bytes(*b"IYUV"),
    data2: 0x0000,
    data3: 0x0010,
    data4: [0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B, 0x71],
};

/// `MFVideoFormat_YV12`, a `MAKEFOURCC` GUID from `include/mfapi.h`.
pub const MF_VIDEO_FORMAT_YV12: Guid = Guid {
    data1: u32::from_le_bytes(*b"YV12"),
    data2: 0x0000,
    data3: 0x0010,
    data4: [0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B, 0x71],
};

/// `MFVideoFormat_NV12`, a `MAKEFOURCC` GUID from `include/mfapi.h`.
pub const MF_VIDEO_FORMAT_NV12: Guid = Guid {
    data1: u32::from_le_bytes(*b"NV12"),
    data2: 0x0000,
    data3: 0x0010,
    data4: [0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B, 0x71],
};

/// `MFVideoFormat_YUY2`, a `MAKEFOURCC` GUID from `include/mfapi.h`.
pub const MF_VIDEO_FORMAT_YUY2: Guid = Guid {
    data1: u32::from_le_bytes(*b"YUY2"),
    data2: 0x0000,
    data3: 0x0010,
    data4: [0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B, 0x71],
};

/// `MFVideoFormat_H264`, a `MAKEFOURCC` GUID from `include/mfapi.h`.
pub const MF_VIDEO_FORMAT_H264: Guid = Guid {
    data1: u32::from_le_bytes(*b"H264"),
    data2: 0x0000,
    data3: 0x0010,
    data4: [0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B, 0x71],
};

/// `MFT_REGISTER_TYPE_INFO`, from `include/mfobjects.idl`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MftRegisterTypeInfo {
    pub major_type: Guid,
    pub subtype: Guid,
}

/// The arguments of the `MFTRegister` call in `DllRegisterServer`. The `attributes` argument is always NULL,
/// so it is not modelled.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MftRegistration {
    pub clsid: Guid,
    pub category: Guid,
    pub name: &'static str,
    pub flags: u32,
    pub inputs: &'static [MftRegisterTypeInfo],
    pub outputs: &'static [MftRegisterTypeInfo],
}

/// `h264_encoder_mft_inputs`, in C order.
const H264_ENCODER_MFT_INPUTS: &[MftRegisterTypeInfo] = &[
    MftRegisterTypeInfo {
        major_type: MF_MEDIA_TYPE_VIDEO,
        subtype: MF_VIDEO_FORMAT_IYUV,
    },
    MftRegisterTypeInfo {
        major_type: MF_MEDIA_TYPE_VIDEO,
        subtype: MF_VIDEO_FORMAT_YV12,
    },
    MftRegisterTypeInfo {
        major_type: MF_MEDIA_TYPE_VIDEO,
        subtype: MF_VIDEO_FORMAT_NV12,
    },
    MftRegisterTypeInfo {
        major_type: MF_MEDIA_TYPE_VIDEO,
        subtype: MF_VIDEO_FORMAT_YUY2,
    },
];

/// `h264_encoder_mft_outputs`, in C order.
const H264_ENCODER_MFT_OUTPUTS: &[MftRegisterTypeInfo] = &[MftRegisterTypeInfo {
    major_type: MF_MEDIA_TYPE_VIDEO,
    subtype: MF_VIDEO_FORMAT_H264,
}];

/// The `MFTRegister` arguments that `DllRegisterServer` passes.
pub const H264_ENCODER_REGISTRATION: MftRegistration = MftRegistration {
    clsid: CLSID_MSH264_ENCODER_MFT,
    category: MFT_CATEGORY_VIDEO_ENCODER,
    name: "H264 Encoder MFT",
    flags: MFT_ENUM_FLAG_SYNCMFT,
    inputs: H264_ENCODER_MFT_INPUTS,
    outputs: H264_ENCODER_MFT_OUTPUTS,
};

/// `DllRegisterServer`. `register_resources` is `__wine_register_resources`, and `mft_register` is
/// `MFTRegister` given [`H264_ENCODER_REGISTRATION`]. A failed step returns its own `HRESULT` and skips the
/// rest. Otherwise the result is `S_OK`.
pub fn dll_register_server(
    register_resources: impl FnOnce() -> Hresult,
    mft_register: impl FnOnce(&MftRegistration) -> Hresult,
) -> Hresult {
    let hr = register_resources();
    if failed(hr) {
        return hr;
    }
    let hr = mft_register(&H264_ENCODER_REGISTRATION);
    if failed(hr) {
        return hr;
    }
    S_OK
}

/// `DllUnregisterServer`. `unregister_resources` is `__wine_unregister_resources`, and `mft_unregister` is
/// `MFTUnregister` given `CLSID_MSH264EncoderMFT`. The failure handling is the same as in
/// [`dll_register_server`].
pub fn dll_unregister_server(
    unregister_resources: impl FnOnce() -> Hresult,
    mft_unregister: impl FnOnce(&Guid) -> Hresult,
) -> Hresult {
    let hr = unregister_resources();
    if failed(hr) {
        return hr;
    }
    let hr = mft_unregister(&CLSID_MSH264_ENCODER_MFT);
    if failed(hr) {
        return hr;
    }
    S_OK
}
