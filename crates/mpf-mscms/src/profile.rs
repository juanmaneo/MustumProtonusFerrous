//! Port of the pure parts of `dlls/mscms/profile.c`: `basename`, `GetColorDirectoryW`,
//! `GetStandardColorSpaceProfileW`, the `ENUMTYPEW` validity check and `match_profile`.
//!
//! Paths are UTF-16, as in Wine. `system_dir` is the `GetSystemDirectoryW` result, which is
//! shorter than `MAX_PATH`; the port does not model Wine's fixed-size stack buffers. Sizes are
//! in bytes and exclude the NUL terminator.

use crate::error::McsError;
use crate::icc::ProfileHeader;

/// `ENUM_TYPE_VERSION`.
pub const ENUM_TYPE_VERSION: u32 = 0x0300;
/// `sizeof(ENUMTYPEW)` on 64-bit targets, the layout this port's record follows.
pub const ENUMTYPEW_SIZE: u32 = 96;
/// `LCS_sRGB`.
pub const LCS_SRGB: u32 = 0x7352_4742;
/// `LCS_WINDOWS_COLOR_SPACE`.
pub const LCS_WINDOWS_COLOR_SPACE: u32 = 0x5769_6e20;

pub const ET_CMMTYPE: u32 = 0x0000_0010;
pub const ET_CLASS: u32 = 0x0000_0020;
pub const ET_DATACOLORSPACE: u32 = 0x0000_0040;
pub const ET_CONNECTIONSPACE: u32 = 0x0000_0080;
pub const ET_SIGNATURE: u32 = 0x0000_0100;
pub const ET_PLATFORM: u32 = 0x0000_0200;
pub const ET_PROFILEFLAGS: u32 = 0x0000_0400;
pub const ET_MANUFACTURER: u32 = 0x0000_0800;
pub const ET_MODEL: u32 = 0x0000_1000;
pub const ET_ATTRIBUTES: u32 = 0x0000_2000;
pub const ET_RENDERINGINTENT: u32 = 0x0000_4000;
pub const ET_CREATOR: u32 = 0x0000_8000;

/// The fields of `ENUMTYPEW` that `match_profile` reads. Device name, media type, dithering
/// mode, resolution and device class only raise `FIXME`s in Wine, so they are not modelled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumRecord {
    pub size: u32,
    pub version: u32,
    pub fields: u32,
    pub cmm_type: u32,
    pub class: u32,
    pub data_color_space: u32,
    pub connection_space: u32,
    pub signature: u32,
    pub platform: u32,
    pub profile_flags: u32,
    pub manufacturer: u32,
    pub model: u32,
    pub attributes: [u32; 2],
    pub rendering_intent: u32,
    pub creator: u32,
}

impl EnumRecord {
    /// The size and version check that `EnumColorProfilesW` applies before enumerating.
    pub fn is_valid(&self) -> bool {
        self.size == ENUMTYPEW_SIZE && self.version == ENUM_TYPE_VERSION
    }
}

/// `basename`: the part of `path` after its last `\` or `/`.
pub fn basename(path: &[u16]) -> &[u16] {
    let start = path
        .iter()
        .rposition(|&unit| unit == u16::from(b'\\') || unit == u16::from(b'/'))
        .map_or(0, |index| index + 1);
    &path[start..]
}

/// `GetColorDirectoryW` for the system colour directory. `capacity` is the caller's buffer in
/// bytes, `None` for a NULL buffer. On failure the error is the size Wine reports.
pub fn color_directory(system_dir: &[u16], capacity: Option<u32>) -> Result<Vec<u16>, u32> {
    let dir = color_dir_path(system_dir);
    fit(byte_len(&dir), capacity)?;
    Ok(dir)
}

/// `GetStandardColorSpaceProfileW` for the sRGB profile. `size` is the caller's size in bytes
/// (`None` for a NULL size pointer). `buffer` tells whether the caller passed a buffer.
pub fn standard_profile(
    machine: bool,
    id: u32,
    system_dir: &[u16],
    size: Option<u32>,
    buffer: bool,
) -> Result<Vec<u16>, McsError> {
    if machine {
        return Err(McsError::NotSupported);
    }
    let capacity = size.ok_or(McsError::InvalidParameter)?;
    if !buffer {
        return Err(McsError::InsufficientBuffer);
    }
    if id != LCS_SRGB && id != LCS_WINDOWS_COLOR_SPACE {
        return Err(McsError::FileNotFound);
    }
    let mut path = color_dir_path(system_dir);
    path.extend(wide(r"\srgb color space profile.icm"));
    fit(byte_len(&path), Some(capacity)).map_err(McsError::MoreData)?;
    Ok(path)
}

/// `match_profile`: whether the header satisfies every field the record selects. Fields not
/// selected in `fields` are ignored.
pub fn match_profile(rec: &EnumRecord, hdr: &ProfileHeader) -> bool {
    let fields = rec.fields;
    field_matches(fields, ET_CMMTYPE, rec.cmm_type == hdr.cmm_type)
        && field_matches(fields, ET_CLASS, rec.class == hdr.class)
        && field_matches(
            fields,
            ET_DATACOLORSPACE,
            rec.data_color_space == hdr.data_color_space,
        )
        && field_matches(
            fields,
            ET_CONNECTIONSPACE,
            rec.connection_space == hdr.connection_space,
        )
        && field_matches(fields, ET_SIGNATURE, rec.signature == hdr.signature)
        && field_matches(fields, ET_PLATFORM, rec.platform == hdr.platform)
        && field_matches(
            fields,
            ET_PROFILEFLAGS,
            rec.profile_flags == hdr.profile_flags,
        )
        && field_matches(
            fields,
            ET_MANUFACTURER,
            rec.manufacturer == hdr.manufacturer,
        )
        && field_matches(fields, ET_MODEL, rec.model == hdr.model)
        && field_matches(fields, ET_ATTRIBUTES, rec.attributes == hdr.attributes)
        && field_matches(
            fields,
            ET_RENDERINGINTENT,
            rec.rendering_intent == hdr.rendering_intent,
        )
        && field_matches(fields, ET_CREATOR, rec.creator == hdr.creator)
}

fn field_matches(fields: u32, flag: u32, equal: bool) -> bool {
    (fields & flag) == 0 || equal
}

/// Checks that `len` bytes plus the NUL terminator fit the caller's buffer. On failure returns
/// the size to report. Wine checks only `len`, so it overflows by the terminator; the port
/// requires the terminator's room too.
fn fit(len: u32, capacity: Option<u32>) -> Result<(), u32> {
    match capacity {
        Some(capacity) if capacity >= len + 2 => Ok(()),
        Some(capacity) if capacity >= len => Err(len + 2),
        _ => Err(len),
    }
}

fn color_dir_path(system_dir: &[u16]) -> Vec<u16> {
    let mut path = system_dir.to_vec();
    path.extend(wide(r"\spool\drivers\color"));
    path
}

fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

fn byte_len(units: &[u16]) -> u32 {
    (units.len() * 2) as u32
}
