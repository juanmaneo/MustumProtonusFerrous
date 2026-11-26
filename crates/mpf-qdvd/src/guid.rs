//! `GUID` values from the `.idl` files Wine's `dlls/qdvd` builds against, and `IsEqualGUID`.

/// A `GUID` as laid out in `guiddef.h`.
#[derive(PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// `IsEqualGUID`: field-by-field equality.
pub fn is_equal_guid(a: &Guid, b: &Guid) -> bool {
    a == b
}

/// `IID_IUnknown`, from `include/unknwn.idl`.
pub const IID_IUNKNOWN: Guid = Guid {
    data1: 0x0000_0000,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `IID_IClassFactory`, from `include/unknwn.idl`.
pub const IID_ICLASSFACTORY: Guid = Guid {
    data1: 0x0000_0001,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `IID_IDvdGraphBuilder`, from `include/dvdif.idl`.
pub const IID_IDVDGRAPHBUILDER: Guid = Guid {
    data1: 0xfcc1_52b6,
    data2: 0xf372,
    data3: 0x11d0,
    data4: [0x8e, 0x00, 0x00, 0xc0, 0x4f, 0xd7, 0xc0, 0x8b],
};

/// `IID_IDvdControl2`, from `include/dvdif.idl`.
pub const IID_IDVDCONTROL2: Guid = Guid {
    data1: 0x33bc_7430,
    data2: 0xeec0,
    data3: 0x11d2,
    data4: [0x82, 0x01, 0x00, 0xa0, 0xc9, 0xd7, 0x48, 0x42],
};

/// `IID_IDvdInfo2`, from `include/dvdif.idl`.
pub const IID_IDVDINFO2: Guid = Guid {
    data1: 0x3415_1510,
    data2: 0xeec0,
    data3: 0x11d2,
    data4: [0x82, 0x01, 0x00, 0xa0, 0xc9, 0xd7, 0x48, 0x42],
};

/// `CLSID_DvdGraphBuilder`, from `dlls/qdvd/qdvd_classes.idl`.
pub const CLSID_DVDGRAPHBUILDER: Guid = Guid {
    data1: 0xfcc1_52b7,
    data2: 0xf372,
    data3: 0x11d0,
    data4: [0x8e, 0x00, 0x00, 0xc0, 0x4f, 0xd7, 0xc0, 0x8b],
};

/// `CLSID_DVDNavigator`, from `dlls/qdvd/qdvd_classes.idl`.
pub const CLSID_DVDNAVIGATOR: Guid = Guid {
    data1: 0x9b8c_4620,
    data2: 0x2c1a,
    data3: 0x11d0,
    data4: [0x84, 0x93, 0x00, 0xa0, 0x24, 0x38, 0xad, 0x48],
};
