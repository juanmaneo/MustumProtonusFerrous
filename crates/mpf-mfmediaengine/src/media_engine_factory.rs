use crate::com::{
    query_interface, Guid, E_NOTIMPL, HRESULT, IID_IMF_MEDIA_ENGINE_CLASS_FACTORY,
    IID_IMF_MEDIA_ENGINE_CLASS_FACTORY_EX, IID_IUNKNOWN,
};

/// The `media_engine_factory` object from `dlls/mfmediaengine/main.c`, the
/// `IMFMediaEngineClassFactoryEx` that `classfactory_CreateInstance` hands out. Its `AddRef` and
/// `Release` are not modelled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MediaEngineClassFactory;

impl MediaEngineClassFactory {
    /// `media_engine_factory_QueryInterface`.
    pub fn query_interface(&self, riid: &Guid) -> Result<(), HRESULT> {
        query_interface(
            riid,
            &[
                IID_IMF_MEDIA_ENGINE_CLASS_FACTORY,
                IID_IMF_MEDIA_ENGINE_CLASS_FACTORY_EX,
                IID_IUNKNOWN,
            ],
        )
    }

    /// `media_engine_factory_IsTypeSupported`. The C code always returns `S_OK` and sets
    /// `is_supported` to FALSE, so this returns `Ok(false)`.
    pub fn is_type_supported(&self) -> Result<bool, HRESULT> {
        Ok(false)
    }

    /// `media_engine_factory_CreateMediaKeys`, which is `E_NOTIMPL` and sets no `keys`.
    pub fn create_media_keys(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `media_engine_factory_CreateMediaSourceExtension`, which is `E_NOTIMPL` and sets `mse` to
    /// NULL. Its `flags` and `attr` arguments are never read.
    pub fn create_media_source_extension(&self) -> HRESULT {
        E_NOTIMPL
    }
}
