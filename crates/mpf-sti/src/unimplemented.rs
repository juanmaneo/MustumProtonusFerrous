//! The `IStillImageW` methods whose C bodies only trace their arguments and return `E_NOTIMPL`
//! (`dlls/sti/sti.c`). The arguments are never read, so the functions take none.

use crate::com::{Hresult, E_NOTIMPL};

pub fn get_device_list() -> Hresult {
    E_NOTIMPL
}

pub fn get_device_info() -> Hresult {
    E_NOTIMPL
}

pub fn create_device() -> Hresult {
    E_NOTIMPL
}

pub fn get_device_value() -> Hresult {
    E_NOTIMPL
}

pub fn set_device_value() -> Hresult {
    E_NOTIMPL
}

pub fn get_sti_launch_information() -> Hresult {
    E_NOTIMPL
}

pub fn enable_hw_notifications() -> Hresult {
    E_NOTIMPL
}

pub fn get_hw_notification_state() -> Hresult {
    E_NOTIMPL
}

pub fn refresh_device_bus() -> Hresult {
    E_NOTIMPL
}

pub fn launch_application_for_device() -> Hresult {
    E_NOTIMPL
}

pub fn setup_device_parameters() -> Hresult {
    E_NOTIMPL
}

pub fn write_to_error_log() -> Hresult {
    E_NOTIMPL
}
