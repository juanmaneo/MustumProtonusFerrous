//! Rows recorded from the verbatim Wine C functions (see the oracle note in `lib.rs`).

use crate::{
    compare_int_safearrays, find_guid, guid, index_from_id, reserve_capacity, Guid, SafeArray,
};

struct CmpRow {
    a_dims: u32,
    a: &'static [i32],
    b_dims: u32,
    b: &'static [i32],
    prop_type: i32,
    expected: i32,
}

struct ReserveRow {
    capacity: usize,
    count: usize,
    size: usize,
    expected: Option<usize>,
}

struct FindRow {
    key: Guid,
    expected: Option<i32>,
}

struct IndexRow {
    id: i32,
    expected: Option<usize>,
}

const PROP_ID_MIN: i32 = 30000;

/// The `prop_id_idx` table of the harness; its length gives the id range `30000..30005`.
const PROP_ID_IDX: [usize; 5] = [3, 0, 4, 1, 2];

/// The sorted GUID table of the harness, in memcmp order, with the ids it maps to.
const FIND_TABLE: &[(Guid, i32)] = &[
    (
        guid(
            0x00000007,
            0x1111,
            0x2222,
            [0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0x00],
        ),
        1002,
    ),
    (
        guid(
            0x9f3c1a2b,
            0x0102,
            0x0304,
            [0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c],
        ),
        1001,
    ),
    (
        guid(
            0x12345678,
            0xabcd,
            0xef01,
            [0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01],
        ),
        1003,
    ),
    (
        guid(
            0x55aa55aa,
            0x0001,
            0x0002,
            [0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a],
        ),
        1004,
    ),
    (
        guid(
            0xdeadbeef,
            0xfeed,
            0xbead,
            [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
        ),
        1005,
    ),
];

const CMP_ROWS: &[CmpRow] = &[
    CmpRow {
        a_dims: 1,
        a: &[1, 2, 3],
        b_dims: 1,
        b: &[1, 2, 3],
        prop_type: 0x10001,
        expected: 0,
    },
    CmpRow {
        a_dims: 1,
        a: &[1, 2, 3],
        b_dims: 1,
        b: &[1, 2, 4],
        prop_type: 0x10001,
        expected: -1,
    },
    CmpRow {
        a_dims: 1,
        a: &[1, 2, 5],
        b_dims: 1,
        b: &[1, 2, 4],
        prop_type: 0x10001,
        expected: 1,
    },
    CmpRow {
        a_dims: 1,
        a: &[1, 2],
        b_dims: 1,
        b: &[1, 2, 3],
        prop_type: 0x10001,
        expected: -1,
    },
    CmpRow {
        a_dims: 1,
        a: &[9, 9, 9, 9],
        b_dims: 1,
        b: &[1],
        prop_type: 0x10001,
        expected: 1,
    },
    CmpRow {
        a_dims: 1,
        a: &[1, 2, 3],
        b_dims: 1,
        b: &[1, 2, 4],
        prop_type: 0x10003,
        expected: -1,
    },
    CmpRow {
        a_dims: 1,
        a: &[1, 2, 3],
        b_dims: 1,
        b: &[1, 2, 3],
        prop_type: 0x10002,
        expected: -1,
    },
    CmpRow {
        a_dims: 2,
        a: &[1, 2],
        b_dims: 1,
        b: &[1, 2],
        prop_type: 0x10001,
        expected: -1,
    },
    CmpRow {
        a_dims: 1,
        a: &[1, 2],
        b_dims: 2,
        b: &[1, 2, 3],
        prop_type: 0x10001,
        expected: -1,
    },
    CmpRow {
        a_dims: 1,
        a: &[-5, 3],
        b_dims: 1,
        b: &[-5, 2],
        prop_type: 0x10001,
        expected: 1,
    },
    CmpRow {
        a_dims: 1,
        a: &[-2147483648],
        b_dims: 1,
        b: &[2147483647],
        prop_type: 0x10001,
        expected: -1,
    },
];

const RESERVE_ROWS: &[ReserveRow] = &[
    ReserveRow {
        capacity: 4,
        count: 3,
        size: 8,
        expected: Some(4),
    },
    ReserveRow {
        capacity: 0,
        count: 5,
        size: 8,
        expected: Some(8),
    },
    ReserveRow {
        capacity: 3,
        count: 7,
        size: 8,
        expected: Some(12),
    },
    ReserveRow {
        capacity: 5,
        count: 6,
        size: 16,
        expected: Some(10),
    },
    ReserveRow {
        capacity: 0,
        count: 3,
        size: 6148914691236517205,
        expected: Some(3),
    },
    ReserveRow {
        capacity: 0,
        count: 4,
        size: 6148914691236517205,
        expected: None,
    },
];

const FIND_ROWS: &[FindRow] = &[
    FindRow {
        key: guid(
            0x00000007,
            0x1111,
            0x2222,
            [0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0x00],
        ),
        expected: Some(1002),
    },
    FindRow {
        key: guid(
            0xdeadbeef,
            0xfeed,
            0xbead,
            [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
        ),
        expected: Some(1005),
    },
    FindRow {
        key: guid(
            0x12345678,
            0xabcd,
            0xef01,
            [0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe],
        ),
        expected: None,
    },
    FindRow {
        key: guid(
            0x00000000,
            0x0000,
            0x0000,
            [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        ),
        expected: None,
    },
];

const INDEX_ROWS: &[IndexRow] = &[
    IndexRow {
        id: 30000,
        expected: Some(3),
    },
    IndexRow {
        id: 30004,
        expected: Some(2),
    },
    IndexRow {
        id: 30005,
        expected: None,
    },
];

#[test]
fn compare_rows_match_wine() {
    for row in CMP_ROWS {
        let a = SafeArray {
            dims: row.a_dims,
            values: row.a,
        };
        let b = SafeArray {
            dims: row.b_dims,
            values: row.b,
        };
        assert_eq!(
            compare_int_safearrays(&a, &b, row.prop_type),
            row.expected,
            "a={:?} b={:?} prop_type={:#x}",
            row.a,
            row.b,
            row.prop_type
        );
    }
}

#[test]
fn reserve_rows_match_wine() {
    for row in RESERVE_ROWS {
        assert_eq!(
            reserve_capacity(row.capacity, row.count, row.size),
            row.expected,
            "capacity={} count={} size={}",
            row.capacity,
            row.count,
            row.size
        );
    }
}

#[test]
fn find_rows_match_wine() {
    for row in FIND_ROWS {
        assert_eq!(
            find_guid(FIND_TABLE, &row.key),
            row.expected,
            "key={:?}",
            row.key
        );
    }
}

#[test]
fn index_rows_match_wine() {
    for row in INDEX_ROWS {
        assert_eq!(
            index_from_id(row.id, PROP_ID_MIN, &PROP_ID_IDX),
            row.expected,
            "id={}",
            row.id
        );
    }
}
