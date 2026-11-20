//! `errno_to_status` from `unixlib.c`, with the Linux errno values.

pub(crate) const STATUS_UNSUCCESSFUL: i32 = 0xC000_0001u32 as i32;
pub(crate) const STATUS_SHARING_VIOLATION: i32 = 0xC000_0043u32 as i32;
pub(crate) const STATUS_INVALID_HANDLE: i32 = 0xC000_0008u32 as i32;
pub(crate) const STATUS_DEVICE_BUSY: i32 = 0x8000_0011u32 as i32;
pub(crate) const STATUS_DISK_FULL: i32 = 0xC000_007Fu32 as i32;
pub(crate) const STATUS_ACCESS_DENIED: i32 = 0xC000_0022u32 as i32;
pub(crate) const STATUS_OBJECT_PATH_NOT_FOUND: i32 = 0xC000_003Au32 as i32;
pub(crate) const STATUS_OBJECT_NAME_NOT_FOUND: i32 = 0xC000_0034u32 as i32;
pub(crate) const STATUS_INVALID_DEVICE_REQUEST: i32 = 0xC000_0010u32 as i32;
pub(crate) const STATUS_TOO_MANY_OPENED_FILES: i32 = 0xC000_011Fu32 as i32;
pub(crate) const STATUS_INVALID_PARAMETER: i32 = 0xC000_000Du32 as i32;
pub(crate) const STATUS_DIRECTORY_NOT_EMPTY: i32 = 0xC000_0101u32 as i32;
pub(crate) const STATUS_PIPE_DISCONNECTED: i32 = 0xC000_00B0u32 as i32;
pub(crate) const STATUS_DEVICE_NOT_READY: i32 = 0xC000_00A3u32 as i32;
pub(crate) const STATUS_NO_MEDIA_IN_DEVICE: i32 = 0xC000_0013u32 as i32;
pub(crate) const STATUS_NO_SUCH_DEVICE: i32 = 0xC000_000Eu32 as i32;
pub(crate) const STATUS_NOT_SUPPORTED: i32 = 0xC000_00BBu32 as i32;
pub(crate) const STATUS_ACCESS_VIOLATION: i32 = 0xC000_0005u32 as i32;
pub(crate) const STATUS_ILLEGAL_FUNCTION: i32 = 0xC000_00AFu32 as i32;
pub(crate) const STATUS_REPARSE_POINT_NOT_RESOLVED: i32 = 0xC000_0280u32 as i32;
pub(crate) const STATUS_IO_TIMEOUT: i32 = 0xC000_00B5u32 as i32;

const EPERM: i32 = 1;
const ENOENT: i32 = 2;
const EIO: i32 = 5;
const ENXIO: i32 = 6;
const EBADF: i32 = 9;
const EAGAIN: i32 = 11;
const EACCES: i32 = 13;
const EFAULT: i32 = 14;
const EBUSY: i32 = 16;
const ENOTDIR: i32 = 20;
const EISDIR: i32 = 21;
const EINVAL: i32 = 22;
const ENFILE: i32 = 23;
const EMFILE: i32 = 24;
const ENOTTY: i32 = 25;
const ENOSPC: i32 = 28;
const ESPIPE: i32 = 29;
const EROFS: i32 = 30;
const EPIPE: i32 = 32;
const ENOTEMPTY: i32 = 39;
const ELOOP: i32 = 40;
const ETIME: i32 = 62;
const EOPNOTSUPP: i32 = 95;
const ECONNRESET: i32 = 104;
const ENOMEDIUM: i32 = 123;

/// Maps a Linux errno to the NTSTATUS that `unixlib.c` returns for it.
///
/// Unmapped values, including `ENOEXEC` and `EEXIST`, give `STATUS_UNSUCCESSFUL`.
pub fn errno_to_status(err: i32) -> i32 {
    match err {
        EAGAIN => STATUS_SHARING_VIOLATION,
        EBADF => STATUS_INVALID_HANDLE,
        EBUSY => STATUS_DEVICE_BUSY,
        ENOSPC => STATUS_DISK_FULL,
        EPERM | EROFS | EACCES => STATUS_ACCESS_DENIED,
        ENOTDIR => STATUS_OBJECT_PATH_NOT_FOUND,
        ENOENT => STATUS_OBJECT_NAME_NOT_FOUND,
        EISDIR => STATUS_INVALID_DEVICE_REQUEST,
        EMFILE | ENFILE => STATUS_TOO_MANY_OPENED_FILES,
        EINVAL => STATUS_INVALID_PARAMETER,
        ENOTEMPTY => STATUS_DIRECTORY_NOT_EMPTY,
        EPIPE | ECONNRESET => STATUS_PIPE_DISCONNECTED,
        EIO => STATUS_DEVICE_NOT_READY,
        ENOMEDIUM => STATUS_NO_MEDIA_IN_DEVICE,
        ENXIO => STATUS_NO_SUCH_DEVICE,
        ENOTTY | EOPNOTSUPP => STATUS_NOT_SUPPORTED,
        EFAULT => STATUS_ACCESS_VIOLATION,
        ESPIPE => STATUS_ILLEGAL_FUNCTION,
        ELOOP => STATUS_REPARSE_POINT_NOT_RESOLVED,
        ETIME => STATUS_IO_TIMEOUT,
        _ => STATUS_UNSUCCESSFUL,
    }
}
