//! ninput subset for MPF: the handle and property logic of `dlls/ninput` interaction contexts.
//!
//! Reference Wine's `dlls/ninput/main.c` has bodies for 12 of the 31 entries in
//! `ninput.spec`; the other 19 are spec-only stubs with no C body. Four of the 12
//! hold real state logic with no input device, window, or live injection state. They
//! are ported here: the interaction context is a plain value that the caller owns.
//!
//! Ported (table tests against a verbatim Wine C oracle, see the tests):
//! - `CreateInteractionContext` as `create_interaction_context`: the null out-slot
//!   check, and a new context with both properties set to TRUE.
//! - `DestroyInteractionContext` as `destroy_interaction_context`: the null handle
//!   check. The context is owned, so destroying it consumes the value.
//! - `GetPropertyInteractionContext` as `get_property_interaction_context`: the
//!   handle and value checks, the `UI_FEEDBACK` and `FILTER_POINTERS` reads, and
//!   `E_INVALIDARG` for any other property.
//! - `SetPropertyInteractionContext` as `set_property_interaction_context`: the
//!   handle check, the TRUE/FALSE check on `UI_FEEDBACK` and `FILTER_POINTERS`,
//!   and `E_INVALIDARG` for any other property.
//!
//! `MEASUREMENT_UNITS` is the one property that the C body does not handle. Both
//! functions keep its Wine result: a get writes 0 and returns `E_NOTIMPL`, and a
//! set returns `E_NOTIMPL` without storing anything. Nothing is implemented for it.
//!
//! Skipped (27 of 31 spec entries):
//! - 19 spec-only entries with no body in `main.c`: nothing to port.
//! - 8 bodies that log `FIXME`. Six are stubs that check a handle or arguments and
//!   return a constant (or `E_NOTIMPL` for `ProcessInertiaInteractionContext`):
//!   `BufferPointerPacketsInteractionContext`, `GetStateInteractionContext`,
//!   `RegisterOutputCallbackInteractionContext`, `ProcessBufferedPacketsInteractionContext`,
//!   `ProcessInertiaInteractionContext`, and the undocumented ordinal 2502 (`NINPUT_2502`).
//!   The other two are the interaction configuration pair `Get`/`SetInteractionConfigurationInteractionContext`,
//!   marked "semi-stub" in C. They have real argument checks and storage, but the FIXME
//!   rule keeps them out of this first pass.
//!
//! Divergences from the C bodies:
//! - `E_OUTOFMEMORY` is not returned. Rust aborts on allocation failure, and the
//!   `calloc` failure path has no equivalent here.
//! - A `HINTERACTIONCONTEXT` that is destroyed twice, or used after destroy, is a
//!   use-after-free in C. The owned value makes both impossible to write.
//! - Overwriting a live handle in the out-slot leaks the old context in C. Here the
//!   old context is dropped.
//! - The `FIXME`, `WARN` and `TRACE` logging is not modelled.

/// `S_OK`.
pub const S_OK: i32 = 0;
/// `E_NOTIMPL`: the result for `INTERACTION_CONTEXT_PROPERTY_MEASUREMENT_UNITS`.
pub const E_NOTIMPL: i32 = 0x8000_4001u32 as i32;
/// `E_POINTER`: returned for a null out-slot or null value pointer.
pub const E_POINTER: i32 = 0x8000_4003u32 as i32;
/// `E_HANDLE`: returned for a null context handle.
pub const E_HANDLE: i32 = 0x8007_0006u32 as i32;
/// `E_INVALIDARG`: returned for an unknown property, or a value that is not a `BOOL`.
pub const E_INVALIDARG: i32 = 0x8007_0057u32 as i32;

/// `INTERACTION_CONTEXT_PROPERTY_MEASUREMENT_UNITS`: not handled by the C body.
pub const INTERACTION_CONTEXT_PROPERTY_MEASUREMENT_UNITS: u32 = 1;
/// `INTERACTION_CONTEXT_PROPERTY_INTERACTION_UI_FEEDBACK`.
pub const INTERACTION_CONTEXT_PROPERTY_INTERACTION_UI_FEEDBACK: u32 = 2;
/// `INTERACTION_CONTEXT_PROPERTY_FILTER_POINTERS`.
pub const INTERACTION_CONTEXT_PROPERTY_FILTER_POINTERS: u32 = 3;

const FALSE: u32 = 0;
const TRUE: u32 = 1;

/// An interaction context: the state behind a `HINTERACTIONCONTEXT`.
///
/// The value is owned and not `Clone`, so each context has one owner. Dropping it
/// is `DestroyInteractionContext`.
#[derive(Debug)]
pub struct InteractionContext {
    filter_pointers: bool,
    ui_feedback: bool,
}

impl InteractionContext {
    /// The state `CreateInteractionContext` gives a new context.
    fn new() -> Self {
        Self {
            filter_pointers: true,
            ui_feedback: true,
        }
    }
}

/// `CreateInteractionContext`: stores a new context in `out` and returns `S_OK`.
///
/// A `None` out-slot returns `E_POINTER`. A context that was already in the slot is
/// dropped when it is overwritten.
pub fn create_interaction_context(out: Option<&mut Option<InteractionContext>>) -> i32 {
    let Some(out) = out else {
        return E_POINTER;
    };
    *out = Some(InteractionContext::new());
    S_OK
}

/// `DestroyInteractionContext`: releases `handle`. `None` returns `E_HANDLE`.
pub fn destroy_interaction_context(handle: Option<InteractionContext>) -> i32 {
    match handle {
        Some(_context) => S_OK,
        None => E_HANDLE,
    }
}

/// `GetPropertyInteractionContext`: writes the property into `value` and returns the status.
///
/// The handle is checked before the value pointer, and the value pointer before the
/// property. A `None` value is `E_POINTER` for any property. Only an unknown property
/// (`E_INVALIDARG`) leaves `value` untouched.
pub fn get_property_interaction_context(
    handle: Option<&InteractionContext>,
    property: u32,
    value: Option<&mut u32>,
) -> i32 {
    let Some(context) = handle else {
        return E_HANDLE;
    };
    let Some(value) = value else {
        return E_POINTER;
    };
    match property {
        INTERACTION_CONTEXT_PROPERTY_MEASUREMENT_UNITS => {
            *value = 0;
            E_NOTIMPL
        }
        INTERACTION_CONTEXT_PROPERTY_INTERACTION_UI_FEEDBACK => {
            *value = u32::from(context.ui_feedback);
            S_OK
        }
        INTERACTION_CONTEXT_PROPERTY_FILTER_POINTERS => {
            *value = u32::from(context.filter_pointers);
            S_OK
        }
        _ => E_INVALIDARG,
    }
}

/// `SetPropertyInteractionContext`: stores `value` for `property` and returns the status.
///
/// `UI_FEEDBACK` and `FILTER_POINTERS` accept only `FALSE` (0) and `TRUE` (1). Any other
/// word is `E_INVALIDARG`, and nothing is stored. `MEASUREMENT_UNITS` returns
/// `E_NOTIMPL` and stores nothing.
pub fn set_property_interaction_context(
    handle: Option<&mut InteractionContext>,
    property: u32,
    value: u32,
) -> i32 {
    let Some(context) = handle else {
        return E_HANDLE;
    };
    match property {
        INTERACTION_CONTEXT_PROPERTY_MEASUREMENT_UNITS => E_NOTIMPL,
        INTERACTION_CONTEXT_PROPERTY_INTERACTION_UI_FEEDBACK => match bool_from_word(value) {
            Some(flag) => {
                context.ui_feedback = flag;
                S_OK
            }
            None => E_INVALIDARG,
        },
        INTERACTION_CONTEXT_PROPERTY_FILTER_POINTERS => match bool_from_word(value) {
            Some(flag) => {
                context.filter_pointers = flag;
                S_OK
            }
            None => E_INVALIDARG,
        },
        _ => E_INVALIDARG,
    }
}

/// The `BOOL` value of a word: `FALSE` and `TRUE` only. The whole word is compared.
fn bool_from_word(value: u32) -> Option<bool> {
    match value {
        FALSE => Some(false),
        TRUE => Some(true),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The value the oracle harness starts its output slot with.
    const SENTINEL: u32 = 0xA5A5_A5A5;

    /// `(ui_feedback, filter_pointers)` read back through `get_property_interaction_context`.
    type State = Option<(u32, u32)>;

    /// `(id, out-slot present, status, state of the created context)`.
    type CreateCase = (&'static str, bool, i32, State);
    /// `(id, handle live, status)`.
    type DestroyCase = (&'static str, bool, i32);
    /// `(id, handle live, property, value pointer present, status, out, state)`.
    type GetCase = (&'static str, bool, u32, bool, i32, u32, State);
    /// `(id, handle live, property, value, status, state)`.
    type SetCase = (&'static str, bool, u32, u32, i32, State);

    fn live_context() -> Option<InteractionContext> {
        let mut slot = None;
        assert_eq!(create_interaction_context(Some(&mut slot)), S_OK);
        slot
    }

    fn read_state(context: Option<&InteractionContext>) -> State {
        let context = context?;
        let (mut ui, mut filter) = (SENTINEL, SENTINEL);
        assert_eq!(
            get_property_interaction_context(
                Some(context),
                INTERACTION_CONTEXT_PROPERTY_INTERACTION_UI_FEEDBACK,
                Some(&mut ui),
            ),
            S_OK
        );
        assert_eq!(
            get_property_interaction_context(
                Some(context),
                INTERACTION_CONTEXT_PROPERTY_FILTER_POINTERS,
                Some(&mut filter),
            ),
            S_OK
        );
        Some((ui, filter))
    }

    #[test]
    fn create_matches_wine_oracle() {
        // Rows C01 and C02 of target/canon/ninput/out.txt.
        let cases: [CreateCase; 2] = [
            ("C01", false, E_POINTER, None),
            ("C02", true, S_OK, Some((1, 1))),
        ];
        for (id, has_slot, expected, state) in cases {
            let mut slot = None;
            let hr = create_interaction_context(if has_slot { Some(&mut slot) } else { None });
            assert_eq!(hr, expected, "{id}");
            assert_eq!(read_state(slot.as_ref()), state, "{id}");
        }
    }

    #[test]
    fn create_overwrites_a_slot_that_holds_a_context() {
        // Rust-only: the C body overwrites the slot too. The old context is dropped here.
        let mut slot = live_context();
        assert_eq!(
            set_property_interaction_context(
                slot.as_mut(),
                INTERACTION_CONTEXT_PROPERTY_FILTER_POINTERS,
                FALSE,
            ),
            S_OK
        );
        assert_eq!(read_state(slot.as_ref()), Some((1, 0)));
        assert_eq!(create_interaction_context(Some(&mut slot)), S_OK);
        assert_eq!(read_state(slot.as_ref()), Some((1, 1)));
    }

    #[test]
    fn destroy_matches_wine_oracle() {
        // Rows D01 and D02 of target/canon/ninput/out.txt.
        let cases: [DestroyCase; 2] = [("D01", false, E_HANDLE), ("D02", true, S_OK)];
        for (id, live, expected) in cases {
            let handle = if live { live_context() } else { None };
            assert_eq!(destroy_interaction_context(handle), expected, "{id}");
        }
    }

    #[test]
    fn get_property_matches_wine_oracle() {
        // Rows G01 to G15 of target/canon/ninput/out.txt. The state is read after the call.
        let cases: [GetCase; 15] = [
            ("G01", false, 2, true, E_HANDLE, SENTINEL, None),
            ("G02", false, 2, false, E_HANDLE, SENTINEL, None),
            ("G03", true, 2, false, E_POINTER, SENTINEL, Some((1, 1))),
            ("G04", true, 1, true, E_NOTIMPL, 0, Some((1, 1))),
            ("G05", true, 2, true, S_OK, 1, Some((1, 1))),
            ("G06", true, 3, true, S_OK, 1, Some((1, 1))),
            ("G07", true, 0, true, E_INVALIDARG, SENTINEL, Some((1, 1))),
            ("G08", true, 4, true, E_INVALIDARG, SENTINEL, Some((1, 1))),
            (
                "G09",
                true,
                0xFFFF_FFFF,
                true,
                E_INVALIDARG,
                SENTINEL,
                Some((1, 1)),
            ),
            ("G10", false, 1, true, E_HANDLE, SENTINEL, None),
            ("G11", false, 0, false, E_HANDLE, SENTINEL, None),
            ("G12", true, 0, false, E_POINTER, SENTINEL, Some((1, 1))),
            ("G13", true, 1, false, E_POINTER, SENTINEL, Some((1, 1))),
            ("G14", true, 5, true, E_INVALIDARG, SENTINEL, Some((1, 1))),
            ("G15", true, 2, false, E_POINTER, SENTINEL, Some((1, 1))),
        ];
        for (id, live, property, has_value, expected, expected_out, state) in cases {
            let context = if live { live_context() } else { None };
            let mut out = SENTINEL;
            let value = if has_value { Some(&mut out) } else { None };
            let hr = get_property_interaction_context(context.as_ref(), property, value);
            assert_eq!(hr, expected, "{id}");
            assert_eq!(out, expected_out, "{id}");
            assert_eq!(read_state(context.as_ref()), state, "{id}");
        }
    }

    #[test]
    fn set_property_matches_wine_oracle() {
        // Rows S01 to S17 of target/canon/ninput/out.txt. The state is read after the call.
        let cases: [SetCase; 17] = [
            ("S01", false, 2, 1, E_HANDLE, None),
            ("S02", true, 2, 0, S_OK, Some((0, 1))),
            ("S03", true, 2, 1, S_OK, Some((1, 1))),
            ("S04", true, 2, 2, E_INVALIDARG, Some((1, 1))),
            ("S05", true, 2, 4_294_967_295, E_INVALIDARG, Some((1, 1))),
            ("S06", true, 3, 0, S_OK, Some((1, 0))),
            ("S07", true, 3, 1, S_OK, Some((1, 1))),
            ("S08", true, 3, 2, E_INVALIDARG, Some((1, 1))),
            ("S09", true, 1, 0, E_NOTIMPL, Some((1, 1))),
            ("S10", true, 1, 1, E_NOTIMPL, Some((1, 1))),
            ("S11", true, 0, 1, E_INVALIDARG, Some((1, 1))),
            ("S12", true, 4, 0, E_INVALIDARG, Some((1, 1))),
            ("S13", true, 0xFFFF_FFFF, 1, E_INVALIDARG, Some((1, 1))),
            ("S14", false, 0, 0, E_HANDLE, None),
            ("S15", true, 3, 42, E_INVALIDARG, Some((1, 1))),
            ("S16", true, 2, 2_147_483_648, E_INVALIDARG, Some((1, 1))),
            ("S17", false, 1, 1, E_HANDLE, None),
        ];
        for (id, live, property, value, expected, state) in cases {
            let mut context = if live { live_context() } else { None };
            let hr = set_property_interaction_context(context.as_mut(), property, value);
            assert_eq!(hr, expected, "{id}");
            assert_eq!(read_state(context.as_ref()), state, "{id}");
        }
    }

    #[test]
    fn set_property_compares_the_whole_word() {
        // Rust-only: a BOOL is compared as a full u32, so a set low byte is not TRUE.
        for value in [0x100, 0x0100_0001, 0x8000_0001] {
            let mut context = live_context();
            assert_eq!(
                set_property_interaction_context(
                    context.as_mut(),
                    INTERACTION_CONTEXT_PROPERTY_INTERACTION_UI_FEEDBACK,
                    value,
                ),
                E_INVALIDARG
            );
            assert_eq!(read_state(context.as_ref()), Some((1, 1)));
        }
    }

    #[test]
    fn get_reads_back_what_set_stored() {
        // Rust-only round trip: each setting is read back as the last value stored.
        let mut context = live_context();
        let setting = [
            (INTERACTION_CONTEXT_PROPERTY_FILTER_POINTERS, FALSE),
            (INTERACTION_CONTEXT_PROPERTY_INTERACTION_UI_FEEDBACK, FALSE),
            (INTERACTION_CONTEXT_PROPERTY_FILTER_POINTERS, TRUE),
        ];
        for (property, value) in setting {
            assert_eq!(
                set_property_interaction_context(context.as_mut(), property, value),
                S_OK
            );
        }
        assert_eq!(read_state(context.as_ref()), Some((0, 1)));
    }

    #[test]
    fn handle_is_checked_before_value_and_property() {
        // Rust-only: a null handle wins over a null value and an unknown property.
        let mut out = SENTINEL;
        assert_eq!(
            get_property_interaction_context(None, 99, Some(&mut out)),
            E_HANDLE
        );
        assert_eq!(out, SENTINEL);
        assert_eq!(get_property_interaction_context(None, 99, None), E_HANDLE);
        assert_eq!(set_property_interaction_context(None, 99, 7), E_HANDLE);
    }

    #[test]
    fn bool_from_word_accepts_only_false_and_true() {
        assert_eq!(bool_from_word(FALSE), Some(false));
        assert_eq!(bool_from_word(TRUE), Some(true));
        assert_eq!(bool_from_word(2), None);
        assert_eq!(bool_from_word(u32::MAX), None);
    }

    #[test]
    fn status_codes_match_winerror() {
        assert_eq!(E_NOTIMPL as u32, 0x8000_4001);
        assert_eq!(E_POINTER as u32, 0x8000_4003);
        assert_eq!(E_HANDLE as u32, 0x8007_0006);
        assert_eq!(E_INVALIDARG as u32, 0x8007_0057);
    }
}
