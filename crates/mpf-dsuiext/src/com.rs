//! COM plumbing shared by the ported objects: the `HRESULT` values `dsuiext.c` returns, the
//! interface tag an out-pointer can receive, and the `LONG` reference count.

/// `HRESULT`.
pub type Hresult = i32;

pub const S_OK: Hresult = 0;
pub const S_FALSE: Hresult = 1;
pub const E_NOINTERFACE: Hresult = 0x8000_4002_u32 as i32;
pub const E_NOTIMPL: Hresult = 0x8000_4001_u32 as i32;
pub const E_INVALIDARG: Hresult = 0x8007_0057_u32 as i32;
pub const CLASS_E_NOAGGREGATION: Hresult = 0x8004_0110_u32 as i32;
pub const CLASS_E_CLASSNOTAVAILABLE: Hresult = 0x8004_0111_u32 as i32;

/// The interface an `void **` out-parameter receives. The port tracks which interface was
/// returned, not the object's address.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Interface {
    IDsDisplaySpecifier,
    IClassFactory,
}

/// A `LONG` reference count. `InterlockedIncrement` and `InterlockedDecrement` wrap on
/// overflow, so the arithmetic wraps here too.
#[derive(Debug, PartialEq, Eq)]
pub struct RefCount(pub(crate) i32);

impl RefCount {
    /// The count every `*_create` and `*_constructor` starts from.
    pub fn with_one_reference() -> Self {
        Self(1)
    }

    /// `InterlockedIncrement`, returned as `ULONG`.
    pub fn add_ref(&mut self) -> u32 {
        self.0 = self.0.wrapping_add(1);
        self.0 as u32
    }

    /// `InterlockedDecrement`, returned as `ULONG`. Zero means the C code frees the object.
    pub fn release(&mut self) -> u32 {
        self.0 = self.0.wrapping_sub(1);
        self.0 as u32
    }
}
