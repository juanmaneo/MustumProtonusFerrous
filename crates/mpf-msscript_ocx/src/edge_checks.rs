use super::{
    enumerate::{self, Next, Skip},
    error_fill::PosFill,
    failed,
    module_table::{self, CTL_E_OUT_OF_RANGE},
    procedure::{self, Entry},
    timeout::{self, CTL_E_INVALIDPROPERTYVALUE},
    S_FALSE, S_OK,
};

fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

#[test]
fn hresult_severity() {
    assert!(!failed(S_OK));
    assert!(!failed(S_FALSE));
    assert!(!failed(0x7FFF_FFFF));
    assert!(failed(0x8000_4005));
    assert!(failed(CTL_E_INVALIDPROPERTYVALUE));
}

#[test]
fn next_edges() {
    assert_eq!(
        enumerate::next(5, 5, 0),
        Next {
            fetched: 0,
            pos: 5,
            hr: S_OK
        }
    );
    assert_eq!(
        enumerate::next(10, 8, u32::MAX),
        Next {
            fetched: 2,
            pos: 10,
            hr: S_FALSE
        }
    );
    assert_eq!(
        enumerate::next(4, 0, 4),
        Next {
            fetched: 4,
            pos: 4,
            hr: S_OK
        }
    );
    assert_eq!(
        enumerate::next(0, 0, 1),
        Next {
            fetched: 0,
            pos: 0,
            hr: S_FALSE
        }
    );
}

#[test]
fn skip_edges() {
    assert_eq!(enumerate::skip(5, 0, 0), Skip { pos: 0, hr: S_OK });
    assert_eq!(enumerate::skip(5, 0, 5), Skip { pos: 5, hr: S_OK });
    assert_eq!(
        enumerate::skip(5, 0, 6),
        Skip {
            pos: 5,
            hr: S_FALSE
        }
    );
    assert_eq!(
        enumerate::skip(5, 5, 1),
        Skip {
            pos: 5,
            hr: S_FALSE
        }
    );
    assert_eq!(
        enumerate::skip(0, 0, u32::MAX),
        Skip {
            pos: 0,
            hr: S_FALSE
        }
    );
}

#[test]
fn growth_edges() {
    assert!(module_table::needs_growth(0));
    assert!(module_table::needs_growth(1));
    assert!(!module_table::needs_growth(6));
    assert!(module_table::needs_growth(8));
    assert!(module_table::needs_growth(1 << 31));
    assert!(!module_table::needs_growth(u32::MAX));
    assert_eq!(module_table::grown_capacity(0), 0);
    assert_eq!(module_table::grown_capacity(4), 8);
    assert_eq!(module_table::grown_capacity(1 << 31), 0);
}

#[test]
fn module_index_edges() {
    assert_eq!(module_table::module_index(1, 1), Ok(0));
    assert_eq!(module_table::module_index(3, 3), Ok(2));
    assert_eq!(module_table::module_index(4, 3), Err(CTL_E_OUT_OF_RANGE));
    assert_eq!(module_table::module_index(1, 0), Err(CTL_E_OUT_OF_RANGE));
    assert_eq!(
        module_table::module_index(i32::MAX, 3),
        Err(CTL_E_OUT_OF_RANGE)
    );
    // Excluded from the oracle: the C `V_INT(&index) - 1` overflows for INT_MIN (UBSan).
    assert_eq!(
        module_table::module_index(i32::MIN, 3),
        Err(CTL_E_OUT_OF_RANGE)
    );
}

#[test]
fn timeout_edges() {
    let mut stored = 7;
    assert_eq!(
        timeout::set_timeout(&mut stored, i32::MIN),
        Err(CTL_E_INVALIDPROPERTYVALUE)
    );
    assert_eq!(stored, 7);
    assert_eq!(timeout::set_timeout(&mut stored, i32::MAX), Ok(()));
    assert_eq!(stored, i32::MAX);
}

#[test]
fn bucket_index_edges() {
    assert_eq!(procedure::bucket_index(0), 0);
    assert_eq!(procedure::bucket_index(42), 42);
    assert_eq!(procedure::bucket_index(43), 0);
    assert_eq!(procedure::bucket_index(u32::MAX), (u32::MAX % 43) as usize);
}

#[test]
fn find_in_bucket_edges() {
    let run = wide("Run");
    let run2 = wide("Run2");
    let rub = wide("Rub");
    let nul = wide("Run\0");

    assert_eq!(procedure::find_in_bucket(&[], 1, &run), None);

    let bucket = [
        Entry {
            hash: 1,
            name: &run2,
        },
        Entry {
            hash: 1,
            name: &run,
        },
        Entry {
            hash: 1,
            name: &run,
        },
        Entry {
            hash: 2,
            name: &run,
        },
    ];
    assert_eq!(procedure::find_in_bucket(&bucket, 1, &run), Some(1));
    assert_eq!(procedure::find_in_bucket(&bucket, 3, &run), None);
    assert_eq!(procedure::find_in_bucket(&bucket[..1], 1, &run), None);
    assert_eq!(procedure::find_in_bucket(&bucket[..2], 1, &rub), None);
    assert_eq!(procedure::find_in_bucket(&bucket[..2], 1, &nul), None);
}

#[test]
fn num_args_and_return_edges() {
    assert_eq!(procedure::num_args(2, 1), 3);
    assert_eq!(procedure::num_args(i16::MAX, 1), 0x8000);
    assert_eq!(procedure::num_args(-1, 0), u16::MAX);
    assert_eq!(procedure::num_args(i16::MIN, i16::MIN), 0);
    assert!(procedure::has_return_value(0));
}

#[test]
fn pos_fill_latches_first_lookup() {
    let mut fill = PosFill::default();
    let mut lookups = 0;
    let mut lookup = || {
        lookups += 1;
        Some((2, -1))
    };
    fill.fill(&mut lookup);
    fill.fill(&mut lookup);
    assert_eq!(lookups, 1);
    assert_eq!((fill.line(), fill.column()), (2, -1));
}
