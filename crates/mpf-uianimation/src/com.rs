/// An `HRESULT`, as the C code returns it.
pub type HRESULT = u32;

pub const S_OK: HRESULT = 0x0000_0000;
pub const S_FALSE: HRESULT = 0x0000_0001;
pub const E_NOINTERFACE: HRESULT = 0x8000_4002;
pub const CLASS_E_CLASSNOTAVAILABLE: HRESULT = 0x8004_0111;

/// A GUID as a 128-bit value, with the digits of its text form in order.
pub type Guid = u128;

/// `IID_IUnknown`, from `include/unknwn.idl`.
pub const IID_IUNKNOWN: Guid = 0x0000000000000000c000000000000046;
/// `IID_IClassFactory`, from `include/unknwn.idl`.
pub const IID_ICLASSFACTORY: Guid = 0x0000000100000000c000000000000046;

/// What a call left in its `void **obj` out-parameter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Out<T> {
    /// The C code did not write the pointer.
    Untouched,
    /// The C code wrote `NULL`.
    Null,
    /// The C code wrote the interface of `T`.
    Set(T),
}

/// The `QueryInterface` that the class factories and the objects they create share: `IUnknown`
/// and the interface `own_iid` are accepted, anything else fails with `E_NOINTERFACE`.
pub fn query_interface<T>(obj: T, iid: Guid, own_iid: Guid) -> (HRESULT, Out<T>) {
    if iid == IID_IUNKNOWN || iid == own_iid {
        (S_OK, Out::Set(obj))
    } else {
        (E_NOINTERFACE, Out::Null)
    }
}
