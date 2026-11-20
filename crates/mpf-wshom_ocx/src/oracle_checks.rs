use crate::{root_key, split_command, split_reg_path, RootKey, PATH_NOT_FOUND};

const ORACLE_ROW_COUNT: usize = 22;

fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

fn narrow(text: &[u16]) -> String {
    String::from_utf16_lossy(text)
}

#[test]
fn split_command_matches_wine_oracle_rows() {
    let rows = [
        (
            "C:\\Windows\\System32\\wscript.exe //B",
            "C:\\Windows\\System32\\wscript.exe",
            Some("//B"),
        ),
        (
            "\"C:\\Program Files\\script.vbs\" arg",
            "\"C:\\Program Files\\script.vbs\"",
            Some("arg"),
        ),
        ("one\" two\" three", "one\" two\"", Some("three")),
        ("\"\"", "\"\"", None),
        ("", "", None),
        (" ", "", Some("")),
        ("a  b", "a", Some(" b")),
    ];
    for (input, expected_program, expected_params) in rows {
        let input = wide(input);
        let (program, params) = split_command(&input);
        assert_eq!(narrow(program), expected_program);
        assert_eq!(params.map(narrow).as_deref(), expected_params);
    }
    assert_eq!(rows.len() + 15, ORACLE_ROW_COUNT);
}

#[test]
fn root_key_matches_wine_oracle_rows() {
    let rows = [
        ("HKEY_CURRENT_USER\\Software", Some(RootKey::CurrentUser)),
        ("HKEY_LOCAL_MACHINE\\Software", Some(RootKey::LocalMachine)),
        ("HKEY_CLASSES_ROOT\\.txt", Some(RootKey::ClassesRoot)),
        ("HKCU\\x", Some(RootKey::CurrentUser)),
        ("HKLM\\x", Some(RootKey::LocalMachine)),
        ("HKCR\\x", Some(RootKey::ClassesRoot)),
        ("HKEY_USERS\\x", Some(RootKey::Users)),
        ("HKEY_CURRENT_CONFIG\\x", Some(RootKey::CurrentConfig)),
        ("HKEY_BAD\\x", None),
    ];
    for (input, expected) in rows {
        assert_eq!(root_key(&wide(input)), expected);
    }
    assert_eq!(
        [
            RootKey::ClassesRoot.hkey(),
            RootKey::CurrentUser.hkey(),
            RootKey::LocalMachine.hkey(),
            RootKey::Users.hkey(),
            RootKey::CurrentConfig.hkey(),
        ],
        [
            0x8000_0000,
            0x8000_0001,
            0x8000_0002,
            0x8000_0003,
            0x8000_0005
        ]
    );
    assert_eq!(7 + rows.len() + 6, ORACLE_ROW_COUNT);
}

#[test]
fn split_reg_path_matches_wine_oracle_rows() {
    let rows = [
        ("HKCU", Err(PATH_NOT_FOUND)),
        ("HKCU\\", Ok(("", None))),
        ("HKCU\\Software\\Value", Ok(("Software", Some("Value")))),
        ("HKCU\\Value", Ok(("\\Value", Some("Value")))),
        ("HKCU\\\\Value", Ok(("\\\\Value", Some("Value")))),
        ("HKCU\\Software\\", Ok(("Software\\", None))),
    ];
    for (input, expected) in rows {
        let input = wide(input);
        let result = split_reg_path(&input);
        match expected {
            Err(error) => assert_eq!(result.unwrap_err(), error),
            Ok((subkey, value)) => {
                let actual = result.unwrap();
                assert_eq!(narrow(actual.subkey), subkey);
                assert_eq!(actual.value.map(narrow).as_deref(), value);
            }
        }
    }
    assert_eq!(7 + 9 + rows.len(), ORACLE_ROW_COUNT);
}
