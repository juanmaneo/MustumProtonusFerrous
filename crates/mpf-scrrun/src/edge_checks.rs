//! Boundary cases for each branch of the ported logic. Every expected value was confirmed
//! against the oracle built from Wine's verbatim C, and the NUL cases follow Wine's C string
//! functions.

use crate::{
    base_name, build_path, create_error, drive_name, extension_name, file_name, flt_hash,
    is_dir_data, is_file_data, num_hash, parent_folder_name, str_hash, version_string,
    CompareMethod, CTL_E_FILEALREADYEXISTS, CTL_E_FILENOTFOUND, CTL_E_ILLEGALFUNCTIONCALL,
    CTL_E_PATHNOTFOUND, CTL_E_PERMISSIONDENIED, E_FAIL, FILE_ATTRIBUTE_DIRECTORY,
};

fn u(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

fn bp(path: Option<&str>, name: Option<&str>) -> Vec<u16> {
    build_path(path.map(u).as_deref(), name.map(u).as_deref())
}

#[test]
fn build_path_joins_at_each_separator_seam() {
    assert_eq!(bp(Some("a\\"), Some("\\b")), u("a\\b"));
    assert_eq!(bp(Some("a"), Some("b")), u("a\\b"));
    assert_eq!(bp(Some("a\\"), Some("b")), u("a\\b"));
    assert_eq!(bp(Some("a"), Some("\\b")), u("a\\b"));
    assert_eq!(bp(Some("\\"), Some("\\")), u("\\"));
}

#[test]
fn build_path_empty_and_absent_sides() {
    assert_eq!(bp(Some(""), Some("x")), u("\\x"));
    assert_eq!(bp(Some(""), Some("")), u("\\"));
    assert_eq!(bp(Some("x"), None), u("x"));
    assert_eq!(bp(None, Some("\\x")), u("\\x"));
    assert_eq!(bp(None, None), Vec::<u16>::new());
}

#[test]
fn build_path_keeps_the_nul_after_a_drive_colon() {
    assert_eq!(bp(Some("C:"), Some("x")), u("C:x\0"));
    assert_eq!(bp(Some("C:"), Some("")), u("C:\0"));
    assert_eq!(bp(Some("C:"), Some("\\x")), u("C:\\x"));
}

#[test]
fn build_path_one_sided_stops_at_nul() {
    assert_eq!(bp(Some("a\0b"), None), u("a"));
    assert_eq!(bp(None, Some("\\a\0b")), u("\\a"));
}

#[test]
fn parent_folder_name_scans() {
    assert_eq!(parent_folder_name(&[]), None);
    assert_eq!(parent_folder_name(&u("\\\\")), None);
    assert_eq!(parent_folder_name(&u("\\x")), None);
    assert_eq!(parent_folder_name(&u("file")), None);
    assert_eq!(parent_folder_name(&u("C:")), None);
    assert_eq!(parent_folder_name(&u("C:\\x")), Some(u("C:\\")));
    assert_eq!(parent_folder_name(&u("C:\\dir\\")), Some(u("C:\\")));
    assert_eq!(parent_folder_name(&u("dir\\x")), Some(u("dir")));
    assert_eq!(parent_folder_name(&u("ab:\\c")), Some(u("ab:")));
    assert_eq!(parent_folder_name(&u("ab\\c")), Some(u("ab")));
    assert_eq!(parent_folder_name(&u("a\0")), None);
    assert_eq!(parent_folder_name(&u("a\\b\0c")), Some(u("a")));
}

#[test]
fn file_name_absent_cases() {
    assert_eq!(file_name(&[]), None);
    assert_eq!(file_name(&u("\\")), None);
    assert_eq!(file_name(&u("C:")), None);
    assert_eq!(file_name(&u("a:")), None);
    assert_eq!(file_name(&u("\0")), None);
}

#[test]
fn file_name_last_component() {
    assert_eq!(file_name(&u("ab")), Some(u("ab")));
    assert_eq!(file_name(&u("C:x")), Some(u("C:x")));
    assert_eq!(file_name(&u("a\\b")), Some(u("b")));
    assert_eq!(file_name(&u("a/b\\")), Some(u("b")));
    assert_eq!(file_name(&u("\\b")), Some(u("b")));
    assert_eq!(file_name(&u("ab\0\\c")), Some(u("ab")));
}

#[test]
fn base_name_strips_the_last_extension() {
    assert_eq!(base_name(&u("a/a.b.c")), Some(u("a.b")));
    assert_eq!(base_name(&u("a.b\\")), Some(u("a")));
    assert_eq!(base_name(&u("a.b\\c")), Some(u("c")));
    assert_eq!(base_name(&u("ab.c\\d.e")), Some(u("d")));
    assert_eq!(base_name(&u("ab")), Some(u("ab")));
    assert_eq!(base_name(&u("a.")), Some(u("a")));
    assert_eq!(base_name(&u("a..")), Some(u("a.")));
    assert_eq!(base_name(&u("a..b")), Some(u("a.")));
    assert_eq!(base_name(&u("..a")), Some(u(".")));
    assert_eq!(base_name(&u("..")), Some(u(".")));
}

#[test]
fn base_name_leading_dot_and_absent_cases() {
    assert_eq!(base_name(&u(".a")), Some(Vec::new()));
    assert_eq!(base_name(&u(".")), Some(Vec::new()));
    assert_eq!(base_name(&[]), None);
    assert_eq!(base_name(&u("/\\")), None);
    assert_eq!(base_name(&u("c:")), None);
    assert_eq!(base_name(&u("\0")), None);
    assert_eq!(base_name(&u("a.b\0c")), Some(u("a")));
}

#[test]
fn extension_name_after_the_last_dot() {
    assert_eq!(extension_name(&u("a.b\\c")), Some(u("b\\c")));
    assert_eq!(extension_name(&u("a.")), Some(Vec::new()));
    assert_eq!(extension_name(&u(".")), Some(Vec::new()));
    assert_eq!(extension_name(&u("noext")), None);
    assert_eq!(extension_name(&u("a.b\0c")), Some(u("b")));
    assert_eq!(extension_name(&[]), None);
}

#[test]
fn drive_name_needs_a_colon_at_index_one() {
    assert_eq!(drive_name(&u("C:")), Some(u("C:")));
    assert_eq!(drive_name(&u("C:\\x")), Some(u("C:")));
    assert_eq!(drive_name(&u("::")), Some(u("::")));
    assert_eq!(drive_name(&u("C")), None);
    assert_eq!(drive_name(&u("ab")), None);
    assert_eq!(drive_name(&[]), None);
    assert_eq!(drive_name(&u("\0:")), None);
}

#[test]
fn str_hash_folds_case_only_for_text_and_database() {
    assert_eq!(str_hash(&u("ABC"), CompareMethod::Binary), 758);
    assert_eq!(str_hash(&u("ABC"), CompareMethod::Text), 974);
    assert_eq!(str_hash(&u("ABC"), CompareMethod::Database), 974);
    assert_eq!(str_hash(&u("abc"), CompareMethod::Text), 974);
    assert_eq!(
        str_hash(&u("[@`"), CompareMethod::Text),
        str_hash(&u("[@`"), CompareMethod::Binary)
    );
    assert_eq!(str_hash(&u("a\0b"), CompareMethod::Binary), 97);
    assert_eq!(str_hash(&[], CompareMethod::Text), 0);
}

#[test]
fn num_hash_uses_the_bit_pattern() {
    assert_eq!(num_hash(0.0), 0);
    assert_eq!(num_hash(1.0), 161);
    assert_eq!(num_hash(-0.0), 769);
}

#[test]
fn flt_hash_infinity_nan_and_finite() {
    assert_eq!(flt_hash(f32::INFINITY), Ok(0));
    assert_eq!(flt_hash(f32::NEG_INFINITY), Ok(0));
    assert_eq!(flt_hash(1.0), Ok(161));
    assert_eq!(flt_hash(f32::NAN), Err(CTL_E_ILLEGALFUNCTIONCALL));
    assert_eq!(
        flt_hash(f32::from_bits(0x7F80_0001)),
        Err(CTL_E_ILLEGALFUNCTIONCALL)
    );
}

#[test]
fn create_error_maps_each_win32_code() {
    assert_eq!(create_error(2), CTL_E_FILENOTFOUND);
    assert_eq!(create_error(3), CTL_E_PATHNOTFOUND);
    assert_eq!(create_error(5), CTL_E_PERMISSIONDENIED);
    assert_eq!(create_error(80), CTL_E_FILEALREADYEXISTS);
    assert_eq!(create_error(183), CTL_E_FILEALREADYEXISTS);
    assert_eq!(create_error(0), E_FAIL);
    assert_eq!(create_error(1000), E_FAIL);
}

#[test]
fn version_string_splits_four_halfwords() {
    assert_eq!(version_string(0, 0), "0.0.0.0");
    assert_eq!(version_string(0x0001_0002, 0x0003_0004), "1.2.3.4");
    assert_eq!(
        version_string(u32::MAX, u32::MAX),
        "65535.65535.65535.65535"
    );
}

#[test]
fn is_dir_data_skips_dot_entries() {
    assert!(is_dir_data(FILE_ATTRIBUTE_DIRECTORY, &u("sub")));
    assert!(is_dir_data(FILE_ATTRIBUTE_DIRECTORY, &u("...")));
    assert!(!is_dir_data(FILE_ATTRIBUTE_DIRECTORY, &u(".")));
    assert!(!is_dir_data(FILE_ATTRIBUTE_DIRECTORY, &u("..")));
    assert!(!is_dir_data(FILE_ATTRIBUTE_DIRECTORY, &u(".\0x")));
    assert!(!is_dir_data(0, &u("sub")));
    assert!(!is_dir_data(0x20, &u("sub")));
}

#[test]
fn is_file_data_is_the_complement_of_the_directory_bit() {
    assert!(is_file_data(0));
    assert!(is_file_data(0x20));
    assert!(!is_file_data(FILE_ATTRIBUTE_DIRECTORY));
}
