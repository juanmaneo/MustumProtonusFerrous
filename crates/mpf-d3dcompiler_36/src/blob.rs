//! `d3dcompiler_get_blob_part` from `dlls/d3dcompiler_43/blob.c`, the body behind the
//! `D3DGetInputSignatureBlob`, `D3DGetOutputSignatureBlob`, `D3DGetInputAndOutputSignatureBlob`
//! and `D3DGetDebugInfo` exports (the last one is `D3DGetCodeDebugInfo` in the 36 spec).

use crate::hresult::{D3DERR_INVALIDCALL, E_FAIL};

/// `DXBC_HEADER_SIZE` from `dlls/d3dcompiler_43/d3dcompiler_private.h`: eight 32-bit words.
pub const DXBC_HEADER_SIZE: usize = 8 * 4;

/// `D3D_BLOB_INPUT_SIGNATURE_BLOB` from `include/d3dcompiler.h`.
pub const D3D_BLOB_INPUT_SIGNATURE_BLOB: u32 = 0;
/// `D3D_BLOB_OUTPUT_SIGNATURE_BLOB` from `include/d3dcompiler.h`.
pub const D3D_BLOB_OUTPUT_SIGNATURE_BLOB: u32 = 1;
/// `D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB` from `include/d3dcompiler.h`.
pub const D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB: u32 = 2;
/// `D3D_BLOB_PATCH_CONSTANT_SIGNATURE_BLOB` from `include/d3dcompiler.h`.
pub const D3D_BLOB_PATCH_CONSTANT_SIGNATURE_BLOB: u32 = 3;
/// `D3D_BLOB_ALL_SIGNATURE_BLOB` from `include/d3dcompiler.h`.
pub const D3D_BLOB_ALL_SIGNATURE_BLOB: u32 = 4;
/// `D3D_BLOB_DEBUG_INFO` from `include/d3dcompiler.h`.
pub const D3D_BLOB_DEBUG_INFO: u32 = 5;
/// `D3D_BLOB_LEGACY_SHADER` from `include/d3dcompiler.h`.
pub const D3D_BLOB_LEGACY_SHADER: u32 = 6;
/// `D3D_BLOB_XNA_PREPASS_SHADER` from `include/d3dcompiler.h`.
pub const D3D_BLOB_XNA_PREPASS_SHADER: u32 = 7;
/// `D3D_BLOB_XNA_SHADER` from `include/d3dcompiler.h`.
pub const D3D_BLOB_XNA_SHADER: u32 = 8;
/// `D3D_BLOB_PDB` from `include/d3dcompiler.h`. `get_blob_part` accepts it but never selects a section for it.
pub const D3D_BLOB_PDB: u32 = 9;
/// `D3D_BLOB_DEBUG_NAME` from `include/d3dcompiler.h`, the last part below the test range.
pub const D3D_BLOB_DEBUG_NAME: u32 = 12;
/// `D3D_BLOB_TEST_ALTERNATE_SHADER` from `include/d3dcompiler.h`, the first test part.
pub const D3D_BLOB_TEST_ALTERNATE_SHADER: u32 = 0x8000;
/// `D3D_BLOB_TEST_COMPILE_REPORT` from `include/d3dcompiler.h`, the last test part.
pub const D3D_BLOB_TEST_COMPILE_REPORT: u32 = 0x8003;

/// `MAKE_TAG` values from `dlls/d3dcompiler_43/d3dcompiler_private.h`. The composition is
/// little-endian, so the tag of `"ISGN"` is `from_le_bytes(b"ISGN")`.
pub const TAG_AON9: u32 = u32::from_le_bytes(*b"Aon9");
/// `TAG_ISGN`.
pub const TAG_ISGN: u32 = u32::from_le_bytes(*b"ISGN");
/// `TAG_OSGN`.
pub const TAG_OSGN: u32 = u32::from_le_bytes(*b"OSGN");
/// `TAG_OSG5`.
pub const TAG_OSG5: u32 = u32::from_le_bytes(*b"OSG5");
/// `TAG_PCSG`.
pub const TAG_PCSG: u32 = u32::from_le_bytes(*b"PCSG");
/// `TAG_SDBG`.
pub const TAG_SDBG: u32 = u32::from_le_bytes(*b"SDBG");
/// `TAG_XNAP`.
pub const TAG_XNAP: u32 = u32::from_le_bytes(*b"XNAP");
/// `TAG_XNAS`.
pub const TAG_XNAS: u32 = u32::from_le_bytes(*b"XNAS");

/// One section of a parsed DXBC container: `struct vkd3d_shader_dxbc_section_desc`.
///
/// The container parse (`vkd3d_shader_parse_dxbc`) is not ported. The caller supplies the sections.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Section<'a> {
    pub tag: u32,
    pub data: &'a [u8],
}

/// What `get_blob_part` produces, before the blob is allocated.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BlobPart<'a> {
    /// The data of the only section, copied into the blob unchanged. The C code does this for
    /// `D3D_BLOB_DEBUG_INFO`, `D3D_BLOB_LEGACY_SHADER` and the two XNA parts, because they are not
    /// full DXBC containers.
    Verbatim(&'a [u8]),
    /// The sections that `vkd3d_shader_serialize_dxbc` packs into a DXBC container. The packing is
    /// not ported.
    Container(Vec<Section<'a>>),
}

/// `check_blob_part`: whether a section with `tag` belongs to `part`.
pub(crate) fn check_blob_part(tag: u32, part: u32) -> bool {
    match part {
        D3D_BLOB_INPUT_SIGNATURE_BLOB => tag == TAG_ISGN,
        D3D_BLOB_OUTPUT_SIGNATURE_BLOB => tag == TAG_OSGN || tag == TAG_OSG5,
        D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB => {
            tag == TAG_ISGN || tag == TAG_OSGN || tag == TAG_OSG5
        }
        D3D_BLOB_PATCH_CONSTANT_SIGNATURE_BLOB => tag == TAG_PCSG,
        D3D_BLOB_ALL_SIGNATURE_BLOB => matches!(tag, TAG_ISGN | TAG_OSGN | TAG_OSG5 | TAG_PCSG),
        D3D_BLOB_DEBUG_INFO => tag == TAG_SDBG,
        D3D_BLOB_LEGACY_SHADER => tag == TAG_AON9,
        D3D_BLOB_XNA_PREPASS_SHADER => tag == TAG_XNAP,
        D3D_BLOB_XNA_SHADER => tag == TAG_XNAS,
        _ => false,
    }
}

/// The section count that `get_blob_part` requires for `part`. The C `default` arm leaves the
/// count unchanged, which here is `true`. Such a part selects no section, so it still fails.
pub(crate) fn section_count_is_valid(part: u32, count: usize) -> bool {
    match part {
        D3D_BLOB_INPUT_SIGNATURE_BLOB
        | D3D_BLOB_OUTPUT_SIGNATURE_BLOB
        | D3D_BLOB_PATCH_CONSTANT_SIGNATURE_BLOB
        | D3D_BLOB_DEBUG_INFO
        | D3D_BLOB_LEGACY_SHADER
        | D3D_BLOB_XNA_PREPASS_SHADER
        | D3D_BLOB_XNA_SHADER => count == 1,
        D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB => count == 2,
        D3D_BLOB_ALL_SIGNATURE_BLOB => count == 3,
        _ => true,
    }
}

/// `d3dcompiler_get_blob_part` from `dlls/d3dcompiler_43/blob.c`, after the container parse.
///
/// `data` is `None` for a NULL pointer, and its length is `data_size`. `blob_is_set` is whether the
/// output pointer is non-NULL. `sections` is the parse result, or `None` when the parse fails.
/// Errors: `D3DERR_INVALIDCALL` for bad arguments and an out-of-range part, `E_FAIL` for a parse
/// failure or a section count the part does not allow.
pub fn get_blob_part<'a>(
    data: Option<&[u8]>,
    flags: u32,
    part: u32,
    blob_is_set: bool,
    sections: Option<&[Section<'a>]>,
) -> Result<BlobPart<'a>, u32> {
    let Some(data) = data else {
        return Err(D3DERR_INVALIDCALL);
    };
    if data.len() < DXBC_HEADER_SIZE || flags != 0 || !blob_is_set {
        return Err(D3DERR_INVALIDCALL);
    }
    if part > D3D_BLOB_TEST_COMPILE_REPORT
        || (part < D3D_BLOB_TEST_ALTERNATE_SHADER && part > D3D_BLOB_DEBUG_NAME)
    {
        return Err(D3DERR_INVALIDCALL);
    }
    let Some(sections) = sections else {
        return Err(E_FAIL);
    };

    let selected: Vec<Section<'a>> = sections
        .iter()
        .filter(|section| check_blob_part(section.tag, part))
        .copied()
        .collect();
    if !section_count_is_valid(part, selected.len()) || selected.is_empty() {
        return Err(E_FAIL);
    }

    if selected.len() == 1
        && matches!(
            part,
            D3D_BLOB_DEBUG_INFO
                | D3D_BLOB_LEGACY_SHADER
                | D3D_BLOB_XNA_PREPASS_SHADER
                | D3D_BLOB_XNA_SHADER
        )
    {
        return Ok(BlobPart::Verbatim(selected[0].data));
    }
    Ok(BlobPart::Container(selected))
}
