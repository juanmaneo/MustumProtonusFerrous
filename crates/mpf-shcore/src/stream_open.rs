//! The mode checks of `SHCreateStreamOnFileEx` and `SHCreateStreamOnFileW` in Wine's
//! `dlls/shcore/main.c`. The file itself is opened by the caller with the flags returned here.

use crate::{Hresult, E_INVALIDARG};

pub(crate) const STGM_READWRITE: u32 = 0x2;
const STGM_WRITE: u32 = 0x1;
const STGM_READ: u32 = 0x0;
const STGM_SHARE_DENY_NONE: u32 = 0x40;
const STGM_SHARE_DENY_READ: u32 = 0x30;
const STGM_SHARE_DENY_WRITE: u32 = 0x20;
const STGM_SHARE_EXCLUSIVE: u32 = 0x10;
const STGM_FAILIFTHERE: u32 = 0x0;
const STGM_CREATE: u32 = 0x1000;
const STGM_TRANSACTED: u32 = 0x1_0000;
const STGM_CONVERT: u32 = 0x2_0000;
const STGM_DELETEONRELEASE: u32 = 0x0400_0000;

const GENERIC_READ: u32 = 0x8000_0000;
const GENERIC_WRITE: u32 = 0x4000_0000;
const FILE_SHARE_READ: u32 = 0x1;
const FILE_SHARE_WRITE: u32 = 0x2;
const CREATE_NEW: u32 = 1;
const CREATE_ALWAYS: u32 = 2;
const OPEN_EXISTING: u32 = 3;

/// The `CreateFileW` arguments that a stream open maps `STGM_*` mode bits to.
#[derive(Debug, PartialEq, Eq)]
pub struct OpenFlags {
    pub access: u32,
    pub share: u32,
    pub disposition: u32,
}

/// The mode checks of `SHCreateStreamOnFileEx`. A template stream is not supported, so
/// `has_template` fails with `E_INVALIDARG`, as the C code does for a non-NULL template.
pub fn stream_on_file_ex_flags(
    mode: u32,
    create: bool,
    has_template: bool,
) -> Result<OpenFlags, Hresult> {
    if has_template {
        return Err(E_INVALIDARG);
    }
    let access = match mode & 0xf {
        STGM_WRITE | STGM_READWRITE => GENERIC_READ | GENERIC_WRITE,
        STGM_READ => GENERIC_READ,
        _ => return Err(E_INVALIDARG),
    };
    let share = match mode & 0xf0 {
        0 | STGM_SHARE_DENY_NONE => FILE_SHARE_READ | FILE_SHARE_WRITE,
        STGM_SHARE_DENY_READ => FILE_SHARE_WRITE,
        STGM_SHARE_DENY_WRITE => FILE_SHARE_READ,
        STGM_SHARE_EXCLUSIVE => 0,
        _ => return Err(E_INVALIDARG),
    };
    let disposition = match mode & 0xf000 {
        STGM_FAILIFTHERE => {
            if create {
                CREATE_NEW
            } else {
                OPEN_EXISTING
            }
        }
        STGM_CREATE => CREATE_ALWAYS,
        _ => return Err(E_INVALIDARG),
    };
    Ok(OpenFlags {
        access,
        share,
        disposition,
    })
}

/// The mode checks of `SHCreateStreamOnFileW`. Transacted, conversion and delete-on-release
/// modes fail with `E_INVALIDARG`. The rest is [stream_on_file_ex_flags] with `create` false.
pub fn stream_on_file_w_flags(mode: u32) -> Result<OpenFlags, Hresult> {
    if mode & (STGM_CONVERT | STGM_DELETEONRELEASE | STGM_TRANSACTED) != 0 {
        return Err(E_INVALIDARG);
    }
    stream_on_file_ex_flags(mode, false, false)
}
