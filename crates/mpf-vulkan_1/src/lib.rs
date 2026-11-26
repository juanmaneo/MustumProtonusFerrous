//! Port of the portable logic in Wine's `dlls/vulkan-1/vulkan.c` and `dlls/vulkan-1/vulkan-1.spec`.
//!
//! Ported:
//! - `DllMain`: [`dll_main`], [`DllMainEffects`], [`DLL_PROCESS_ATTACH`]. The `BOOL` result and
//!   the two OS calls the body makes on attach are modelled as data.
//! - `vulkan-1.spec` export table: [`EXPORTS`], [`Export`], [`ExportKind`], [`find_export`],
//!   [`export_kind`], [`forward_target`]. 252 `@ stdcall` exports forward to `winevulkan.dll`
//!   under the same name; the table keeps the argument type list and the forward target.
//!
//! Modelled:
//! - The ten `@ stub` exports (`vkCreateDisplayModeKHR`, `vkCreateDisplayPlaneSurfaceKHR`,
//!   `vkCreateSharedSwapchainsKHR`, `vkGetCommandPoolMemoryConsumption`,
//!   `vkGetDisplayModePropertiesKHR`, `vkGetDisplayPlaneCapabilitiesKHR`,
//!   `vkGetDisplayPlaneSupportedDisplaysKHR`, `vkGetFaultData`,
//!   `vkGetPhysicalDeviceDisplayPlanePropertiesKHR`,
//!   `vkGetPhysicalDeviceDisplayPropertiesKHR`) and the `__wine_spec_unimplemented_stub` record
//!   they raise: [`wine_spec_unimplemented_stub`], [`stub_exception`], [`StubException`], one
//!   helper per export.
//!
//! Skipped:
//! - The forwarded calls themselves: they run in `winevulkan.dll`, not in this DLL.
//! - The OS effects of `DisableThreadLibraryCalls` and `GetDpiForSystem`.
//! - The `__stdcall` calling convention.

mod dll_main;
mod exports;
mod stub;

pub use dll_main::{dll_main, DllMainEffects, DLL_PROCESS_ATTACH};
pub use exports::{export_kind, find_export, forward_target, Export, ExportKind, EXPORTS};
pub use stub::*;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
