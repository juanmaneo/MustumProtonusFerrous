//! Port of `DllGetClassObject` and the class factories of `dlls/wmvdecod/video_decoder.c` and
//! `video_decoder.h`, and of `DllCanUnloadNow` from `libs/winecrt0/dll_canunload.c`.

use crate::guid::{
    is_equal_guid, Guid, CLSID_WG_H264_DECODER, CLSID_WG_WMV_DECODER, CLSID_WMVDECODER_MFT,
    IID_ICLASSFACTORY, IID_IUNKNOWN,
};
use crate::hresult::{Hresult, CLASS_E_CLASSNOTAVAILABLE, E_NOINTERFACE, S_FALSE, S_OK};

/// `CLSCTX_INPROC_SERVER`, from `include/wtypesbase.idl`.
pub const CLSCTX_INPROC_SERVER: u32 = 0x0000_0001;

/// The two class factories of `video_decoder.c`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClassFactory {
    /// `wmv_decoder_factory`.
    Wmv,
    /// `h264_decoder_factory`. `DllGetClassObject` never returns it.
    H264,
}

/// The `CoCreateInstance` request a factory's `CreateInstance` makes. The C call also passes the
/// `outer`, `riid` and out pointer through, and the port does not model those.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CoCreateRequest {
    pub clsid: Guid,
    pub clsctx: u32,
}

/// `DllGetClassObject`. `Ok` is the factory the C code writes to `*out` and returns `S_OK` for.
/// `Err` is the HRESULT returned, with `*out` set to NULL.
pub fn dll_get_class_object(clsid: &Guid, riid: &Guid) -> Result<ClassFactory, Hresult> {
    if is_equal_guid(clsid, &CLSID_WMVDECODER_MFT) {
        return factory_query_interface(ClassFactory::Wmv, riid);
    }
    Err(CLASS_E_CLASSNOTAVAILABLE)
}

/// `class_factory_QueryInterface`: the factory is returned for `IClassFactory` and `IUnknown`.
pub fn factory_query_interface(
    factory: ClassFactory,
    riid: &Guid,
) -> Result<ClassFactory, Hresult> {
    if is_equal_guid(riid, &IID_ICLASSFACTORY) || is_equal_guid(riid, &IID_IUNKNOWN) {
        Ok(factory)
    } else {
        Err(E_NOINTERFACE)
    }
}

/// `class_factory_AddRef`: the factories report a fixed count of 2.
pub const fn factory_add_ref() -> u32 {
    2
}

/// `class_factory_Release`: the factories report a fixed count of 1.
pub const fn factory_release() -> u32 {
    1
}

/// `class_factory_LockServer`: always `S_OK`, whatever the lock flag.
pub const fn factory_lock_server(_dolock: bool) -> Hresult {
    S_OK
}

/// `CreateInstance` of each factory: the CLSID of the decoder it forwards to, and `CLSCTX_INPROC_SERVER`.
pub fn create_instance_request(factory: ClassFactory) -> CoCreateRequest {
    let clsid = match factory {
        ClassFactory::Wmv => CLSID_WG_WMV_DECODER,
        ClassFactory::H264 => CLSID_WG_H264_DECODER,
    };
    CoCreateRequest {
        clsid,
        clsctx: CLSCTX_INPROC_SERVER,
    }
}

/// `DllCanUnloadNow` from `libs/winecrt0/dll_canunload.c`: always `S_FALSE`.
pub const fn dll_can_unload_now() -> Hresult {
    S_FALSE
}
