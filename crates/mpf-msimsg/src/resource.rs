/// `RT_RCDATA`, the type of the reboot prompt: `MAKEINTRESOURCE(10)` in `include/winuser.h`.
pub const RT_RCDATA: u16 = 10;

/// The name id of the reboot prompt in `msimsg.rc`: `1703 RCDATA`.
pub const REBOOT_PROMPT_ID: u16 = 1703;

/// `LANG_ENGLISH` from `include/winnt.rh`.
pub const LANG_ENGLISH: u16 = 0x09;

/// `SUBLANG_DEFAULT` from `include/winnt.rh`.
pub const SUBLANG_DEFAULT: u16 = 0x01;

/// `MAKELANGID(p, s)` from `include/winnt.h`. The C macro works in `WORD`, so bits shifted past
/// 16 are dropped and `primary` is not masked; `u16` arithmetic gives the same result.
pub const fn make_lang_id(primary: u16, sublang: u16) -> u16 {
    (sublang << 10) | primary
}

/// The language of the reboot prompt: `LANGUAGE LANG_ENGLISH, SUBLANG_DEFAULT` in `msimsg.rc`.
pub const REBOOT_PROMPT_LANG_ID: u16 = make_lang_id(LANG_ENGLISH, SUBLANG_DEFAULT);

/// The text of the reboot prompt, without the two NUL bytes that `msimsg.rc` appends. The
/// `[2]` placeholder is kept as written.
pub const REBOOT_PROMPT: &str = "You must simulate a Windows reboot to complete the installation of [2]. Click Yes to do this now or No if you want to restart later.";

/// A resource as the resource compiler lays it out: its type and name ids, its language id, and
/// its data bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceEntry {
    pub type_id: u16,
    pub name_id: u16,
    pub lang_id: u16,
    pub data: Vec<u8>,
}

/// The data of `1703 RCDATA { "...\0\0" }`: the bytes of the literal, then the two NUL bytes
/// from the `\0\0` escapes.
pub fn reboot_prompt_rcdata() -> Vec<u8> {
    let mut data = REBOOT_PROMPT.as_bytes().to_vec();
    data.extend_from_slice(&[0, 0]);
    data
}

/// The `1703 RCDATA` entry that `msimsg.rc` defines.
pub fn reboot_prompt_resource() -> ResourceEntry {
    ResourceEntry {
        type_id: RT_RCDATA,
        name_id: REBOOT_PROMPT_ID,
        lang_id: REBOOT_PROMPT_LANG_ID,
        data: reboot_prompt_rcdata(),
    }
}
