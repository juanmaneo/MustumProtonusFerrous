//! GUIDs from `include/directmanipulation.idl` and `include/unknwn.idl`, and IID/CLSID classification.

use std::fmt;

/// A COM `GUID`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl Guid {
    pub const fn new(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Self {
        Self {
            data1,
            data2,
            data3,
            data4,
        }
    }
}

/// Lowercase `8-4-4-4-12` form, as written in the IDL files.
impl fmt::Display for Guid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d = self.data4;
        write!(
            f,
            "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.data1, self.data2, self.data3, d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7]
        )
    }
}

pub const IID_UNKNOWN: Guid = Guid::new(
    0x00000000,
    0x0000,
    0x0000,
    [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
);
pub const IID_CLASS_FACTORY: Guid = Guid::new(
    0x00000001,
    0x0000,
    0x0000,
    [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
);
pub const IID_UPDATE_MANAGER: Guid = Guid::new(
    0xb0ae62fd,
    0xbe34,
    0x46e7,
    [0x9c, 0xaa, 0xd3, 0x61, 0xfa, 0xcb, 0xb9, 0xcc],
);
pub const IID_MANAGER: Guid = Guid::new(
    0xfbf5d3b4,
    0x70c7,
    0x4163,
    [0x93, 0x22, 0x5a, 0x6f, 0x66, 0x0d, 0x6f, 0xbc],
);
pub const IID_MANAGER2: Guid = Guid::new(
    0xfa1005e9,
    0x3d16,
    0x484c,
    [0xbf, 0xc9, 0x62, 0xb6, 0x1e, 0x56, 0xec, 0x4e],
);
pub const IID_CONTENT: Guid = Guid::new(
    0xb89962cb,
    0x3d89,
    0x442b,
    [0xbb, 0x58, 0x50, 0x98, 0xfa, 0x0f, 0x9f, 0x16],
);
pub const IID_PRIMARY_CONTENT: Guid = Guid::new(
    0xc12851e4,
    0x1698,
    0x4625,
    [0xb9, 0xb1, 0x7c, 0xa3, 0xec, 0x18, 0x63, 0x0b],
);
pub const IID_COMPOSITOR: Guid = Guid::new(
    0x537a0825,
    0x0387,
    0x4efa,
    [0xb6, 0x2f, 0x71, 0xeb, 0x1f, 0x08, 0x5a, 0x7e],
);
pub const IID_COMPOSITOR2: Guid = Guid::new(
    0xd38c7822,
    0xf1cb,
    0x43cb,
    [0xb4, 0xb9, 0xac, 0x0c, 0x76, 0x7a, 0x41, 0x2e],
);
pub const IID_VIEWPORT: Guid = Guid::new(
    0x28b85a3d,
    0x60a0,
    0x48bd,
    [0x9b, 0xa1, 0x5c, 0xe8, 0xd9, 0xea, 0x3a, 0x6d],
);
pub const IID_VIEWPORT2: Guid = Guid::new(
    0x923ccaac,
    0x61e1,
    0x4385,
    [0xb7, 0x26, 0x01, 0x7a, 0xf1, 0x89, 0x88, 0x2a],
);
pub const IID_FRAME_INFO_PROVIDER: Guid = Guid::new(
    0xfb759dba,
    0x6f4c,
    0x4c01,
    [0x87, 0x4e, 0x19, 0xc8, 0xa0, 0x59, 0x07, 0xf9],
);
pub const CLSID_MANAGER: Guid = Guid::new(
    0x54e211b6,
    0x3650,
    0x4f75,
    [0x83, 0x34, 0xfa, 0x35, 0x95, 0x98, 0xe1, 0xc5],
);
pub const CLSID_SHARED_MANAGER: Guid = Guid::new(
    0x99793286,
    0x77cc,
    0x4b57,
    [0x96, 0xdb, 0x3b, 0x35, 0x4f, 0x6f, 0x9f, 0xb5],
);
pub const CLSID_DCOMP_COMPOSITOR: Guid = Guid::new(
    0x79dea627,
    0xa08a,
    0x43ac,
    [0x8e, 0xf5, 0x69, 0x00, 0xb9, 0x29, 0x91, 0x26],
);

/// The interface IDs that the DLL's `QueryInterface` implementations compare against.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iid {
    Unknown,
    ClassFactory,
    UpdateManager,
    Manager,
    Manager2,
    Content,
    PrimaryContent,
    Compositor,
    Compositor2,
    Viewport,
    Viewport2,
    FrameInfoProvider,
}

/// The class IDs that `DllGetClassObject` accepts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clsid {
    Manager,
    Shared,
    DCompCompositor,
}

const IIDS: [(Guid, Iid); 12] = [
    (IID_UNKNOWN, Iid::Unknown),
    (IID_CLASS_FACTORY, Iid::ClassFactory),
    (IID_UPDATE_MANAGER, Iid::UpdateManager),
    (IID_MANAGER, Iid::Manager),
    (IID_MANAGER2, Iid::Manager2),
    (IID_CONTENT, Iid::Content),
    (IID_PRIMARY_CONTENT, Iid::PrimaryContent),
    (IID_COMPOSITOR, Iid::Compositor),
    (IID_COMPOSITOR2, Iid::Compositor2),
    (IID_VIEWPORT, Iid::Viewport),
    (IID_VIEWPORT2, Iid::Viewport2),
    (IID_FRAME_INFO_PROVIDER, Iid::FrameInfoProvider),
];

const CLSIDS: [(Guid, Clsid); 3] = [
    (CLSID_MANAGER, Clsid::Manager),
    (CLSID_SHARED_MANAGER, Clsid::Shared),
    (CLSID_DCOMP_COMPOSITOR, Clsid::DCompCompositor),
];

pub fn classify_iid(iid: &Guid) -> Option<Iid> {
    IIDS.iter().find(|(guid, _)| guid == iid).map(|&(_, id)| id)
}

pub fn classify_clsid(clsid: &Guid) -> Option<Clsid> {
    CLSIDS
        .iter()
        .find(|(guid, _)| guid == clsid)
        .map(|&(_, id)| id)
}
