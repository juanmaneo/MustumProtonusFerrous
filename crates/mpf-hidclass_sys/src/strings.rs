/// `HID_STRING_ID_IPRODUCT`, the only string index answered from the table.
pub const HID_STRING_ID_IPRODUCT: u32 = 15;

/// `device_strings`: the 17 character `VID_xxxx&PID_xxxx` match id of a
/// controller and its product string.
pub(crate) const DEVICE_STRINGS: &[(&str, &str)] = &[
    // Microsoft controllers
    ("VID_045E&PID_028E", "Controller (XBOX 360 For Windows)"),
    ("VID_045E&PID_028F", "Controller (XBOX 360 For Windows)"),
    ("VID_045E&PID_02D1", "Controller (Xbox One For Windows)"),
    ("VID_045E&PID_02DD", "Controller (Xbox One For Windows)"),
    ("VID_045E&PID_02E3", "Controller (Xbox One For Windows)"),
    ("VID_045E&PID_02EA", "Controller (Xbox One For Windows)"),
    ("VID_045E&PID_02FD", "Controller (Xbox One For Windows)"),
    ("VID_045E&PID_0719", "Controller (XBOX 360 For Windows)"),
    ("VID_045E&PID_0B00", "Controller (Xbox One For Windows)"),
    ("VID_045E&PID_0B05", "Controller (Xbox One For Windows)"),
    ("VID_045E&PID_0B12", "Controller (Xbox One For Windows)"),
    ("VID_045E&PID_0B13", "Controller (Xbox One For Windows)"),
    // Sony controllers
    ("VID_054C&PID_05C4", "Wireless Controller"),
    ("VID_054C&PID_09CC", "Wireless Controller"),
    ("VID_054C&PID_0BA0", "Wireless Controller"),
    ("VID_054C&PID_0CE6", "Wireless Controller"),
    ("VID_054C&PID_0DF2", "Wireless Controller"),
];

/// Length of a match id in `DEVICE_STRINGS`, the characters `wcsnicmp` compares.
const MATCH_ID_LEN: usize = 17;

/// `find_device_string`: the product string of a known controller.
///
/// The match id is the part of `device_id` after its last backslash. Only
/// index `HID_STRING_ID_IPRODUCT` is answered. Case folding is ASCII only.
pub fn find_device_string(device_id: &[u16], index: u32) -> Option<&'static str> {
    if index != HID_STRING_ID_IPRODUCT {
        return None;
    }
    let start = device_id.iter().rposition(|&c| c == u16::from(b'\\'))? + 1;
    let match_id = &device_id[start..];

    DEVICE_STRINGS
        .iter()
        .find(|(id, _)| matches_id(match_id, id))
        .map(|&(_, product)| product)
}

fn matches_id(match_id: &[u16], id: &str) -> bool {
    match_id.len() >= MATCH_ID_LEN
        && id
            .bytes()
            .zip(match_id)
            .all(|(a, &b)| ascii_lower(u16::from(a)) == ascii_lower(b))
}

fn ascii_lower(c: u16) -> u16 {
    if (u16::from(b'A')..=u16::from(b'Z')).contains(&c) {
        c + 32
    } else {
        c
    }
}
