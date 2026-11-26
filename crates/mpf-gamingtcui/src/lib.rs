//! Port of the portable logic in Wine's `dlls/gamingtcui/gamingtcui_main.c`
//! and the `@ stub` exports in `dlls/gamingtcui/gamingtcui.spec`.
//!
//! Ported:
//! - `ProcessPendingGameUI`: [`process_pending_game_ui`]
//! - `ShowPlayerPickerUI`: [`show_player_picker_ui`], which reports the preselected list
//!   to the completion routine as the selection
//! - `ShowProfileCardUI`: [`show_profile_card_ui`]
//! - The 23 `@ stub` exports: [`STUB_EXPORTS`], and [`unimplemented_stub`], which models
//!   `__wine_spec_unimplemented_stub` from `libs/winecrt0/stub.c`
//!
//! Skipped:
//! - `FIXME` and `WINE_DEFAULT_DEBUG_CHANNEL` trace output.
//! - The `WINAPI` calling convention. C completion routines become `FnOnce` closures.
//! - Dereferencing `HSTRING` arrays. Handles are opaque `usize` values passed through unchanged.
//! - The `RaiseException` call. The C stub never returns, so [`unimplemented_stub`]
//!   returns the record it would raise.

mod stub;
mod ui;

pub use stub::{
    unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB, MODULE_NAME,
    STUB_EXPORTS,
};
pub use ui::{
    process_pending_game_ui, show_player_picker_ui, show_profile_card_ui, HStringArray, Handle,
    Hresult, S_OK,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
