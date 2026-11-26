//! Port of the portable logic in Wine's `dlls/mfh264enc/mfh264enc.c` and `dlls/mfh264enc/mfh264enc.spec`,
//! and of the `DllCanUnloadNow` default in `libs/winecrt0/dll_canunload.c` that the spec exports.
//!
//! Ported:
//! - The `IClassFactory` vtable: [`class_factory_query_interface`], [`class_factory_add_ref`],
//!   [`class_factory_release`], [`class_factory_lock_server`], [`ClassFactory`]
//! - `h264_encoder_factory_CreateInstance`, with the `CLSID_wg_h264_encoder` it requests:
//!   [`h264_encoder_factory_create_instance`]
//! - `DllGetClassObject` and `DllCanUnloadNow`: [`dll_get_class_object`], [`dll_can_unload_now`]
//! - `DllRegisterServer` and `DllUnregisterServer`, with the `MFTRegister` table:
//!   [`dll_register_server`], [`dll_unregister_server`], [`H264_ENCODER_REGISTRATION`],
//!   [`MftRegistration`], [`MftRegisterTypeInfo`]
//! - The `mfh264enc.spec` export names: [`EXPORTS`], [`is_export`]
//! - The GUID and `HRESULT` values the above use: [`Guid`], [`is_equal_guid`], [`Hresult`], [`failed`]
//!
//! Skipped:
//! - `TRACE` and `FIXME` logging, and `WINE_DEFAULT_DEBUG_CHANNEL`.
//! - The OS calls, which are closure arguments: `CoCreateInstance`, `__wine_register_resources`,
//!   `__wine_unregister_resources`, `MFTRegister` and `MFTUnregister`.
//! - The `outer` and `out` arguments of `CreateInstance`, and the `attributes` argument of `MFTRegister`.
//! - The COM vtable dispatch: the vtable functions are plain functions.
//! - `__wine_spec_unimplemented_stub`: the spec has no `@ stub` exports.

mod class_factory;
mod dll;
mod exports;
mod guid;
mod hresult;
mod registration;

pub use class_factory::{
    class_factory_add_ref, class_factory_lock_server, class_factory_query_interface,
    class_factory_release, h264_encoder_factory_create_instance, ClassFactory,
    CLSCTX_INPROC_SERVER,
};
pub use dll::{dll_can_unload_now, dll_get_class_object};
pub use exports::{is_export, EXPORTS};
pub use guid::{
    is_equal_guid, Guid, CLSID_MSH264_ENCODER_MFT, CLSID_WG_H264_ENCODER, IID_CLASS_FACTORY,
    IID_UNKNOWN,
};
pub use hresult::{failed, Hresult, CLASS_E_CLASSNOTAVAILABLE, E_NOINTERFACE, S_FALSE, S_OK};
pub use registration::{
    dll_register_server, dll_unregister_server, MftRegisterTypeInfo, MftRegistration,
    H264_ENCODER_REGISTRATION, MFT_CATEGORY_VIDEO_ENCODER, MFT_ENUM_FLAG_SYNCMFT,
    MF_MEDIA_TYPE_VIDEO, MF_VIDEO_FORMAT_H264, MF_VIDEO_FORMAT_IYUV, MF_VIDEO_FORMAT_NV12,
    MF_VIDEO_FORMAT_YUY2, MF_VIDEO_FORMAT_YV12,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
