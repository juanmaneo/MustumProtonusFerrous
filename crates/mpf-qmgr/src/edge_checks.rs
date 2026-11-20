//! Safe edge behavior and Wine quirks that are outside ordinary valid inputs.

use crate::auth::{index_from_scheme, scheme_from_index, target_from_index, AuthScheme};
use crate::enumerate::{next, skip};
use crate::http::hresult_from_http_response;
use crate::notify::check_notify_flags;
use crate::{
    is_job_done, transition_job_state, JobState, BG_E_INVALID_STATE, E_NOTIMPL, S_FALSE, S_OK,
};

#[test]
fn all_job_states_and_terminal_conditions_are_stable() {
    let states = [
        JobState::Queued,
        JobState::Connecting,
        JobState::Transferring,
        JobState::Suspended,
        JobState::Error,
        JobState::TransientError,
        JobState::Transferred,
        JobState::Acknowledged,
        JobState::Cancelled,
    ];
    for state in states {
        assert_eq!(
            is_job_done(state),
            matches!(state, JobState::Acknowledged | JobState::Cancelled)
        );
        let mut unchanged = state;
        let from = if state == JobState::Queued {
            JobState::Connecting
        } else {
            JobState::Queued
        };
        assert!(!transition_job_state(&mut unchanged, from, JobState::Error));
        assert_eq!(unchanged, state);
    }
    let mut queued = JobState::Queued;
    assert!(transition_job_state(
        &mut queued,
        JobState::Queued,
        JobState::Connecting
    ));
    assert_eq!(queued, JobState::Connecting);
}

#[test]
fn auth_unknown_indices_are_explicitly_absent() {
    assert_eq!(target_from_index(2), None);
    assert_eq!(scheme_from_index(5), None);
    assert_eq!(index_from_scheme(0), None);
    assert_eq!(index_from_scheme(6), None);
}

#[test]
fn scheme_table_keeps_wines_non_round_trip_order() {
    assert_eq!(index_from_scheme(2), Some(1));
    assert_eq!(scheme_from_index(1), Some(AuthScheme::Ntlm));
    assert_ne!(scheme_from_index(1), Some(AuthScheme::Digest));
}

#[test]
fn notify_validation_checks_done_before_unknown_bits() {
    assert_eq!(
        check_notify_flags(JobState::Cancelled, u32::MAX),
        BG_E_INVALID_STATE
    );
    assert_eq!(check_notify_flags(JobState::Queued, 0x8000_0000), E_NOTIMPL);
    assert_eq!(check_notify_flags(JobState::Queued, 0), S_OK);
}

#[test]
fn unhandled_http_codes_succeed_like_wine() {
    assert_eq!(hresult_from_http_response(0), S_OK);
    assert_eq!(hresult_from_http_response(599), S_OK);
}

#[test]
fn enumerator_zero_requests_and_past_end_are_safe() {
    assert_eq!(next(4, 2, 0).hresult, S_OK);
    assert_eq!(next(4, 2, 0).next_index, 2);
    assert_eq!(skip(4, 2, 0).next_index, 2);
    assert_eq!(next(2, 3, 1).count, 0);
    assert_eq!(next(2, 3, 1).next_index, 3);
    assert_eq!(skip(2, 3, 0).next_index, 3);
    assert_eq!(skip(2, 1, 2).hresult, S_FALSE);
}
