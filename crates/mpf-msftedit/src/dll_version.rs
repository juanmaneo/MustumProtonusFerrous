/// `S_OK` from `include/winerror.h`.
pub const S_OK: u32 = 0;

/// `E_INVALIDARG` from `include/winerror.h`.
pub const E_INVALIDARG: u32 = 0x8007_0057;

/// `DLLVER_PLATFORM_NT` from `include/shlwapi.h`.
pub const DLLVER_PLATFORM_NT: u32 = 0x02;

/// `sizeof(DLLVERSIONINFO)`: five DWORDs.
pub const DLLVERSIONINFO_SIZE: u32 = 20;

/// `sizeof(DLLVERSIONINFO2)`: `DLLVERSIONINFO`, a DWORD `dwFlags`, then a 64-bit `ullVersion`.
pub const DLLVERSIONINFO2_SIZE: u32 = 32;

/// `VS_FFI_SIGNATURE` from `include/verrsrc.h`.
const VS_FFI_SIGNATURE: u32 = 0xfeef_04bd;

/// `sizeof(VS_FIXEDFILEINFO)`: thirteen DWORDs.
const VS_FIXEDFILEINFO_SIZE: usize = 52;

/// The caller's `DLLVERSIONINFO` or `DLLVERSIONINFO2` buffer. `cb_size` selects the layout.
/// `flags` and `ull_version` are only written in the 32-byte layout.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DllVersionInfo {
    pub cb_size: u32,
    pub major: u32,
    pub minor: u32,
    pub build: u32,
    pub platform_id: u32,
    pub flags: u32,
    pub ull_version: u64,
}

/// `DllGetVersion` from `libs/winecrt0/dll_version.c`. `version_resource` is the `VS_VERSION_INFO`
/// resource of this module, or `None` when `FindResourceW` or `LoadResource` fails.
pub fn dll_get_version(info: Option<&mut DllVersionInfo>, version_resource: Option<&[u8]>) -> u32 {
    let Some(info) = info else {
        return E_INVALIDARG;
    };
    let Some((ms, ls)) = version_resource.and_then(file_version) else {
        return E_INVALIDARG;
    };
    match info.cb_size {
        DLLVERSIONINFO2_SIZE => {
            info.flags = 0;
            info.ull_version = (u64::from(ms) << 32) | u64::from(ls);
        }
        DLLVERSIONINFO_SIZE => {}
        _ => return E_INVALIDARG,
    }
    info.major = ms >> 16;
    info.minor = ms & 0xffff;
    info.build = ls >> 16;
    info.platform_id = DLLVER_PLATFORM_NT;
    S_OK
}

/// Reads `dwFileVersionMS` and `dwFileVersionLS` from the `VS_FIXEDFILEINFO` of a `VS_VERSION_INFO`
/// resource. Returns `None` where the C code returns `E_INVALIDARG`, and also where the C code would
/// read past the end of the resource.
fn file_version(resource: &[u8]) -> Option<(u32, u32)> {
    // The header is three WORDs: `wLength`, `wValueLength` and `wType`.
    if resource.len() < 6 || usize::from(le_u16(resource, 2)) < VS_FIXEDFILEINFO_SIZE {
        return None;
    }
    let key_nul = key_terminator(resource)?;
    // `GET_VERSION_VALUE`: the value follows the key's NUL, rounded up to a 4-byte boundary.
    let value = (key_nul + 2 + 3) & !3;
    let fixed = resource.get(value..value + VS_FIXEDFILEINFO_SIZE)?;
    if le_u32(fixed, 0) != VS_FFI_SIGNATURE {
        return None;
    }
    Some((le_u32(fixed, 8), le_u32(fixed, 12)))
}

/// The byte offset of the NUL unit that ends the UTF-16 key, which starts at offset 6. Returns `None`
/// when the resource ends first. The C code's `lstrlenW` has no such bound.
fn key_terminator(resource: &[u8]) -> Option<usize> {
    let mut at = 6;
    while at + 2 <= resource.len() {
        if le_u16(resource, at) == 0 {
            return Some(at);
        }
        at += 2;
    }
    None
}

fn le_u16(bytes: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([bytes[at], bytes[at + 1]])
}

fn le_u32(bytes: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
}
