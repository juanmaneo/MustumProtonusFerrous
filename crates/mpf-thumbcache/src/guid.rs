/// A `GUID` laid out as in `include/guiddef.h`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// `IID_IUnknown` from `include/guiddef.h`.
pub const IID_IUNKNOWN: Guid = Guid {
    data1: 0,
    data2: 0,
    data3: 0,
    data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46],
};

/// `IID_IClassFactory` from `include/unknwn.idl`.
pub const IID_ICLASSFACTORY: Guid = Guid {
    data1: 1,
    data2: 0,
    data3: 0,
    data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46],
};

/// `IID_IThumbnailCache` from `include/thumbcache.idl`.
pub const IID_ITHUMBNAILCACHE: Guid = Guid {
    data1: 0xf676_c15d,
    data2: 0x596a,
    data3: 0x4ce2,
    data4: [0x82, 0x34, 0x33, 0x99, 0x6f, 0x44, 0x5d, 0xb1],
};

/// `CLSID_LocalThumbnailCache` from `dlls/thumbcache/thumbcache_reg.idl`.
pub const CLSID_LOCAL_THUMBNAIL_CACHE: Guid = Guid {
    data1: 0x50ef_4544,
    data2: 0xac9f,
    data3: 0x4a8e,
    data4: [0xb2, 0x1b, 0x8a, 0x26, 0x18, 0x0d, 0xb1, 0x3f],
};
