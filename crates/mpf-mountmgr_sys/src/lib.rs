//! Headless subset of Wine's `dlls/mountmgr.sys`.
//!
//! The ported functions compute from their arguments what the C bodies compute.
//! Buffers are slices and status codes are `i32`, as in the other `_sys` crates.
//!
//! Ported:
//! - `errno_to_status` (`unixlib.c`): the errno to NTSTATUS table, with the
//!   Linux errno values.
//! - `parse_uuid` (`dbus.c`): the 8-4-4-4-12 UUID form and the FAT `XXXX-XXXX`
//!   volume serial, including what glibc `sscanf` accepts in those fields
//!   (leading white space, a sign, a `0x` prefix).
//! - `frame_to_msf` and `track_to_frame` (`cdrom.c`), and
//!   `VOLUME_GetAudioCDSerial` (`device.c`), over a `Toc`.
//! - `check_credential_string` (`cred.c`): the bounds and NUL-terminator check.
//!
//! Skipped:
//! - The IOCTL, device, DBus, Core Foundation, Security framework and DHCP paths:
//!   they need handles, the OS, or a bus.
//! - `match_credential` and `matching_mount_point`: they compare wide strings
//!   case-insensitively through the C runtime, not a portable rule.
//! - `get_mountmgr_fs_type`, `map_option` and the other lookup tables that are
//!   a single `switch` on an enum from another header.
//! - The TRACE and FIXME output, the `static` caches and the `GUID` storage.
//!
//! Divergences from the C:
//! - `parse_uuid` stops at the first NUL, as `strlen` does, and never reads past it.
//! - `check_credential_string` rejects `offset + size` that overflows `u32`. The C
//!   wraps the sum, and a wrapped sum of zero reads before the buffer (ASan reports
//!   it). A wrapped sum that is small but non-zero is accepted by C.
//! - `audio_cd_serial` returns `None` when `last_track < first_track`, and when the
//!   `Toc` lacks an entry it reads. For `last_track + 1 == first_track` (no tracks),
//!   C returns `0` without reading out of bounds; this port returns `None`. For
//!   `last_track + 1 < first_track`, C reads before `TrackData` (ASan reports it).
//! - `track_to_frame` returns `None` for a track outside the entries. C indexes
//!   with an unsigned difference, which can wrap below the first track.

mod cdrom;
mod credential;
mod errno;
mod guid;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use cdrom::{audio_cd_serial, frame_to_msf, track_to_frame, Toc};
pub use credential::check_credential_string;
pub use errno::errno_to_status;
pub use guid::{parse_uuid, Guid};
