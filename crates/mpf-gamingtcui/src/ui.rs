/// `HRESULT`.
pub type Hresult = i32;

/// `S_OK`.
pub const S_OK: Hresult = 0;

/// An opaque pointer-sized value (`HSTRING`, `void *`). The port never dereferences it.
pub type Handle = usize;

/// A `(const HSTRING *, size_t)` pair, passed through unchanged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HStringArray {
    pub ptr: Handle,
    pub count: usize,
}

/// `ProcessPendingGameUI`. The reference ignores the `BOOL` and always returns `S_OK`.
pub fn process_pending_game_ui(_wait_for_completion: i32) -> Hresult {
    S_OK
}

/// `ShowPlayerPickerUI`. Returns `S_OK`. If a completion routine is given, it is called
/// once with `S_OK`, the context, and the preselected list as the selection.
pub fn show_player_picker_ui<F>(
    _prompt_display_text: Handle,
    _xuids: HStringArray,
    preselected_xuids: HStringArray,
    _min_selection_count: usize,
    _max_selection_count: usize,
    completion_routine: Option<F>,
    context: Handle,
) -> Hresult
where
    F: FnOnce(Hresult, Handle, HStringArray),
{
    if let Some(completion) = completion_routine {
        completion(S_OK, context, preselected_xuids);
    }
    S_OK
}

/// `ShowProfileCardUI`. Returns `S_OK`. If a completion routine is given, it is called once
/// with `S_OK` and the context.
pub fn show_profile_card_ui<F>(
    _target_user_xuid: Handle,
    completion_routine: Option<F>,
    context: Handle,
) -> Hresult
where
    F: FnOnce(Hresult, Handle),
{
    if let Some(completion) = completion_routine {
        completion(S_OK, context);
    }
    S_OK
}
