//! The bodies of `dlls/windowscodecsext/main.c`.

use crate::hresult::{failed, succeeded, CLASS_E_CLASSNOTAVAILABLE, E_INVALIDARG, HRESULT};

/// `DllGetClassObject`. The C body only traces its arguments and returns
/// `CLASS_E_CLASSNOTAVAILABLE`, so the port takes none.
pub fn dll_get_class_object() -> HRESULT {
    CLASS_E_CLASSNOTAVAILABLE
}

/// The COM calls that `WICCreateColorTransform_Proxy` makes, in the order the C body makes them.
pub trait ColorTransformHost<T> {
    /// `CoInitializeEx(NULL, COINIT_MULTITHREADED)`.
    fn co_initialize_ex(&mut self) -> HRESULT;
    /// `CoCreateInstance(&CLSID_WICImagingFactory, ..., &IID_IWICImagingFactory, ...)`. The host
    /// keeps the factory; the port reads only the HRESULT.
    fn co_create_imaging_factory(&mut self) -> HRESULT;
    /// `IWICImagingFactory_CreateColorTransformer` on the factory from
    /// [`Self::co_create_imaging_factory`].
    fn create_color_transformer(&mut self, out: &mut T) -> HRESULT;
    /// `IWICImagingFactory_Release` on that factory.
    fn release_imaging_factory(&mut self);
    /// `CoUninitialize`.
    fn co_uninitialize(&mut self);
}

/// `WICCreateColorTransform_Proxy`. `out` is `ppIWICColorTransform`, and `None` is a NULL pointer.
pub fn wic_create_color_transform_proxy<T, H: ColorTransformHost<T>>(
    host: &mut H,
    out: Option<&mut T>,
) -> HRESULT {
    let Some(out) = out else {
        return E_INVALIDARG;
    };
    let init = host.co_initialize_ex();
    let hr = host.co_create_imaging_factory();
    if failed(hr) {
        if succeeded(init) {
            host.co_uninitialize();
        }
        return hr;
    }
    let hr = host.create_color_transformer(out);
    host.release_imaging_factory();
    if succeeded(init) {
        host.co_uninitialize();
    }
    hr
}

/// The `IWICColorTransform` vtable call that `IWICColorTransform_Initialize_Proxy_W` makes.
pub trait ColorTransformVtable<P> {
    /// `IWICColorTransform_Initialize(iface, ...)`, where `self` is `iface`.
    fn initialize(&mut self, source: P, ctx_source: P, ctx_dest: P, pixel_fmt_dest: P) -> HRESULT;
}

/// `IWICColorTransform_Initialize_Proxy_W`: forwards its arguments unchanged and returns the
/// vtable's HRESULT.
pub fn initialize_proxy_w<P, I: ColorTransformVtable<P>>(
    iface: &mut I,
    source: P,
    ctx_source: P,
    ctx_dest: P,
    pixel_fmt_dest: P,
) -> HRESULT {
    iface.initialize(source, ctx_source, ctx_dest, pixel_fmt_dest)
}
