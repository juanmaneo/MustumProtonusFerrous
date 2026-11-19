use crate::{
    error_code_text, facility_code_text, ObjectPath, WBEMPATH_GET_ORIGINAL,
    WBEMPATH_GET_RELATIVE_ONLY, WBEM_E_INVALID_OBJECT_PATH, WBEM_E_INVALID_PARAMETER,
    WBEM_E_NOT_AVAILABLE,
};

fn utf16(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

fn text(units: &[u16]) -> String {
    let end = units.iter().position(|&c| c == 0).unwrap_or(units.len());
    String::from_utf16_lossy(&units[..end])
}

fn parsed(s: &str) -> ObjectPath {
    let mut path = ObjectPath::default();
    path.set_text(1, Some(&utf16(s))).unwrap();
    path
}

fn get_text(path: &ObjectPath, flags: i32) -> (Result<usize, i32>, String) {
    let mut buf = [0u16; 64];
    let ret = path.get_text(flags, buf.len(), Some(&mut buf[..]));
    (ret, text(&buf))
}

fn assert_rejected(input: &str) {
    let mut path = parsed("Win32_X.A=1");
    assert_eq!(
        path.set_text(1, Some(&utf16(input))),
        Err(WBEM_E_INVALID_PARAMETER),
        "input [{input}]"
    );
    assert_eq!(path, ObjectPath::default(), "input [{input}]");
}

#[test]
fn mode_zero_and_missing_text_are_rejected_without_clearing() {
    let mut path = parsed("Win32_X.A=1");
    let before = path.clone();
    assert_eq!(
        path.set_text(0, Some(&utf16("Win32_Y"))),
        Err(WBEM_E_INVALID_PARAMETER)
    );
    assert_eq!(path, before);
    assert_eq!(path.set_text(1, None), Err(WBEM_E_INVALID_PARAMETER));
    assert_eq!(path, before);
}

#[test]
fn text_is_cut_at_the_first_nul() {
    let mut path = parsed("Win32_X.A=1");
    path.set_text(1, Some(&utf16("Cls\0x"))).unwrap();
    assert_eq!(
        get_text(&path, WBEMPATH_GET_ORIGINAL),
        (Ok(4), "Cls".to_string())
    );

    let mut path = parsed("Win32_X.A=1");
    path.set_text(1, Some(&utf16("\0x"))).unwrap();
    assert_eq!(path, ObjectPath::default());
}

#[test]
fn malformed_keys_are_rejected() {
    for input in [
        "Win32_Process.A",
        "Win32_Process.",
        "Win32_Process.=",
        "Win32_Process.A=",
        "Win32_Process.A=1,",
        "Win32_Process.A=1,,B=2",
        "Win32_Process.A=1, B=2",
        "Win32_Process.A=,B=1",
        "Win32_Process=",
        "Win32_Process=,",
        "Win32_Process=\t1",
        "Win32_Process.A= 1",
        "A=",
        "A= 1",
    ] {
        assert_rejected(input);
    }
}

#[test]
fn whitespace_in_a_key_name_is_rejected() {
    for ws in [' ', '\t', '\n', '\u{b}', '\u{c}', '\r'] {
        assert_rejected(&format!("Win32_Process.a{ws}b=1"));
    }
}

#[test]
fn accepted_inputs_keep_the_c_quirks() {
    let path = parsed("Win32_Process=5,b=6");
    assert_eq!(path.get_key_count(), 1);
    assert_eq!(path.keys[0].value, utf16("5"));

    let path = parsed("Win32_Process=1,");
    assert_eq!(path.get_key_count(), 1);

    let path = parsed("Win32_Process.A=1 B=2");
    assert_eq!(path.keys[0].value, utf16("1 B=2"));
}

#[test]
fn a_leading_colon_has_no_namespaces() {
    let path = parsed(":a\\b");
    assert_eq!(path.get_namespace_count(), 0);
    assert_eq!(
        get_text(&path, WBEMPATH_GET_RELATIVE_ONLY),
        (Ok(4), "a\\b".to_string())
    );
}

#[test]
fn info_follows_the_server_and_namespace_state() {
    let mut path = parsed("Win32_X");
    assert_eq!(path.get_info(0), Ok(0xc15));
    path.set_server(Some(&utf16("srv")));
    assert_eq!(path.get_info(0), Ok(0x20e16));
    path.set_server(None);
    assert_eq!(path.get_info(0), Ok(0x15));
    assert_eq!(path.get_info(1), Err(WBEM_E_INVALID_PARAMETER));

    let mut path = ObjectPath::default();
    assert_eq!(path.get_info(0), Ok(0x4001));
    path.set_namespace_at(0, Some(&utf16("root"))).unwrap();
    assert_eq!(path.get_info(0), Ok(0x4e01));
}

#[test]
fn an_implied_key_reports_one_extra_unit() {
    let path = parsed("Win32_X=1");
    assert_eq!(path.get_text(0, 0, None), Ok(12));
    assert_eq!(get_text(&path, 0), (Ok(12), ":Win32_X=1".to_string()));
}

#[test]
fn get_text_reports_the_length_before_it_writes() {
    let path = parsed("Win32_X.Name=\"a\"");
    let mut buf = [0u16; 17];
    assert_eq!(
        path.get_text(WBEMPATH_GET_ORIGINAL, 16, Some(&mut buf[..16])),
        Ok(17)
    );
    assert_eq!(buf, [0u16; 17]);
    assert_eq!(
        path.get_text(WBEMPATH_GET_ORIGINAL, 17, Some(&mut buf[..])),
        Ok(17)
    );
    assert_eq!(text(&buf), "Win32_X.Name=\"a\"");
    assert_eq!(
        get_text(&path, 0),
        (Ok(18), ":Win32_X.Name=\"a\"".to_string())
    );
    assert_eq!(path.get_text(0, 1, None), Ok(18));
    assert_eq!(path.get_text(0, 18, None), Err(WBEM_E_INVALID_PARAMETER));

    let mut buf = [7u16; 4];
    assert_eq!(path.get_text(1, 4, Some(&mut buf[..])), Ok(1));
    assert_eq!(buf[0], 0);

    assert_eq!(
        parsed("\\\\srv").get_text(WBEMPATH_GET_RELATIVE_ONLY, 0, None),
        Ok(1)
    );
    assert_eq!(
        ObjectPath::default().get_text(WBEMPATH_GET_ORIGINAL, 0, None),
        Ok(1)
    );
    assert_eq!(
        get_text(&parsed("Win32_X.A=1"), WBEMPATH_GET_RELATIVE_ONLY),
        (Ok(12), "Win32_X.A=1".to_string())
    );
}

#[test]
fn get_server_reports_missing_and_short_buffers() {
    let mut path = ObjectPath::default();
    assert_eq!(path.get_server(0, None), Err(WBEM_E_NOT_AVAILABLE));
    assert_eq!(path.get_server(1, None), Err(WBEM_E_INVALID_PARAMETER));
    path.set_server(Some(&utf16("srv")));
    assert_eq!(path.get_server(0, None), Ok(4));
    let mut buf = [7u16; 4];
    assert_eq!(path.get_server(3, Some(&mut buf[..3])), Ok(4));
    assert_eq!(buf, [7u16; 4]);
    assert_eq!(path.get_server(4, Some(&mut buf[..])), Ok(4));
    assert_eq!(text(&buf), "srv");
}

#[test]
fn namespace_accessors_check_index_capacity_and_length() {
    let mut path = parsed("root\\cimv2:C");
    assert_eq!(path.get_namespace_at(0, 0, None), Ok(5));
    assert_eq!(
        path.get_namespace_at(0, 3, None),
        Err(WBEM_E_INVALID_PARAMETER)
    );
    assert_eq!(
        path.get_namespace_at(2, 0, None),
        Err(WBEM_E_INVALID_PARAMETER)
    );
    let mut buf = [7u16; 8];
    assert_eq!(path.get_namespace_at(1, 5, Some(&mut buf[..5])), Ok(6));
    assert_eq!(buf, [7u16; 8]);
    assert_eq!(path.get_namespace_at(1, 8, Some(&mut buf[..])), Ok(6));
    assert_eq!(text(&buf), "cimv2");

    assert_eq!(
        path.set_namespace_at(0, None),
        Err(WBEM_E_INVALID_PARAMETER)
    );
    assert_eq!(
        path.set_namespace_at(3, Some(&utf16("end"))),
        Err(WBEM_E_INVALID_PARAMETER)
    );
    path.set_namespace_at(1, Some(&utf16("mid"))).unwrap();
    path.set_namespace_at(3, Some(&utf16("end"))).unwrap();
    assert_eq!(
        path.namespaces,
        vec![utf16("root"), utf16("mid"), utf16("cimv2"), utf16("end")]
    );

    assert_eq!(path.remove_namespace_at(4), Err(WBEM_E_INVALID_PARAMETER));
    path.remove_namespace_at(0).unwrap();
    assert_eq!(path.get_namespace_count(), 3);
    path.remove_all_namespaces();
    assert_eq!(path.get_namespace_count(), 0);
}

#[test]
fn class_accessors_check_presence_capacity_and_length() {
    let mut path = ObjectPath::default();
    assert_eq!(
        path.get_class_name(0, None),
        Err(WBEM_E_INVALID_OBJECT_PATH)
    );
    assert_eq!(path.set_class_name(None), Err(WBEM_E_INVALID_PARAMETER));
    path.set_class_name(Some(&utf16("Cls"))).unwrap();
    assert_eq!(path.get_info(0), Ok(0xc15));
    assert_eq!(path.get_class_name(1, None), Err(WBEM_E_INVALID_PARAMETER));
    assert_eq!(path.get_class_name(0, None), Ok(4));
    let mut buf = [7u16; 4];
    assert_eq!(path.get_class_name(3, Some(&mut buf[..3])), Ok(4));
    assert_eq!(buf, [7u16; 4]);
    assert_eq!(path.get_class_name(4, Some(&mut buf[..])), Ok(4));
    assert_eq!(text(&buf), "Cls");
}

#[test]
fn remove_all_keys_checks_flags() {
    let mut path = parsed("Win32_X.A=1");
    assert_eq!(path.remove_all_keys(1), Err(WBEM_E_INVALID_PARAMETER));
    assert_eq!(path.get_key_count(), 1);
    path.remove_all_keys(0).unwrap();
    assert_eq!(path.get_key_count(), 0);
}

#[test]
fn clone_eq_and_debug_cover_the_keys() {
    let path = parsed("Win32_X.A=1");
    assert_eq!(path.clone(), path);
    assert!(format!("{path:?}").contains("keys: [Key"));
}

#[test]
fn status_text_prints_the_full_code() {
    assert_eq!(error_code_text(1), "Error code: 0x00000001");
    assert_eq!(error_code_text(0x7fff_ffff), "Error code: 0x7fffffff");
    assert_eq!(
        facility_code_text(0x8004_1008u32 as i32),
        "Facility code: 0x80041008"
    );
}
