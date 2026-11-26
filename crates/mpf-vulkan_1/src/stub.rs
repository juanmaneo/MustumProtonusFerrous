/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = vulkan-1.dll` in `dlls/vulkan-1/Makefile.in`.
const VULKAN_1_DLL_NAME: &str = "vulkan-1.dll";

/// The record that `__wine_spec_unimplemented_stub` raises.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`. The C function raises
/// the record in a loop and never returns, so the port returns the record instead.
pub fn wine_spec_unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        // The module and the function name.
        number_parameters: 2,
        module,
        function,
    }
}

/// The exception raised by the `@ stub` export `name`, or `None` when `name` is not a stub
/// export of `vulkan-1.spec`.
pub fn stub_exception(name: &str) -> Option<StubException> {
    let export = crate::find_export(name)?;
    match export.kind {
        crate::ExportKind::Stub => {
            Some(wine_spec_unimplemented_stub(VULKAN_1_DLL_NAME, export.name))
        }
        crate::ExportKind::Forward(_) => None,
    }
}

/// `vkCreateDisplayModeKHR`, exported as `@ stub` in `dlls/vulkan-1/vulkan-1.spec`.
pub fn vk_create_display_mode_khr() -> StubException {
    wine_spec_unimplemented_stub(VULKAN_1_DLL_NAME, "vkCreateDisplayModeKHR")
}

/// `vkCreateDisplayPlaneSurfaceKHR`, exported as `@ stub` in `dlls/vulkan-1/vulkan-1.spec`.
pub fn vk_create_display_plane_surface_khr() -> StubException {
    wine_spec_unimplemented_stub(VULKAN_1_DLL_NAME, "vkCreateDisplayPlaneSurfaceKHR")
}

/// `vkCreateSharedSwapchainsKHR`, exported as `@ stub` in `dlls/vulkan-1/vulkan-1.spec`.
pub fn vk_create_shared_swapchains_khr() -> StubException {
    wine_spec_unimplemented_stub(VULKAN_1_DLL_NAME, "vkCreateSharedSwapchainsKHR")
}

/// `vkGetCommandPoolMemoryConsumption`, exported as `@ stub` in `dlls/vulkan-1/vulkan-1.spec`.
pub fn vk_get_command_pool_memory_consumption() -> StubException {
    wine_spec_unimplemented_stub(VULKAN_1_DLL_NAME, "vkGetCommandPoolMemoryConsumption")
}

/// `vkGetDisplayModePropertiesKHR`, exported as `@ stub` in `dlls/vulkan-1/vulkan-1.spec`.
pub fn vk_get_display_mode_properties_khr() -> StubException {
    wine_spec_unimplemented_stub(VULKAN_1_DLL_NAME, "vkGetDisplayModePropertiesKHR")
}

/// `vkGetDisplayPlaneCapabilitiesKHR`, exported as `@ stub` in `dlls/vulkan-1/vulkan-1.spec`.
pub fn vk_get_display_plane_capabilities_khr() -> StubException {
    wine_spec_unimplemented_stub(VULKAN_1_DLL_NAME, "vkGetDisplayPlaneCapabilitiesKHR")
}

/// `vkGetDisplayPlaneSupportedDisplaysKHR`, exported as `@ stub` in `dlls/vulkan-1/vulkan-1.spec`.
pub fn vk_get_display_plane_supported_displays_khr() -> StubException {
    wine_spec_unimplemented_stub(VULKAN_1_DLL_NAME, "vkGetDisplayPlaneSupportedDisplaysKHR")
}

/// `vkGetFaultData`, exported as `@ stub` in `dlls/vulkan-1/vulkan-1.spec`.
pub fn vk_get_fault_data() -> StubException {
    wine_spec_unimplemented_stub(VULKAN_1_DLL_NAME, "vkGetFaultData")
}

/// `vkGetPhysicalDeviceDisplayPlanePropertiesKHR`, exported as `@ stub` in `dlls/vulkan-1/vulkan-1.spec`.
pub fn vk_get_physical_device_display_plane_properties_khr() -> StubException {
    wine_spec_unimplemented_stub(
        VULKAN_1_DLL_NAME,
        "vkGetPhysicalDeviceDisplayPlanePropertiesKHR",
    )
}

/// `vkGetPhysicalDeviceDisplayPropertiesKHR`, exported as `@ stub` in `dlls/vulkan-1/vulkan-1.spec`.
pub fn vk_get_physical_device_display_properties_khr() -> StubException {
    wine_spec_unimplemented_stub(VULKAN_1_DLL_NAME, "vkGetPhysicalDeviceDisplayPropertiesKHR")
}
