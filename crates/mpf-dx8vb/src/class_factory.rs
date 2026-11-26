//! `DllGetClassObject` and the `IClassFactoryImpl` methods from `dlls/dx8vb/main.c`.

/// `HRESULT` value.
pub type HResult = u32;

pub const S_OK: HResult = 0x0000_0000;
pub const E_NOINTERFACE: HResult = 0x8000_4002;
pub const E_OUTOFMEMORY: HResult = 0x8007_000E;
pub const CLASS_E_CLASSNOTAVAILABLE: HResult = 0x8004_0111;

/// Windows `GUID` layout. Equality is field-wise, the same as `IsEqualGUID`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

pub const GUID_NULL: Guid = Guid {
    data1: 0,
    data2: 0,
    data3: 0,
    data4: [0; 8],
};

pub const IID_UNKNOWN: Guid = Guid {
    data1: 0,
    data2: 0,
    data3: 0,
    data4: [0xC0, 0, 0, 0, 0, 0, 0, 0x46],
};

pub const IID_CLASS_FACTORY: Guid = Guid {
    data1: 1,
    data2: 0,
    data3: 0,
    data4: [0xC0, 0, 0, 0, 0, 0, 0, 0x46],
};

/// CLSIDs in `object_creation[]`. The only entry is `GUID_NULL`, with a NULL creation callback.
const OBJECT_CREATION_CLSIDS: &[Guid] = &[GUID_NULL];

/// `IClassFactoryImpl` without its vtable pointer and creation callback. Only the reference count is kept.
#[derive(Debug, PartialEq, Eq)]
pub struct ClassFactory {
    ref_count: u32,
}

impl ClassFactory {
    /// The state `DllGetClassObject` leaves a new factory in: `factory->ref = 1`.
    pub fn created() -> Self {
        Self { ref_count: 1 }
    }

    pub fn ref_count(&self) -> u32 {
        self.ref_count
    }

    /// `classfactory_QueryInterface` for `IUnknown` and `IClassFactory`. Success hands out a new
    /// reference, so the count goes up. Any other riid is `E_NOINTERFACE` with no change.
    pub fn query_interface(&mut self, riid: &Guid) -> Result<(), HResult> {
        if *riid == IID_UNKNOWN || *riid == IID_CLASS_FACTORY {
            self.add_ref();
            Ok(())
        } else {
            Err(E_NOINTERFACE)
        }
    }

    /// `classfactory_AddRef`. Wraps like the 32-bit `LONG` behind `InterlockedIncrement`.
    pub fn add_ref(&mut self) -> u32 {
        self.ref_count = self.ref_count.wrapping_add(1);
        self.ref_count
    }

    /// `classfactory_Release`. Returns the new count. The C code frees the object at 0, which
    /// the caller models by dropping the value. Wraps like `InterlockedDecrement` does on `ULONG`.
    pub fn release(&mut self) -> u32 {
        self.ref_count = self.ref_count.wrapping_sub(1);
        self.ref_count
    }
}

/// `classfactory_LockServer`. A stub that reports success for either value of `dolock`.
pub fn lock_server(_dolock: bool) -> HResult {
    S_OK
}

/// `DllGetClassObject`. Checks run in the C order: riid, then clsid, then allocation.
/// `alloc_ok` is the outcome of the `HeapAlloc` call.
pub fn dll_get_class_object(
    rclsid: &Guid,
    riid: &Guid,
    alloc_ok: bool,
) -> Result<ClassFactory, HResult> {
    if *riid != IID_CLASS_FACTORY && *riid != IID_UNKNOWN {
        return Err(E_NOINTERFACE);
    }
    if !OBJECT_CREATION_CLSIDS.contains(rclsid) {
        return Err(CLASS_E_CLASSNOTAVAILABLE);
    }
    if !alloc_ok {
        return Err(E_OUTOFMEMORY);
    }
    Ok(ClassFactory::created())
}
