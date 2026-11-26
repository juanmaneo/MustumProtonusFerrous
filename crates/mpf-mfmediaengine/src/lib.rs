//! Port of the portable logic in Wine's `dlls/mfmediaengine/main.c` and `dlls/mfmediaengine/mfmediaengine.spec`.
//!
//! Ported:
//! - `mftime_to_seconds` (`MFTIME` ticks of 100 ns to seconds): [`mftime_to_seconds`]
//! - `get_gcd` (Euclid's algorithm): [`get_gcd`]
//! - the capacity rule of `mf_array_reserve`: [`array_reserve_capacity`]
//! - the `IMFMediaTimeRange` table (`struct time_range`): [`TimeRange`], [`Range`]
//! - `DllGetClassObject`, `classfactory` and `media_engine_factory` with their `QueryInterface`,
//!   `CreateInstance`, `LockServer`, `IsTypeSupported`, `CreateMediaKeys` and
//!   `CreateMediaSourceExtension` methods: [`dll_get_class_object`], [`ClassFactory`],
//!   [`MediaEngineClassFactory`]
//! - the `HRESULT`, `GUID` and `IID` values those functions use: [`com`]
//! - `mfmediaengine.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - `DllGetActivationFactory` (`@ stub`) and the `__wine_spec_unimplemented_stub` record it
//!   raises: [`dll_get_activation_factory`], [`wine_spec_unimplemented_stub`], [`StubException`]
//!
//! Skipped:
//! - `media_engine_factory_CreateInstance`, which builds the `IMFMediaEngineEx` object, and that
//!   object's vtable methods. Also skipped are `media_engine_factory_CreateTimeRange`,
//!   `media_engine_factory_CreateError`, and the `set_rect` and `media_engine_set_rate` helpers.
//! - COM `AddRef` and `Release`, and `QueryInterface` on the time range and media error objects.
//! - The `realloc` and `E_OUTOFMEMORY` path of `time_range_AddRange`. The port stores the ranges in
//!   a `Vec`, so [`array_reserve_capacity`] is exported but `TimeRange` does not call it.
//! - `video_frame_sink.c`, which is a Media Foundation sink built on `IMFSample` and
//!   `IMFMediaType` objects.
//! - `DllCanUnloadNow`: the spec exports it `-private`, but no file under `dlls/mfmediaengine`
//!   defines it, so [`ExportKind::Undefined`] records it without a return value.
//! - The `TRACE`, `FIXME` and `WARN` output.

pub mod com;

mod array;
mod class_factory;
mod exports;
mod gcd;
mod media_engine_factory;
mod stub;
mod time;
mod time_range;

pub use array::array_reserve_capacity;
pub use class_factory::{dll_get_class_object, ClassFactory};
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use gcd::get_gcd;
pub use media_engine_factory::MediaEngineClassFactory;
pub use stub::{
    dll_get_activation_factory, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};
pub use time::mftime_to_seconds;
pub use time_range::{Range, TimeRange};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
