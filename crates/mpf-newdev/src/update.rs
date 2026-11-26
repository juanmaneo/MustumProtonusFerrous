/// `MAX_DEVICE_ID_LEN` from `include/cfgmgr32.h`: the hardware ID buffer of the ANSI thunk.
pub const MAX_DEVICE_ID_LEN: usize = 200;

/// `MAX_PATH` from `include/windef.h`: the INF path buffer of the ANSI thunk.
pub const MAX_PATH: usize = 260;

/// The ANSI thunk `UpdateDriverForPlugAndPlayDevicesA` from `dlls/newdev/main.c`. It widens
/// `hardware_id` into a [`MAX_DEVICE_ID_LEN`] buffer and `inf_path` into a [`MAX_PATH`] buffer, then
/// calls the wide function with them. `update_w` stands in for `UpdateDriverForPlugAndPlayDevicesW`
/// and receives `flags` and the `reboot` pointer unchanged. Its return value is the thunk's return
/// value. `BOOL` is modelled as `bool`.
pub fn update_driver_for_plug_and_play_devices_a<W>(
    hardware_id: &[u8],
    inf_path: &[u8],
    flags: u32,
    reboot: Option<&mut bool>,
    update_w: W,
) -> bool
where
    W: FnOnce(&[u16], &[u16], u32, Option<&mut bool>) -> bool,
{
    let hardware_id_w = ansi_to_wide::<MAX_DEVICE_ID_LEN>(hardware_id);
    let inf_path_w = ansi_to_wide::<MAX_PATH>(inf_path);
    update_w(&hardware_id_w, &inf_path_w, flags, reboot)
}

/// `MultiByteToWideChar(CP_ACP, 0, src, -1, dst, N)` into a zeroed `N`-unit buffer. `src` is read up
/// to its first NUL. CP_ACP is modelled as ASCII-compatible: each byte becomes the code unit of the
/// same value. A source that does not fit with its NUL is cut to `N - 1` units, so the buffer stays
/// NUL-terminated.
fn ansi_to_wide<const N: usize>(src: &[u8]) -> [u16; N] {
    let mut dst = [0u16; N];
    let len = src.iter().position(|&b| b == 0).unwrap_or(src.len());
    for (out, &b) in dst.iter_mut().take(N - 1).zip(&src[..len]) {
        *out = u16::from(b);
    }
    dst
}

/// `hardware_id_matches` from `dlls/newdev/main.c`. `id` is a NUL-terminated wide string. `device_ids`
/// is a multi-string: each entry is NUL-terminated, and an empty entry ends the list. The search
/// stops at that empty entry, or at the end of the slice when the list has no terminator.
pub fn hardware_id_matches(id: &[u16], device_ids: &[u16]) -> bool {
    let id = wide_str(id);
    let mut rest = device_ids;
    while rest.first().is_some_and(|&c| c != 0) {
        let entry = wide_str(rest);
        if entry == id {
            return true;
        }
        rest = rest.get(entry.len() + 1..).unwrap_or_default();
    }
    false
}

/// The characters of a wide string up to its first NUL, without the NUL.
fn wide_str(s: &[u16]) -> &[u16] {
    let len = s.iter().position(|&c| c == 0).unwrap_or(s.len());
    &s[..len]
}
