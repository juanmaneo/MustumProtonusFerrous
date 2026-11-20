//! Twenty rows emitted by `target/canon/qmgr/oracle.c`, compiled from the pinned Wine helpers
//! with GCC AddressSanitizer and UndefinedBehaviorSanitizer. No vector was sanitizer-flagged.

use crate::auth::{
    index_from_scheme, index_from_target, scheme_from_index, target_from_index, AuthScheme,
    AuthTarget,
};
use crate::enumerate::{next, skip};
use crate::http::hresult_from_http_response;
use crate::notify::check_notify_flags;
use crate::{is_job_done, transition_job_state, HResult, JobState};

#[derive(Clone, Copy)]
struct Case {
    code: u32,
    state: u32,
    from: u32,
    to: u32,
    flags: u32,
    target_index: u32,
    scheme_index: u32,
    target: u32,
    scheme: u32,
    total: u32,
    index: u32,
    requested: u32,
    expected: Expected,
}

#[derive(Clone, Copy)]
struct Expected {
    http: HResult,
    state: u32,
    done: bool,
    changed: bool,
    flags: u32,
    notify: HResult,
    target: u32,
    scheme: u32,
    target_index: i32,
    scheme_index: i32,
    next_hr: HResult,
    next_count: u32,
    next_index: u32,
    skip_hr: HResult,
    skip_index: u32,
}

#[rustfmt::skip]
const ROWS: &[Case] = &[
    Case { code:200,state:0,from:0,to:2,flags:0,target_index:0,scheme_index:0,target:1,scheme:1,total:3,index:0,requested:2,expected:Expected{http:0,state:2,done:false,changed:true,flags:0,notify:0,target:0,scheme:1,target_index:0,scheme_index:0,next_hr:0,next_count:2,next_index:2,skip_hr:0,skip_index:2}},
    Case { code:400,state:1,from:0,to:2,flags:1,target_index:1,scheme_index:1,target:2,scheme:2,total:3,index:2,requested:2,expected:Expected{http:(0x80190190_u32 as HResult),state:1,done:false,changed:false,flags:1,notify:0,target:1,scheme:2,target_index:1,scheme_index:1,next_hr:1,next_count:1,next_index:3,skip_hr:1,skip_index:3}},
    Case { code:401,state:2,from:2,to:3,flags:2,target_index:2,scheme_index:2,target:0,scheme:3,total:3,index:3,requested:1,expected:Expected{http:(0x80190191_u32 as HResult),state:3,done:false,changed:true,flags:2,notify:0,target:0,scheme:4,target_index:-1,scheme_index:2,next_hr:1,next_count:0,next_index:3,skip_hr:1,skip_index:3}},
    Case { code:404,state:3,from:2,to:4,flags:4,target_index:3,scheme_index:3,target:3,scheme:4,total:3,index:1,requested:3,expected:Expected{http:(0x80190194_u32 as HResult),state:3,done:false,changed:false,flags:4,notify:0,target:0,scheme:8,target_index:-1,scheme_index:3,next_hr:1,next_count:2,next_index:3,skip_hr:1,skip_index:3}},
    Case { code:407,state:4,from:4,to:5,flags:8,target_index:4,scheme_index:4,target:1,scheme:5,total:3,index:3,requested:0,expected:Expected{http:(0x80190197_u32 as HResult),state:5,done:false,changed:true,flags:8,notify:0,target:0,scheme:16,target_index:0,scheme_index:4,next_hr:0,next_count:0,next_index:3,skip_hr:0,skip_index:3}},
    Case { code:414,state:5,from:0,to:1,flags:16,target_index:5,scheme_index:5,target:2,scheme:6,total:3,index:0,requested:4,expected:Expected{http:(0x8019019e_u32 as HResult),state:5,done:false,changed:false,flags:16,notify:0,target:0,scheme:0,target_index:1,scheme_index:-1,next_hr:1,next_count:3,next_index:3,skip_hr:1,skip_index:3}},
    Case { code:501,state:6,from:6,to:7,flags:31,target_index:0,scheme_index:0,target:1,scheme:0,total:5,index:0,requested:5,expected:Expected{http:(0x801901f5_u32 as HResult),state:7,done:true,changed:true,flags:0,notify:(0x80200002_u32 as HResult),target:0,scheme:1,target_index:0,scheme_index:-1,next_hr:0,next_count:5,next_index:5,skip_hr:0,skip_index:5}},
    Case { code:503,state:7,from:7,to:8,flags:32,target_index:1,scheme_index:1,target:2,scheme:1,total:5,index:4,requested:2,expected:Expected{http:(0x801901f7_u32 as HResult),state:8,done:true,changed:true,flags:0,notify:(0x80200002_u32 as HResult),target:1,scheme:2,target_index:1,scheme_index:0,next_hr:1,next_count:1,next_index:5,skip_hr:1,skip_index:5}},
    Case { code:504,state:8,from:8,to:0,flags:0x80000000,target_index:2,scheme_index:2,target:0,scheme:6,total:5,index:5,requested:1,expected:Expected{http:(0x801901f8_u32 as HResult),state:0,done:false,changed:true,flags:0,notify:(0x80004001_u32 as HResult),target:0,scheme:4,target_index:-1,scheme_index:-1,next_hr:1,next_count:0,next_index:5,skip_hr:1,skip_index:5}},
    Case { code:505,state:0,from:1,to:4,flags:0x21,target_index:3,scheme_index:3,target:3,scheme:5,total:4,index:1,requested:3,expected:Expected{http:(0x801901f9_u32 as HResult),state:0,done:false,changed:false,flags:0,notify:(0x80004001_u32 as HResult),target:0,scheme:8,target_index:-1,scheme_index:4,next_hr:0,next_count:3,next_index:4,skip_hr:0,skip_index:4}},
    Case { code:599,state:1,from:1,to:2,flags:0x10,target_index:4,scheme_index:4,target:2,scheme:4,total:4,index:2,requested:2,expected:Expected{http:0,state:2,done:false,changed:true,flags:16,notify:0,target:0,scheme:16,target_index:1,scheme_index:3,next_hr:0,next_count:2,next_index:4,skip_hr:0,skip_index:4}},
    Case { code:200,state:2,from:0,to:1,flags:0x20,target_index:0,scheme_index:5,target:1,scheme:5,total:4,index:0,requested:1,expected:Expected{http:0,state:2,done:false,changed:false,flags:0,notify:(0x80004001_u32 as HResult),target:0,scheme:0,target_index:0,scheme_index:4,next_hr:0,next_count:1,next_index:1,skip_hr:0,skip_index:1}},
    Case { code:400,state:3,from:3,to:6,flags:0x1f,target_index:1,scheme_index:0,target:2,scheme:0,total:2,index:0,requested:2,expected:Expected{http:(0x80190190_u32 as HResult),state:6,done:false,changed:true,flags:31,notify:0,target:1,scheme:1,target_index:1,scheme_index:-1,next_hr:0,next_count:2,next_index:2,skip_hr:0,skip_index:2}},
    Case { code:401,state:4,from:3,to:5,flags:0x40,target_index:2,scheme_index:1,target:1,scheme:1,total:2,index:1,requested:2,expected:Expected{http:(0x80190191_u32 as HResult),state:4,done:false,changed:false,flags:0,notify:(0x80004001_u32 as HResult),target:0,scheme:2,target_index:0,scheme_index:0,next_hr:1,next_count:1,next_index:2,skip_hr:1,skip_index:2}},
    Case { code:404,state:5,from:5,to:6,flags:0x0f,target_index:0,scheme_index:2,target:2,scheme:2,total:2,index:2,requested:1,expected:Expected{http:(0x80190194_u32 as HResult),state:6,done:false,changed:true,flags:15,notify:0,target:0,scheme:4,target_index:1,scheme_index:1,next_hr:1,next_count:0,next_index:2,skip_hr:1,skip_index:2}},
    Case { code:407,state:6,from:4,to:2,flags:0x10,target_index:1,scheme_index:3,target:0,scheme:3,total:1,index:0,requested:2,expected:Expected{http:(0x80190197_u32 as HResult),state:6,done:false,changed:false,flags:16,notify:0,target:1,scheme:8,target_index:-1,scheme_index:2,next_hr:1,next_count:1,next_index:1,skip_hr:1,skip_index:1}},
    Case { code:414,state:7,from:6,to:8,flags:0x80,target_index:2,scheme_index:4,target:3,scheme:4,total:1,index:1,requested:1,expected:Expected{http:(0x8019019e_u32 as HResult),state:7,done:true,changed:false,flags:0,notify:(0x80200002_u32 as HResult),target:0,scheme:16,target_index:-1,scheme_index:3,next_hr:1,next_count:0,next_index:1,skip_hr:1,skip_index:1}},
    Case { code:501,state:8,from:8,to:7,flags:4,target_index:3,scheme_index:5,target:1,scheme:5,total:0,index:0,requested:0,expected:Expected{http:(0x801901f5_u32 as HResult),state:7,done:true,changed:true,flags:0,notify:(0x80200002_u32 as HResult),target:0,scheme:0,target_index:0,scheme_index:4,next_hr:0,next_count:0,next_index:0,skip_hr:0,skip_index:0}},
    Case { code:503,state:0,from:6,to:2,flags:2,target_index:4,scheme_index:0,target:2,scheme:6,total:4,index:3,requested:1,expected:Expected{http:(0x801901f7_u32 as HResult),state:0,done:false,changed:false,flags:2,notify:0,target:0,scheme:1,target_index:1,scheme_index:-1,next_hr:0,next_count:1,next_index:4,skip_hr:0,skip_index:4}},
    Case { code:999,state:1,from:1,to:0,flags:8,target_index:5,scheme_index:5,target:3,scheme:0,total:5,index:3,requested:3,expected:Expected{http:0,state:0,done:false,changed:true,flags:8,notify:0,target:0,scheme:0,target_index:-1,scheme_index:-1,next_hr:1,next_count:2,next_index:5,skip_hr:1,skip_index:5}},
];

fn state(value: u32) -> JobState {
    match value {
        0 => JobState::Queued,
        1 => JobState::Connecting,
        2 => JobState::Transferring,
        3 => JobState::Suspended,
        4 => JobState::Error,
        5 => JobState::TransientError,
        6 => JobState::Transferred,
        7 => JobState::Acknowledged,
        8 => JobState::Cancelled,
        _ => JobState::Queued,
    }
}

fn raw_target(value: Option<AuthTarget>) -> u32 {
    value.map_or(0, |target| target as u32)
}

fn raw_scheme(value: Option<AuthScheme>) -> u32 {
    value.map_or(0, |scheme| scheme as u32)
}

#[test]
fn helpers_match_twenty_sanitized_wine_oracle_rows() {
    assert_eq!(ROWS.len(), 20);
    assert_eq!(state(9), JobState::Queued);
    for (row_number, row) in ROWS.iter().enumerate() {
        assert_eq!(
            hresult_from_http_response(row.code),
            row.expected.http,
            "row {row_number}: HTTP"
        );

        let mut current = state(row.state);
        let changed = transition_job_state(&mut current, state(row.from), state(row.to));
        assert_eq!(
            current as u32, row.expected.state,
            "row {row_number}: state"
        );
        assert_eq!(
            changed, row.expected.changed,
            "row {row_number}: transition"
        );
        assert_eq!(
            is_job_done(current),
            row.expected.done,
            "row {row_number}: done"
        );

        let notify = check_notify_flags(current, row.flags);
        let stored_flags = if notify == 0 { row.flags } else { 0 };
        assert_eq!(
            notify, row.expected.notify,
            "row {row_number}: notify HRESULT"
        );
        assert_eq!(
            stored_flags, row.expected.flags,
            "row {row_number}: notify flags"
        );

        assert_eq!(
            raw_target(target_from_index(row.target_index)),
            row.expected.target,
            "row {row_number}: target table"
        );
        assert_eq!(
            raw_scheme(scheme_from_index(row.scheme_index)),
            row.expected.scheme,
            "row {row_number}: scheme table"
        );
        assert_eq!(
            index_from_target(row.target).map_or(-1, |v| v as i32),
            row.expected.target_index,
            "row {row_number}: target index"
        );
        assert_eq!(
            index_from_scheme(row.scheme).map_or(-1, |v| v as i32),
            row.expected.scheme_index,
            "row {row_number}: scheme index"
        );

        let got_next = next(row.total, row.index, row.requested);
        assert_eq!(
            got_next.hresult, row.expected.next_hr,
            "row {row_number}: Next HRESULT"
        );
        assert_eq!(
            got_next.count, row.expected.next_count,
            "row {row_number}: Next count"
        );
        assert_eq!(
            got_next.next_index, row.expected.next_index,
            "row {row_number}: Next index"
        );
        let got_skip = skip(row.total, row.index, row.requested);
        assert_eq!(
            got_skip.hresult, row.expected.skip_hr,
            "row {row_number}: Skip HRESULT"
        );
        assert_eq!(
            got_skip.next_index, row.expected.skip_index,
            "row {row_number}: Skip index"
        );
    }
}
