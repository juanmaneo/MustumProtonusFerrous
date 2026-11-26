//! `DllRegisterServer` and `DllUnregisterServer` from msvdsp.c: the registration tables and the
//! order of the OS calls. The calls themselves go through [`Registrar`].

use crate::guid::*;
use crate::hresult::{checked, HResult, S_OK};

/// `MFT_ENUM_FLAG_SYNCMFT` (include/mfapi.h:472).
const MFT_ENUM_FLAG_SYNCMFT: u32 = 0x0000_0001;

/// `MFT_REGISTER_TYPE_INFO`: one media type in an MFT input or output list.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MftRegisterTypeInfo {
    pub major_type: Guid,
    pub subtype: Guid,
}

/// `DMO_PARTIAL_MEDIATYPE`: one media type in a DMO input or output list.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DmoPartialMediaType {
    pub major_type: Guid,
    pub subtype: Guid,
}

/// The `color_converter_mft_inputs` array of msvdsp.c.
#[rustfmt::skip]
pub const COLOR_CONVERTER_MFT_INPUTS: [MftRegisterTypeInfo; 20] = [
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: MF_VIDEO_FORMAT_YV12 },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: MF_VIDEO_FORMAT_YUY2 },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: MF_VIDEO_FORMAT_UYVY },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: MF_VIDEO_FORMAT_AYUV },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: MF_VIDEO_FORMAT_NV12 },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: DMO_VIDEO_FORMAT_RGB32 },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: DMO_VIDEO_FORMAT_RGB565 },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: MF_VIDEO_FORMAT_I420 },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: MF_VIDEO_FORMAT_IYUV },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: MF_VIDEO_FORMAT_YVYU },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: DMO_VIDEO_FORMAT_RGB24 },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: DMO_VIDEO_FORMAT_RGB555 },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: DMO_VIDEO_FORMAT_RGB8 },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: MEDIASUBTYPE_V216 },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: MEDIASUBTYPE_V410 },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: MF_VIDEO_FORMAT_NV11 },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: MF_VIDEO_FORMAT_Y41P },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: MF_VIDEO_FORMAT_Y41T },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: MF_VIDEO_FORMAT_Y42T },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: MF_VIDEO_FORMAT_YVU9 },
];

/// The `color_converter_mft_outputs` array of msvdsp.c: the first 16 input entries.
#[rustfmt::skip]
pub const COLOR_CONVERTER_MFT_OUTPUTS: [MftRegisterTypeInfo; 16] = [
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: MF_VIDEO_FORMAT_YV12 },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: MF_VIDEO_FORMAT_YUY2 },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: MF_VIDEO_FORMAT_UYVY },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: MF_VIDEO_FORMAT_AYUV },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: MF_VIDEO_FORMAT_NV12 },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: DMO_VIDEO_FORMAT_RGB32 },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: DMO_VIDEO_FORMAT_RGB565 },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: MF_VIDEO_FORMAT_I420 },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: MF_VIDEO_FORMAT_IYUV },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: MF_VIDEO_FORMAT_YVYU },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: DMO_VIDEO_FORMAT_RGB24 },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: DMO_VIDEO_FORMAT_RGB555 },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: DMO_VIDEO_FORMAT_RGB8 },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: MEDIASUBTYPE_V216 },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: MEDIASUBTYPE_V410 },
    MftRegisterTypeInfo { major_type: MF_MEDIA_TYPE_VIDEO, subtype: MF_VIDEO_FORMAT_NV11 },
];

/// The `color_converter_dmo_inputs` array of msvdsp.c.
#[rustfmt::skip]
pub const COLOR_CONVERTER_DMO_INPUTS: [DmoPartialMediaType; 20] = [
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_YV12 },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_YUY2 },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_UYVY },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_AYUV },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_NV12 },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_RGB32 },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_RGB565 },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_I420 },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_IYUV },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_YVYU },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_RGB24 },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_RGB555 },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_RGB8 },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_V216 },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_V410 },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_NV11 },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_Y41P },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_Y41T },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_Y42T },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_YVU9 },
];

/// The `color_converter_dmo_outputs` array of msvdsp.c: the first 16 input entries.
#[rustfmt::skip]
pub const COLOR_CONVERTER_DMO_OUTPUTS: [DmoPartialMediaType; 16] = [
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_YV12 },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_YUY2 },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_UYVY },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_AYUV },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_NV12 },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_RGB32 },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_RGB565 },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_I420 },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_IYUV },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_YVYU },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_RGB24 },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_RGB555 },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_RGB8 },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_V216 },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_V410 },
    DmoPartialMediaType { major_type: MEDIATYPE_VIDEO, subtype: MEDIASUBTYPE_NV11 },
];

/// The OS calls that msvdsp.c makes, one method per call with the arguments the C code passes.
/// `MFTRegister` is always called with NULL attributes, so the trait has no attributes argument.
pub trait Registrar {
    /// `__wine_register_resources`.
    fn register_resources(&mut self) -> HResult;
    /// `__wine_unregister_resources`.
    fn unregister_resources(&mut self) -> HResult;
    /// `MFTRegister`.
    fn mft_register(
        &mut self,
        clsid: &Guid,
        category: &Guid,
        name: &str,
        flags: u32,
        inputs: &[MftRegisterTypeInfo],
        outputs: &[MftRegisterTypeInfo],
    ) -> HResult;
    /// `MFTUnregister`.
    fn mft_unregister(&mut self, clsid: &Guid) -> HResult;
    /// `DMORegister`.
    fn dmo_register(
        &mut self,
        name: &str,
        clsid: &Guid,
        category: &Guid,
        flags: u32,
        inputs: &[DmoPartialMediaType],
        outputs: &[DmoPartialMediaType],
    ) -> HResult;
    /// `DMOUnregister`.
    fn dmo_unregister(&mut self, clsid: &Guid, category: &Guid) -> HResult;
}

/// `DllRegisterServer`: the first failed call ends the sequence and its HRESULT is returned.
pub fn dll_register_server<R: Registrar>(r: &mut R) -> HResult {
    register_all(r).err().unwrap_or(S_OK)
}

/// `DllUnregisterServer`: the first failed call ends the sequence and its HRESULT is returned.
pub fn dll_unregister_server<R: Registrar>(r: &mut R) -> HResult {
    unregister_all(r).err().unwrap_or(S_OK)
}

fn register_all<R: Registrar>(r: &mut R) -> Result<(), HResult> {
    checked(r.register_resources())?;
    checked(r.mft_register(
        &CLSID_C_FRAME_RATE_CONVERT_DMO,
        &MFT_CATEGORY_VIDEO_EFFECT,
        "Frame Rate Converter",
        MFT_ENUM_FLAG_SYNCMFT,
        &COLOR_CONVERTER_MFT_INPUTS,
        &COLOR_CONVERTER_MFT_OUTPUTS,
    ))?;
    checked(r.dmo_register(
        "Frame Rate Converter",
        &CLSID_C_FRAME_RATE_CONVERT_DMO,
        &DMOCATEGORY_VIDEO_EFFECT,
        0,
        &COLOR_CONVERTER_DMO_INPUTS,
        &COLOR_CONVERTER_DMO_OUTPUTS,
    ))?;
    checked(r.mft_register(
        &CLSID_C_COLOR_CONTROL_DMO,
        &MFT_CATEGORY_VIDEO_EFFECT,
        "Color Control",
        MFT_ENUM_FLAG_SYNCMFT,
        &COLOR_CONVERTER_MFT_INPUTS,
        &COLOR_CONVERTER_MFT_OUTPUTS,
    ))?;
    checked(r.dmo_register(
        "Color Control",
        &CLSID_C_COLOR_CONTROL_DMO,
        &DMOCATEGORY_VIDEO_EFFECT,
        0,
        &COLOR_CONVERTER_DMO_INPUTS,
        &COLOR_CONVERTER_DMO_OUTPUTS,
    ))?;
    Ok(())
}

fn unregister_all<R: Registrar>(r: &mut R) -> Result<(), HResult> {
    checked(r.unregister_resources())?;
    checked(r.mft_unregister(&CLSID_C_FRAME_RATE_CONVERT_DMO))?;
    checked(r.dmo_unregister(&CLSID_C_FRAME_RATE_CONVERT_DMO, &DMOCATEGORY_VIDEO_EFFECT))?;
    checked(r.mft_unregister(&CLSID_C_COLOR_CONTROL_DMO))?;
    checked(r.dmo_unregister(&CLSID_C_COLOR_CONTROL_DMO, &DMOCATEGORY_VIDEO_EFFECT))?;
    Ok(())
}
