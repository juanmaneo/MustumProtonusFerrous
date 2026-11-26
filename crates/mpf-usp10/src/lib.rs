//! Port of the portable logic in Wine's Uniscribe, `dlls/gdi32/uniscribe/`
//! (the implementation behind `dlls/usp10`).
//!
//! Ported:
//! - `BIDI_ReorderV2lLevel` and `BIDI_ReorderL2vLevel` (rule L2 reordering):
//!   [`bidi_reorder_v2l_level`], [`bidi_reorder_l2v_level`]
//! - `BIDI_GetStrengths` with its `classify` helper and the legacy `+`/`-` rule,
//!   over the generated bidi class table: [`bidi_get_strengths`], [`BIDI_STRONG`],
//!   [`BIDI_WEAK`], [`BIDI_NEUTRAL`]
//! - `mirror_char` over the generated `wine_mirror_map`: [`mirror_char`]
//!
//! Skipped:
//! - `BIDI_DetermineLevels` and its resolve, bracket and isolating-run passes
//!   (UAX #9 X1-I2): table-driven and large; not ported in this pass.
//! - `BREAK_line` (line-break table), the Indic syllable parser and the
//!   shaping and OpenType code: not ported in this pass. Some of the Indic and
//!   shaping paths take an `HDC` and read font data.
//! - The `usp10.c` API surface (`Script*`, `Usp*`): script cache, `HDC`,
//!   critical sections and `TRACE` logging.

mod bidi;
mod direction;
mod mirror;

pub use bidi::{
    bidi_get_strengths, bidi_reorder_l2v_level, bidi_reorder_v2l_level, BIDI_NEUTRAL, BIDI_STRONG,
    BIDI_WEAK,
};
pub use mirror::mirror_char;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
