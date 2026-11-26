//! Boundary and branch tests for the tapi32 port. Each expectation is marked "not oracle-derived"
//! when no oracle row covers it, and its value comes from reading the C source or the headers.

use super::*;

/// `LINEERR_INIFILECORRUPT` from `include/tapi.h`. Not oracle-derived.
const LINEERR_INIFILECORRUPT: u32 = 0x8000_000E;
/// `LINEERR_STRUCTURETOOSMALL` from `include/tapi.h`. Not oracle-derived.
const LINEERR_STRUCTURETOOSMALL: u32 = 0x8000_004D;
/// `TAPIERR_REQUESTFAILED` from `include/tapi.h`. Not oracle-derived.
const TAPIERR_REQUESTFAILED: u32 = 0xFFFF_FFF0;

fn rules(country_code: u32, name: &str) -> CountryRules {
    CountryRules {
        country_code,
        international_rule: b"+1\x00".to_vec(),
        long_distance_rule: b"1\x00".to_vec(),
        country_name: format!("{name}\x00").into_bytes(),
        same_area_rule: b"\x00".to_vec(),
    }
}

/// A `LINECOUNTRYLIST` buffer of `len` bytes whose `dwTotalSize` is `total`, filled with 0xAA.
fn list_of(total: u32, len: usize) -> Vec<u8> {
    let mut buf = vec![0xaa; len];
    buf[..4].copy_from_slice(&total.to_le_bytes());
    buf
}

/// The six header DWORDs: total, needed, used, number of countries, list size and list offset.
fn header(buf: &[u8]) -> [u32; 6] {
    std::array::from_fn(|i| u32::from_le_bytes(buf[4 * i..4 * i + 4].try_into().unwrap()))
}

fn wide_units(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

#[test]
fn country_id_of_parses_like_wcstol() {
    // Not oracle-derived: every oracle subkey name is a plain decimal string.
    assert_eq!(country_id_of("44"), 44);
    assert_eq!(country_id_of(" \t\r\n44"), 44);
    assert_eq!(country_id_of("+7"), 7);
    assert_eq!(country_id_of("-5"), 0xFFFF_FFFB);
    assert_eq!(country_id_of("33x"), 33);
    assert_eq!(country_id_of("abc"), 0);
    assert_eq!(country_id_of(""), 0);
    assert_eq!(country_id_of("-"), 0);
    // `long` is 32 bits on Windows, so the value clamps to the `long` range.
    assert_eq!(country_id_of("99999999999"), i32::MAX as u32);
    assert_eq!(country_id_of("-99999999999"), i32::MIN as u32);
}

#[test]
fn line_get_country_without_list_key_fails_and_writes_nothing() {
    // Not oracle-derived: the country list key does not open.
    let mut buf = list_of(24, 24);
    let before = buf.clone();
    assert_eq!(line_get_country(&mut buf, 0, None), LINEERR_INIFILECORRUPT);
    assert_eq!(buf, before);
}

#[test]
fn line_get_country_buffer_without_a_header_is_too_small() {
    // Not oracle-derived: a buffer shorter than the header reads as a total of 0.
    let no_keys: &[CountryKey] = &[];
    assert_eq!(
        line_get_country(&mut [], 0, Some(no_keys)),
        LINEERR_STRUCTURETOOSMALL
    );
    assert_eq!(
        line_get_country(&mut [0; 3], 0, Some(no_keys)),
        LINEERR_STRUCTURETOOSMALL
    );
}

#[test]
fn line_get_country_reserves_entry_slots_for_unopened_keys() {
    // Not oracle-derived: the needed size counts an entry slot for each enumerated key, even when
    // the key does not open. The 24-byte buffer is smaller than the reported need, so nothing is written.
    let keys = [
        CountryKey {
            name: "1",
            rules: None,
        },
        CountryKey {
            name: "2",
            rules: None,
        },
    ];
    let mut buf = list_of(24, 24);
    assert_eq!(line_get_country(&mut buf, 0, Some(&keys)), 0);
    assert_eq!(header(&buf), [24, 24 + 2 * 44, 24, 0, 0, 24]);
}

#[test]
fn line_get_country_by_id_skips_other_and_unopened_keys() {
    // Not oracle-derived: key "1" is not the requested ID, and key "2" does not open.
    let keys = [
        CountryKey {
            name: "1",
            rules: Some(rules(1, "United States")),
        },
        CountryKey {
            name: "2",
            rules: None,
        },
    ];
    let mut buf = list_of(512, 512);
    assert_eq!(line_get_country(&mut buf, 2, Some(&keys)), 0);
    assert_eq!(header(&buf), [512, 44, 24, 0, 0, 24]);
}

#[test]
fn line_get_country_by_id_reports_needed_size_when_entry_does_not_fit() {
    // Not oracle-derived: the entry needs 20 bytes of rule data after its 44-byte header, so the
    // needed size is 64 and no entry is written into the 30-byte buffer.
    let keys = [CountryKey {
        name: "1",
        rules: Some(rules(1, "United States")),
    }];
    let mut buf = list_of(30, 30);
    assert_eq!(line_get_country(&mut buf, 1, Some(&keys)), 0);
    assert_eq!(header(&buf), [30, 64, 24, 0, 0, 24]);
    assert_eq!(&buf[24..], &[0; 6]);
}

#[test]
fn line_get_country_drops_writes_past_the_buffer() {
    // Not oracle-derived: the header says 512 bytes but the buffer holds 30. The C body writes past
    // the end (undefined behaviour). The port keeps the bytes that fit, which are the same bytes as
    // the full-size result.
    let keys = [CountryKey {
        name: "1",
        rules: Some(rules(1, "United States")),
    }];
    let mut full = list_of(512, 512);
    assert_eq!(line_get_country(&mut full, 0, Some(&keys)), 0);
    let mut short = list_of(512, 30);
    assert_eq!(line_get_country(&mut short, 0, Some(&keys)), 0);
    assert_eq!(short.as_slice(), &full[..30]);
}

#[test]
fn export_kind_is_none_for_names_outside_the_spec() {
    // Not oracle-derived: names that tapi32.spec does not export, including a wrong-case name.
    assert_eq!(export_kind("lineNotAnExport"), None);
    assert_eq!(export_kind("lineaccept"), None);
}

#[test]
fn location_w_without_want_flags_returns_no_strings() {
    // Not oracle-derived: both out-parameters are NULL, so the lookup succeeds with no strings.
    let reg = LocationRegistry {
        key_opens: true,
        current_id: Some(RegData::Dword(2)),
        subkey_opens: true,
        area_code: Some(RegData::Sz(vec![50, 49, 50, 0])),
        country: Some(RegData::Dword(1)),
    };
    assert_eq!(
        tapi_get_location_info_w(&reg, false, false),
        Ok(LocationInfo {
            country: None,
            city: None
        })
    );
}

#[test]
fn location_a_maps_non_ascii_units_to_question_mark() {
    // Not oracle-derived: the oracle's WideCharToMultiByte shim copies the low byte, so no row
    // holds a non-ASCII unit. The `?` default is the CP_ACP default for an unmappable character,
    // and code page tables are not modelled.
    let reg = LocationRegistry {
        key_opens: true,
        current_id: Some(RegData::Dword(2)),
        subkey_opens: true,
        area_code: Some(RegData::Sz(vec![0xe9, 0x31, 0])),
        country: Some(RegData::Dword(1)),
    };
    assert_eq!(
        tapi_get_location_info_a(&reg, true, true),
        Ok(AnsiLocationInfo {
            country: Some(b"1".to_vec()),
            city: Some(b"?1".to_vec())
        })
    );
}

#[test]
fn location_area_code_of_exactly_the_buffer_size_is_used() {
    // Not oracle-derived: 99 characters and a NUL are 200 bytes, the largest value that fits the
    // buffer. Only the first 7 characters are copied.
    let mut area = vec![0x31; 99];
    area.push(0);
    let reg = LocationRegistry {
        key_opens: true,
        current_id: Some(RegData::Dword(2)),
        subkey_opens: true,
        area_code: Some(RegData::Sz(area)),
        country: None,
    };
    assert_eq!(
        tapi_get_location_info_w(&reg, false, true),
        Ok(LocationInfo {
            country: None,
            city: Some(wide_units("1111111"))
        })
    );
}

#[test]
fn location_lookup_failures_return_the_request_failed_code() {
    // Not oracle-derived: the error code is the same for every failing step, and the A variant
    // passes it through unchanged.
    let reg = LocationRegistry {
        key_opens: false,
        current_id: Some(RegData::Dword(2)),
        subkey_opens: true,
        area_code: None,
        country: None,
    };
    assert_eq!(
        tapi_get_location_info_a(&reg, true, true),
        Err(TAPIERR_REQUESTFAILED)
    );
}
