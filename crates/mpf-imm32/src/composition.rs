//! Composition blocks and the composition calls of `dlls/imm32/ime.c`. A block is the byte image of
//! a `COMPOSITIONSTRING` that the input context holds. The strings and clauses sit at the offsets
//! that the header names, so the block stays as bytes.

use crate::messages::{WM_IME_COMPOSITION, WM_IME_ENDCOMPOSITION, WM_IME_STARTCOMPOSITION};
use crate::{LParam, TransMsg};

pub const GCS_COMPSTR: u32 = 0x0008;
pub const GCS_COMPATTR: u32 = 0x0010;
pub const GCS_COMPCLAUSE: u32 = 0x0020;
pub const GCS_CURSORPOS: u32 = 0x0080;
pub const GCS_DELTASTART: u32 = 0x0100;
pub const GCS_RESULTSTR: u32 = 0x0800;
pub const GCS_RESULTCLAUSE: u32 = 0x1000;

pub const SCS_SETSTR: u32 = 0x0009;
pub const SCS_CHANGEATTR: u32 = 0x0012;
pub const SCS_CHANGECLAUSE: u32 = 0x0024;
pub const SCS_SETRECONVERTSTRING: u32 = 0x0001_0000;
pub const SCS_QUERYRECONVERTSTRING: u32 = 0x0002_0000;

/// The size of the `COMPOSITIONSTRING` header: 25 DWORDs.
pub const HEADER_LEN: usize = 100;

/// `ATTR_INPUT`: the attribute of each unit of an input composition.
const ATTR_INPUT: u8 = 0;
/// The size of the clause array that `input_context_set_comp_str` writes: two DWORDs.
const CLAUSE_LEN: u32 = 8;
/// The `GCS_` flags that `ImeSetCompositionString` sends with `SCS_SETSTR`.
const SETSTR_FLAGS: u32 =
    GCS_COMPSTR | GCS_COMPCLAUSE | GCS_COMPATTR | GCS_DELTASTART | GCS_CURSORPOS;

/// The `COMPOSITIONSTRING` header, with the fields in the order of `include/immdev.h`.
#[derive(Default)]
pub struct CompositionString {
    pub size: u32,
    pub comp_read_attr_len: u32,
    pub comp_read_attr_offset: u32,
    pub comp_read_clause_len: u32,
    pub comp_read_clause_offset: u32,
    pub comp_read_str_len: u32,
    pub comp_read_str_offset: u32,
    pub comp_attr_len: u32,
    pub comp_attr_offset: u32,
    pub comp_clause_len: u32,
    pub comp_clause_offset: u32,
    pub comp_str_len: u32,
    pub comp_str_offset: u32,
    pub cursor_pos: u32,
    pub delta_start: u32,
    pub result_read_clause_len: u32,
    pub result_read_clause_offset: u32,
    pub result_read_str_len: u32,
    pub result_read_str_offset: u32,
    pub result_clause_len: u32,
    pub result_clause_offset: u32,
    pub result_str_len: u32,
    pub result_str_offset: u32,
    pub private_size: u32,
    pub private_offset: u32,
}

impl CompositionString {
    /// Reads the header at the start of `block`. `None` if the block is shorter than the header.
    pub fn read(block: &[u8]) -> Option<Self> {
        let header = block.first_chunk::<HEADER_LEN>()?;
        let words: [u32; 25] =
            std::array::from_fn(|i| u32::from_le_bytes(std::array::from_fn(|j| header[4 * i + j])));
        let [size, comp_read_attr_len, comp_read_attr_offset, comp_read_clause_len, comp_read_clause_offset, comp_read_str_len, comp_read_str_offset, comp_attr_len, comp_attr_offset, comp_clause_len, comp_clause_offset, comp_str_len, comp_str_offset, cursor_pos, delta_start, result_read_clause_len, result_read_clause_offset, result_read_str_len, result_read_str_offset, result_clause_len, result_clause_offset, result_str_len, result_str_offset, private_size, private_offset] =
            words;
        Some(Self {
            size,
            comp_read_attr_len,
            comp_read_attr_offset,
            comp_read_clause_len,
            comp_read_clause_offset,
            comp_read_str_len,
            comp_read_str_offset,
            comp_attr_len,
            comp_attr_offset,
            comp_clause_len,
            comp_clause_offset,
            comp_str_len,
            comp_str_offset,
            cursor_pos,
            delta_start,
            result_read_clause_len,
            result_read_clause_offset,
            result_read_str_len,
            result_read_str_offset,
            result_clause_len,
            result_clause_offset,
            result_str_len,
            result_str_offset,
            private_size,
            private_offset,
        })
    }

    /// The header as the 100 bytes that the input context holds.
    pub fn to_bytes(&self) -> Vec<u8> {
        [
            self.size,
            self.comp_read_attr_len,
            self.comp_read_attr_offset,
            self.comp_read_clause_len,
            self.comp_read_clause_offset,
            self.comp_read_str_len,
            self.comp_read_str_offset,
            self.comp_attr_len,
            self.comp_attr_offset,
            self.comp_clause_len,
            self.comp_clause_offset,
            self.comp_str_len,
            self.comp_str_offset,
            self.cursor_pos,
            self.delta_start,
            self.result_read_clause_len,
            self.result_read_clause_offset,
            self.result_read_str_len,
            self.result_read_str_offset,
            self.result_clause_len,
            self.result_clause_offset,
            self.result_str_len,
            self.result_str_offset,
            self.private_size,
            self.private_offset,
        ]
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect()
    }
}

/// The effect of a composition call: the new block, and the messages to send, in order.
pub struct CompositionUpdate {
    pub block: Vec<u8>,
    pub messages: Vec<TransMsg>,
}

/// `input_context_set_comp_str`: the block for a composition of `units`. An empty composition is
/// a bare header. Otherwise the block holds the header, the units, a clause array `[0, len]`, and
/// one attribute byte per unit.
pub fn build_comp_str(units: &[u16]) -> Vec<u8> {
    if units.is_empty() {
        return CompositionString {
            size: HEADER_LEN as u32,
            ..Default::default()
        }
        .to_bytes();
    }
    let len = units.len() as u32;
    let clause_offset = HEADER_LEN as u32 + 2 * len;
    let attr_offset = clause_offset + CLAUSE_LEN;
    let size = attr_offset + len;
    let mut block = CompositionString {
        size,
        cursor_pos: len,
        comp_str_len: len,
        comp_str_offset: HEADER_LEN as u32,
        comp_clause_len: CLAUSE_LEN,
        comp_clause_offset: clause_offset,
        comp_attr_len: len,
        comp_attr_offset: attr_offset,
        ..Default::default()
    }
    .to_bytes();
    block.extend(units.iter().flat_map(|unit| unit.to_le_bytes()));
    block.extend([0, len].into_iter().flat_map(u32::to_le_bytes));
    block.resize(size as usize, ATTR_INPUT);
    block
}

/// The unit at byte `offset` of `block`, or `None` if it does not fit.
pub fn unit_at(block: &[u8], offset: u32) -> Option<u16> {
    let unit = block.get(offset as usize..)?.first_chunk::<2>()?;
    Some(u16::from_le_bytes(*unit))
}

/// `ime_set_composition_status`: records whether a composition is open. Returns the start message
/// when one opens, and the end message when one closes.
pub fn set_status(in_composition: &mut bool, composition: bool) -> Option<TransMsg> {
    let message = match (*in_composition, composition) {
        (false, true) => Some(WM_IME_STARTCOMPOSITION),
        (true, false) => Some(WM_IME_ENDCOMPOSITION),
        _ => None,
    };
    *in_composition = composition;
    message.map(|message| TransMsg::new(message, 0, 0))
}

/// `ImeSetCompositionString` with `SCS_SETSTR`. `comp` is UTF-16LE. The first unit is the
/// `WM_IME_COMPOSITION` wParam, and trailing NUL units are dropped. A trailing odd byte is ignored.
pub fn set_string(in_composition: &mut bool, comp: &[u8]) -> CompositionUpdate {
    let wparam = comp
        .first_chunk::<2>()
        .map_or(0, |unit| usize::from(u16::from_le_bytes(*unit)));
    let mut units: Vec<u16> = comp
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u16::from_le_bytes(*pair))
        .collect();
    while units.last() == Some(&0) {
        units.pop();
    }
    let mut messages: Vec<TransMsg> = set_status(in_composition, !units.is_empty())
        .into_iter()
        .collect();
    messages.push(TransMsg::new(
        WM_IME_COMPOSITION,
        wparam,
        SETSTR_FLAGS as LParam,
    ));
    CompositionUpdate {
        block: build_comp_str(&units),
        messages,
    }
}

/// `NotifyIME` with `CPS_COMPLETE`: the composition becomes the result, and then it is cancelled.
/// The result message carries the first unit of the composition. `None` if `block` cannot be read
/// there, and then nothing changes.
pub fn complete_composition(in_composition: &mut bool, block: &[u8]) -> Option<CompositionUpdate> {
    let header = CompositionString::read(block)?;
    let wchr = unit_at(block, header.comp_str_offset)?;
    let mut flags = 0;
    if header.comp_str_len != 0 {
        flags |= GCS_RESULTSTR;
    }
    if header.comp_clause_len != 0 {
        flags |= GCS_RESULTCLAUSE;
    }
    let mut messages = Vec::new();
    if flags != 0 {
        messages.push(TransMsg::new(
            WM_IME_COMPOSITION,
            usize::from(wchr),
            flags as LParam,
        ));
    }
    let cancel = cancel_composition(in_composition);
    messages.extend(cancel.messages);
    Some(CompositionUpdate {
        block: cancel.block,
        messages,
    })
}

/// `NotifyIME` with `CPS_CANCEL`: clears the composition, and ends it if one was open.
pub fn cancel_composition(in_composition: &mut bool) -> CompositionUpdate {
    CompositionUpdate {
        block: build_comp_str(&[]),
        messages: set_status(in_composition, false).into_iter().collect(),
    }
}

/// The index check of `ImeSetCompositionString` in `ime.c`.
pub const fn ime_accepts_scs_index(index: u32) -> bool {
    matches!(index, SCS_SETSTR | SCS_CHANGECLAUSE | SCS_CHANGEATTR)
}

/// The index check of `ImmSetCompositionStringW` in `imm.c`.
pub const fn imm_accepts_scs_index(index: u32) -> bool {
    matches!(
        index,
        SCS_SETSTR
            | SCS_CHANGEATTR
            | SCS_CHANGECLAUSE
            | SCS_SETRECONVERTSTRING
            | SCS_QUERYRECONVERTSTRING
    )
}
