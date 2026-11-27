//! Edge-case tests for the ported setupapi logic.
//!
//! Expectations marked "not oracle-derived" come from the reference source or the spec, not
//! from the harness output.

use super::*;

fn parse_text(text: &str) -> Result<InfFile, InfError> {
    let units: Vec<u16> = text.encode_utf16().collect();
    parse_inf_buffer(&units)
}

#[test]
fn error_codes_are_pinned() {
    // Not oracle-derived: values from include/setupapi.h.
    assert_eq!(ERROR_EXPECTED_SECTION_NAME, 0xE000_0000);
    assert_eq!(ERROR_BAD_SECTION_NAME_LINE, 0xE000_0001);
    assert_eq!(ERROR_SECTION_NAME_TOO_LONG, 0xE000_0002);
    assert_eq!(ERROR_CALL_NOT_IMPLEMENTED, 120);
}

#[test]
fn section_name_limit_is_255_units() {
    // Not oracle-derived: the 256-unit boundary of `token_len > MAX_SECTION_NAME_LEN` in parser.c.
    let name = "a".repeat(256);
    let err = parse_text(&format!("[{name}]\n")).unwrap_err();
    assert_eq!(
        err,
        InfError {
            code: ERROR_SECTION_NAME_TOO_LONG,
            line: 1
        }
    );
}

#[test]
fn field_limit_keeps_511_units() {
    // Not oracle-derived: a 511-unit token is not truncated. The oracle has the 607-unit case.
    let value = "a".repeat(511);
    let file = parse_text(&format!("[A]\nk={value}\n")).unwrap();
    assert_eq!(file.fields[1], value.encode_utf16().collect::<Vec<u16>>());
}

#[test]
fn section_names_ignore_ascii_case() {
    // Not oracle-derived: `find_section` uses wcsicmp, which folds ASCII case.
    let file = parse_text("[Strings]\nk=v\n[STRINGS]\nj=w\n").unwrap();
    assert_eq!(file.sections.len(), 1);
    assert_eq!(file.strings_section, Some(0));
    assert_eq!(file.sections[0].lines.len(), 2);
}

#[test]
fn empty_input_has_no_sections() {
    // Not oracle-derived: the empty buffer is the first input the parse loop sees.
    let file = parse_text("").unwrap();
    assert!(file.sections.is_empty());
    assert!(file.fields.is_empty());
    assert_eq!(file.strings_section, None);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the fixed setupapi names. The code and flags
    // are the same for any names.
    let other = wine_spec_unimplemented_stub("other.dll", "Foo");
    assert_eq!(other.module, "other.dll");
    assert_eq!(other.function, "Foo");
    assert_eq!(other.code, EXCEPTION_WINE_STUB);
    assert_eq!(other.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(other.number_parameters, 2);
}

#[test]
fn export_kinds_follow_the_spec() {
    // Not oracle-derived: the entries of dlls/setupapi/setupapi.spec.
    assert_eq!(export_kind("AcquireSCMLock"), Some(ExportKind::Stub));
    assert_eq!(
        export_kind("SetupInitializeFileLogW"),
        Some(ExportKind::Stdcall)
    );
    assert_eq!(export_kind("pSetupFree"), Some(ExportKind::Alias("MyFree")));
    assert_eq!(
        export_kind("CM_Get_Device_IDA"),
        Some(ExportKind::Forward("cfgmgr32.CM_Get_Device_IDA"))
    );
    assert_eq!(export_kind("acquirescmlock"), None);
    assert_eq!(export_kind(""), None);
}

#[test]
fn export_table_counts_match_the_spec() {
    // Not oracle-derived: 617 entries, 241 stubs, 287 plain stdcall, 16 aliases, 73 forwards.
    let count = |want: fn(&ExportKind) -> bool| EXPORTS.iter().filter(|(_, k)| want(k)).count();
    assert_eq!(EXPORTS.len(), 617);
    assert_eq!(count(|k| *k == ExportKind::Stub), 241);
    assert_eq!(count(|k| *k == ExportKind::Stdcall), 287);
    assert_eq!(count(|k| matches!(k, ExportKind::Alias(_))), 16);
    assert_eq!(count(|k| matches!(k, ExportKind::Forward(_))), 73);
}
