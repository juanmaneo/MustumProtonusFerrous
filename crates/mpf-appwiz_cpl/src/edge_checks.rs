//! Edge cases the oracle rows do not reach: length limits, non-`REG_SZ` registry values, the
//! retry branch of `install_file` and the order of the search loops.

use crate::*;
use std::cell::Cell;

fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

fn wides(list: &[&str]) -> Vec<Vec<u16>> {
    list.iter().map(|text| wide(text)).collect()
}

#[test]
fn start_params_ignores_null_unknown_and_text_after_nul() {
    assert_eq!(start_params(None), None);
    assert_eq!(start_params(Some(wide("install_gecko_x").as_slice())), None);
    assert_eq!(start_params(Some(wide("INSTALL_MONO").as_slice())), None);
    assert_eq!(start_params(Some(wide("install_mo").as_slice())), None);
    let mut params = wide("install_mono");
    params.extend([0, u16::from(b'x')]);
    assert_eq!(start_params(Some(&params)), Some(Addon::Mono));
}

#[test]
fn append_url_params_fits_up_to_the_buffer_limit() {
    let fixed = "?arch=x86&v=11.3.0&winev=1".len();
    let fits = wide(&"h".repeat(INTERNET_MAX_URL_LENGTH - 1 - fixed));
    let out = append_url_params(&fits, "x86", "11.3.0", Some("1")).unwrap();
    assert_eq!(out.len(), INTERNET_MAX_URL_LENGTH - 1);
    let too_long = wide(&"h".repeat(INTERNET_MAX_URL_LENGTH - fixed));
    assert_eq!(
        append_url_params(&too_long, "x86", "11.3.0", Some("1")),
        None
    );
}

#[test]
fn get_url_uses_registry_size_for_non_reg_sz_values() {
    let short = [0x61u16, 0x62, 0x63];
    assert_eq!(
        get_url(Addon::Gecko, Some((4, &short)), Some("1")),
        Some(wide(Addon::Gecko.url_default()))
    );
    let long = [0x61u16; 5];
    assert_eq!(
        get_url(Addon::Mono, Some((4, &long)), Some("10.0")),
        Some(wide(
            "http://source.winehq.org/winemono.php?arch=x86&v=11.3.0&winev=10.0"
        ))
    );
}

#[test]
fn get_url_checks_http_and_size_for_reg_sz_values() {
    let http = wide("http");
    assert_eq!(
        get_url(Addon::Gecko, Some((REG_SZ, &http)), None),
        Some(wide("http"))
    );
    let mut with_nul = wide("http://a");
    with_nul.extend([0, u16::from(b'z'), u16::from(b'z')]);
    assert_eq!(
        get_url(Addon::Gecko, Some((REG_SZ, &with_nul)), None),
        Some(wide("http://a?arch=x86_64&v=2.47.4&winev="))
    );
    let ftp = wide("ftp://h/x");
    assert_eq!(get_url(Addon::Mono, Some((REG_SZ, &ftp)), None), Some(ftp));
}

#[test]
fn get_url_without_registry_value_appends_to_default() {
    assert_eq!(
        get_url(Addon::Mono, None, Some("10.0")),
        Some(wide(
            "http://source.winehq.org/winemono.php?arch=x86&v=11.3.0&winev=10.0"
        ))
    );
}

#[test]
fn get_url_gives_none_when_the_url_overflows() {
    let mut long = wide("http://");
    long.extend(std::iter::repeat_n(
        u16::from(b'a'),
        INTERNET_MAX_URL_LENGTH - 1 - 7,
    ));
    assert_eq!(
        get_url(Addon::Gecko, Some((REG_SZ, &long)), Some("1")),
        None
    );
}

#[test]
fn digest_matches_is_exact_lowercase_hex() {
    let mut digest = [0u8; 32];
    digest[31] = 0xff;
    let expected = format!("{}ff", "00".repeat(31));
    assert!(digest_matches(&digest, &expected));
    assert!(!digest_matches(&digest, &expected.to_uppercase()));
    assert!(!digest_matches(&digest, "???"));
    digest[0] = 1;
    assert!(!digest_matches(&digest, &expected));
}

#[test]
fn install_file_retries_only_on_product_version() {
    let calls = Cell::new(0);
    let retry_result = Cell::new(0);
    let retry = || {
        calls.set(calls.get() + 1);
        retry_result.get()
    };
    assert_eq!(install_file_result(0, retry), InstallRes::Ok);
    assert_eq!(install_file_result(5, retry), InstallRes::Failed);
    assert_eq!(calls.get(), 0);
    assert_eq!(install_file_result(1638, retry), InstallRes::Ok);
    retry_result.set(1603);
    assert_eq!(install_file_result(1638, retry), InstallRes::Failed);
    assert_eq!(calls.get(), 2);
}

#[test]
fn install_target_needs_more_than_six_units_and_strips_drive_prefix() {
    assert_eq!(install_target(&wide("\\\\?\\C:")), None);
    let drive = wide("\\\\?\\C:\\f");
    assert_eq!(install_target(&drive), Some(&drive[4..]));
    let unix = wide("\\\\?\\unix");
    assert_eq!(install_target(&unix), Some(unix.as_slice()));
}

#[test]
fn build_install_path_joins_with_single_separators() {
    assert_eq!(
        build_install_path(&wide("\\??"), &wide("s"), &wide("f")),
        wide("\\??\\s\\f")
    );
    assert_eq!(
        build_install_path(&wide("x/"), &wide("s"), &wide("f")),
        wide("x/s\\f")
    );
    assert_eq!(
        build_install_path(&wide("x\\"), &wide("s"), &wide("f")),
        wide("x\\s\\f")
    );
    assert_eq!(
        build_install_path(&wide(""), &wide("s"), &wide("f")),
        wide("s\\f")
    );
    assert_eq!(
        build_install_path(&wide("ab\0cd"), &wide("s"), &wide("f")),
        wide("ab\\s\\f")
    );
}

#[test]
fn default_dir_candidates_order() {
    assert_eq!(
        default_dir_candidates(Some(&wide("C:\\b")), Some(&wide("C:\\d")), "/usr/share"),
        wides(&[
            "C:\\b\\..\\",
            "\\\\?\\unix/usr/share/wine/",
            "\\\\?\\unix/opt/wine/"
        ])
    );
    assert_eq!(
        default_dir_candidates(Some(&wide("")), None, "/usr/share"),
        wides(&[
            "\\..\\",
            "\\\\?\\unix/usr/share/wine/",
            "\\\\?\\unix/opt/wine/"
        ])
    );
    assert_eq!(
        default_dir_candidates(None, Some(&wide("D")), "/usr/local/share"),
        wides(&[
            "D",
            "\\\\?\\unix/usr/local/share/wine/",
            "\\\\?\\unix/usr/share/wine/",
            "\\\\?\\unix/opt/wine/",
        ])
    );
    assert_eq!(
        default_dir_candidates(None, None, "/usr/share"),
        wides(&["\\\\?\\unix/usr/share/wine/", "\\\\?\\unix/opt/wine/"])
    );
}

#[test]
fn search_dirs_stops_at_first_result_other_than_next() {
    let mut seen = Vec::new();
    let mut install = |dir: &[u16]| {
        seen.push(dir.to_vec());
        if dir == wide("b").as_slice() {
            InstallRes::Failed
        } else {
            InstallRes::Next
        }
    };
    assert_eq!(
        search_dirs(&wides(&["a", "b", "c"]), &mut install),
        InstallRes::Failed
    );
    assert_eq!(
        search_dirs(&wides(&["a", "c"]), &mut install),
        InstallRes::Next
    );
    assert_eq!(search_dirs(&[], &mut install), InstallRes::Next);
    assert_eq!(seen, wides(&["a", "b", "a", "c"]));
}

#[test]
fn first_settled_stops_at_first_result_other_than_next() {
    let failed_second: [fn() -> InstallRes; 2] = [|| InstallRes::Next, || InstallRes::Failed];
    assert_eq!(first_settled(failed_second), Some(InstallRes::Failed));
    let all_next: [fn() -> InstallRes; 2] = [|| InstallRes::Next, || InstallRes::Next];
    assert_eq!(first_settled(all_next), None);
}

#[test]
fn cache_file_path_prefers_non_empty_xdg_and_needs_a_source() {
    assert_eq!(cache_file_path(None, None, &wide("f")), None);
    assert_eq!(cache_file_path(Some(&wide("")), None, &wide("f")), None);
    assert_eq!(
        cache_file_path(Some(&wide("/x")), Some(&wide("\\\\h")), &wide("f")),
        Some(wide("\\\\?\\unix/x\\wine\\f"))
    );
    assert_eq!(
        cache_file_path(Some(&wide("")), Some(&wide("\\\\h")), &wide("f")),
        Some(wide("\\\\h\\.cache\\wine\\f"))
    );
}
