//! Port of the portable logic in Wine's `dlls/wmvdecod/wmvdecod.c` and `dlls/wmvdecod/video_decoder.c`,
//! and of `DllCanUnloadNow` from `libs/winecrt0/dll_canunload.c`, which `dlls/wmvdecod/wmvdecod.spec` exports.
//!
//! Ported:
//! - `DllGetClassObject` CLSID dispatch, and the `class_factory_*` helpers of `video_decoder.h`:
//!   [`dll_get_class_object`], [`factory_query_interface`], [`factory_add_ref`], [`factory_release`],
//!   [`factory_lock_server`], [`create_instance_request`], with [`ClassFactory`]
//! - The MFT and DMO registration tables and the `DllRegisterServer` / `DllUnregisterServer` sequences,
//!   which stop at the first call that fails: [`dll_register_server`], [`dll_unregister_server`],
//!   [`Registrar`], [`MediaType`], [`DmoMediaType`], [`MFT_INPUT_TYPES`], [`MFT_OUTPUT_TYPES`],
//!   [`DMO_INPUT_TYPES`], [`DMO_OUTPUT_TYPES`]
//! - `DllCanUnloadNow`: [`dll_can_unload_now`]
//! - The GUID and `HRESULT` values these use: [`Guid`], [`is_equal_guid`], and the constants in [`guid`] and [`hresult`]
//!
//! Skipped:
//! - The `CoCreateInstance` call itself, the `IClassFactory` vtable and `this` pointers, and the `outer`,
//!   `riid` and out-pointer arguments of `CreateInstance`, which the C body passes through unread.
//! - The `__wine_register_resources` and `__wine_unregister_resources` bodies (`libs/winecrt0/register.c`),
//!   and the bodies of `MFTRegister`, `MFTUnregister`, `DMORegister` and `DMOUnregister`. These are OS calls,
//!   so the port models them as the [`Registrar`] trait.
//! - The `FIXME` and `TRACE` output and `WINE_DEFAULT_DEBUG_CHANNEL`.
//! - `MEDIASUBTYPE_WMV_Unknown`, which `video_decoder.c` declares but the DLL never uses.
//! - The decoders the factories forward to (`CLSID_wg_wmv_decoder`, `CLSID_wg_h264_decoder`), which live
//!   outside this DLL. The port returns their CLSIDs and does not model what they do.
//! - The `@ stub` model: `wmvdecod.spec` has no `@ stub` exports, so there is no `StubException` to port.

mod class_object;
mod guid;
mod hresult;
mod register;

pub use class_object::{
    create_instance_request, dll_can_unload_now, dll_get_class_object, factory_add_ref,
    factory_lock_server, factory_query_interface, factory_release, ClassFactory, CoCreateRequest,
    CLSCTX_INPROC_SERVER,
};
pub use guid::{
    is_equal_guid, media_type_guid, rgb_guid, Guid, CLSID_WG_H264_DECODER, CLSID_WG_WMV_DECODER,
    CLSID_WMVDECODER_MFT, DMOCATEGORY_VIDEO_DECODER, DMOVIDEOFORMAT_RGB24, DMOVIDEOFORMAT_RGB32,
    DMOVIDEOFORMAT_RGB555, DMOVIDEOFORMAT_RGB565, DMOVIDEOFORMAT_RGB8, IID_ICLASSFACTORY,
    IID_IUNKNOWN, MEDIASUBTYPE_NV11, MEDIASUBTYPE_NV12, MEDIASUBTYPE_RGB24, MEDIASUBTYPE_RGB32,
    MEDIASUBTYPE_RGB555, MEDIASUBTYPE_RGB565, MEDIASUBTYPE_RGB8, MEDIASUBTYPE_UYVY,
    MEDIASUBTYPE_VC1S, MEDIASUBTYPE_WMV1, MEDIASUBTYPE_WMV2, MEDIASUBTYPE_WMV3, MEDIASUBTYPE_WMVA,
    MEDIASUBTYPE_WMVP, MEDIASUBTYPE_WMVR, MEDIASUBTYPE_WVC1, MEDIASUBTYPE_WVP2, MEDIASUBTYPE_YUY2,
    MEDIASUBTYPE_YV12, MEDIASUBTYPE_YVYU, MEDIATYPE_VIDEO, MFT_CATEGORY_VIDEO_DECODER,
};
pub use hresult::{
    failed, Hresult, CLASS_E_CLASSNOTAVAILABLE, E_FAIL, E_NOINTERFACE, E_OUTOFMEMORY,
    REGDB_E_CLASSNOTREG, S_FALSE, S_OK,
};
pub use register::{
    dll_register_server, dll_unregister_server, DmoMediaType, MediaType, Registrar,
    DMO_INPUT_TYPES, DMO_NAME, DMO_OUTPUT_TYPES, MFT_ENUM_FLAG_SYNCMFT, MFT_INPUT_TYPES, MFT_NAME,
    MFT_OUTPUT_TYPES,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
