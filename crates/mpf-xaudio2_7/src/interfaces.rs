use crate::guid::{is_equal_guid, Guid};
use crate::hresult::E_NOINTERFACE;

pub const IID_IUNKNOWN: Guid = Guid {
    data1: 0,
    data2: 0,
    data3: 0,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};
pub const IID_IXAUDIO2: Guid = Guid {
    data1: 0x8bcf_1f58,
    data2: 0x9fe7,
    data3: 0x4583,
    data4: [0x8a, 0xc6, 0xe2, 0xad, 0xc4, 0x65, 0xc8, 0xbb],
};
pub const IID_IXAPO: Guid = Guid {
    data1: 0xa410_b984,
    data2: 0x9839,
    data3: 0x4819,
    data4: [0xa0, 0xbe, 0x28, 0x56, 0xae, 0x6b, 0x3a, 0xdb],
};
pub const IID_IXAPO_PARAMETERS: Guid = Guid {
    data1: 0x26d9_5c66,
    data2: 0x80f2,
    data3: 0x499a,
    data4: [0xad, 0x54, 0x5a, 0xe7, 0xf0, 0x1c, 0x6d, 0x98],
};
pub const IID_IXAPO27: Guid = Guid {
    data1: 0xa90b_c001,
    data2: 0xe897,
    data3: 0xe897,
    data4: [0x55, 0xe4, 0x9e, 0x47, 0x00, 0x00, 0x00, 0x00],
};
pub const IID_IXAPO27_PARAMETERS: Guid = Guid {
    data1: 0xa90b_c001,
    data2: 0xe897,
    data3: 0xe897,
    data4: [0x55, 0xe4, 0x9e, 0x47, 0x00, 0x00, 0x00, 0x01],
};

#[derive(PartialEq, Eq)]
pub enum XapoInterface {
    Xapo,
    Parameters,
}

/// Port of `XAPOFX_QueryInterface`. Returns which interface pointer the C code would hand out;
/// the reference count it adds is not modelled.
pub fn xapo_query_interface(riid: &Guid) -> Result<XapoInterface, u32> {
    if is_equal_guid(riid, &IID_IUNKNOWN)
        || is_equal_guid(riid, &IID_IXAPO)
        || is_equal_guid(riid, &IID_IXAPO27)
    {
        Ok(XapoInterface::Xapo)
    } else if is_equal_guid(riid, &IID_IXAPO_PARAMETERS)
        || is_equal_guid(riid, &IID_IXAPO27_PARAMETERS)
    {
        Ok(XapoInterface::Parameters)
    } else {
        Err(E_NOINTERFACE)
    }
}

/// Port of `IXAudio2Impl_QueryInterface`: only `IUnknown` and `IXAudio2` are served.
pub fn xaudio2_query_interface(riid: &Guid) -> Result<(), u32> {
    if is_equal_guid(riid, &IID_IUNKNOWN) || is_equal_guid(riid, &IID_IXAUDIO2) {
        Ok(())
    } else {
        Err(E_NOINTERFACE)
    }
}
