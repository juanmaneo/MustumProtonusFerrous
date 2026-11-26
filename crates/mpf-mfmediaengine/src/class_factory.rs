use crate::com::{
    query_interface, Guid, CLASS_E_CLASSNOTAVAILABLE, CLASS_E_NOAGGREGATION,
    CLSID_MF_MEDIA_ENGINE_CLASS_FACTORY, HRESULT, IID_ICLASSFACTORY, IID_IUNKNOWN, S_OK,
};
use crate::media_engine_factory::MediaEngineClassFactory;

/// `classfactory` from `dlls/mfmediaengine/main.c`, the `IClassFactory` that
/// `DllGetClassObject` hands out for `CLSID_MFMediaEngineClassFactory`. Its `AddRef` and
/// `Release` return fixed counts in C, so the reference count is not modelled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClassFactory;

impl ClassFactory {
    /// `classfactory_QueryInterface`.
    pub fn query_interface(&self, riid: &Guid) -> Result<(), HRESULT> {
        query_interface(riid, &[IID_ICLASSFACTORY, IID_IUNKNOWN])
    }

    /// `classfactory_CreateInstance`. `has_outer` is whether the C `outer` argument is non-NULL,
    /// and it is checked before `riid`.
    pub fn create_instance(
        &self,
        has_outer: bool,
        riid: &Guid,
    ) -> Result<MediaEngineClassFactory, HRESULT> {
        if has_outer {
            return Err(CLASS_E_NOAGGREGATION);
        }
        MediaEngineClassFactory
            .query_interface(riid)
            .map(|()| MediaEngineClassFactory)
    }

    /// `classfactory_LockServer`, which returns `S_OK` for both values of `dolock`.
    pub fn lock_server(&self, _dolock: bool) -> HRESULT {
        S_OK
    }
}

/// `DllGetClassObject` from `dlls/mfmediaengine/main.c`. Only `CLSID_MFMediaEngineClassFactory`
/// is served, and any other class gives `CLASS_E_CLASSNOTAVAILABLE`.
pub fn dll_get_class_object(clsid: &Guid, riid: &Guid) -> Result<ClassFactory, HRESULT> {
    if *clsid != CLSID_MF_MEDIA_ENGINE_CLASS_FACTORY {
        return Err(CLASS_E_CLASSNOTAVAILABLE);
    }
    ClassFactory.query_interface(riid).map(|()| ClassFactory)
}
