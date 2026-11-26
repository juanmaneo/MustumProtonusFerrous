//! Edge-case tests for the ported dmloader logic.
//!
//! These cover boundary values and every branch. Expectations that the oracle did not print are
//! marked "Not oracle-derived"; they follow from reading the Wine source or the header constants.

use super::*;

/// `GUID_DirectMusicAllTypes`, the table entry at index 0.
const ALL_TYPES: Guid = CLASSES[0];

/// `CLSID_DirectMusicSegmentState`: not in `classes`.
const SEGMENT_STATE: Guid = Guid {
    data1: 0xd2ac2883,
    data2: 0xb39b,
    data3: 0x11d1,
    data4: [0x87, 0x04, 0x00, 0x60, 0x08, 0x93, 0xb1, 0xbd],
};

fn units(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

fn paths_with(entries: &[(usize, Vec<u16>)]) -> [Option<Vec<u16>>; CLASS_COUNT] {
    let mut out: [Option<Vec<u16>>; CLASS_COUNT] = Default::default();
    for (index, path) in entries {
        out[*index] = Some(path.clone());
    }
    out
}

#[test]
fn is_valid_dmform_accepts_every_form() {
    // Not oracle-derived: only DMAP and DLS were printed. The other nine IDs come from
    // dmusicf.h, dls1.h and the `WAVE` literal in debug.c.
    let forms = [
        b"DMAP", b"DMBD", b"DMPR", b"DMCN", b"DLS ", b"DMSC", b"DMSG", b"DMST", b"DMTG", b"DMTK",
        b"WAVE",
    ];
    for form in forms {
        assert!(is_valid_dmform(u32::from_le_bytes(*form)));
    }
}

#[test]
fn is_valid_dmform_rejects_near_misses() {
    // Not oracle-derived: byte-reversed, one-letter-off and truncated IDs, and zero.
    assert!(!is_valid_dmform(0));
    assert!(!is_valid_dmform(u32::from_be_bytes(*b"DMAP")));
    assert!(!is_valid_dmform(u32::from_le_bytes(*b"DMAQ")));
    assert!(!is_valid_dmform(u32::from_le_bytes(*b"DLS\0")));
    assert!(!is_valid_dmform(u32::from_le_bytes(*b"wave")));
}

#[test]
fn resolve_stream_seek_cur_and_unknowns() {
    // Not oracle-derived: STREAM_SEEK_CUR was not printed, and the extremes are not in the rows.
    assert_eq!(resolve_stream_seek(1), "STREAM_SEEK_CUR");
    assert_eq!(resolve_stream_seek(4), "()");
    assert_eq!(resolve_stream_seek(u32::MAX), "()");
}

#[test]
fn index_from_class_every_entry_and_near_misses() {
    // Not oracle-derived: the other table entries were not printed. Their GUID values were
    // copied from Wine's dmusici.h and dmusicc.h.
    for (index, class) in CLASSES.iter().enumerate() {
        assert_eq!(index_from_class(class), Some(index));
    }
    let mut last_byte = CLASSES[2];
    last_byte.data4[7] ^= 1;
    assert_eq!(index_from_class(&last_byte), None);
    let mut first_word = CLASSES[2];
    first_word.data1 ^= 1;
    assert_eq!(index_from_class(&first_word), None);
    let zero = Guid {
        data1: 0,
        data2: 0,
        data3: 0,
        data4: [0; 8],
    };
    assert_eq!(index_from_class(&zero), None);
}

#[test]
fn is_cache_enabled_each_class_bit() {
    // Not oracle-derived except the Band row: each class tests only its own bit.
    for (index, _) in CLASSES.iter().enumerate() {
        let cache = 1u32 << index;
        for (other, class) in CLASSES.iter().enumerate() {
            assert_eq!(is_cache_enabled(cache, class), other == index);
        }
    }
    assert!(!is_cache_enabled(0, &CLASSES[11]));
    assert!(is_cache_enabled(0x800, &CLASSES[11]));
    assert!(!is_cache_enabled(0x7ff, &CLASSES[11]));
}

#[test]
fn is_cache_enabled_unknown_class_tests_bit_31() {
    // Not oracle-derived: the unknown-class rows are flagged by UBSan. The expectations follow
    // the x86 `shl` that masks the count of `1 << -1` to 31.
    assert!(is_cache_enabled(0x8000_0000, &SEGMENT_STATE));
    assert!(is_cache_enabled(u32::MAX, &SEGMENT_STATE));
    assert!(!is_cache_enabled(0x7fff_ffff, &SEGMENT_STATE));
}

#[test]
fn enable_cache_single_class_bit() {
    // Not oracle-derived except the Band rows. Index 0 is the AllTypes special case, so the
    // single-bit path is checked for indices 1 to 11.
    for (index, class) in CLASSES.iter().enumerate().skip(1) {
        let bit = 1u32 << index;
        assert_eq!(
            enable_cache(0, class, true),
            CacheToggle {
                cache_class: bit,
                clear_cache: false,
                hr: S_OK,
            }
        );
        assert_eq!(
            enable_cache(bit, class, true),
            CacheToggle {
                cache_class: bit,
                clear_cache: false,
                hr: S_FALSE,
            }
        );
        assert_eq!(
            enable_cache(u32::MAX, class, false),
            CacheToggle {
                cache_class: !bit,
                clear_cache: true,
                hr: S_OK,
            }
        );
        assert_eq!(
            enable_cache(0, class, false),
            CacheToggle {
                cache_class: 0,
                clear_cache: true,
                hr: S_FALSE,
            }
        );
    }
}

#[test]
fn enable_cache_all_types_sets_and_clears_every_bit() {
    // Not oracle-derived except the two disable rows from the oracle.
    assert_eq!(
        enable_cache(0, &ALL_TYPES, true),
        CacheToggle {
            cache_class: u32::MAX,
            clear_cache: false,
            hr: S_OK,
        }
    );
    assert_eq!(
        enable_cache(u32::MAX, &ALL_TYPES, true),
        CacheToggle {
            cache_class: u32::MAX,
            clear_cache: false,
            hr: S_FALSE,
        }
    );
    assert_eq!(
        enable_cache(0x1, &ALL_TYPES, false),
        CacheToggle {
            cache_class: 0,
            clear_cache: true,
            hr: S_OK,
        }
    );
    assert_eq!(
        enable_cache(0x1, &ALL_TYPES, true),
        CacheToggle {
            cache_class: u32::MAX,
            clear_cache: false,
            hr: S_FALSE,
        }
    );
}

#[test]
fn enable_cache_unknown_class_is_a_noop() {
    // Not oracle-derived: the unknown-class rows are flagged by UBSan. The expectations come from
    // the early return in loader_EnableCache, which changes nothing and never clears.
    for enable in [true, false] {
        assert_eq!(
            enable_cache(0x1234_5678, &SEGMENT_STATE, enable),
            CacheToggle {
                cache_class: 0x1234_5678,
                clear_cache: false,
                hr: S_FALSE,
            }
        );
    }
}

#[test]
fn get_search_path_truncates_at_max_path_minus_one() {
    // Not oracle-derived: lstrcpynW(path, src, MAX_PATH) copies at most 259 units. Only the
    // 300-unit case was printed.
    for len in [0usize, 258, 259, 260, 1000] {
        let path = vec![u16::from(b'x'); len];
        let out = get_search_path(&ALL_TYPES, &paths_with(&[(0, path)]));
        assert_eq!(out.len(), len.min(259), "input length {len}");
        assert!(out.iter().all(|&unit| unit == u16::from(b'x')));
    }
}

#[test]
fn get_search_path_stops_at_first_nul() {
    // Not oracle-derived: a NUL before the end ends the copy. A leading NUL gives an empty path.
    let out = get_search_path(&ALL_TYPES, &paths_with(&[(0, vec![0, 0x61])]));
    assert!(out.is_empty());
    let out = get_search_path(&ALL_TYPES, &paths_with(&[(0, vec![0x61, 0, 0x62])]));
    assert_eq!(out, units("a"));
}

#[test]
fn get_search_path_falls_back_to_all_types() {
    // Not oracle-derived except the Band fallback row. Covers the unknown class, a known class
    // with no path, a known class with a path, and a set but empty path.
    let all = paths_with(&[(0, units("all"))]);
    assert_eq!(get_search_path(&SEGMENT_STATE, &all), units("all"));
    assert_eq!(get_search_path(&CLASSES[2], &all), units("all"));

    let both = paths_with(&[(0, units("all")), (2, units("band"))]);
    assert_eq!(get_search_path(&CLASSES[2], &both), units("band"));

    let none = paths_with(&[]);
    assert!(get_search_path(&CLASSES[2], &none).is_empty());
    assert!(get_search_path(&SEGMENT_STATE, &none).is_empty());

    let empty_band = paths_with(&[(0, units("all")), (2, Vec::new())]);
    assert!(get_search_path(&CLASSES[2], &empty_band).is_empty());
}
