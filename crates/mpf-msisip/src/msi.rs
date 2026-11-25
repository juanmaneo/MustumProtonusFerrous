//! Class-ID test from `MsiSIPIsMyTypeOfFile`.

/// Windows `GUID`, with the same field layout as Wine's `GUID`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// `mySubject`: the SIP subject that `DllRegisterServer` registers and
/// `MsiSIPIsMyTypeOfFile` reports for MSI storages.
pub const MY_SUBJECT: Guid = Guid {
    data1: 0x000c_10f1,
    data2: 0,
    data3: 0,
    data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46],
};

/// `CLSID_MsiTransform`.
pub const CLSID_MSI_TRANSFORM: Guid = Guid {
    data1: 0x000c_1082,
    data2: 0,
    data3: 0,
    data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46],
};

/// `CLSID_MsiDatabase`.
pub const CLSID_MSI_DATABASE: Guid = Guid {
    data1: 0x000c_1084,
    data2: 0,
    data3: 0,
    data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46],
};

/// `CLSID_MsiPatch`.
pub const CLSID_MSI_PATCH: Guid = Guid {
    data1: 0x000c_1086,
    data2: 0,
    data3: 0,
    data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46],
};

/// The subject `MsiSIPIsMyTypeOfFile` reports for a storage with class `clsid`,
/// or `None` when the class is not an MSI database, patch or transform.
pub fn msi_subject_for_clsid(clsid: &Guid) -> Option<Guid> {
    if *clsid == CLSID_MSI_DATABASE || *clsid == CLSID_MSI_PATCH || *clsid == CLSID_MSI_TRANSFORM {
        Some(MY_SUBJECT)
    } else {
        None
    }
}
