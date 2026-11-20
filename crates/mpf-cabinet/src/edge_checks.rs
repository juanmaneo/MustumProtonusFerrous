//! Edge checks for the cases the oracle rows do not reach: the header's error
//! paths and name bounds, the Huffman table's size and length limits, both
//! rescale branches of the QTM model and its sentinel index, and the derived
//! formatting and clone impls. The one case Wine's C cannot run, a cabinet size
//! with the high bit set, is checked here only.

use crate::header::{is_cabinet, read_header, Link, Reserve};
use crate::huffman::{make_decode_table, TableError};
use crate::quantum::{Model, QtmState, Sym};
use crate::Error;

/// A 36-byte CFHEADER with the given version, flags and cabinet size, then `tail`.
fn cfheader(major: u8, minor: u8, flags: u16, cab_size: u32, tail: &[u8]) -> Vec<u8> {
    let mut data = vec![0u8; 0x24];
    data[..4].copy_from_slice(b"MSCF");
    data[0x08..0x0C].copy_from_slice(&cab_size.to_le_bytes());
    data[0x18] = minor;
    data[0x19] = major;
    data[0x1E..0x20].copy_from_slice(&flags.to_le_bytes());
    data.extend_from_slice(tail);
    data
}

fn cumfreqs(model: &Model) -> Vec<u16> {
    model.syms.iter().map(|s| s.cumfreq).collect()
}

#[test]
fn short_data_and_wrong_signature_are_not_cabinets() {
    assert_eq!(read_header(&[0; 10]).err(), Some(Error::NotACabinet));
    assert_eq!(is_cabinet(&[0; 10]).err(), Some(Error::NotACabinet));
    let mut data = cfheader(1, 3, 0, 36, &[]);
    data[..4].copy_from_slice(b"MSCX");
    assert_eq!(read_header(&data).err(), Some(Error::NotACabinet));
}

#[test]
fn versions_newer_than_1_3_are_rejected() {
    assert_eq!(
        read_header(&cfheader(1, 4, 0, 36, &[])).err(),
        Some(Error::UnknownVersion)
    );
    assert_eq!(
        read_header(&cfheader(2, 0, 0, 36, &[])).err(),
        Some(Error::UnknownVersion)
    );
    assert!(read_header(&cfheader(1, 3, 0, 36, &[])).is_ok());
}

#[test]
fn names_need_a_nul_before_the_cabinet_size() {
    let no_nul = cfheader(1, 3, 0x0001, 39, b"abc");
    assert_eq!(read_header(&no_nul).err(), Some(Error::CorruptCabinet));
    // A cabinet size of 36 leaves no bytes to search.
    let empty = cfheader(1, 3, 0x0001, 36, b"a\0");
    assert_eq!(read_header(&empty).err(), Some(Error::CorruptCabinet));
    // The NUL is at byte 44, past a cabinet size of 40.
    let past_size = cfheader(1, 3, 0x0001, 40, b"abcdefgh\0");
    assert_eq!(read_header(&past_size).err(), Some(Error::CorruptCabinet));
}

#[test]
fn names_and_infos_are_split_at_nuls() {
    let data = cfheader(1, 3, 0x0003, 48, b"a\0b\0c\0");
    let header = read_header(&data).unwrap();
    assert_eq!(
        header.prev,
        Some(Link {
            name: b"a",
            info: Some(b"b"),
        })
    );
    assert_eq!(
        header.next,
        Some(Link {
            name: b"c",
            info: None,
        })
    );
}

#[test]
fn a_cabinet_size_outside_the_header_searches_to_the_end_of_the_data() {
    let zero = cfheader(1, 3, 0x0001, 0, b"ab\0");
    let header = read_header(&zero).unwrap();
    assert_eq!(
        header.prev,
        Some(Link {
            name: b"ab",
            info: None,
        })
    );
    // Wine shifts 0x80 into the sign bit to read this size, which is undefined
    // in C, so only the Rust port runs this case.
    let negative = cfheader(1, 3, 0x0001, 0x8000_0000, b"ab\0");
    let header = read_header(&negative).unwrap();
    assert_eq!(header.cab_size, i32::MIN);
    assert_eq!(
        header.prev,
        Some(Link {
            name: b"ab",
            info: None,
        })
    );
}

#[test]
fn is_cabinet_clears_next_but_still_parses_it() {
    let data = cfheader(1, 3, 0x0003, 48, b"a\0b\0c\0");
    let info = is_cabinet(&data).unwrap();
    assert!(info.next.is_none());
    assert!(info.prev.is_some());
    let corrupt_next = cfheader(1, 3, 0x0002, 0, b"x");
    assert_eq!(is_cabinet(&corrupt_next).err(), Some(Error::CorruptCabinet));
}

#[test]
fn reserved_sizes_are_read_then_skipped_before_names() {
    let tail = [2, 0, 3, 4, 0xAA, 0xBB, b'p', 0];
    let data = cfheader(1, 3, 0x0005, 44, &tail);
    let header = read_header(&data).unwrap();
    assert_eq!(
        header.reserve,
        Some(Reserve {
            header: 2,
            folder: 3,
            data: 4,
        })
    );
    assert_eq!(
        header.prev,
        Some(Link {
            name: b"p",
            info: None,
        })
    );
}

#[test]
fn reserved_block_must_be_complete_and_names_must_fit_after_it() {
    let short = cfheader(1, 3, 0x0004, 36, &[]);
    assert_eq!(read_header(&short).err(), Some(Error::CorruptCabinet));
    let skipped_past_end = cfheader(1, 3, 0x0005, 36, &[0xFF, 0xFF, 0, 0, b'p', 0]);
    assert_eq!(
        read_header(&skipped_past_end).err(),
        Some(Error::CorruptCabinet)
    );
}

#[test]
fn decode_table_rejects_oversized_inputs() {
    let mut table = [0u16; 2];
    assert_eq!(
        make_decode_table(16, &[1], &mut table).err(),
        Some(TableError::Invalid)
    );
    assert_eq!(
        make_decode_table(1, &[0u8; 65536], &mut table).err(),
        Some(TableError::Invalid)
    );
}

#[test]
fn decode_table_rejects_codes_that_overrun_or_are_incomplete() {
    let mut table = [0u16; 10];
    assert_eq!(
        make_decode_table(2, &[1, 1, 1], &mut table).err(),
        Some(TableError::Invalid)
    );
    let mut table = [0u16; 4];
    assert_eq!(
        make_decode_table(1, &[1, 2, 2, 2], &mut table).err(),
        Some(TableError::Invalid)
    );
    let mut table = [0u16; 8];
    assert_eq!(
        make_decode_table(3, &[2, 2, 2], &mut table).err(),
        Some(TableError::Invalid)
    );
}

#[test]
fn decode_table_reports_a_table_that_is_too_small() {
    let mut table = [0u16; 2];
    assert_eq!(
        make_decode_table(2, &[2, 2, 3], &mut table).err(),
        Some(TableError::TableTooSmall)
    );
    let mut table = [0u16; 3];
    assert_eq!(
        make_decode_table(2, &[2, 2, 2, 2], &mut table).err(),
        Some(TableError::TableTooSmall)
    );
}

#[test]
fn decode_table_accepts_complete_and_empty_codes() {
    let mut table = [9u16; 2];
    assert!(make_decode_table(1, &[1, 1], &mut table).is_ok());
    assert_eq!(table, [0, 1]);
    let mut table = [9u16; 1];
    assert!(make_decode_table(0, &[0], &mut table).is_ok());
    assert_eq!(table, [0]);
}

#[test]
fn qtm_init_sizes_the_models_by_window() {
    assert_eq!(QtmState::init(9).err(), Some(Error::DataFormat));
    assert_eq!(QtmState::init(22).err(), Some(Error::DataFormat));
    let small = QtmState::init(10).unwrap();
    assert_eq!(small.window_size, 1 << 10);
    assert_eq!(small.model6pos.entries, 20);
    let large = QtmState::init(21).unwrap();
    assert_eq!(large.window_size, 1 << 21);
    assert_eq!(
        (
            large.model4.entries,
            large.model5.entries,
            large.model6pos.entries
        ),
        (24, 36, 42)
    );
}

#[test]
fn update_counts_the_symbols_before_the_index() {
    let mut model = Model::new(3, 0);
    model.update(2);
    assert_eq!(cumfreqs(&model), vec![11, 10, 1, 0]);
}

#[test]
fn update_accepts_the_sentinel_index() {
    let mut model = Model::new(2, 0);
    model.update(3);
    assert_eq!(cumfreqs(&model), vec![10, 9, 8]);
}

#[test]
#[should_panic(expected = "out of range")]
fn update_past_the_sentinel_panics() {
    let mut model = Model::new(2, 0);
    model.update(4);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn new_panics_when_the_symbols_leave_the_lookup_table() {
    Model::new(2, 255);
}

#[test]
fn halve_keeps_the_cumulative_frequencies_strictly_decreasing() {
    let mut model = Model::new(2, 0);
    model.syms[0].cumfreq = 3801;
    model.update(0);
    assert_eq!(model.shiftsleft, 3);
    assert_eq!(cumfreqs(&model), vec![1900, 1, 0]);
}

#[test]
fn rebuild_sorts_the_frequencies_and_updates_the_lookup() {
    let mut model = Model::new(3, 0);
    for (s, cum) in model.syms.iter_mut().zip([3801, 3000, 10, 0]) {
        s.cumfreq = cum;
    }
    model.shiftsleft = 1;
    model.update(0);
    assert_eq!(model.shiftsleft, 50);
    assert_eq!(cumfreqs(&model), vec![1901, 406, 5, 0]);
    let order: Vec<u16> = model.syms.iter().map(|s| s.sym).collect();
    assert_eq!(order, vec![1, 0, 2, 0]);
    assert_eq!(&model.tabloc[..3], &[1, 0, 2]);
}

#[test]
fn errors_format_clone_and_report_wine_codes() {
    let cases = [
        (Error::DataFormat, 1, "DataFormat"),
        (Error::NotACabinet, 2, "NotACabinet"),
        (Error::UnknownVersion, 3, "UnknownVersion"),
        (Error::CorruptCabinet, 4, "CorruptCabinet"),
    ];
    for (err, code, name) in cases {
        assert_eq!(err.code(), code);
        assert_eq!(format!("{err:?}"), name);
        assert_eq!(Clone::clone(&err), err);
    }
}

#[test]
fn table_errors_format_and_clone() {
    let cases = [
        (TableError::Invalid, "Invalid"),
        (TableError::TableTooSmall, "TableTooSmall"),
    ];
    for (err, name) in cases {
        assert_eq!(format!("{err:?}"), name);
        assert_eq!(Clone::clone(&err), err);
    }
}

#[test]
fn state_types_format_and_clone() {
    let sym = Sym { sym: 1, cumfreq: 2 };
    assert_eq!(format!("{sym:?}"), "Sym { sym: 1, cumfreq: 2 }");
    assert_eq!(Clone::clone(&sym), sym);

    let model = Model::new(2, 0);
    assert!(format!("{model:?}").starts_with("Model { shiftsleft: 4"));
    assert_eq!(model.clone(), model);
    assert_ne!(model, Model::new(3, 0));

    let state = QtmState::init(10).unwrap();
    assert!(format!("{state:?}").starts_with("QtmState { window_size: 1024"));
    assert_eq!(state.clone(), state);

    let reserve = Reserve {
        header: 1,
        folder: 2,
        data: 3,
    };
    assert_eq!(
        format!("{reserve:?}"),
        "Reserve { header: 1, folder: 2, data: 3 }"
    );
    assert_eq!(Clone::clone(&reserve), reserve);

    let link = Link {
        name: b"n",
        info: None,
    };
    assert!(format!("{link:?}").starts_with("Link { name: [110]"));
    assert_eq!(Clone::clone(&link), link);

    let data = cfheader(1, 3, 0, 36, &[]);
    let header = read_header(&data).unwrap();
    assert!(format!("{header:?}").starts_with("CabinetHeader { cab_size: 36"));
    assert_eq!(header.clone(), header);
}
