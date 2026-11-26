//! `GUID` values and `IsEqualGUID` from the headers that `dlls/mfh264enc` includes.

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

/// `CLSID_MSH264EncoderMFT`, from `include/mfidl.idl`. `DllGetClassObject` serves this class only.
pub const CLSID_MSH264_ENCODER_MFT: Guid = Guid {
    data1: 0x6ca5_0344,
    data2: 0x051a,
    data3: 0x4ded,
    data4: [0x97, 0x79, 0xA4, 0x33, 0x05, 0x16, 0x5E, 0x35],
};

/// `CLSID_wg_h264_encoder`, from `dlls/mfh264enc/mfh264enc.c`. The factory's `CreateInstance` requests this
/// class, not `CLSID_MSH264EncoderMFT`.
pub const CLSID_WG_H264_ENCODER: Guid = Guid {
    data1: 0x6c34_de69,
    data2: 0x4670,
    data3: 0x46cd,
    data4: [0x8C, 0xB4, 0x1F, 0x2F, 0xA1, 0xDF, 0xFB, 0x65],
};
