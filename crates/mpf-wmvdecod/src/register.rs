//! Port of `DllRegisterServer` and `DllUnregisterServer` in `dlls/wmvdecod/wmvdecod.c`, with their
//! MFT and DMO registration tables. The OS calls (`__wine_register_resources`, `MFTRegister`, ...) go
//! through [`Registrar`].

use crate::guid::{
    Guid, CLSID_WMVDECODER_MFT, DMOCATEGORY_VIDEO_DECODER, DMOVIDEOFORMAT_RGB24,
    DMOVIDEOFORMAT_RGB32, DMOVIDEOFORMAT_RGB555, DMOVIDEOFORMAT_RGB565, DMOVIDEOFORMAT_RGB8,
    MEDIASUBTYPE_NV11, MEDIASUBTYPE_NV12, MEDIASUBTYPE_RGB24, MEDIASUBTYPE_RGB32,
    MEDIASUBTYPE_RGB555, MEDIASUBTYPE_RGB565, MEDIASUBTYPE_RGB8, MEDIASUBTYPE_UYVY,
    MEDIASUBTYPE_VC1S, MEDIASUBTYPE_WMV1, MEDIASUBTYPE_WMV2, MEDIASUBTYPE_WMV3, MEDIASUBTYPE_WMVA,
    MEDIASUBTYPE_WMVP, MEDIASUBTYPE_WMVR, MEDIASUBTYPE_WVC1, MEDIASUBTYPE_WVP2, MEDIASUBTYPE_YUY2,
    MEDIASUBTYPE_YV12, MEDIASUBTYPE_YVYU, MEDIATYPE_VIDEO, MFT_CATEGORY_VIDEO_DECODER,
};
use crate::hresult::{failed, Hresult, S_OK};

/// `MFT_ENUM_FLAG_SYNCMFT`, from `include/mfapi.h`.
pub const MFT_ENUM_FLAG_SYNCMFT: u32 = 0x0000_0001;

/// The name `DllRegisterServer` gives the MFT.
pub const MFT_NAME: &str = "WMVideo Decoder MFT";

/// The name `DllRegisterServer` gives the DMO.
pub const DMO_NAME: &str = "WMVideo Decoder DMO";

/// `MFT_REGISTER_TYPE_INFO`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MediaType {
    pub major: Guid,
    pub subtype: Guid,
}

/// `DMO_PARTIAL_MEDIATYPE`. The C field `type` is `media_type` here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DmoMediaType {
    pub media_type: Guid,
    pub subtype: Guid,
}

/// `wmv_decoder_mft_inputs`, in C order.
pub const MFT_INPUT_TYPES: [MediaType; 9] = [
    MediaType {
        major: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_WMV1,
    },
    MediaType {
        major: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_WMV2,
    },
    MediaType {
        major: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_WMV3,
    },
    MediaType {
        major: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_WMVP,
    },
    MediaType {
        major: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_WVP2,
    },
    MediaType {
        major: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_WMVR,
    },
    MediaType {
        major: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_WMVA,
    },
    MediaType {
        major: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_WVC1,
    },
    MediaType {
        major: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_VC1S,
    },
];

/// `wmv_decoder_mft_outputs`, in C order.
pub const MFT_OUTPUT_TYPES: [MediaType; 11] = [
    MediaType {
        major: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_YV12,
    },
    MediaType {
        major: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_YUY2,
    },
    MediaType {
        major: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_UYVY,
    },
    MediaType {
        major: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_YVYU,
    },
    MediaType {
        major: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_NV11,
    },
    MediaType {
        major: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_NV12,
    },
    MediaType {
        major: MEDIATYPE_VIDEO,
        subtype: DMOVIDEOFORMAT_RGB32,
    },
    MediaType {
        major: MEDIATYPE_VIDEO,
        subtype: DMOVIDEOFORMAT_RGB24,
    },
    MediaType {
        major: MEDIATYPE_VIDEO,
        subtype: DMOVIDEOFORMAT_RGB565,
    },
    MediaType {
        major: MEDIATYPE_VIDEO,
        subtype: DMOVIDEOFORMAT_RGB555,
    },
    MediaType {
        major: MEDIATYPE_VIDEO,
        subtype: DMOVIDEOFORMAT_RGB8,
    },
];

/// `wmv_decoder_dmo_inputs`, in C order.
pub const DMO_INPUT_TYPES: [DmoMediaType; 8] = [
    DmoMediaType {
        media_type: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_WMV1,
    },
    DmoMediaType {
        media_type: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_WMV2,
    },
    DmoMediaType {
        media_type: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_WMV3,
    },
    DmoMediaType {
        media_type: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_WMVA,
    },
    DmoMediaType {
        media_type: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_WVC1,
    },
    DmoMediaType {
        media_type: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_WMVP,
    },
    DmoMediaType {
        media_type: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_WVP2,
    },
    DmoMediaType {
        media_type: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_VC1S,
    },
];

/// `wmv_decoder_dmo_outputs`, in C order.
pub const DMO_OUTPUT_TYPES: [DmoMediaType; 11] = [
    DmoMediaType {
        media_type: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_YV12,
    },
    DmoMediaType {
        media_type: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_YUY2,
    },
    DmoMediaType {
        media_type: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_UYVY,
    },
    DmoMediaType {
        media_type: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_YVYU,
    },
    DmoMediaType {
        media_type: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_NV11,
    },
    DmoMediaType {
        media_type: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_NV12,
    },
    DmoMediaType {
        media_type: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_RGB32,
    },
    DmoMediaType {
        media_type: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_RGB24,
    },
    DmoMediaType {
        media_type: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_RGB565,
    },
    DmoMediaType {
        media_type: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_RGB555,
    },
    DmoMediaType {
        media_type: MEDIATYPE_VIDEO,
        subtype: MEDIASUBTYPE_RGB8,
    },
];

/// The registration calls that `DllRegisterServer` and `DllUnregisterServer` make. Each returns the
/// HRESULT of the OS call.
pub trait Registrar {
    fn register_resources(&mut self) -> Hresult;
    fn unregister_resources(&mut self) -> Hresult;
    fn mft_register(
        &mut self,
        clsid: &Guid,
        category: &Guid,
        name: &str,
        flags: u32,
        inputs: &[MediaType],
        outputs: &[MediaType],
    ) -> Hresult;
    fn mft_unregister(&mut self, clsid: &Guid) -> Hresult;
    fn dmo_register(
        &mut self,
        name: &str,
        clsid: &Guid,
        category: &Guid,
        flags: u32,
        inputs: &[DmoMediaType],
        outputs: &[DmoMediaType],
    ) -> Hresult;
    fn dmo_unregister(&mut self, clsid: &Guid, category: &Guid) -> Hresult;
}

/// `DllRegisterServer`. Each step runs only if the one before it did not fail, and a failure is returned as is.
pub fn dll_register_server(registrar: &mut impl Registrar) -> Hresult {
    let hr = registrar.register_resources();
    if failed(hr) {
        return hr;
    }
    let hr = registrar.mft_register(
        &CLSID_WMVDECODER_MFT,
        &MFT_CATEGORY_VIDEO_DECODER,
        MFT_NAME,
        MFT_ENUM_FLAG_SYNCMFT,
        &MFT_INPUT_TYPES,
        &MFT_OUTPUT_TYPES,
    );
    if failed(hr) {
        return hr;
    }
    let hr = registrar.dmo_register(
        DMO_NAME,
        &CLSID_WMVDECODER_MFT,
        &DMOCATEGORY_VIDEO_DECODER,
        0,
        &DMO_INPUT_TYPES,
        &DMO_OUTPUT_TYPES,
    );
    if failed(hr) {
        return hr;
    }
    S_OK
}

/// `DllUnregisterServer`, with the same early return as [`dll_register_server`].
pub fn dll_unregister_server(registrar: &mut impl Registrar) -> Hresult {
    let hr = registrar.unregister_resources();
    if failed(hr) {
        return hr;
    }
    let hr = registrar.mft_unregister(&CLSID_WMVDECODER_MFT);
    if failed(hr) {
        return hr;
    }
    let hr = registrar.dmo_unregister(&CLSID_WMVDECODER_MFT, &DMOCATEGORY_VIDEO_DECODER);
    if failed(hr) {
        return hr;
    }
    S_OK
}
