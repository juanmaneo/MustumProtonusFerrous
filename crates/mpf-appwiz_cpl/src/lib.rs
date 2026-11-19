//! Deterministic part of Wine's `dlls/appwiz.cpl`: the `CPL_STARTWPARMSW` parameter check in
//! `appwiz.c`, and the addon installer helpers in `addons.c`.
//!
//! Ported:
//! - `start_params`: maps `install_gecko` and `install_mono` to an addon. A NULL or unknown
//!   parameter installs nothing.
//! - `append_url_params`, `get_url`: the `?arch=&v=&winev=` query and the registry or default URL.
//! - `digest_matches`: the lowercase hex comparison in `sha_check`.
//! - `build_install_path`, `install_target`: path composition and the `\\?\` strip of
//!   `install_from_file`.
//! - `install_file_result`: the `ERROR_PRODUCT_VERSION` retry in `install_file`.
//! - `default_dir_candidates`, `search_dirs`, `first_settled`: the directory order of
//!   `install_from_default_dir` and the source order of `install_addon`.
//! - `cache_file_path`: `get_cache_file_name` with `ensure_exists` false.
//!
//! Skipped: the applet UI, registry reads, file and MSI calls, the download and dialogs, and the
//! directory creation of `get_cache_file_name`.
//!
//! Targets the x86_64 host, as the `__x86_64__` branches of `addons.c`.

mod addon;
mod digest;
mod install;
mod paths;
mod url;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use addon::Addon;
pub use digest::digest_matches;
pub use install::{first_settled, install_file_result, install_target, search_dirs, InstallRes};
pub use paths::{build_install_path, cache_file_path, default_dir_candidates};
pub use url::{append_url_params, get_url, INTERNET_MAX_URL_LENGTH, REG_SZ};

/// `start_params`. `Some(addon)` means the call returns TRUE and installs `addon`. `None` means
/// FALSE, for a NULL parameter or an unknown one. Text after a NUL is ignored, as `wcscmp` does.
pub fn start_params(params: Option<&[u16]>) -> Option<Addon> {
    let params = c_str(params?);
    if eq_str(params, "install_gecko") {
        Some(Addon::Gecko)
    } else if eq_str(params, "install_mono") {
        Some(Addon::Mono)
    } else {
        None
    }
}

fn eq_str(units: &[u16], text: &str) -> bool {
    units.iter().copied().eq(text.encode_utf16())
}

/// The units of a NUL-terminated string: up to the first NUL, or the whole slice without one.
pub(crate) fn c_str(units: &[u16]) -> &[u16] {
    let end = units
        .iter()
        .position(|&unit| unit == 0)
        .unwrap_or(units.len());
    &units[..end]
}
