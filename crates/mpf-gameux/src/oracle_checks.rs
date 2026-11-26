//! Oracle-derived test vectors for the ported gameux logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies from
//! `dlls/gameux/gameexplorer.c` and `dlls/gameux/gamestatistics.c`, plus the
//! `StringFromGUID2`, `lstrcpynW`, `lstrcatW`, and `StrRChrW` bodies, with
//! `gcc -fsanitize=address,undefined -g -fshort-wchar` under
//! `target/canon/gameux/`. The harness printed 24 rows, and ASan and UBSan
//! flagged none of them, so 0 vectors were excluded.
//!
//! Inputs are written out as literals. The 60-, 61- and 31-unit strings come from
//! the harness's `cyc_wide(n)`, which fills unit `i` with `'a' + i % 26`.
//! The Rust `cyc` helper below builds the same strings.

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
const SID: &str = "S-1-5-21-111-222-333-1001";
/// `HRESULT_FROM_WIN32(ERROR_ACCESS_DENIED)`.
const ERR_ACCESS_DENIED: Hresult = 0x8007_0005_u32 as i32;

#[test]
fn oracle_registry_rows() {
    let calls = Cell::new(0);
    // row 01 registry all_users guid
    assert_eq!(
        build_game_registry_path(GIS_ALL_USERS, &mut sid_lookup(&calls), Some(&GX)),
        Ok(w(
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\GameUX\\Games\\{12345678-9ABC-DEF0-0102-0304ABCDEF10}"
        ))
    );
    // row 02 registry current_user sid guid
    assert_eq!(
        build_game_registry_path(GIS_CURRENT_USER, &mut || Ok(w(SID)), Some(&GX)),
        Ok(w(
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\GameUX\\S-1-5-21-111-222-333-1001\\{12345678-9ABC-DEF0-0102-0304ABCDEF10}"
        ))
    );
    // row 03 registry current_user sid noguid
    assert_eq!(
        build_game_registry_path(GIS_CURRENT_USER, &mut || Ok(w(SID)), None),
        Ok(w(
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\GameUX\\S-1-5-21-111-222-333-1001"
        ))
    );
    // row 04 registry current_user open_token_fails: the SID lookup fails with ERROR_ACCESS_DENIED
    assert_eq!(
        build_game_registry_path(GIS_CURRENT_USER, &mut || Err(ERR_ACCESS_DENIED), None),
        Err(0x8007_0005_u32 as i32)
    );
    // row 05 registry current_user token_info_fails
    assert_eq!(
        build_game_registry_path(GIS_CURRENT_USER, &mut || Err(ERR_ACCESS_DENIED), Some(&GX)),
        Err(0x8007_0005_u32 as i32)
    );
    // row 06 registry current_user convert_fails: ERROR_INVALID_PARAMETER maps to E_INVALIDARG
    assert_eq!(
        build_game_registry_path(GIS_CURRENT_USER, &mut || Err(E_INVALIDARG), Some(&GX)),
        Err(E_INVALIDARG)
    );
    // row 07 registry not_installed guid
    assert_eq!(
        build_game_registry_path(1, &mut sid_lookup(&calls), Some(&GX)),
        Err(E_INVALIDARG)
    );
    // row 08 registry scope0 noguid
    assert_eq!(
        build_game_registry_path(0, &mut sid_lookup(&calls), None),
        Err(E_INVALIDARG)
    );
    assert_eq!(calls.get(), 0);
}

#[test]
fn oracle_statistics_path_rows() {
    // row 09 path normal
    assert_eq!(
        build_statistics_file_path(
            &w("C:\\Users\\neo\\AppData\\Local"),
            &w("1234-abcd")
        ),
        w("C:\\Users\\neo\\AppData\\Local\\Microsoft\\Windows\\GameExplorer\\GameStatistics\\1234-abcd\\1234-abcd.gamestats")
    );
    // row 10 path app_id_with_nul: the app id ends at its NUL, as lstrcatW does
    assert_eq!(
        build_statistics_file_path(
            &w("C:\\Users\\neo\\AppData\\Local"),
            &[0x41, 0x42, 0, 0x43, 0x44, 0]
        ),
        w("C:\\Users\\neo\\AppData\\Local\\Microsoft\\Windows\\GameExplorer\\GameStatistics\\AB\\AB.gamestats")
    );
}

#[test]
fn oracle_stats_directory_rows() {
    // row 11 dir file
    assert_eq!(
        stats_directory(&w("C:\\a\\b\\c.gamestats")).map(<[u16]>::to_vec),
        Some(w("C:\\a\\b"))
    );
    // row 12 dir trailing_backslash
    assert_eq!(
        stats_directory(&w("C:\\a\\")).map(<[u16]>::to_vec),
        Some(w("C:\\a"))
    );
}

#[test]
fn oracle_game_statistics_rows() {
    let mut stats = GameStatistics::default();
    let stored60 = w("abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefgh");

    // row 13 set_title cat=0 Kills
    assert_eq!(stats.set_category_title(0, Some(&w("Kills"))), Ok(S_OK));
    assert_eq!(stats.get_category_title(0), Ok(Some(w("Kills"))));

    // row 14 set_title cat=9 len=60
    assert_eq!(stats.set_category_title(9, Some(&cyc(60))), Ok(S_OK));
    assert_eq!(stats.get_category_title(9), Ok(Some(stored60.clone())));

    // row 15 set_title cat=1 len=61: cut to 60, S_FALSE
    assert_eq!(stats.set_category_title(1, Some(&cyc(61))), Ok(S_FALSE));
    assert_eq!(stats.get_category_title(1), Ok(Some(stored60)));

    // row 16 set_title cat=10
    assert_eq!(
        stats.set_category_title(10, Some(&w("Kills"))),
        Err(E_INVALIDARG)
    );

    // row 17 set_title cat=2 title=NULL
    assert_eq!(stats.set_category_title(2, None), Err(E_INVALIDARG));

    // row 18 set_title cat=4 embedded_nul: stops at the NUL
    assert_eq!(
        stats.set_category_title(4, Some(&[0x61, 0, 0x62, 0])),
        Ok(S_OK)
    );
    assert_eq!(stats.get_category_title(4), Ok(Some(w("a"))));

    // row 19 get_title cat=3 never_set
    assert_eq!(stats.get_category_title(3), Ok(None));

    // row 20 set_stat (0,0) Deaths=12
    assert_eq!(
        stats.set_statistic(0, 0, Some(&w("Deaths")), Some(&w("12"))),
        Ok(S_OK)
    );
    assert_eq!(
        stats.get_statistic(0, 0),
        Ok((Some(w("Deaths")), Some(w("12"))))
    );

    // row 21 set_stat (0,1) name=31 value=NULL: name cut to 30, value cleared
    assert_eq!(stats.set_statistic(0, 1, Some(&cyc(31)), None), Ok(S_FALSE));
    assert_eq!(
        stats.get_statistic(0, 1),
        Ok((Some(w("abcdefghijklmnopqrstuvwxyzabcd")), None))
    );

    // row 22 set_stat (0,2) value=31: value cut to 30, S_FALSE
    assert_eq!(
        stats.set_statistic(0, 2, Some(&w("Deaths")), Some(&cyc(31))),
        Ok(S_FALSE)
    );
    assert_eq!(
        stats.get_statistic(0, 2),
        Ok((Some(w("Deaths")), Some(w("abcdefghijklmnopqrstuvwxyzabcd"))))
    );

    // row 23 set_stat (0,0) name=NULL: S_FALSE, nothing changes
    assert_eq!(
        stats.set_statistic(0, 0, None, Some(&w("hits"))),
        Ok(S_FALSE)
    );
    assert_eq!(
        stats.get_statistic(0, 0),
        Ok((Some(w("Deaths")), Some(w("12"))))
    );

    // row 24 set_stat (10,0)
    assert_eq!(
        stats.set_statistic(10, 0, Some(&w("Kills")), Some(&w("hits"))),
        Err(E_INVALIDARG)
    );
}
