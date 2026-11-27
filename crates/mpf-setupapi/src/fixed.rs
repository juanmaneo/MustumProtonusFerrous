//! Functions from `dlls/setupapi/stubs.c` whose C bodies return a fixed value.
//!
//! The C bodies ignore their arguments, so the Rust functions take none. A body that calls
//! `SetLastError(ERROR_CALL_NOT_IMPLEMENTED)` is modelled with that code in
//! [`FixedReturn::last_error`].

/// `ERROR_CALL_NOT_IMPLEMENTED` from `include/winerror.h`.
pub const ERROR_CALL_NOT_IMPLEMENTED: u32 = 120;

/// `INVALID_HANDLE_VALUE` from `include/winbase.h`, the `HSPFILELOG` value of a failed call.
const INVALID_HANDLE_VALUE: isize = -1;

/// The value a fixed-return function returns, and the last error it leaves set.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FixedReturn<T> {
    /// The return value.
    pub value: T,
    /// The code the body passes to `SetLastError`, or `None` when it does not call it.
    pub last_error: Option<u32>,
}

const fn returns<T>(value: T) -> FixedReturn<T> {
    FixedReturn {
        value,
        last_error: None,
    }
}

const fn returns_not_implemented<T>(value: T) -> FixedReturn<T> {
    FixedReturn {
        value,
        last_error: Some(ERROR_CALL_NOT_IMPLEMENTED),
    }
}

/// `SetupInitializeFileLogW`: returns `INVALID_HANDLE_VALUE`.
pub fn setup_initialize_file_log_w() -> FixedReturn<isize> {
    returns(INVALID_HANDLE_VALUE)
}

/// `SetupInitializeFileLogA`: returns `INVALID_HANDLE_VALUE`.
pub fn setup_initialize_file_log_a() -> FixedReturn<isize> {
    returns(INVALID_HANDLE_VALUE)
}

/// `SetupTerminateFileLog`: returns `TRUE`.
pub fn setup_terminate_file_log() -> FixedReturn<bool> {
    returns(true)
}

/// `RegistryDelnode`: returns `FALSE`.
pub fn registry_delnode() -> FixedReturn<bool> {
    returns(false)
}

/// `SetupPromptReboot`: returns 0.
pub fn setup_prompt_reboot() -> FixedReturn<i32> {
    returns(0)
}

/// `SetupQueryDrivesInDiskSpaceListA`: returns `FALSE`.
pub fn setup_query_drives_in_disk_space_list_a() -> FixedReturn<bool> {
    returns(false)
}

/// `SetupQueryDrivesInDiskSpaceListW`: returns `FALSE`.
pub fn setup_query_drives_in_disk_space_list_w() -> FixedReturn<bool> {
    returns(false)
}

/// `SetupAddToSourceListA`: returns `TRUE`.
pub fn setup_add_to_source_list_a() -> FixedReturn<bool> {
    returns(true)
}

/// `SetupAddToSourceListW`: returns `TRUE`.
pub fn setup_add_to_source_list_w() -> FixedReturn<bool> {
    returns(true)
}

/// `SetupSetSourceListA`: returns `FALSE`.
pub fn setup_set_source_list_a() -> FixedReturn<bool> {
    returns(false)
}

/// `SetupSetSourceListW`: returns `FALSE`.
pub fn setup_set_source_list_w() -> FixedReturn<bool> {
    returns(false)
}

/// `SetupDiDestroyClassImageList`: returns `TRUE`.
pub fn setup_di_destroy_class_image_list() -> FixedReturn<bool> {
    returns(true)
}

/// `SetupDiGetClassImageList`: returns `FALSE`.
pub fn setup_di_get_class_image_list() -> FixedReturn<bool> {
    returns(false)
}

/// `SetupDiGetClassImageIndex`: returns `FALSE`.
pub fn setup_di_get_class_image_index() -> FixedReturn<bool> {
    returns(false)
}

/// `SetupLogFileW`: returns `FALSE` and sets `ERROR_CALL_NOT_IMPLEMENTED`.
pub fn setup_log_file_w() -> FixedReturn<bool> {
    returns_not_implemented(false)
}

/// `SetupLogFileA`: returns `FALSE` and sets `ERROR_CALL_NOT_IMPLEMENTED`.
pub fn setup_log_file_a() -> FixedReturn<bool> {
    returns_not_implemented(false)
}

/// `SetupDiDestroyDriverInfoList`: returns `FALSE` and sets `ERROR_CALL_NOT_IMPLEMENTED`.
pub fn setup_di_destroy_driver_info_list() -> FixedReturn<bool> {
    returns_not_implemented(false)
}

/// `SetupDiDrawMiniIcon`: returns 0 and sets `ERROR_CALL_NOT_IMPLEMENTED`.
pub fn setup_di_draw_mini_icon() -> FixedReturn<i32> {
    returns_not_implemented(0)
}

/// `SetupDiGetClassBitmapIndex`: returns `FALSE` and sets `ERROR_CALL_NOT_IMPLEMENTED`.
pub fn setup_di_get_class_bitmap_index() -> FixedReturn<bool> {
    returns_not_implemented(false)
}

/// `SetupDiLoadClassIcon`: returns `FALSE` and sets `ERROR_CALL_NOT_IMPLEMENTED`.
pub fn setup_di_load_class_icon() -> FixedReturn<bool> {
    returns_not_implemented(false)
}

/// `SetupDiSetSelectedDevice`: returns `TRUE`.
pub fn setup_di_set_selected_device() -> FixedReturn<bool> {
    returns(true)
}

/// `SetupDiGetClassRegistryPropertyW`: returns `FALSE` and sets `ERROR_CALL_NOT_IMPLEMENTED`.
pub fn setup_di_get_class_registry_property_w() -> FixedReturn<bool> {
    returns_not_implemented(false)
}
