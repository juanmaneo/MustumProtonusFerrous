//! Port of the portable logic in Wine's `dlls/graphicscapture/main.c`, `dlls/graphicscapture/session.c`
//! and `libs/winecrt0/dll_canunload.c`.
//!
//! Ported:
//! - `DllGetActivationFactory`: [`dll_get_activation_factory`] and [`CLASS_E_CLASSNOTAVAILABLE`],
//!   with the `wcscmp` comparison of the class name
//! - the session object: [`Session`] with `QueryInterface` ([`Session::query_interface`]), `AddRef`
//!   and `Release` on one shared count, the [`Interface`] pointers it hands out, the [`S_OK`] and
//!   [`E_NOINTERFACE`] values, and the `IID_*` constants it compares against
//! - the `E_NOTIMPL` methods of the factory and of the statics interface: [`Session::get_iids`],
//!   [`Session::get_runtime_class_name`], [`Session::get_trust_level`] and
//!   [`Session::activate_instance`]
//! - `IGraphicsCaptureSessionStatics::IsSupported`: [`is_supported`]
//! - `DllCanUnloadNow`: [`dll_can_unload_now`] and [`S_FALSE`], the default from `winecrt0`
//!
//! Skipped:
//! - The `TRACE` and `FIXME` output and the `debugstr_*` helpers.
//! - The `session_factory` global. Each call takes the one session object as `&mut Session`.
//! - The `DEFINE_IINSPECTABLE` glue. The statics interface delegates to the factory methods, so the
//!   port keeps one set of methods on [`Session`].
//! - Real `HSTRING` objects. The class name is a `&[u16]`. An empty slice stands for a NULL
//!   `HSTRING`, which `WindowsGetStringRawBuffer` maps to an empty string.
//! - The `.spec` export table. Both exports are `@ stdcall -private` and the spec has no `@ stub`
//!   entries, so the `__wine_spec_unimplemented_stub` model does not apply.
//! - The atomicity of `InterlockedIncrement` and `InterlockedDecrement`. The count is a plain
//!   `i32` that wraps on overflow, as the `__sync_*_and_fetch` builtins behind them do.

mod dll;
mod factory;
mod session;

pub use dll::{dll_can_unload_now, S_FALSE};
pub use factory::{dll_get_activation_factory, CLASS_E_CLASSNOTAVAILABLE};
pub use session::{
    is_supported, Guid, Interface, Session, E_NOINTERFACE, E_NOTIMPL, IID_ACTIVATION_FACTORY,
    IID_AGILE_OBJECT, IID_GRAPHICS_CAPTURE_SESSION_STATICS, IID_INSPECTABLE, IID_UNKNOWN, S_OK,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
