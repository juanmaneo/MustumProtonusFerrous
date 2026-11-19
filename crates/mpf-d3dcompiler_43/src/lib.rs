//! Deterministic part of Wine's `dlls/d3dcompiler_43`: the blob part and strip checks of
//! `blob.c`, and the bytecode writer token mappings of `bytecodewriter.c`.
//!
//! Ported:
//! - `blob`: `check_blob_part` and `check_blob_strip`, the section count and part range rules
//!   of `d3dcompiler_get_blob_part`, and its argument checks.
//! - `lib`: `sm1_version`, the shader version token of `bytecodewriter.c`.
//! - `token`: `d3d9_swizzle`, `d3d9_writemask`, `d3d9_srcmod`, `d3d9_dstmod`,
//!   `d3d9_comparetype`, `d3d9_sampler`, `d3d9_register` and `d3dsp_register`.
//! - `opcode`: `d3d9_opcode`.
//!
//! Skipped, not portable without blob allocation, DXBC container parsing or compiler state:
//! - `blob.c`: the exported `D3D*` functions and `d3dcompiler_get_blob_part` itself. They
//!   allocate blobs and read or write DXBC containers.
//! - `bytecodewriter.c`: the bytecode buffer and writer, and the backend tables.
//! - `asmparser.c`, `asmshader.l` and `asmshader.y`: the assembler front end.
//! - `compiler.c`, `linker.c`, `reflection.c` and `d3dcompiler_43.spec`: compiler, linker and
//!   reflection state, and the export table.
//! - `utils.c`: the `debug_*` formatters, which only produce trace output.
//!
//! Oracle provenance: the ported functions are copied verbatim from Wine's `blob.c` and
//! `bytecodewriter.c`, and the enums and macros come from `include/d3d9types.h`,
//! `include/d3dcompiler.h` and `d3dcompiler_private.h`. The extract compiles under
//! `gcc -fsanitize=address,undefined` against scalar shims in `target/canon/d3dcompiler_43/`
//! (not tracked in git).

pub mod blob;
pub mod opcode;
pub mod token;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// `enum shader_type` from `d3dcompiler_private.h`: `ST_UNKNOWN`, `ST_VERTEX` and `ST_PIXEL`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShaderType {
    Unknown,
    Vertex,
    Pixel,
}

/// The shader version token for `shader_type`. Unknown shader types give 0.
pub fn sm1_version(shader_type: ShaderType, major: u8, minor: u8) -> u32 {
    let version = (u32::from(major) << 8) | u32::from(minor);
    match shader_type {
        ShaderType::Unknown => 0,
        ShaderType::Vertex => 0xfffe_0000 | version,
        ShaderType::Pixel => 0xffff_0000 | version,
    }
}
