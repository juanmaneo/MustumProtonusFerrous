//! The window message numbers that `dlls/imm32` tests or sends, and the message filters of
//! `imm.c`: `ImmIsUIMessageA` and `ImmIsUIMessageW`, which decide, and `is_ime_ui_msg`.

pub const WM_KEYDOWN: u32 = 0x0100;
pub const WM_KEYUP: u32 = 0x0101;
pub const WM_IME_STARTCOMPOSITION: u32 = 0x010d;
pub const WM_IME_ENDCOMPOSITION: u32 = 0x010e;
pub const WM_IME_COMPOSITION: u32 = 0x010f;
pub const WM_IME_KEYLAST: u32 = 0x010f;
pub const WM_IME_SETCONTEXT: u32 = 0x0281;
pub const WM_IME_NOTIFY: u32 = 0x0282;
pub const WM_IME_CONTROL: u32 = 0x0283;
pub const WM_IME_COMPOSITIONFULL: u32 = 0x0284;
pub const WM_IME_SELECT: u32 = 0x0285;
pub const WM_IME_CHAR: u32 = 0x0286;
pub const WM_IME_SYSTEM: u32 = 0x0287;
pub const WM_IME_REQUEST: u32 = 0x0288;
pub const WM_IME_KEYDOWN: u32 = 0x0290;
pub const WM_IME_KEYUP: u32 = 0x0291;

/// Whether `ImmIsUIMessageA` and `ImmIsUIMessageW` take `msg` as an IME UI message. The caller
/// sends it to the IME window when that window is not `NULL`.
pub fn is_ui_message(msg: u32) -> bool {
    matches!(
        msg,
        WM_IME_STARTCOMPOSITION
            ..=WM_IME_KEYLAST
                | WM_IME_SETCONTEXT
                | WM_IME_NOTIFY
                | WM_IME_COMPOSITIONFULL
                | WM_IME_SELECT
                | WM_IME_SYSTEM
    )
}

/// The `WM_MSIME_*` message numbers. The IME registers them at run time, so `imm.c` keeps them in
/// globals.
pub struct MsimeMessages {
    pub service: u32,
    pub reconvert_options: u32,
    pub mouse: u32,
    pub reconvert_request: u32,
    pub reconvert: u32,
    pub query_position: u32,
    pub document_feed: u32,
}

impl MsimeMessages {
    fn contains(&self, msg: u32) -> bool {
        [
            self.service,
            self.reconvert_options,
            self.mouse,
            self.reconvert_request,
            self.reconvert,
            self.query_position,
            self.document_feed,
        ]
        .contains(&msg)
    }
}

/// `is_ime_ui_msg`: whether `msg` is an IME message, fixed or registered at run time.
pub fn is_ime_ui_msg(msg: u32, msime: &MsimeMessages) -> bool {
    matches!(
        msg,
        WM_IME_STARTCOMPOSITION
            | WM_IME_ENDCOMPOSITION
            | WM_IME_COMPOSITION
            | WM_IME_SETCONTEXT
            | WM_IME_NOTIFY
            | WM_IME_CONTROL
            | WM_IME_COMPOSITIONFULL
            | WM_IME_SELECT
            | WM_IME_CHAR
            | WM_IME_REQUEST
            | WM_IME_KEYDOWN
            | WM_IME_KEYUP
    ) || msime.contains(msg)
}
