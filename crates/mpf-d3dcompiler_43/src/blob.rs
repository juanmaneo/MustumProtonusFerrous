//! Blob part and strip checks of Wine's `dlls/d3dcompiler_43/blob.c`.

// BEGIN GENERATED: consts-blob
pub const D3DCOMPILER_STRIP_REFLECTION_DATA: u32 = 0x1;
pub const D3DCOMPILER_STRIP_DEBUG_INFO: u32 = 0x2;
pub const D3D_BLOB_INPUT_SIGNATURE_BLOB: u32 = 0x0;
pub const D3D_BLOB_OUTPUT_SIGNATURE_BLOB: u32 = 0x1;
pub const D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB: u32 = 0x2;
pub const D3D_BLOB_PATCH_CONSTANT_SIGNATURE_BLOB: u32 = 0x3;
pub const D3D_BLOB_ALL_SIGNATURE_BLOB: u32 = 0x4;
pub const D3D_BLOB_DEBUG_INFO: u32 = 0x5;
pub const D3D_BLOB_LEGACY_SHADER: u32 = 0x6;
pub const D3D_BLOB_XNA_PREPASS_SHADER: u32 = 0x7;
pub const D3D_BLOB_XNA_SHADER: u32 = 0x8;
pub const D3D_BLOB_DEBUG_NAME: u32 = 0xc;
pub const D3D_BLOB_TEST_ALTERNATE_SHADER: u32 = 0x8000;
pub const D3D_BLOB_TEST_COMPILE_REPORT: u32 = 0x8003;
pub const DXBC_HEADER_SIZE: usize = 0x20;
pub const TAG_AON9: u32 = 0x396e6f41;
pub const TAG_DXBC: u32 = 0x43425844;
pub const TAG_ISGN: u32 = 0x4e475349;
pub const TAG_OSGN: u32 = 0x4e47534f;
pub const TAG_OSG5: u32 = 0x3547534f;
pub const TAG_PCSG: u32 = 0x47534350;
pub const TAG_RDEF: u32 = 0x46454452;
pub const TAG_SDBG: u32 = 0x47424453;
pub const TAG_STAT: u32 = 0x54415453;
pub const TAG_XNAP: u32 = 0x50414e58;
pub const TAG_XNAS: u32 = 0x53414e58;
// END GENERATED: consts-blob

/// `check_blob_part`: whether a DXBC section with `tag` belongs to blob `part`.
pub fn check_blob_part(tag: u32, part: u32) -> bool {
    match part {
        D3D_BLOB_INPUT_SIGNATURE_BLOB => tag == TAG_ISGN,
        D3D_BLOB_OUTPUT_SIGNATURE_BLOB => matches!(tag, TAG_OSGN | TAG_OSG5),
        D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB => matches!(tag, TAG_ISGN | TAG_OSGN | TAG_OSG5),
        D3D_BLOB_PATCH_CONSTANT_SIGNATURE_BLOB => tag == TAG_PCSG,
        D3D_BLOB_ALL_SIGNATURE_BLOB => matches!(tag, TAG_ISGN | TAG_OSGN | TAG_OSG5 | TAG_PCSG),
        D3D_BLOB_DEBUG_INFO => tag == TAG_SDBG,
        D3D_BLOB_LEGACY_SHADER => tag == TAG_AON9,
        D3D_BLOB_XNA_PREPASS_SHADER => tag == TAG_XNAP,
        D3D_BLOB_XNA_SHADER => tag == TAG_XNAS,
        _ => false,
    }
}

/// `check_blob_strip`: whether a section with `tag` is kept when stripping with `flags`.
pub fn check_blob_strip(tag: u32, flags: u32) -> bool {
    match tag {
        TAG_RDEF | TAG_STAT => (flags & D3DCOMPILER_STRIP_REFLECTION_DATA) == 0,
        TAG_SDBG => (flags & D3DCOMPILER_STRIP_DEBUG_INFO) == 0,
        _ => true,
    }
}

/// The switch in `d3dcompiler_get_blob_part`: a part that needs one, two or three signature
/// sections keeps `section_count` only when it matches, and becomes 0 otherwise. Parts that
/// switch does not handle keep `section_count` unchanged.
pub fn blob_part_section_count(part: u32, section_count: u32) -> u32 {
    let expected = match part {
        D3D_BLOB_INPUT_SIGNATURE_BLOB
        | D3D_BLOB_OUTPUT_SIGNATURE_BLOB
        | D3D_BLOB_PATCH_CONSTANT_SIGNATURE_BLOB
        | D3D_BLOB_DEBUG_INFO
        | D3D_BLOB_LEGACY_SHADER
        | D3D_BLOB_XNA_PREPASS_SHADER
        | D3D_BLOB_XNA_SHADER => 1,
        D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB => 2,
        D3D_BLOB_ALL_SIGNATURE_BLOB => 3,
        _ => return section_count,
    };
    if section_count == expected {
        section_count
    } else {
        0
    }
}

/// The range rule of `d3dcompiler_get_blob_part`: parts past the last test part, and the gap
/// between the test parts and `D3D_BLOB_DEBUG_NAME`.
pub fn blob_part_out_of_range(part: u32) -> bool {
    part > D3D_BLOB_TEST_COMPILE_REPORT
        || (part < D3D_BLOB_TEST_ALTERNATE_SHADER && part > D3D_BLOB_DEBUG_NAME)
}

/// The argument checks of `d3dcompiler_get_blob_part`. `data` is `None` for a null pointer,
/// and its length is the DXBC size. `flags` must be zero, `has_blob` is whether the output
/// pointer is set, and `part` must be in range.
pub fn get_blob_part_args_valid(
    data: Option<&[u8]>,
    part: u32,
    flags: u32,
    has_blob: bool,
) -> bool {
    let Some(data) = data else {
        return false;
    };
    data.len() >= DXBC_HEADER_SIZE && flags == 0 && has_blob && !blob_part_out_of_range(part)
}
