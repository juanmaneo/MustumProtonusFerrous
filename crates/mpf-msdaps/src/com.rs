//! COM types, `HRESULT` values and the GUIDs that `main.c` and `row_server.c` compare or return.
//! The GUIDs are the `uuid(...)` values in `dlls/msdaps/row_server.idl` and `include/unknwn.idl`.

/// `HRESULT` from `include/winnt.h`.
pub type HRESULT = i32;

/// `S_OK` from `include/winerror.h`.
pub const S_OK: HRESULT = 0;

/// `E_NOINTERFACE` from `include/winerror.h`.
pub const E_NOINTERFACE: HRESULT = 0x8000_4002_u32 as HRESULT;

/// `E_NOTIMPL` from `include/winerror.h`.
pub const E_NOTIMPL: HRESULT = 0x8000_4001_u32 as HRESULT;

/// `GUID` from `include/guiddef.h`. `repr(C)` gives the 16-byte layout that `sizeof(GUID)` and
/// the `memcmp` in `IsEqualGUID` use.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// `IID_IUnknown` from `include/unknwn.idl`.
pub const IID_IUNKNOWN: Guid = Guid {
    data1: 0x0000_0000,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `IID_IClassFactory` from `include/unknwn.idl`.
pub const IID_ICLASSFACTORY: Guid = Guid {
    data1: 0x0000_0001,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `CLSID_wine_row_server`, the `wine_row_server` coclass in `dlls/msdaps/row_server.idl`.
pub const CLSID_WINE_ROW_SERVER: Guid = Guid {
    data1: 0x3824_8178,
    data2: 0xcf6d,
    data3: 0x11de,
    data4: [0xab, 0xe5, 0x00, 0x0c, 0x29, 0x16, 0xd8, 0x65],
};

/// `CLSID_wine_row_proxy`, the `wine_row_proxy` coclass in `dlls/msdaps/row_server.idl`.
pub const CLSID_WINE_ROW_PROXY: Guid = Guid {
    data1: 0x3824_8179,
    data2: 0xcf6d,
    data3: 0x11de,
    data4: [0xab, 0xe5, 0x00, 0x0c, 0x29, 0x16, 0xd8, 0x65],
};

/// `CLSID_wine_rowset_server`, the `wine_rowset_server` coclass in `dlls/msdaps/row_server.idl`.
pub const CLSID_WINE_ROWSET_SERVER: Guid = Guid {
    data1: 0x3824_817a,
    data2: 0xcf6d,
    data3: 0x11de,
    data4: [0xab, 0xe5, 0x00, 0x0c, 0x29, 0x16, 0xd8, 0x65],
};

/// `CLSID_wine_rowset_proxy`, the `wine_rowset_proxy` coclass in `dlls/msdaps/row_server.idl`.
pub const CLSID_WINE_ROWSET_PROXY: Guid = Guid {
    data1: 0x3824_817b,
    data2: 0xcf6d,
    data3: 0x11de,
    data4: [0xab, 0xe5, 0x00, 0x0c, 0x29, 0x16, 0xd8, 0x65],
};

/// `IsEqualGUID` and `IsEqualCLSID`, which compare the 16 bytes with `memcmp`. The fields are
/// compared one by one, which is the same as comparing the bytes for this layout.
pub fn is_equal_guid(a: &Guid, b: &Guid) -> bool {
    a.data1 == b.data1 && a.data2 == b.data2 && a.data3 == b.data3 && a.data4 == b.data4
}
