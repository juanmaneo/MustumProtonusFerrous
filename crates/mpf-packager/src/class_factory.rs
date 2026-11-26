use crate::com::{
    Guid, E_NOINTERFACE, E_OUTOFMEMORY, HRESULT, IID_ICLASS_FACTORY, IID_IUNKNOWN, S_OK,
};
use crate::ole_object::{self, OleInterface, Package};

/// `PackageCF_QueryInterface`. The class factory is static, so the port returns only the
/// `HRESULT`.
pub fn query_interface(riid: &Guid) -> HRESULT {
    if *riid == IID_IUNKNOWN || *riid == IID_ICLASS_FACTORY {
        S_OK
    } else {
        E_NOINTERFACE
    }
}

/// `PackageCF_AddRef`. The static class factory always reports a count of 2.
pub fn add_ref() -> u32 {
    2
}

/// `PackageCF_Release`. The static class factory always reports a count of 1.
pub fn release() -> u32 {
    1
}

/// `PackageCF_CreateInstance`. `allocated` stands for the `HeapAlloc` result; the outer
/// aggregate is ignored, as in the C body. The new package starts at zero references and the
/// interface query takes the first one.
pub fn create_instance(allocated: bool, iid: &Guid) -> Result<OleInterface, HRESULT> {
    if !allocated {
        return Err(E_OUTOFMEMORY);
    }
    let mut pkg = Package::default();
    ole_object::query_interface(&mut pkg, iid)
}

/// `PackageCF_LockServer`. The lock is accepted and nothing is counted.
pub fn lock_server(_lock: bool) -> HRESULT {
    S_OK
}
