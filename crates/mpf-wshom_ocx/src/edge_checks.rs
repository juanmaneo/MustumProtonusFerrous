use crate::{root_key, split_command, split_reg_path, RootKey, PATH_NOT_FOUND};

fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

#[test]
fn command_split_handles_utf16_quotes_and_c_string_termination() {
    let input = [
        0x03bb,
        b' '.into(),
        b'x'.into(),
        0,
        b' '.into(),
        b'y'.into(),
    ];
    let (program, params) = split_command(&input);
    assert_eq!(program, &[0x03bb]);
    assert_eq!(params, Some(&[b'x' as u16][..]));

    let input = wide("app\targument");
    let (program, params) = split_command(&input);
    assert_eq!(program, wide("app\targument"));
    assert_eq!(params, None);

    let input = wide("\"unterminated arg");
    let (program, params) = split_command(&input);
    assert_eq!(program, wide("\"unterminated arg"));
    assert_eq!(params, None);
}

#[test]
fn root_lookup_preserves_prefix_and_case_sensitive_matching() {
    assert_eq!(
        root_key(&wide("HKEY_CURRENT_USERish")),
        Some(RootKey::CurrentUser)
    );
    assert_eq!(root_key(&wide("HKCUsuffix")), Some(RootKey::CurrentUser));
    assert_eq!(root_key(&wide("hkey_current_user\\x")), None);
    assert_eq!(root_key(&wide("HK")), None);
    assert_eq!(
        root_key(&[
            b'H' as u16,
            b'K' as u16,
            b'C' as u16,
            b'U' as u16,
            0,
            b'x' as u16
        ]),
        Some(RootKey::CurrentUser)
    );
}

#[test]
fn registry_split_covers_empty_and_separator_edges() {
    assert_eq!(split_reg_path(&[]), Err(PATH_NOT_FOUND));
    assert_eq!(split_reg_path(&wide("value")), Err(PATH_NOT_FOUND));

    let input = wide("\\");
    let result = split_reg_path(&input);
    assert_eq!(result.unwrap().subkey, []);

    let input = wide("root\\key\\value\\");
    let result = split_reg_path(&input);
    let result = result.unwrap();
    assert_eq!(result.subkey, wide("key\\value\\"));
    assert_eq!(result.value, None);

    let result = split_reg_path(&[
        b'H' as u16,
        b'K' as u16,
        0x5c,
        b'V' as u16,
        0,
        0x5c,
        b'x' as u16,
    ])
    .unwrap();
    assert_eq!(result.subkey, wide("\\V"));
    assert_eq!(result.value, Some(&wide("V")[..]));
}
