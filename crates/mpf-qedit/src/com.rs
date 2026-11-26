/// `HRESULT`, as a signed 32-bit value.
pub type Hresult = i32;

pub const S_OK: Hresult = 0;
pub const S_FALSE: Hresult = 1;
pub const E_FAIL: Hresult = 0x8000_4005u32 as i32;
pub const E_NOINTERFACE: Hresult = 0x8000_4002u32 as i32;
pub const E_OUTOFMEMORY: Hresult = 0x8007_000Eu32 as i32;
pub const CLASS_E_CLASSNOTAVAILABLE: Hresult = 0x8004_0111u32 as i32;
pub const REGDB_E_CLASSNOTREG: Hresult = 0x8004_0154u32 as i32;

/// `FAILED`: a negative HRESULT.
pub fn failed(hr: Hresult) -> bool {
    hr < 0
}

/// A `GUID`. Equality compares all 16 bytes, as `IsEqualGUID` does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

/// `GUID_NULL`, from `include/guiddef.h`.
pub const GUID_NULL: Guid = Guid {
    data1: 0,
    data2: 0,
    data3: 0,
    data4: [0; 8],
};

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

/// `CLSID_AMTimeline`, from `dlls/qedit/qedit_classes.idl`.
pub const CLSID_AMTIMELINE: Guid = Guid {
    data1: 0x7853_0b75,
    data2: 0x61f9,
    data3: 0x11d2,
    data4: [0x8c, 0xad, 0x00, 0xa0, 0x24, 0x58, 0x09, 0x02],
};

/// `CLSID_MediaDet`, from `dlls/qedit/qedit_classes.idl`.
pub const CLSID_MEDIADET: Guid = Guid {
    data1: 0x65bd_0711,
    data2: 0x24d2,
    data3: 0x4ff7,
    data4: [0x93, 0x24, 0xed, 0x2e, 0x5d, 0x3a, 0xba, 0xfa],
};

/// `CLSID_NullRenderer`, from `dlls/qedit/qedit_classes.idl`.
pub const CLSID_NULLRENDERER: Guid = Guid {
    data1: 0xc1f4_00a4,
    data2: 0x3f08,
    data3: 0x11d3,
    data4: [0x9f, 0x0b, 0x00, 0x60, 0x08, 0x03, 0x9e, 0x37],
};

/// `CLSID_SampleGrabber`, from `dlls/qedit/qedit_classes.idl`.
pub const CLSID_SAMPLEGRABBER: Guid = Guid {
    data1: 0xc1f4_00a0,
    data2: 0x3f08,
    data3: 0x11d3,
    data4: [0x9f, 0x0b, 0x00, 0x60, 0x08, 0x03, 0x9e, 0x37],
};
