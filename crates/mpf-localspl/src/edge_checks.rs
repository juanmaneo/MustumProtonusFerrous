//! Edge-case tests for the ported localspl logic.
//!
//! These cover boundary lengths, every branch of the classifiers and the empty
//! and `NULL` inputs. Expectations marked "not oracle-derived" come from reading
//! the C source, not from the captured harness output.

use super::*;

fn utf16(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

fn text(s: &[u16]) -> String {
    String::from_utf16(s).unwrap()
}

#[test]
fn port_type_values_match_localspl_private_h() {
    // Not oracle-derived: the values are the PORT_IS_* defines in localspl_private.h.
    assert_eq!(PortType::Lpt as u32, 1);
    assert_eq!(PortType::Com as u32, 2);
    assert_eq!(PortType::File as u32, 3);
    assert_eq!(PortType::Unixname as u32, 5);
    assert_eq!(PortType::Pipe as u32, 6);
    assert_eq!(PortType::Cups as u32, 7);
    assert_eq!(PortType::Lpr as u32, 8);
}

#[test]
fn classify_prefix_boundaries() {
    // Not oracle-derived: short names and exact-match boundaries follow from the C wcsncmp/lstrcmpW tests.
    assert_eq!(classify_port_name(&utf16("LPT")), Some(PortType::Lpt));
    assert_eq!(classify_port_name(&utf16("LP")), None);
    assert_eq!(classify_port_name(&utf16("COM")), Some(PortType::Com));
    assert_eq!(classify_port_name(&utf16("CO")), None);
    assert_eq!(classify_port_name(&utf16("FILE:")), Some(PortType::File));
    assert_eq!(classify_port_name(&utf16("FILE:x")), None);
    assert_eq!(classify_port_name(&utf16("FILE")), None);
    assert_eq!(classify_port_name(&utf16("CUPS:")), Some(PortType::Cups));
    assert_eq!(classify_port_name(&utf16("CUPS")), None);
    assert_eq!(classify_port_name(&utf16("LPR:")), Some(PortType::Lpr));
    assert_eq!(classify_port_name(&utf16("LPR")), None);
}

#[test]
fn classify_single_char_and_empty_names() {
    // Not oracle-derived: the '/' and '|' rules test name[0] only.
    assert_eq!(classify_port_name(&utf16("/")), Some(PortType::Unixname));
    assert_eq!(classify_port_name(&utf16("|")), Some(PortType::Pipe));
    assert_eq!(classify_port_name(&utf16("")), None);
}

#[test]
fn classify_rules_are_case_sensitive_and_ordered() {
    // Not oracle-derived: the comparisons are exact; a leading '/' wins over later prefixes.
    assert_eq!(classify_port_name(&utf16("com1")), None);
    assert_eq!(classify_port_name(&utf16("file:")), None);
    assert_eq!(
        classify_port_name(&utf16("/LPT1")),
        Some(PortType::Unixname)
    );
    assert_eq!(classify_port_name(&utf16(" COM1")), None);
}

#[test]
fn multi_sz_null_and_unterminated_buffers() {
    // Not oracle-derived: NULL gives 0; a buffer without a double NUL would make the C loop read past its end.
    assert_eq!(multi_sz_len_bytes(None), Some(0));
    assert_eq!(multi_sz_len_bytes(Some(&[])), None);
    assert_eq!(multi_sz_len_bytes(Some(&[0])), None);
    assert_eq!(multi_sz_len_bytes(Some(&[0x61])), None);
    assert_eq!(multi_sz_len_bytes(Some(&[0x61, 0])), None);
    assert_eq!(multi_sz_len_bytes(Some(&[0, 0x61, 0])), None);
}

#[test]
fn multi_sz_terminator_boundaries() {
    // Not oracle-derived: smallest terminated list, a leading empty string, and bytes past the first terminator.
    assert_eq!(multi_sz_len_bytes(Some(&[0, 0])), Some(4));
    assert_eq!(multi_sz_len_bytes(Some(&[0x61, 0, 0])), Some(6));
    assert_eq!(multi_sz_len_bytes(Some(&[0, 0x78, 0, 0])), Some(8));
    assert_eq!(
        multi_sz_len_bytes(Some(&utf16("ab\0cd\0\0ignored"))),
        Some(14)
    );
}

#[test]
fn servername_non_unc_and_empty_server() {
    // Not oracle-derived: NULL, a single separator, a bare pair with no server, and an empty server all return Empty.
    assert_eq!(copy_servername_from_name(None, None), ServerName::Empty);
    assert_eq!(
        copy_servername_from_name(Some(&utf16("\\")), None),
        ServerName::Empty
    );
    assert_eq!(
        copy_servername_from_name(Some(&utf16("\\x")), None),
        ServerName::Empty
    );
    assert_eq!(
        copy_servername_from_name(Some(&utf16("\\\\")), None),
        ServerName::Empty
    );
    assert_eq!(
        copy_servername_from_name(Some(&utf16("\\\\\\x")), None),
        ServerName::Empty
    );
    assert_eq!(
        copy_servername_from_name(Some(&utf16("")), None),
        ServerName::Empty
    );
}

#[test]
fn servername_length_boundary() {
    // Not oracle-derived: 15 units is the largest server name (MAX_COMPUTERNAME_LENGTH); 16 is too long.
    let fifteen = utf16("\\\\abcdefghijklmno\\x");
    let sixteen = utf16("\\\\abcdefghijklmnop\\x");
    assert_eq!(
        copy_servername_from_name(Some(&fifteen), None),
        ServerName::Remote(&fifteen[2..17])
    );
    assert_eq!(
        copy_servername_from_name(Some(&sixteen), None),
        ServerName::TooLong(16)
    );
}

#[test]
fn servername_without_separator_and_trailing_separator() {
    // Not oracle-derived: the server runs to the end of the name when no separator follows it.
    let bare = utf16("\\\\srv");
    assert_eq!(
        copy_servername_from_name(Some(&bare), None),
        ServerName::Remote(&bare[2..])
    );
    let trailing = utf16("\\\\srv\\");
    assert_eq!(
        copy_servername_from_name(Some(&trailing), None),
        ServerName::Remote(&trailing[2..5])
    );
}

#[test]
fn servername_local_matching_rules() {
    // Not oracle-derived: a failed GetComputerNameW, a length mismatch, a differing name and an over-long computer name all count as remote.
    let server = utf16("\\\\WINEBOX\\x");
    assert_eq!(
        copy_servername_from_name(Some(&server), None),
        ServerName::Remote(&server[2..9])
    );
    assert_eq!(
        copy_servername_from_name(Some(&server), Some(&utf16("WINEBOX1"))),
        ServerName::Remote(&server[2..9])
    );
    assert_eq!(
        copy_servername_from_name(Some(&server), Some(&utf16("WINEBOY"))),
        ServerName::Remote(&server[2..9])
    );
    assert_eq!(
        copy_servername_from_name(Some(&server), Some(&utf16("winebox"))),
        ServerName::Local(&server[2..9])
    );
    let fifteen_server = utf16("\\\\ABCDEFGHIJKLMNO\\x");
    assert_eq!(
        copy_servername_from_name(Some(&fifteen_server), Some(&utf16("abcdefghijklmno"))),
        ServerName::Local(&fifteen_server[2..17])
    );
    assert_eq!(
        copy_servername_from_name(Some(&fifteen_server), Some(&utf16("abcdefghijklmnop"))),
        ServerName::Remote(&fifteen_server[2..17])
    );
    let sixteen_server = utf16("\\\\ABCDEFGHIJKLMNOP\\x");
    assert_eq!(
        copy_servername_from_name(Some(&sixteen_server), Some(&utf16("ABCDEFGHIJKLMNOP"))),
        ServerName::TooLong(16)
    );
}

#[test]
fn basename_null_and_non_unc() {
    // Not oracle-derived: NULL gives None; a single backslash is not UNC, so the name is returned unchanged.
    assert_eq!(get_basename_from_name(None), None);
    assert_eq!(
        get_basename_from_name(Some(&utf16("\\"))),
        Some(&utf16("\\")[..])
    );
    assert_eq!(
        get_basename_from_name(Some(&utf16("\\x"))),
        Some(&utf16("\\x")[..])
    );
}

#[test]
fn basename_unc_boundaries() {
    // Not oracle-derived: no separator after the server, an empty basename, and only the first separator splits.
    assert_eq!(get_basename_from_name(Some(&utf16("\\\\srv"))), None);
    assert_eq!(get_basename_from_name(Some(&utf16("\\\\"))), None);
    assert_eq!(
        get_basename_from_name(Some(&utf16("\\\\\\x"))),
        Some(&utf16("x")[..])
    );
    assert_eq!(
        get_basename_from_name(Some(&utf16("\\\\srv\\a\\b"))),
        Some(&utf16("a\\b")[..])
    );
}

#[test]
fn file_part_boundaries() {
    // Not oracle-derived: no separator returns the whole name; the last separator splits; an empty name stays empty.
    assert_eq!(text(get_file_part(&utf16("plain"))), "plain");
    assert_eq!(text(get_file_part(&utf16("a\\b\\c"))), "c");
    assert_eq!(text(get_file_part(&utf16("\\"))), "");
    assert_eq!(text(get_file_part(&utf16(""))), "");
}

#[test]
fn validate_env_every_entry_and_case() {
    // Not oracle-derived: each printenv_t name resolves to its own entry, and the match ignores case.
    for entry in [
        PrintEnv::X86,
        PrintEnv::X64,
        PrintEnv::Ia64,
        PrintEnv::Arm,
        PrintEnv::Arm64,
        PrintEnv::Win40,
    ] {
        assert_eq!(
            validate_env(Some(&utf16(entry.name())), false, PrintEnv::X64),
            Some(entry)
        );
    }
    assert_eq!(
        validate_env(Some(&utf16("WINDOWS arm64")), false, PrintEnv::X64),
        Some(PrintEnv::Arm64)
    );
}

#[test]
fn validate_env_unknown_and_partial_names() {
    // Not oracle-derived: an unsupported or truncated name is rejected.
    assert_eq!(
        validate_env(Some(&utf16("Windows 95")), false, PrintEnv::X64),
        None
    );
    assert_eq!(
        validate_env(Some(&utf16("Windows")), true, PrintEnv::X64),
        None
    );
}

#[test]
fn validate_env_default_selection() {
    // Not oracle-derived: an empty or NULL name uses the Win9x entry when the GetVersion high bit is set, otherwise the build arch.
    assert_eq!(
        validate_env(Some(&[]), false, PrintEnv::Arm),
        Some(PrintEnv::Arm)
    );
    assert_eq!(
        validate_env(None, false, PrintEnv::Arm),
        Some(PrintEnv::Arm)
    );
    assert_eq!(
        validate_env(None, true, PrintEnv::Ia64),
        Some(PrintEnv::Win40)
    );
    assert_eq!(
        validate_env(Some(&[]), true, PrintEnv::Ia64),
        Some(PrintEnv::Win40)
    );
}
