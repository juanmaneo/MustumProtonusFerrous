//! `GUID` values and `IsEqualGUID` from the headers that `dlls/msauddecmft` includes.

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

/// `IID_IUnknown`, from `include/unknwn.idl`.
pub const IID_UNKNOWN: Guid = Guid {
    data1: 0x0000_0000,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `IID_IClassFactory`, from `include/unknwn.idl`.
pub const IID_CLASS_FACTORY: Guid = Guid {
    data1: 0x0000_0001,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `CLSID_MSAACDecMFT`, from `include/mfidl.idl`. `DllGetClassObject` serves this class only.
pub const CLSID_MSAAC_DEC_MFT: Guid = Guid {
    data1: 0x32d1_86a7,
    data2: 0x218f,
    data3: 0x4c75,
    data4: [0x88, 0x76, 0xDD, 0x77, 0x27, 0x3A, 0x89, 0x99],
};

/// `CLSID_wg_aac_decoder`, from `dlls/msauddecmft/msauddecmft.c`. The factory's `CreateInstance` requests this
/// class, not `CLSID_MSAACDecMFT`.
pub const CLSID_WG_AAC_DECODER: Guid = Guid {
    data1: 0xe788_9a8a,
    data2: 0x2083,
    data3: 0x4844,
    data4: [0x83, 0x70, 0x5E, 0xE3, 0x49, 0xB1, 0x45, 0x03],
};
