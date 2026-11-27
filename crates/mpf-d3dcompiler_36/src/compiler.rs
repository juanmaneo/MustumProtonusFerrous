//! Argument and path logic from `dlls/d3dcompiler_43/compiler.c`, the source of `dlls/d3dcompiler_36`:
//! the gate at the top of `D3DPreprocess`, the source choice in `D3DDisassemble`, and the path that
//! `d3dcompiler_include_from_file_open` builds for an include.

use crate::hresult::{E_FAIL, E_INVALIDARG};

/// `MAX_PATH` from `include/windef.h`.
pub const MAX_PATH: usize = 260;

/// `TAG_DXBC` from `dlls/d3dcompiler_43/d3dcompiler_private.h`.
pub const TAG_DXBC: u32 = u32::from_le_bytes(*b"DXBC");

/// The `VKD3D_SHADER_SOURCE_*` value that `D3DDisassemble` chooses for its input.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceType {
    /// `VKD3D_SHADER_SOURCE_DXBC_TPF`: the input starts with the `DXBC` tag.
    DxbcTpf,
    /// `VKD3D_SHADER_SOURCE_D3D_BYTECODE`: anything else, including inputs shorter than four bytes.
    D3dBytecode,
}

/// The source choice in `D3DDisassemble`: a `DXBC` tag in the first four bytes selects the DXBC
/// source. The C code reads the tag only when `size >= 4`.
pub fn disassemble_source_type(data: &[u8]) -> SourceType {
    match data.first_chunk::<4>() {
        Some(tag) if u32::from_le_bytes(*tag) == TAG_DXBC => SourceType::DxbcTpf,
        _ => SourceType::D3dBytecode,
    }
}

/// The argument gate at the top of `D3DPreprocess`: `if (!data || !shader) return E_INVALIDARG;`.
///
/// `data` is `None` for a NULL pointer. `shader_is_set` is whether the output pointer is non-NULL.
pub fn check_preprocess_args(data: Option<&[u8]>, shader_is_set: bool) -> Result<(), u32> {
    if data.is_none() || !shader_is_set {
        return Err(E_INVALIDARG);
    }
    Ok(())
}

/// The path that `d3dcompiler_include_from_file_open` builds for `include_name`.
///
/// When `initial_filename` has a backslash, the include is looked up in the directory of
/// `initial_filename`, which is everything up to and including the last backslash. Otherwise it is
/// looked up in the current directory, which the caller passes as `current_dir`. That is `None`
/// when `GetCurrentDirectoryA` returned 0. An empty or `MAX_PATH`-or-longer directory fails with
/// `E_FAIL`, as in the C code.
///
/// The open itself, the file read and the allocation are outside the port. The `E_FAIL` for a
/// missing file comes from them, not from this function.
pub fn include_full_path(
    initial_filename: &str,
    include_name: &str,
    current_dir: Option<&str>,
) -> Result<String, u32> {
    let dir = match initial_filename.rfind('\\') {
        Some(pos) => initial_filename[..=pos].to_owned(),
        None => match current_dir {
            Some(cwd) if !cwd.is_empty() && cwd.len() < MAX_PATH => format!("{cwd}\\"),
            _ => return Err(E_FAIL),
        },
    };
    Ok(format!("{dir}{include_name}"))
}
