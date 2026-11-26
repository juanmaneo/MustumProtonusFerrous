/// A `GUID`. Equality compares all 16 bytes, as `IsEqualGUID` does.
#[derive(PartialEq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// `IsEqualGUID`.
pub fn is_equal_guid(a: &Guid, b: &Guid) -> bool {
    a == b
}

/// `IUnknown`, from `include/unknwn.idl`.
pub const IID_IUNKNOWN: Guid = Guid {
    data1: 0x0000_0000,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `IClassFactory`, from `include/unknwn.idl`.
pub const IID_ICLASSFACTORY: Guid = Guid {
    data1: 0x0000_0001,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `IUIFramework`, from `include/uiribbon.idl`.
pub const IID_IUIFRAMEWORK: Guid = Guid {
    data1: 0xf4f0_385d,
    data2: 0x6872,
    data3: 0x43a8,
    data4: [0xad, 0x09, 0x4c, 0x33, 0x9c, 0xb3, 0xf5, 0xc5],
};

/// `CLSID_UIRibbonFramework`, from `include/uiribbon.idl`.
pub const CLSID_UIRIBBONFRAMEWORK: Guid = Guid {
    data1: 0x9267_49fa,
    data2: 0x2615,
    data3: 0x4987,
    data4: [0x88, 0x45, 0xc3, 0x3e, 0x65, 0xf2, 0xb9, 0x57],
};

/// The interface pointers that the C code hands out through `ppv` or `ppobj`.
#[derive(Debug, PartialEq, Eq)]
pub enum Interface {
    UIRibbonFramework,
    ClassFactory,
}
