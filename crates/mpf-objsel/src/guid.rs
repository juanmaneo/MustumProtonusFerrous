//! `GUID` values and `IsEqualGUID` from the headers that `dlls/objsel` includes.

/// A COM `GUID`, laid out as in the Windows header.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// `IsEqualGUID`: compares all sixteen bytes, field by field.
pub fn is_equal_guid(a: &Guid, b: &Guid) -> bool {
    a == b
}

/// `IID_IUnknown`, declared in `include/unknwn.idl`.
pub const IID_UNKNOWN: Guid = Guid {
    data1: 0x0000_0000,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `IID_IClassFactory`, declared in `include/unknwn.idl`.
pub const IID_CLASS_FACTORY: Guid = Guid {
    data1: 0x0000_0001,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `IID_IDsObjectPicker`, from `include/objsel.h`.
pub const IID_DS_OBJECT_PICKER: Guid = Guid {
    data1: 0x0c87_e64e,
    data2: 0x3b7a,
    data3: 0x11d2,
    data4: [0xB9, 0xE0, 0x00, 0xC0, 0x4F, 0xD8, 0xDB, 0xF7],
};

/// `CLSID_DsObjectPicker`, from `include/objsel.h`. `DllGetClassObject` serves this class only.
pub const CLSID_DS_OBJECT_PICKER: Guid = Guid {
    data1: 0x17d6_ccd8,
    data2: 0x3b7b,
    data3: 0x11d2,
    data4: [0xB9, 0xE0, 0x00, 0xC0, 0x4F, 0xD8, 0xDB, 0xF7],
};
