//! COM plumbing shared by the ported objects: `HRESULT` values, the interface tag that
//! stands in for `*out`, GUIDs, and the base-interface test every `QueryInterface` uses.

/// `HRESULT`, as a signed 32-bit value so that `FAILED` is a sign test.
pub type Hresult = i32;

pub const S_OK: Hresult = 0;
pub const E_NOINTERFACE: Hresult = 0x8000_4002u32 as i32;
pub const E_NOTIMPL: Hresult = 0x8000_4001u32 as i32;
pub const CLASS_E_CLASSNOTAVAILABLE: Hresult = 0x8004_0111u32 as i32;

/// The interface a successful `QueryInterface` returns, in place of `*out`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Interface {
    ActivationFactory,
    AppCapabilityStatics,
    AppCapability,
}

/// `GUID`. Compared by value, as `IsEqualGUID` does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

pub const IID_IUNKNOWN: Guid = Guid {
    data1: 0x0000_0000,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

pub const IID_IINSPECTABLE: Guid = Guid {
    data1: 0xAF86_E2E0,
    data2: 0xB12D,
    data3: 0x4C6A,
    data4: [0x9C, 0x5A, 0xD7, 0xAA, 0x65, 0x10, 0x1E, 0x90],
};

pub const IID_IAGILEOBJECT: Guid = Guid {
    data1: 0x94EA_2B94,
    data2: 0xE9CC,
    data3: 0x49E0,
    data4: [0xC0, 0xFF, 0xEE, 0x64, 0xCA, 0x8F, 0x5B, 0x90],
};

pub const IID_IACTIVATIONFACTORY: Guid = Guid {
    data1: 0x0000_0035,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

pub const IID_IAPPCAPABILITY: Guid = Guid {
    data1: 0x4C49_D915,
    data2: 0x8A2A,
    data3: 0x4295,
    data4: [0x94, 0x37, 0x2D, 0xF7, 0xC3, 0x96, 0xAF, 0xF4],
};

pub const IID_IAPPCAPABILITYSTATICS: Guid = Guid {
    data1: 0x7C35_3E2A,
    data2: 0x46EE,
    data3: 0x44E5,
    data4: [0xAF, 0x3D, 0x6A, 0xD3, 0xFC, 0x49, 0xBD, 0x22],
};

/// `IUnknown`, `IInspectable` or `IAgileObject`: the IIDs every object in this DLL answers.
pub fn is_base_interface(iid: &Guid) -> bool {
    *iid == IID_IUNKNOWN || *iid == IID_IINSPECTABLE || *iid == IID_IAGILEOBJECT
}
