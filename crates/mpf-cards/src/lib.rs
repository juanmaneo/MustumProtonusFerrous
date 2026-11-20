//! Port of the portable logic in Wine's `dlls/cards/cards.c`.
//!
//! `cards.spec` exports 5 functions. Only `cdtDrawExt` and `cdtDraw` contain
//! deterministic logic; the others need OS resources (bitmaps, GDI) or are
//! stubs. The port extracts the argument validation, mode flag handling, and
//! the mode → card/erase/draw/rasterOp state machine.
//!
//! Ported (2 of 5):
//! - `cdtDrawExt` argument checks and mode resolution: [`resolve_draw_args`]
//! - `cdtDraw` delegates to `cdtDrawExt` with default size: [`resolve_draw_args`] with `dx = card_width`, `dy = card_height`
//!
//! Skipped (3 of 5):
//! - `cdtInit`: loads bitmaps from resources (LoadBitmapA, GetObjectA); needs OS
//! - `cdtAnimate`: FIXME stub in Wine, returns FALSE
//! - `cdtTerm`: frees GDI bitmaps (DeleteObject); needs OS

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// Maximum card index (0-51 = faces, 52 = ghost mask, 53-68 = backs).
pub const CARD_MAX: i32 = 68;

/// Ghost card mask (used for MODE_GHOST and MODE_INVISIBLEGHOST).
pub const CARD_FREE_MASK: i32 = 52;
/// "X" card back (used for MODE_DECKX).
pub const CARD_BACK_THE_X: i32 = 67;
/// "O" card back (used for MODE_DECKO).
pub const CARD_BACK_THE_O: i32 = 68;

/// Drawing modes.
pub const MODE_FACEUP: i32 = 0;
pub const MODE_FACEDOWN: i32 = 1;
pub const MODE_HILITE: i32 = 2;
pub const MODE_GHOST: i32 = 3;
pub const MODE_REMOVE: i32 = 4;
pub const MODE_INVISIBLEGHOST: i32 = 5;
pub const MODE_DECKX: i32 = 6;
pub const MODE_DECKO: i32 = 7;

/// Mode flag: when set, disables rounded-corner clipping.
pub const MODEFLAG_DONT_ROUND_CORNERS: i32 = -2147483648; // 0x8000_0000

/// GDI raster operation codes.
pub const SRCCOPY: u32 = 0x00CC_2094;
pub const NOTSRCCOPY: u32 = 0x0033_0006;
pub const SRCAND: u32 = 0x0088_00C6;

/// Error returned when the card index is out of range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawError {
    InvalidCard,
    InvalidMode,
}

/// Resolved draw arguments after mode processing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DrawArgs {
    /// Effective card index (may differ from input for MODE_GHOST, MODE_INVISIBLEGHOST, MODE_DECKX, MODE_DECKO).
    pub card: i32,
    /// Whether to erase the background before drawing.
    pub erase_flag: bool,
    /// Whether to draw the card bitmap.
    pub draw_flag: bool,
    /// Raster operation to use for the blit.
    pub raster_op: u32,
    /// Whether rounded-corner clipping applies (dx==card_width && dy==card_height && high bit of mode clear).
    pub round_corners: bool,
}

/// Resolves the `cdtDrawExt` arguments: validates `card` and `mode`, applies
/// `MODEFLAG_DONT_ROUND_CORNERS`, and runs the mode state machine to produce
/// the effective card index, erase/draw flags, and raster operation.
///
/// This is the portable core of `cdtDrawExt` (lines 101-141 of cards.c).
///
/// # Errors
/// - `DrawError::InvalidCard` if `card < 0 || card > CARD_MAX`
/// - `DrawError::InvalidMode` if `mode` (after clearing `MODEFLAG_DONT_ROUND_CORNERS`) is not in `MODE_FACEUP..=MODE_DECKO`
pub fn resolve_draw_args(
    card: i32,
    mode: i32,
    dx: i32,
    dy: i32,
    card_width: i32,
    card_height: i32,
) -> Result<DrawArgs, DrawError> {
    // roundCornersFlag = !(mode & MODEFLAG_DONT_ROUND_CORNERS) && (dx == cardWidth) && (dy == cardHeight)
    let round_corners =
        (mode & MODEFLAG_DONT_ROUND_CORNERS) == 0 && dx == card_width && dy == card_height;

    // mode &= ~MODEFLAG_DONT_ROUND_CORNERS
    let mode = mode & !MODEFLAG_DONT_ROUND_CORNERS;

    // Card range check
    if !(0..=CARD_MAX).contains(&card) {
        return Err(DrawError::InvalidCard);
    }

    // Mode range check
    if !(MODE_FACEUP..=MODE_DECKO).contains(&mode) {
        return Err(DrawError::InvalidMode);
    }

    // Mode state machine. The range check above guarantees `mode` is a valid
    // table index; each row is (card override, erase, draw, raster op).
    const MODE_TABLE: [(Option<i32>, bool, bool, u32); 8] = [
        /* MODE_FACEUP */ (None, false, true, SRCCOPY),
        /* MODE_FACEDOWN */ (None, false, true, SRCCOPY),
        /* MODE_HILITE */ (None, false, true, NOTSRCCOPY),
        /* MODE_GHOST */ (Some(CARD_FREE_MASK), true, true, SRCAND),
        /* MODE_REMOVE */ (None, true, false, SRCCOPY),
        /* MODE_INVISIBLEGHOST */ (Some(CARD_FREE_MASK), false, true, SRCAND),
        /* MODE_DECKX */ (Some(CARD_BACK_THE_X), false, true, SRCCOPY),
        /* MODE_DECKO */ (Some(CARD_BACK_THE_O), false, true, SRCCOPY),
    ];
    let (card_override, erase_flag, draw_flag, raster_op) = MODE_TABLE[mode as usize];
    let card = card_override.unwrap_or(card);

    Ok(DrawArgs {
        card,
        erase_flag,
        draw_flag,
        raster_op,
        round_corners,
    })
}

/// Convenience wrapper matching `cdtDraw` semantics: calls `resolve_draw_args`
/// with `dx = card_width` and `dy = card_height`.
pub fn resolve_draw(
    card: i32,
    mode: i32,
    card_width: i32,
    card_height: i32,
) -> Result<DrawArgs, DrawError> {
    resolve_draw_args(card, mode, card_width, card_height, card_width, card_height)
}
