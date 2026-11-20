//! The scan code mapping of `map_dik_code` in `dlls/dinput/keyboard.c`.

/// `DIDEVTYPEKEYBOARD_JAPAN106`: the keyboard subtype that gets the JP106 remap.
const DIDEVTYPEKEYBOARD_JAPAN106: u32 = 10;

const DIK_LBRACKET: u32 = 0x1a;
const DIK_RBRACKET: u32 = 0x1b;
const DIK_BACKSLASH: u32 = 0x2b;
const DIK_CIRCUMFLEX: u32 = 0x90;
const DIK_AT: u32 = 0x91;
const DIK_COLON: u32 = 0x92;
const DIK_KANJI: u32 = 0x94;

/// Maps a keyboard scan code to the `DIK_*` value that DirectInput reports.
///
/// `vk_to_scan` is the `MapVirtualKeyW(vk, MAPVK_VK_TO_VSC)` result. The C code asks
/// for it only when there is no scan code and the version predates 0x0800.
pub fn map_dik_code(scan_code: u32, sub_type: u32, version: u32, vk_to_scan: u32) -> u8 {
    let mut scan_code = scan_code;
    if scan_code == 0 && version < 0x0800 {
        scan_code = vk_to_scan;
    }
    if sub_type == DIDEVTYPEKEYBOARD_JAPAN106 {
        scan_code = match scan_code {
            0x0d => DIK_CIRCUMFLEX,
            0x1a => DIK_AT,
            0x1b => DIK_LBRACKET,
            0x28 => DIK_COLON,
            0x29 => DIK_KANJI,
            0x2b => DIK_RBRACKET,
            0x73 => DIK_BACKSLASH,
            _ => scan_code,
        };
    }
    if scan_code & 0x100 != 0 {
        scan_code |= 0x80;
    }
    scan_code as u8
}
