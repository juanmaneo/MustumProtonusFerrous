//! Deterministic part of Wine's `dlls/opencl`: the host extension filter behind
//! `clGetPlatformInfo` and `clGetDeviceInfo` (`pe_wrappers.c`, `pe_thunks.c`), and the
//! `CL_EXEC_NATIVE_KERNEL` mask that hides native-kernel execution from applications.

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub const CL_SUCCESS: i32 = 0;
pub const CL_INVALID_VALUE: i32 = -30;
pub const CL_EXEC_NATIVE_KERNEL: u64 = 1 << 1;

/// Extensions the PE side does not expose.
const UNSUPPORTED: &[&str] = &[
    "cl_apple_contextloggingfunctions",
    "cl_apple_setmemobjectdestructor",
    "cl_arm_import_memory",
    "cl_arm_shared_virtual_memory",
    "cl_ext_device_fission",
    "cl_ext_migrate_memobject",
    "cl_img_generate_mipmap",
    "cl_img_use_gralloc_ptr",
    "cl_intel_accelerator",
    "cl_intel_create_buffer_with_properties",
    "cl_intel_d3d11_nv12_media_sharing",
    "cl_intel_dx9_media_sharing",
    "cl_intel_unified_shared_memory",
    "cl_intel_va_api_media_sharing",
    "cl_khr_create_command_queue",
    "cl_khr_d3d10_sharing",
    "cl_khr_d3d11_sharing",
    "cl_khr_dx9_media_sharing",
    "cl_khr_egl_event",
    "cl_khr_egl_image",
    "cl_khr_gl_event",
    "cl_khr_gl_sharing",
    "cl_khr_icd",
    "cl_khr_il_program",
    "cl_khr_subgroups",
    "cl_khr_terminate_context",
    "cl_loader_layers",
    "cl_nv_d3d10_sharing",
    "cl_nv_d3d11_sharing",
    "cl_nv_d3d9_sharing",
    "cl_qcom_ext_host_ptr",
];

/// `extension_is_supported`. A name is blocked when it is a case-insensitive prefix of a blocked
/// extension, so `cl_khr_gl` is blocked together with `cl_khr_gl_sharing`.
pub fn extension_is_supported(name: &[u8]) -> bool {
    !UNSUPPORTED
        .iter()
        .any(|blocked| prefix_matches_ignore_case(name, blocked.as_bytes()))
}

/// `_strnicmp(name, blocked, name.len()) == 0`, which also stops early where both strings end.
fn prefix_matches_ignore_case(name: &[u8], blocked: &[u8]) -> bool {
    for (i, &c) in name.iter().enumerate() {
        let lower = c.to_ascii_lowercase();
        let other = blocked.get(i).map_or(0, u8::to_ascii_lowercase);
        if lower != other {
            return false;
        }
        if lower == 0 {
            return true;
        }
    }
    true
}

/// The space-separated tokens of `exts` that are supported, each with its start offset. Empty
/// tokens are never supported, so repeated and trailing spaces need no special case.
fn supported_tokens(exts: &[u8]) -> impl Iterator<Item = (usize, &[u8])> {
    exts.split(|&c| c == b' ')
        .scan(0, |start, token| {
            let at = *start;
            *start += token.len() + 1;
            Some((at, token))
        })
        .filter(|&(_, token)| extension_is_supported(token))
}

/// `filter_extensions`. Returns the status and the size the caller needs. With `win_exts`, the
/// supported extensions are written space-separated and NUL-terminated.
///
/// Wine counts `strlen` of the rest of the input for each kept extension, so the reported size
/// can exceed the written length. A buffer shorter than the reported size is `CL_INVALID_VALUE`.
/// Input is read up to its first NUL, as a C string would be.
pub fn filter_extensions(unix_exts: &[u8], win_exts: Option<&mut [u8]>) -> (i32, usize) {
    let end = unix_exts
        .iter()
        .position(|&c| c == 0)
        .unwrap_or(unix_exts.len());
    let unix_exts = &unix_exts[..end];
    let required = supported_tokens(unix_exts)
        .map(|(start, _)| unix_exts.len() - start + 1)
        .sum::<usize>();
    let Some(out) = win_exts else {
        return (CL_SUCCESS, required);
    };
    // The terminator is written even when nothing is supported, so an empty buffer cannot hold it.
    if out.len() < required.max(1) {
        return (CL_INVALID_VALUE, required);
    }
    let mut written = 0;
    for (_, token) in supported_tokens(unix_exts) {
        if written != 0 {
            out[written] = b' ';
            written += 1;
        }
        out[written..written + token.len()].copy_from_slice(token);
        written += token.len();
    }
    out[written] = 0;
    (CL_SUCCESS, required)
}

/// Clears `CL_EXEC_NATIVE_KERNEL` from a `cl_device_exec_capabilities` value.
pub fn filter_exec_capabilities(caps: u64) -> u64 {
    caps & !CL_EXEC_NATIVE_KERNEL
}
