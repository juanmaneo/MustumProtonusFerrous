//! `GetDispenserManager` from Wine's `dlls/mtxdm/mtxdm_main.c`, and the `CoCreateInstance`
//! arguments it passes.

/// `HRESULT` is a 32-bit signed value, as in Wine's `include/winerror.h`.
pub type HRESULT = i32;

/// `CLSCTX_ALL` from `include/combaseapi.h`: the `CLSCTX_*` values from `include/wtypesbase.idl`
/// OR-ed together.
pub const CLSCTX_ALL: u32 = 0x01 | 0x02 | 0x04 | 0x10;

/// A COM `GUID`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// `CLSID_DispenserManager`, the `uuid` of the `DispenserManager` coclass in `include/comsvcs.idl`.
pub const CLSID_DISPENSER_MANAGER: Guid = Guid {
    data1: 0xecabb0c0,
    data2: 0x7f19,
    data3: 0x11d2,
    data4: [0x97, 0x8e, 0x00, 0x00, 0xf8, 0x75, 0x7e, 0x2a],
};

/// `IID_IDispenserManager`, the `uuid` of `IDispenserManager` in `include/comsvcs.idl`.
pub const IID_IDISPENSER_MANAGER: Guid = Guid {
    data1: 0x5cb31e10,
    data2: 0x2b5f,
    data3: 0x11cf,
    data4: [0xbe, 0x10, 0x00, 0xaa, 0x00, 0xa2, 0xfa, 0x25],
};

/// `GetDispenserManager`: `CoCreateInstance(&CLSID_DispenserManager, NULL, CLSCTX_ALL,
/// &IID_IDispenserManager, dispenser)`, with its result returned unchanged.
///
/// `co_create_instance` stands in for `CoCreateInstance` and gets the class, the class context,
/// the interface and the out-pointer, in C order. `pUnkOuter` is NULL in every call, so it is not
/// passed. The out-pointer is forwarded as it is, so a NULL `dispenser` arrives as `None`.
pub fn get_dispenser_manager<T>(
    dispenser: Option<&mut T>,
    co_create_instance: impl FnOnce(&Guid, u32, &Guid, Option<&mut T>) -> HRESULT,
) -> HRESULT {
    co_create_instance(
        &CLSID_DISPENSER_MANAGER,
        CLSCTX_ALL,
        &IID_IDISPENSER_MANAGER,
        dispenser,
    )
}
