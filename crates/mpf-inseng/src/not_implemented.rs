//! The `InstallEngine` and `InstallEngine2` methods in `inseng_main.c` that trace their
//! arguments and return `E_NOTIMPL`. The arguments are not modelled.

use crate::com::{HResult, E_NOTIMPL};

pub fn install_engine_get_engine_status() -> HResult {
    E_NOTIMPL
}

pub fn install_engine_set_cif_file() -> HResult {
    E_NOTIMPL
}

pub fn install_engine_download_components() -> HResult {
    E_NOTIMPL
}

pub fn install_engine_install_components() -> HResult {
    E_NOTIMPL
}

pub fn install_engine_enum_install_ids() -> HResult {
    E_NOTIMPL
}

pub fn install_engine_enum_download_ids() -> HResult {
    E_NOTIMPL
}

pub fn install_engine_is_component_installed() -> HResult {
    E_NOTIMPL
}

pub fn install_engine_register_install_engine_callback() -> HResult {
    E_NOTIMPL
}

pub fn install_engine_unregister_install_engine_callback() -> HResult {
    E_NOTIMPL
}

pub fn install_engine_set_action() -> HResult {
    E_NOTIMPL
}

pub fn install_engine_get_sizes() -> HResult {
    E_NOTIMPL
}

pub fn install_engine_launch_extra_command() -> HResult {
    E_NOTIMPL
}

pub fn install_engine_get_display_name() -> HResult {
    E_NOTIMPL
}

pub fn install_engine_set_base_url() -> HResult {
    E_NOTIMPL
}

pub fn install_engine_set_download_dir() -> HResult {
    E_NOTIMPL
}

pub fn install_engine_set_install_drive() -> HResult {
    E_NOTIMPL
}

pub fn install_engine_set_install_options() -> HResult {
    E_NOTIMPL
}

pub fn install_engine_set_hwnd() -> HResult {
    E_NOTIMPL
}

pub fn install_engine_set_istream() -> HResult {
    E_NOTIMPL
}

pub fn install_engine_abort() -> HResult {
    E_NOTIMPL
}

pub fn install_engine_suspend() -> HResult {
    E_NOTIMPL
}

pub fn install_engine_resume() -> HResult {
    E_NOTIMPL
}

pub fn install_engine2_set_local_cif() -> HResult {
    E_NOTIMPL
}

pub fn install_engine2_get_icif_file() -> HResult {
    E_NOTIMPL
}
