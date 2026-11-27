/// `-DD3D_COMPILER_VERSION=35` from `dlls/d3dcompiler_35/Makefile.in`.
pub const D3D_COMPILER_VERSION: u32 = 35;

/// `E_INVALIDARG` from `include/winerror.h`.
pub const E_INVALIDARG: u32 = 0x8007_0057;

/// The check at the top of `D3DDisassemble` in `dlls/d3dcompiler_43/compiler.c`. An empty input
/// returns `E_INVALIDARG`, but the C check is compiled only for `D3D_COMPILER_VERSION >= 46`.
/// Returns `Some(E_INVALIDARG)` when the call fails here, and `None` when it goes on.
pub fn disassemble_precheck(version: u32, size: usize) -> Option<u32> {
    if version >= 46 && size == 0 {
        Some(E_INVALIDARG)
    } else {
        None
    }
}

/// The export that `dlls/d3dcompiler_43/reflection.c` defines for `version`, and the version it
/// passes to `vkd3d_D3DReflectVKD3D`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReflectForward {
    pub entry: &'static str,
    pub vkd3d_version: u32,
}

/// `D3D10ReflectShader` is compiled only when `D3D_COMPILER_VERSION` is 0, and it passes 0.
/// `D3DReflect` is compiled for every other version, and it passes the version.
pub fn reflect_forward(version: u32) -> ReflectForward {
    if version == 0 {
        ReflectForward {
            entry: "D3D10ReflectShader",
            vkd3d_version: 0,
        }
    } else {
        ReflectForward {
            entry: "D3DReflect",
            vkd3d_version: version,
        }
    }
}
