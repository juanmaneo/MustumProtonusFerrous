//! The `IClassFactory` vtable of the single static `h264_encoder_factory` in `dlls/mfh264enc/mfh264enc.c`.

use crate::guid::{is_equal_guid, Guid, CLSID_WG_H264_ENCODER, IID_CLASS_FACTORY, IID_UNKNOWN};
use crate::hresult::{Hresult, E_NOINTERFACE, S_OK};

/// `CLSCTX_INPROC_SERVER`, from `include/wtypesbase.idl`.
pub const CLSCTX_INPROC_SERVER: u32 = 0x01;

/// The static factory object. The C object is only a vtable pointer, so the port has no state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClassFactory;

/// `class_factory_QueryInterface`. `Ok` is `S_OK` with `*out` set to the factory. `Err` is `E_NOINTERFACE`
/// with `*out` set to NULL. The factory answers `IClassFactory` and `IUnknown` only.
pub fn class_factory_query_interface(riid: &Guid) -> Result<ClassFactory, Hresult> {
    if is_equal_guid(riid, &IID_CLASS_FACTORY) || is_equal_guid(riid, &IID_UNKNOWN) {
        Ok(ClassFactory)
    } else {
        Err(E_NOINTERFACE)
    }
}

/// `class_factory_AddRef`: always reports 2, because the factory is static.
pub fn class_factory_add_ref() -> u32 {
    2
}

/// `class_factory_Release`: always reports 1, because the factory is static.
pub fn class_factory_release() -> u32 {
    1
}

/// `class_factory_LockServer`: succeeds for either value of `dolock` and changes nothing.
pub fn class_factory_lock_server(_dolock: bool) -> Hresult {
    S_OK
}

/// `h264_encoder_factory_CreateInstance`. It calls `CoCreateInstance` with `CLSID_wg_h264_encoder`, the
/// `CLSCTX_INPROC_SERVER` context, and `riid`, and returns the result. `co_create_instance` stands in for the
/// OS call. The `outer` and `out` arguments are passed through by the C and are not modelled.
pub fn h264_encoder_factory_create_instance(
    riid: &Guid,
    co_create_instance: impl FnOnce(&Guid, u32, &Guid) -> Hresult,
) -> Hresult {
    co_create_instance(&CLSID_WG_H264_ENCODER, CLSCTX_INPROC_SERVER, riid)
}
