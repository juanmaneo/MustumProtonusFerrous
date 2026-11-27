//! Edge-case tests for the ported shfolder logic.
//!
//! These cover the export table, the name lookup, the argument-size boundary and the `.def` text.
//! Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn export_table_matches_spec_order() {
    // Oracle-derived: rows 17-24 show these names, ordinals and forward targets, in this order.
    assert_eq!(EXPORTS.len(), 2);
    assert_eq!(EXPORTS[0].name, "SHGetFolderPathA");
    assert_eq!(EXPORTS[0].ordinal, 1);
    assert_eq!(EXPORTS[0].forward, "shell32.SHGetFolderPathA");
    assert_eq!(EXPORTS[1].name, "SHGetFolderPathW");
    assert_eq!(EXPORTS[1].ordinal, 2);
    assert_eq!(EXPORTS[1].forward, "shell32.SHGetFolderPathW");
    // Oracle-derived: the signature `long long long long ptr` from the spec, and the 20-byte i386 total in row 03.
    assert_eq!(
        EXPORTS[0].args,
        &[
            ArgType::Long,
            ArgType::Long,
            ArgType::Long,
            ArgType::Long,
            ArgType::Ptr
        ]
    );
    assert_eq!(EXPORTS[1].args, EXPORTS[0].args);
}

#[test]
fn export_by_name_is_case_sensitive() {
    // Not oracle-derived: the lookup is a plain name comparison, so the names need exact case.
    assert_eq!(
        export_by_name("SHGetFolderPathW").map(|export| export.ordinal),
        Some(2)
    );
    assert_eq!(export_by_name("shgetfolderpathw"), None);
    assert_eq!(export_by_name("SHGetFolderPath"), None);
    assert_eq!(export_by_name(""), None);
}

#[test]
fn argument_size_at_the_winebuild_limit() {
    // Not oracle-derived: winebuild's MAX_ARGUMENTS is 32 (tools/winebuild/build.h). Each argument
    // takes one pointer-sized slot, so 32 arguments are 128 bytes on i386 and 256 on x86_64.
    assert_eq!(get_args_size(&[ArgType::Ptr; 32], Cpu::I386), 128);
    assert_eq!(get_args_size(&[ArgType::Ptr; 32], Cpu::X86_64), 256);
}

#[test]
fn zero_argument_export_is_undecorated_off_i386() {
    // Not oracle-derived: the zero-argument spec was only run for i386 (row 25). The non-i386
    // branch of `output_def_file` skips the `@N` suffix for every argument count.
    let zero_args = Export {
        name: "Z",
        ordinal: 1,
        args: &[],
        forward: "ntdll.Z",
    };
    assert_eq!(def_line(&zero_args, Cpu::X86_64), "  Z=ntdll.Z @1");
    assert_eq!(def_line(&zero_args, Cpu::Arm64), "  Z=ntdll.Z @1");
}

#[test]
fn ordinal_is_printed_in_decimal() {
    // Not oracle-derived: winebuild prints the ordinal with `@%d` (`output_def_file`). The spec in
    // shfolder has ordinals 1 and 2 only.
    let tenth = Export {
        name: "Q",
        ordinal: 10,
        args: &[],
        forward: "x.Q",
    };
    assert_eq!(def_line(&tenth, Cpu::X86_64), "  Q=x.Q @10");
}

#[test]
fn module_name_matches_makefile() {
    // Oracle-derived: the `LIBRARY shfolder.dll` line in the four full .def files of
    // `oracle_def_files_match_winebuild_output`. Also `MODULE = shfolder.dll` in dlls/shfolder/Makefile.in.
    assert_eq!(MODULE_NAME, "shfolder.dll");
}
