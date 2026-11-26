//! Edge-case tests for the ported gameux logic.
//!
//! These cover index and length boundaries, every branch of the registry scope
//! and SID handling, and the string cut rules. Expectations that are not
//! oracle-derived say so in a comment. Those come from reading the Wine C source.

use super::*;
use std::cell::Cell;

fn w(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

fn cyc(n: usize) -> Vec<u16> {
    (0..n).map(|i| u16::from(b'a' + (i % 26) as u8)).collect()
}

const GX: Guid = Guid {
    data1: 0x1234_5678,
    data2: 0x9abc,
    data3: 0xdef0,
    data4: [0x01, 0x02, 0x03, 0x04, 0xab, 0xcd, 0xef, 0x10],
};
const KEY_PREFIX: &str = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\GameUX\\";

#[test]
fn registry_scope_boundaries() {
    // Not oracle-derived: scopes 1 and 4 are outside the two valid values, and
    // the SID closure must not run for them. Oracle rows 07 and 08 cover scopes 1 and 0.
    let calls = Cell::new(0);
    assert_eq!(
        build_game_registry_path(4, &mut sid_lookup(&calls), Some(&GX)),
        Err(E_INVALIDARG)
    );
    assert_eq!(
        build_game_registry_path(-1, &mut sid_lookup(&calls), None),
        Err(E_INVALIDARG)
    );
    assert_eq!(
        build_game_registry_path(GIS_ALL_USERS, &mut sid_lookup(&calls), None),
        Ok(w(&format!("{KEY_PREFIX}Games")))
    );
    assert_eq!(calls.get(), 0);
}

#[test]
fn registry_current_user_looks_up_sid_once() {
    // Not oracle-derived: the lookup runs once for GIS_CURRENT_USER and its result is used.
    let calls = Cell::new(0);
    assert_eq!(
        build_game_registry_path(GIS_CURRENT_USER, &mut sid_lookup(&calls), None),
        Ok(w(&format!("{KEY_PREFIX}S-1-5-21-111-222-333-1001")))
    );
    assert_eq!(calls.get(), 1);
}

#[test]
fn registry_sid_error_is_returned_unchanged() {
    // Not oracle-derived: the value is arbitrary. Oracle rows 04 to 06 cover three other errors.
    assert_eq!(
        build_game_registry_path(GIS_CURRENT_USER, &mut || Err(0x1234), Some(&GX)),
        Err(0x1234)
    );
}

#[test]
fn registry_sid_is_cut_at_nul() {
    // Not oracle-derived: lstrcatW copies the SID only up to its NUL.
    assert_eq!(
        build_game_registry_path(GIS_CURRENT_USER, &mut || Ok(vec![0x53, 0, 0x58]), None),
        Ok(w(&format!("{KEY_PREFIX}S")))
    );
}

#[test]
fn registry_empty_sid_leaves_key_prefix() {
    // Not oracle-derived: an empty SID appends nothing.
    assert_eq!(
        build_game_registry_path(GIS_CURRENT_USER, &mut || Ok(Vec::new()), None),
        Ok(w(KEY_PREFIX))
    );
}

#[test]
fn registry_guid_formatting_edges() {
    // Not oracle-derived: derived from the StringFromGUID2 format string
    // (uppercase hex, 8-4-4-4-12 digits, braces). No oracle row uses these values.
    let calls = Cell::new(0);
    let zero = Guid {
        data1: 0,
        data2: 0,
        data3: 0,
        data4: [0; 8],
    };
    let max = Guid {
        data1: u32::MAX,
        data2: u16::MAX,
        data3: u16::MAX,
        data4: [0xff; 8],
    };
    let mixed = Guid {
        data1: 0xab,
        data2: 0x0c,
        data3: 0xd00e,
        data4: [0x0f, 0x10, 0xa0, 0xb0, 0xc0, 0xd0, 0xe0, 0xf0],
    };
    assert_eq!(
        build_game_registry_path(GIS_ALL_USERS, &mut sid_lookup(&calls), Some(&zero)),
        Ok(w(&format!(
            "{KEY_PREFIX}Games\\{{00000000-0000-0000-0000-000000000000}}"
        )))
    );
    assert_eq!(
        build_game_registry_path(GIS_ALL_USERS, &mut sid_lookup(&calls), Some(&max)),
        Ok(w(&format!(
            "{KEY_PREFIX}Games\\{{FFFFFFFF-FFFF-FFFF-FFFF-FFFFFFFFFFFF}}"
        )))
    );
    assert_eq!(
        build_game_registry_path(GIS_ALL_USERS, &mut sid_lookup(&calls), Some(&mixed)),
        Ok(w(&format!(
            "{KEY_PREFIX}Games\\{{000000AB-000C-D00E-0F10-A0B0C0D0E0F0}}"
        )))
    );
    assert_eq!(calls.get(), 0);
}

#[test]
fn statistics_path_with_empty_parts() {
    // Not oracle-derived: empty folder and app id leave the fixed separators in place.
    assert_eq!(
        build_statistics_file_path(&[], &[]),
        w("\\Microsoft\\Windows\\GameExplorer\\GameStatistics\\\\.gamestats")
    );
}

#[test]
fn statistics_path_folder_is_cut_at_nul() {
    // Not oracle-derived: lstrcatW copies the folder only up to its NUL.
    assert_eq!(
        build_statistics_file_path(&[0x43, 0, 0x58], &w("1")),
        w("C\\Microsoft\\Windows\\GameExplorer\\GameStatistics\\1\\1.gamestats")
    );
}

#[test]
fn stats_directory_edges() {
    // Not oracle-derived: the no-backslash input is outside the C contract, so the
    // port returns None. These cases check the split around each backslash.
    assert_eq!(stats_directory(&[]), None);
    assert_eq!(stats_directory(&w("c.gamestats")), None);
    assert_eq!(
        stats_directory(&w("\\x")).map(<[u16]>::to_vec),
        Some(Vec::new())
    );
    assert_eq!(
        stats_directory(&w("C:\\a\\b\\c")).map(<[u16]>::to_vec),
        Some(w("C:\\a\\b"))
    );
    // The search stops at the NUL, so the last backslash is the one at index 2.
    assert_eq!(
        stats_directory(&[0x43, 0x3a, 0x5c, 0x61, 0, 0x5c, 0x62]).map(<[u16]>::to_vec),
        Some(w("C:"))
    );
}

#[test]
fn category_index_and_title_boundaries() {
    let mut stats = GameStatistics::default();

    // Not oracle-derived: index 9 is the last valid category, and exactly 60 units is not cut.
    assert_eq!(
        stats.set_category_title(9, Some(&cyc(MAX_CATEGORY_LENGTH))),
        Ok(S_OK)
    );
    assert_eq!(
        stats.get_category_title(9),
        Ok(Some(cyc(MAX_CATEGORY_LENGTH)))
    );

    // Not oracle-derived: an empty title is stored as empty and read back as None.
    assert_eq!(stats.set_category_title(0, Some(&[])), Ok(S_OK));
    assert_eq!(stats.get_category_title(0), Ok(None));

    // Not oracle-derived: a NUL cuts the title before the length check, so 61 units with
    // a NUL at index 5 is S_OK.
    let mut long_with_nul = cyc(61);
    long_with_nul[5] = 0;
    assert_eq!(stats.set_category_title(1, Some(&long_with_nul)), Ok(S_OK));
    assert_eq!(stats.get_category_title(1), Ok(Some(cyc(5))));

    // Not oracle-derived: a shorter title replaces a longer one completely.
    assert_eq!(stats.set_category_title(1, Some(&w("ab"))), Ok(S_OK));
    assert_eq!(stats.get_category_title(1), Ok(Some(w("ab"))));

    // Not oracle-derived: index 10 is out of range in get as well as set.
    assert_eq!(stats.get_category_title(10), Err(E_INVALIDARG));

    // Not oracle-derived: a default store has no titles.
    assert_eq!(GameStatistics::default().get_category_title(0), Ok(None));
}

#[test]
fn statistic_index_and_length_boundaries() {
    let mut stats = GameStatistics::default();
    let name30 = cyc(MAX_NAME_LENGTH);
    let value30 = cyc(MAX_VALUE_LENGTH);

    // Not oracle-derived: the largest valid indexes and exact-length strings are not cut.
    assert_eq!(
        stats.set_statistic(9, 9, Some(&name30), Some(&value30)),
        Ok(S_OK)
    );
    assert_eq!(stats.get_statistic(9, 9), Ok((Some(name30), Some(value30))));

    // Not oracle-derived: an out-of-range category or statistic index is E_INVALIDARG.
    assert_eq!(
        stats.set_statistic(10, 0, Some(&w("n")), None),
        Err(E_INVALIDARG)
    );
    assert_eq!(
        stats.set_statistic(0, 10, Some(&w("n")), None),
        Err(E_INVALIDARG)
    );
    assert_eq!(stats.get_statistic(0, 10), Err(E_INVALIDARG));
    assert_eq!(stats.get_statistic(10, 0), Err(E_INVALIDARG));

    // Not oracle-derived: a NULL name returns S_FALSE before the index check, so an
    // invalid index is not an error and nothing changes.
    assert_eq!(stats.set_statistic(10, 10, None, None), Ok(S_FALSE));

    // Not oracle-derived: a NULL value clears a stored value.
    assert_eq!(
        stats.set_statistic(0, 0, Some(&w("n")), Some(&w("v"))),
        Ok(S_OK)
    );
    assert_eq!(stats.set_statistic(0, 0, Some(&w("n")), None), Ok(S_OK));
    assert_eq!(stats.get_statistic(0, 0), Ok((Some(w("n")), None)));

    // Not oracle-derived: an empty value is stored as empty and read back as None.
    assert_eq!(
        stats.set_statistic(0, 1, Some(&w("n")), Some(&[])),
        Ok(S_OK)
    );
    assert_eq!(stats.get_statistic(0, 1), Ok((Some(w("n")), None)));

    // Not oracle-derived: an empty stored name reads back as None.
    assert_eq!(stats.set_statistic(0, 2, Some(&[]), None), Ok(S_OK));
    assert_eq!(stats.get_statistic(0, 2), Ok((None, None)));

    // Not oracle-derived: a 31-unit name is cut to 30 and returns S_FALSE, while
    // a 31-unit value cut to 30 does the same.
    assert_eq!(
        stats.set_statistic(0, 3, Some(&cyc(31)), Some(&cyc(31))),
        Ok(S_FALSE)
    );
    assert_eq!(
        stats.get_statistic(0, 3),
        Ok((Some(cyc(30)), Some(cyc(30))))
    );

    // Not oracle-derived: a 30-unit name with a 31-unit value is cut and returns S_FALSE,
    // while a 31-unit name with a 30-unit value also does.
    assert_eq!(
        stats.set_statistic(0, 4, Some(&cyc(30)), Some(&cyc(31))),
        Ok(S_FALSE)
    );
    assert_eq!(
        stats.set_statistic(0, 5, Some(&cyc(31)), Some(&cyc(30))),
        Ok(S_FALSE)
    );
}

#[test]
fn constants_match_wine() {
    // Not oracle-derived: the limits are the MAX_* defines at the top of gamestatistics.c.
    assert_eq!(MAX_CATEGORY_LENGTH, 60);
    assert_eq!(MAX_NAME_LENGTH, 30);
    assert_eq!(MAX_VALUE_LENGTH, 30);
    assert_eq!(MAX_CATEGORIES, 10);
    assert_eq!(MAX_STATS_PER_CATEGORY, 10);
    assert_eq!(GIS_CURRENT_USER, 2);
    assert_eq!(GIS_ALL_USERS, 3);
}
