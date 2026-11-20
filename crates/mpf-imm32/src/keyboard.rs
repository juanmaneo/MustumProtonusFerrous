//! `ImeProcessKey` and `ImeToAsciiEx` in `dlls/imm32/ime.c`: which keys the IME consumes, and the
//! messages that a key press queues.

use crate::composition::{
    set_status, unit_at, CompositionString, GCS_COMPATTR, GCS_COMPCLAUSE, GCS_COMPSTR,
    GCS_CURSORPOS, GCS_DELTASTART, GCS_RESULTCLAUSE, GCS_RESULTSTR,
};
use crate::hkl::is_ime_hkl;
use crate::messages::{WM_IME_COMPOSITION, WM_IME_KEYDOWN, WM_KEYDOWN, WM_KEYUP};
use crate::{LParam, TransMsg, WParam};

pub const VK_SHIFT: u32 = 0x10;
pub const VK_CONTROL: u32 = 0x11;
pub const VK_MENU: u32 = 0x12;
pub const VK_CAPITAL: u32 = 0x14;
pub const KF_UP: u32 = 0x8000;
pub const CS_INSERTCHAR: u32 = 0x2000;
pub const CS_NOMOVECARET: u32 = 0x4000;

/// `ImeProcessKey`: whether the IME consumes `vkey` in `layout`. The modifier and caps-lock keys
/// are not consumed. `ctx_locked` is whether the caller could lock the input context.
pub fn process_key(layout: usize, vkey: u32, ctx_locked: bool) -> bool {
    if !is_ime_hkl(layout) || !ctx_locked {
        return false;
    }
    !matches!(vkey & 0xffff, VK_SHIFT | VK_CONTROL | VK_CAPITAL | VK_MENU)
}

/// The final result of the driver's `WINE_IME_TO_ASCII_EX` call.
pub enum DriverOutcome {
    /// `STATUS_NOT_IMPLEMENTED`: the driver does not handle the key.
    NotImplemented,
    /// Any other failure status.
    Failed,
    /// Success. `block` is the `COMPOSITIONSTRING` that the driver wrote, and `key_consumed` is
    /// whether the driver consumed the key.
    Filled { block: Vec<u8>, key_consumed: bool },
}

/// `ImeToAsciiEx`: the messages for one key press, in order, or `None` if the driver's block
/// cannot be read. Then the in-composition flag does not change.
pub fn to_ascii_ex(
    in_composition: &mut bool,
    layout: usize,
    vkey: u32,
    vsc: u32,
    outcome: DriverOutcome,
) -> Option<Vec<TransMsg>> {
    if !is_ime_hkl(layout) {
        return Some(Vec::new());
    }
    match outcome {
        DriverOutcome::NotImplemented => {
            let message = if (vsc & KF_UP) != 0 {
                WM_KEYUP
            } else {
                WM_KEYDOWN
            };
            Some(vec![TransMsg::new(
                message,
                vkey as WParam,
                key_lparam(vsc),
            )])
        }
        DriverOutcome::Failed => Some(Vec::new()),
        DriverOutcome::Filled {
            block,
            key_consumed,
        } => filled_messages(in_composition, &block, vkey, vsc, key_consumed),
    }
}

/// The `lParam` of a key message: a repeat count of 1, and the scan code in the high word.
pub const fn key_lparam(vsc: u32) -> LParam {
    (((vsc & 0xffff) << 16) | 1) as LParam
}

fn filled_messages(
    in_composition: &mut bool,
    block: &[u8],
    vkey: u32,
    vsc: u32,
    key_consumed: bool,
) -> Option<Vec<TransMsg>> {
    let header = CompositionString::read(block)?;
    let result = if header.result_str_len != 0 {
        Some(unit_at(block, header.result_str_offset)?)
    } else {
        None
    };
    let comp = if header.comp_str_offset != 0 {
        Some(unit_at(block, header.comp_str_offset)?)
    } else {
        None
    };

    let mut messages = Vec::new();
    if header.comp_str_offset != 0 || header.result_str_len != 0 {
        messages.extend(set_status(in_composition, true));
    }
    if let Some(unit) = result {
        let mut flags = GCS_RESULTSTR;
        if header.result_clause_len != 0 {
            flags |= GCS_RESULTCLAUSE;
        }
        messages.push(TransMsg::new(
            WM_IME_COMPOSITION,
            usize::from(unit),
            flags as LParam,
        ));
    }
    if let Some(unit) = comp {
        let mut flags = GCS_COMPSTR | GCS_CURSORPOS | GCS_DELTASTART;
        if header.comp_attr_offset != 0 {
            flags |= GCS_COMPATTR;
        }
        if header.comp_clause_offset != 0 {
            flags |= GCS_COMPCLAUSE;
        } else {
            flags |= CS_INSERTCHAR | CS_NOMOVECARET;
        }
        messages.push(TransMsg::new(
            WM_IME_COMPOSITION,
            usize::from(unit),
            flags as LParam,
        ));
    }
    if header.comp_str_len == 0 {
        messages.extend(set_status(in_composition, false));
    }
    if !key_consumed {
        messages.push(TransMsg::new(
            WM_IME_KEYDOWN,
            vkey as WParam,
            key_lparam(vsc),
        ));
    }
    Some(messages)
}
