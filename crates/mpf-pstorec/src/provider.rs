/// `S_OK` from `include/winerror.h`.
pub const S_OK: u32 = 0;

/// `E_NOINTERFACE` from `include/winerror.h`.
pub const E_NOINTERFACE: u32 = 0x8000_4002;

/// `E_OUTOFMEMORY` from `include/winerror.h`.
pub const E_OUTOFMEMORY: u32 = 0x8007_000E;

/// The fields of the C `GUID` structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// `IID_IPStore`, `5a6f1ec0-2db1-11d0-8c39-00c04fd9126b` from `include/pstore.idl`.
pub const IID_IPSTORE: Guid = Guid {
    data1: 0x5a6f_1ec0,
    data2: 0x2db1,
    data3: 0x11d0,
    data4: [0x8c, 0x39, 0x00, 0xc0, 0x4f, 0xd9, 0x12, 0x6b],
};

/// `IID_IUnknown`, `00000000-0000-0000-c000-000000000046` from `include/unknwn.idl`.
pub const IID_IUNKNOWN: Guid = Guid {
    data1: 0,
    data2: 0,
    data3: 0,
    data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46],
};

/// The `PStore_impl` object: its `IPStore` interface and reference count.
#[derive(Debug, PartialEq, Eq)]
pub struct PStoreProvider {
    /// The `LONG ref` field. Changes wrap, as `InterlockedIncrement` and `InterlockedDecrement` do.
    pub ref_count: i32,
}

impl PStoreProvider {
    /// `PStore_fnQueryInterface`. `S_OK` takes a reference through [`Self::add_ref`], and
    /// `E_NOINTERFACE` takes none. The C `*ppvObj` out pointer is implied by the return value:
    /// it holds the provider on `S_OK` and NULL otherwise.
    pub fn query_interface(&mut self, riid: &Guid) -> u32 {
        if *riid == IID_IPSTORE || *riid == IID_IUNKNOWN {
            self.add_ref();
            S_OK
        } else {
            E_NOINTERFACE
        }
    }

    /// `PStore_fnAddRef`: `InterlockedIncrement`, which returns the new count.
    pub fn add_ref(&mut self) -> u32 {
        self.ref_count = self.ref_count.wrapping_add(1);
        self.ref_count as u32
    }

    /// `PStore_fnRelease`: `InterlockedDecrement`. The C body frees the object when the new count
    /// is 0. The port reports that through the return value, and the owner drops the object.
    pub fn release(&mut self) -> u32 {
        self.ref_count = self.ref_count.wrapping_sub(1);
        self.ref_count as u32
    }
}

/// `PStoreCreateInstance` from `dlls/pstorec/pstorec.c`. `allocate` stands for `HeapAlloc`. When
/// it returns `false` the call returns `E_OUTOFMEMORY` and leaves `pp_provider` untouched.
/// Otherwise the new provider starts with one reference and `pp_provider` holds it. The
/// `pProviderID`, `pReserved` and `dwFlags` arguments are only traced.
pub fn pstore_create_instance(
    pp_provider: &mut Option<PStoreProvider>,
    allocate: &mut dyn FnMut() -> bool,
) -> u32 {
    if !allocate() {
        return E_OUTOFMEMORY;
    }
    *pp_provider = Some(PStoreProvider { ref_count: 1 });
    S_OK
}
