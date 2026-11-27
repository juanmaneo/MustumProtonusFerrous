//! DllGetClassObject and the DSOUND_CF table from dlls/dsound/dsound_main.c.

use crate::hresult::{Hresult, CLASS_E_CLASSNOTAVAILABLE, E_INVALIDARG, E_NOINTERFACE, S_OK};

/// A GUID, laid out as in include/guiddef.h.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

pub const IID_UNKNOWN: Guid = Guid {
    data1: 0x0000_0000,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

pub const IID_CLASS_FACTORY: Guid = Guid {
    data1: 0x0000_0001,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

const CLSID_DIRECT_SOUND: Guid = Guid {
    data1: 0x47d4_d946,
    data2: 0x62e8,
    data3: 0x11cf,
    data4: [0x93, 0xbc, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00],
};

const CLSID_DIRECT_SOUND8: Guid = Guid {
    data1: 0x3901_cc3f,
    data2: 0x84b5,
    data3: 0x4fa4,
    data4: [0xba, 0x35, 0xaa, 0x81, 0x72, 0xb8, 0xa0, 0x9b],
};

const CLSID_DIRECT_SOUND_CAPTURE: Guid = Guid {
    data1: 0xb021_0780,
    data2: 0x89cd,
    data3: 0x11d0,
    data4: [0xaf, 0x08, 0x00, 0xa0, 0xc9, 0x25, 0xcd, 0x16],
};

const CLSID_DIRECT_SOUND_CAPTURE8: Guid = Guid {
    data1: 0xe4bc_ac13,
    data2: 0x7f99,
    data3: 0x4908,
    data4: [0x9a, 0x8e, 0x74, 0xe3, 0xbf, 0x24, 0xb6, 0xe1],
};

const CLSID_DIRECT_SOUND_FULL_DUPLEX: Guid = Guid {
    data1: 0xfea4_300c,
    data2: 0x7959,
    data3: 0x4147,
    data4: [0xb2, 0x6a, 0x23, 0x77, 0xb9, 0xe7, 0xa9, 0x1d],
};

const CLSID_DIRECT_SOUND_PRIVATE: Guid = Guid {
    data1: 0x11ab_3ec0,
    data2: 0x25ec,
    data3: 0x11d1,
    data4: [0xa4, 0xd8, 0x00, 0xc0, 0x4f, 0xc2, 0x8a, 0xca],
};

/// One DSOUND_CF entry. The C CreateInstance pointer is not modelled.
#[derive(Debug, PartialEq, Eq)]
pub struct ClassFactory {
    pub clsid: Guid,
}

/// The DSOUND_CF table in its C order.
pub static CLASS_FACTORIES: [ClassFactory; 6] = [
    ClassFactory {
        clsid: CLSID_DIRECT_SOUND,
    },
    ClassFactory {
        clsid: CLSID_DIRECT_SOUND8,
    },
    ClassFactory {
        clsid: CLSID_DIRECT_SOUND_CAPTURE,
    },
    ClassFactory {
        clsid: CLSID_DIRECT_SOUND_CAPTURE8,
    },
    ClassFactory {
        clsid: CLSID_DIRECT_SOUND_FULL_DUPLEX,
    },
    ClassFactory {
        clsid: CLSID_DIRECT_SOUND_PRIVATE,
    },
];

/// DllGetClassObject. The output is cleared before the checks; a NULL `ppv` is `None`.
pub fn dll_get_class_object(
    rclsid: &Guid,
    riid: &Guid,
    ppv: Option<&mut Option<&'static ClassFactory>>,
) -> Hresult {
    let Some(ppv) = ppv else {
        return E_INVALIDARG;
    };
    *ppv = None;
    if *riid != IID_CLASS_FACTORY && *riid != IID_UNKNOWN {
        return E_NOINTERFACE;
    }
    match CLASS_FACTORIES.iter().find(|cf| cf.clsid == *rclsid) {
        Some(cf) => {
            *ppv = Some(cf);
            S_OK
        }
        None => CLASS_E_CLASSNOTAVAILABLE,
    }
}
