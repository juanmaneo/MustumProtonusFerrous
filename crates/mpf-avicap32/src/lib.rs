//! Port of the portable logic in Wine's `dlls/avicap32/v4l.c`.
//!
//! Ported:
//! - Device node name in `get_device_desc`, `snprintf(device, 16, "/dev/video%u", index)`:
//!   [`device_node_path`]
//! - Capture-device test on `caps.capabilities` and `caps.device_caps`: [`is_capture_device`]
//! - Driver version text, `snprintf(version, 32, "%s v%u.%u.%u", ...)`: [`format_driver_version`]
//!
//! Skipped:
//! - Window class registration, `capCreateCaptureWindowA/W` and `DllMain`: window and process state.
//! - `capGetDriverDescriptionA`: `WideCharToMultiByte` with `CP_ACP` depends on the active code page.
//! - `capGetDriverDescriptionW`: the unix call and the `lstrcpynW` copies into caller buffers.
//! - `v4l_umbstowcs`: `ntdll_umbstowcs` converts through the Unix locale charset. Its length
//!   argument is snprintf's untruncated return value, which reads past the 32-byte buffer when
//!   it is 32 or more. That is not reproduced here.
//! - The `stat`, `open`, `ioctl(VIDIOC_QUERYCAP)` and `close` calls in `get_device_desc`, and the
//!   `NTSTATUS` values they select.
//! - The `HAVE_LINUX_VIDEODEV2_H` stub that returns `STATUS_NOT_IMPLEMENTED`.
//! - The `#else` of `#ifdef V4L2_CAP_DEVICE_CAPS`: [`is_capture_device`] follows the branch
//!   that current headers compile.

/// `V4L2_CAP_VIDEO_CAPTURE` from `linux/videodev2.h`.
pub const V4L2_CAP_VIDEO_CAPTURE: u32 = 0x0000_0001;
/// `V4L2_CAP_DEVICE_CAPS` from `linux/videodev2.h`.
pub const V4L2_CAP_DEVICE_CAPS: u32 = 0x8000_0000;

/// `CAP_DESC_MAX` from `dlls/avicap32/unixlib.h`.
const CAP_DESC_MAX: usize = 32;

/// `get_device_desc`'s device name, `snprintf(device, 16, "/dev/video%u", index)`.
/// The longest result, `/dev/video65535`, is 15 bytes plus NUL, so the 16-byte buffer never truncates.
pub fn device_node_path(index: u16) -> String {
    format!("/dev/video{index}")
}

/// The capture test in `get_device_desc`. When `V4L2_CAP_DEVICE_CAPS` is set in
/// `capabilities`, `device_caps` decides; otherwise `capabilities` does.
pub fn is_capture_device(capabilities: u32, device_caps: u32) -> bool {
    let caps = if capabilities & V4L2_CAP_DEVICE_CAPS != 0 {
        device_caps
    } else {
        capabilities
    };
    caps & V4L2_CAP_VIDEO_CAPTURE != 0
}

/// The `version` buffer that `get_device_desc` builds with
/// `snprintf(version, 32, "%s v%u.%u.%u", driver, ...)`.
///
/// `driver` is the C string: its bytes up to the first NUL, or all of `driver` if there is no NUL.
/// snprintf stores at most 31 bytes, so the result is cut to 31 bytes.
pub fn format_driver_version(driver: &[u8], version: u32) -> Vec<u8> {
    let driver = &driver[..driver.iter().position(|&b| b == 0).unwrap_or(driver.len())];
    let mut out = driver.to_vec();
    let suffix = format!(
        " v{}.{}.{}",
        (version >> 16) & 0xff,
        (version >> 8) & 0xff,
        version & 0xff
    );
    out.extend_from_slice(suffix.as_bytes());
    out.truncate(CAP_DESC_MAX - 1);
    out
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
