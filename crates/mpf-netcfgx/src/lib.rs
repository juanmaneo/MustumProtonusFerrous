//! Port of the portable logic in Wine's `dlls/netcfgx/netcfg.c` and `dlls/netcfgx/netcfgx.spec`.
//!
//! Ported:
//! - `DllGetClassObject` for `CLSID_CNetCfg`: [`dll_get_class_object`], and its class factory,
//!   [`ClassFactory`]
//! - the `CNetCfg` object: [`inet_cfg_create_instance`], [`NetConfiguration`] and [`Iface`], with
//!   its reference counting and the fixed return values of its `INetCfg` and `INetCfgLock` methods
//! - the GUIDs and HRESULTs those functions compare and return, from `include/netcfgx.idl` and
//!   `include/winerror.h`: the `com` module
//! - the `netcfgx.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - the twelve `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`StubException`], [`wine_spec_unimplemented_stub`] and one helper per export
//!
//! Skipped:
//! - The `TRACE`, `FIXME`, `ERR` and `WARN` calls and `debugstr_guid`.
//! - Arguments the C bodies never read or write, such as `pvReserved`, `pguidClass`, `pszwInfId`,
//!   `cmsTimeout` and the `dolock` flag of `LockServer`.
//! - `HeapAlloc` and `HeapFree`. [`inet_cfg_create_instance`] takes a `heap_alloc_ok` flag in
//!   place of the allocation, and dropping a [`NetConfiguration`] stands in for `HeapFree`.
//! - `DllCanUnloadNow`, `DllRegisterServer` and `DllUnregisterServer`. `dlls/netcfgx` has no body
//!   for them; their defaults come from `libs/winecrt0`, so they are not modelled here.
//! - The `__stdcall` calling convention and the vtable layout.

mod class_factory;
pub mod com;
mod dll;
mod exports;
mod netcfg;
mod stub;

pub use class_factory::ClassFactory;
pub use dll::dll_get_class_object;
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use netcfg::{inet_cfg_create_instance, Iface, NetConfiguration};
pub use stub::{
    hr_di_add_component_to_inet_cfg, lana_cfg_from_command_args, modem_class_co_installer,
    net_cfg_diag_from_command_args, net_cfg_diag_repair_registry_bindings, net_class_installer,
    net_prop_page_provider, ras_add_bindings, ras_count_bindings, ras_remove_bindings,
    svchost_change_svchost_group, update_lana_config_using_answerfile,
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
