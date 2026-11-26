//! Port of the portable logic in Wine's `dlls/l3codecx.ax/l3codecx.c` and `dlls/l3codecx.ax/l3codecx.ax.spec`.
//!
//! Ported:
//! - The `IClassFactory` object behind `CLSID_MPEGLayer3Decoder`: [`ClassFactory`] and
//!   [`MPEG_LAYER3_DECODER_FACTORY`] (`QueryInterface`, `AddRef`, `Release`, `LockServer`,
//!   `CreateInstance`).
//! - `DllGetClassObject`: [`dll_get_class_object`].
//! - `DllRegisterServer` and `DllUnregisterServer`, with the `REGFILTER2` descriptor they register:
//!   [`dll_register_server`], [`dll_unregister_server`], [`MPEG_LAYER3_DECODER_FILTER`].
//! - The `l3codecx.ax.spec` export table: [`EXPORTS`], [`export_signature`].
//! - The GUIDs, HRESULTs and `REG_PINFLAG_B_OUTPUT` those bodies use.
//!
//! Skipped:
//! - The `TRACE` and `FIXME` output and `debugstr_guid`.
//! - The COM and registry calls: `CoCreateInstance`, `IFilterMapper2`, `__wine_register_resources`
//!   and `__wine_unregister_resources`. They sit behind [`ComHost`]. The port decides the call order
//!   and what each result means; the host performs the calls and holds the mapper pointer and the
//!   `out` pointers. The aggregation `outer` pointer is reduced to a flag.
//! - The C `IClassFactory` vtable and the `WINAPI` calling convention. The vtable entries are
//!   inherent methods.
//! - The `@ stub` model. `l3codecx.ax.spec` has no `@ stub` entries: its three `-private` exports
//!   are real implementations in `l3codecx.c`.

mod class_factory;
mod com;
mod exports;
mod registration;

pub use class_factory::{
    ClassFactory, CLSID_WG_MP3_DECODER, IID_ICLASS_FACTORY, IID_IUNKNOWN,
    MPEG_LAYER3_DECODER_FACTORY,
};
pub use com::{
    is_failed, ComHost, Guid, Hresult, CLASS_E_CLASSNOTAVAILABLE, E_FAIL, E_NOINTERFACE, S_FALSE,
    S_OK,
};
pub use exports::{dll_get_class_object, export_signature, EXPORTS};
pub use registration::{
    dll_register_server, dll_unregister_server, RegFilter2, RegFilterPins2, RegPinTypes,
    CLSID_FILTER_MAPPER2, CLSID_MPEG_LAYER3_DECODER, IID_IFILTER_MAPPER2, MEDIASUBTYPE_MP3,
    MEDIASUBTYPE_PCM, MEDIATYPE_AUDIO, MPEG_LAYER3_DECODER_FILTER, MPEG_LAYER3_FILTER_NAME,
    REG_PINFLAG_B_OUTPUT,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
