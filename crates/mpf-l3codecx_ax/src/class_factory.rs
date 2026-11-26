//! Port of the `IClassFactory` object in `l3codecx.c`: `mpeg_layer3_decoder_factory` and its vtable.

use crate::com::{ComHost, Guid, Hresult, E_NOINTERFACE, S_OK};

/// `CLSID_wg_mp3_decoder`, the class that `CreateInstance` activates.
pub const CLSID_WG_MP3_DECODER: Guid = Guid {
    data1: 0x84cd_8e3e,
    data2: 0xb221,
    data3: 0x434a,
    data4: [0x88, 0x82, 0x9d, 0x6c, 0x8d, 0xf4, 0x90, 0xe1],
};

/// `IID_IClassFactory`, from include/unknwn.idl.
pub const IID_ICLASS_FACTORY: Guid = Guid {
    data1: 0x0000_0001,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `IID_IUnknown`, from include/unknwn.idl.
pub const IID_IUNKNOWN: Guid = Guid {
    data1: 0x0000_0000,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// The factory object. Its methods are the vtable entries of the C `IClassFactory`.
#[derive(Debug)]
pub struct ClassFactory;

/// The static factory object `mpeg_layer3_decoder_factory`.
pub static MPEG_LAYER3_DECODER_FACTORY: ClassFactory = ClassFactory;

impl ClassFactory {
    /// `class_factory_QueryInterface`: hands out the factory itself for `IClassFactory` and
    /// `IUnknown`, and nothing for any other interface.
    pub fn query_interface(&self, riid: &Guid) -> (Hresult, Option<&ClassFactory>) {
        if *riid == IID_ICLASS_FACTORY || *riid == IID_IUNKNOWN {
            (S_OK, Some(self))
        } else {
            (E_NOINTERFACE, None)
        }
    }

    /// `class_factory_AddRef`. The factory is static, so the count is always 2.
    pub fn add_ref(&self) -> u32 {
        2
    }

    /// `class_factory_Release`. The factory is static, so the count is always 1.
    pub fn release(&self) -> u32 {
        1
    }

    /// `class_factory_LockServer`. `dolock` has no effect.
    pub fn lock_server(&self, _dolock: bool) -> Hresult {
        S_OK
    }

    /// `mpeg_layer3_decoder_factory_CreateInstance`: activates `CLSID_wg_mp3_decoder` through the host.
    pub fn create_instance(&self, host: &mut impl ComHost, outer: bool, riid: &Guid) -> Hresult {
        host.co_create_instance(&CLSID_WG_MP3_DECODER, outer, riid)
    }
}
