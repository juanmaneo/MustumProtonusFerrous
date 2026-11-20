//! Headless port of the deterministic parts of Wine's `dlls/imm32`, the Input Method Manager.
//!
//! The C reference locks the input context, calls the IME driver through `NtUserMessageCall`,
//! sends window messages and reads the registry. The helpers below run here as pure functions over
//! values. A composition string stays as its `COMPOSITIONSTRING` byte block, the bytes that the
//! driver writes. The caller supplies the in-composition flag, the keyboard layout and the driver
//! outcome, and gets back the messages to send, in order.
//!
//! Ported (names in `dlls/imm32`):
//! - `is_ime_hkl` in `ime.c`: [hkl::is_ime_hkl].
//! - `input_context_set_comp_str` in `ime.c`: [composition::build_comp_str].
//! - `ime_set_composition_status` in `ime.c`: [composition::set_status].
//! - `ImeSetCompositionString` with `SCS_SETSTR`, and its index check: [composition::set_string],
//!   [composition::ime_accepts_scs_index].
//! - `NotifyIME` with `CPS_COMPLETE` and `CPS_CANCEL`: [composition::complete_composition],
//!   [composition::cancel_composition].
//! - `ImeProcessKey` in `ime.c`: [keyboard::process_key].
//! - `ImeToAsciiEx` in `ime.c`: [keyboard::to_ascii_ex], [keyboard::key_lparam].
//! - `ime_is_unicode` and `EscapeRequiresWA` in `imm.c`: [property::is_unicode],
//!   [property::escape_requires_wa].
//! - The `ImmGetProperty` switch in `imm.c`: [property::get_property].
//! - `ImmIsUIMessageA` and `ImmIsUIMessageW` in `imm.c`: [messages::is_ui_message].
//! - `is_ime_ui_msg` in `imm.c`: [messages::is_ime_ui_msg].
//! - The `ImmSetCompositionStringW` index check in `imm.c`: [composition::imm_accepts_scs_index].
//!
//! Skipped: the candidate-list and copy-string helpers; `ime_internal_msg`; `ImmGetHotKey`; the
//! registry and other OS code; `debugstr` and the `TRACE`, `FIXME`, `WARN` and `ERR` logging;
//! `ImeConfigure` and the other stubs; the font and status updates of `NI_CONTEXTUPDATED`; the
//! `IMC_SETOPENSTATUS` call to `ImmNotifyIME`, which the caller makes; the `ime_send_message`
//! queue; the `NotifyIME` action and index dispatch, which the caller makes; and the `uMsgCount`
//! overflow check. The bodies of `SCS_CHANGECLAUSE` and `SCS_CHANGEATTR` are not ported, since
//! Wine only logs a `FIXME` for them.
//!
//! Deviations from Wine, where Wine reads or writes outside its data, or passes `NULL`:
//! - [composition::set_string] treats a `NULL` string as empty. Wine copies from `NULL` when the
//!   length is non-zero.
//! - [composition::CompositionString::read], [composition::unit_at], [composition::complete_composition]
//!   and [keyboard::to_ascii_ex] return `None` when a read falls outside the block. They leave the
//!   in-composition flag unchanged. C reads the whole buffer there.
//! - The lock failures of `ImmLockIMC` and `ImmLockIMCC` are the caller's. [keyboard::process_key]
//!   takes `ctx_locked`.
//! - The `STATUS_BUFFER_TOO_SMALL` retry loop is the caller's. [keyboard::to_ascii_ex] takes the
//!   outcome of the last driver call.
//! - [messages::is_ui_message] decides only. The caller sends the message when the window handle
//!   is not `NULL`.
//! - [keyboard::to_ascii_ex] ignores the driver outcome for a non-IME layout and returns no
//!   messages. Wine returns before it calls the driver.

pub mod composition;
pub mod hkl;
pub mod keyboard;
pub mod messages;
pub mod property;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// `WPARAM`.
pub type WParam = usize;

/// `LPARAM`.
pub type LParam = isize;

/// `TRANSMSG`: one message that the IME queues for the application.
pub struct TransMsg {
    pub message: u32,
    pub w_param: WParam,
    pub l_param: LParam,
}

impl TransMsg {
    pub const fn new(message: u32, w_param: WParam, l_param: LParam) -> Self {
        Self {
            message,
            w_param,
            l_param,
        }
    }
}

/// Formats messages as `message:wParam:lParam` in hex, space-separated, or `-` for none. The oracle
/// prints in the same form.
#[cfg(test)]
fn render(msgs: &[TransMsg]) -> String {
    if msgs.is_empty() {
        return "-".to_owned();
    }
    msgs.iter()
        .map(|m| format!("{:x}:{:x}:{:x}", m.message, m.w_param, m.l_param))
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
