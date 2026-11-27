//! Oracle-derived test vectors for the ported shfolder logic.
//!
//! `target/canon/shfolder/run_oracle.sh` prints 25 rows:
//! - Rows 01-16: `get_args_size` from `tools/winebuild/utils.c`, copied verbatim and compiled with
//!   `gcc -fsanitize=address,undefined -g` against the real `build.h` and `tools.h`. The only shim is
//!   the `target` global, which `main.c` defines.
//! - Rows 17-24: `winebuild --def -E shfolder.spec` for i386, x86_64, arm and aarch64, from the
//!   reference `tools/winebuild` built with ASan and UBSan.
//! - Row 25: `winebuild --def` on the one-line spec `@ stdcall Z() ntdll.Z`, for the zero-argument case.
//!
//! LeakSanitizer is off for the winebuild runs: winebuild leaks its option strings at exit, which is
//! not row output. No ASan or UBSan report flagged a row, so no row is excluded.

use super::*;

#[test]
fn oracle_get_args_size_rows() {
    // row 01 H|i386||0
    assert_eq!(get_args_size(&[], Cpu::I386), 0);
    // row 02 H|i386|long|4
    assert_eq!(get_args_size(&[ArgType::Long], Cpu::I386), 4);
    // row 03 H|i386|long long long long ptr|20
    assert_eq!(get_args_size(EXPORTS[0].args, Cpu::I386), 20);
    // row 04 H|i386|ptr ptr ptr|12
    assert_eq!(get_args_size(&[ArgType::Ptr; 3], Cpu::I386), 12);

    // row 05 H|x86_64||0
    assert_eq!(get_args_size(&[], Cpu::X86_64), 0);
    // row 06 H|x86_64|long|8
    assert_eq!(get_args_size(&[ArgType::Long], Cpu::X86_64), 8);
    // row 07 H|x86_64|long long long long ptr|40
    assert_eq!(get_args_size(EXPORTS[0].args, Cpu::X86_64), 40);
    // row 08 H|x86_64|ptr ptr ptr|24
    assert_eq!(get_args_size(&[ArgType::Ptr; 3], Cpu::X86_64), 24);

    // row 09 H|arm||0
    assert_eq!(get_args_size(&[], Cpu::Arm), 0);
    // row 10 H|arm|long|4
    assert_eq!(get_args_size(&[ArgType::Long], Cpu::Arm), 4);
    // row 11 H|arm|long long long long ptr|20
    assert_eq!(get_args_size(EXPORTS[0].args, Cpu::Arm), 20);
    // row 12 H|arm|ptr ptr ptr|12
    assert_eq!(get_args_size(&[ArgType::Ptr; 3], Cpu::Arm), 12);

    // row 13 H|aarch64||0
    assert_eq!(get_args_size(&[], Cpu::Arm64), 0);
    // row 14 H|aarch64|long|8
    assert_eq!(get_args_size(&[ArgType::Long], Cpu::Arm64), 8);
    // row 15 H|aarch64|long long long long ptr|40
    assert_eq!(get_args_size(EXPORTS[0].args, Cpu::Arm64), 40);
    // row 16 H|aarch64|ptr ptr ptr|24
    assert_eq!(get_args_size(&[ArgType::Ptr; 3], Cpu::Arm64), 24);
}

#[test]
fn oracle_winebuild_export_lines() {
    // row 17 W|i386|  SHGetFolderPathA@20=shell32.SHGetFolderPathA @1
    assert_eq!(
        def_line(&EXPORTS[0], Cpu::I386),
        "  SHGetFolderPathA@20=shell32.SHGetFolderPathA @1"
    );
    // row 18 W|i386|  SHGetFolderPathW@20=shell32.SHGetFolderPathW @2
    assert_eq!(
        def_line(&EXPORTS[1], Cpu::I386),
        "  SHGetFolderPathW@20=shell32.SHGetFolderPathW @2"
    );
    // row 19 W|x86_64|  SHGetFolderPathA=shell32.SHGetFolderPathA @1
    assert_eq!(
        def_line(&EXPORTS[0], Cpu::X86_64),
        "  SHGetFolderPathA=shell32.SHGetFolderPathA @1"
    );
    // row 20 W|x86_64|  SHGetFolderPathW=shell32.SHGetFolderPathW @2
    assert_eq!(
        def_line(&EXPORTS[1], Cpu::X86_64),
        "  SHGetFolderPathW=shell32.SHGetFolderPathW @2"
    );
    // row 21 W|arm|  SHGetFolderPathA=shell32.SHGetFolderPathA @1
    assert_eq!(
        def_line(&EXPORTS[0], Cpu::Arm),
        "  SHGetFolderPathA=shell32.SHGetFolderPathA @1"
    );
    // row 22 W|arm|  SHGetFolderPathW=shell32.SHGetFolderPathW @2
    assert_eq!(
        def_line(&EXPORTS[1], Cpu::Arm),
        "  SHGetFolderPathW=shell32.SHGetFolderPathW @2"
    );
    // row 23 W|aarch64|  SHGetFolderPathA=shell32.SHGetFolderPathA @1
    assert_eq!(
        def_line(&EXPORTS[0], Cpu::Arm64),
        "  SHGetFolderPathA=shell32.SHGetFolderPathA @1"
    );
    // row 24 W|aarch64|  SHGetFolderPathW=shell32.SHGetFolderPathW @2
    assert_eq!(
        def_line(&EXPORTS[1], Cpu::Arm64),
        "  SHGetFolderPathW=shell32.SHGetFolderPathW @2"
    );
    // row 25 V|i386|zero_args.spec|  Z@0=ntdll.Z @1
    let zero_args = Export {
        name: "Z",
        ordinal: 1,
        args: &[],
        forward: "ntdll.Z",
    };
    assert_eq!(def_line(&zero_args, Cpu::I386), "  Z@0=ntdll.Z @1");
}

#[test]
fn oracle_def_files_match_winebuild_output() {
    // Rows 17-24 are the export lines of these four files, as `winebuild --def` wrote them.
    assert_eq!(
        def_file(Cpu::I386),
        "; File generated automatically from shfolder.spec; do not edit!\n\nLIBRARY shfolder.dll\n\nEXPORTS\n  SHGetFolderPathA@20=shell32.SHGetFolderPathA @1\n  SHGetFolderPathW@20=shell32.SHGetFolderPathW @2\n"
    );
    assert_eq!(
        def_file(Cpu::X86_64),
        "; File generated automatically from shfolder.spec; do not edit!\n\nLIBRARY shfolder.dll\n\nEXPORTS\n  SHGetFolderPathA=shell32.SHGetFolderPathA @1\n  SHGetFolderPathW=shell32.SHGetFolderPathW @2\n"
    );
    assert_eq!(
        def_file(Cpu::Arm),
        "; File generated automatically from shfolder.spec; do not edit!\n\nLIBRARY shfolder.dll\n\nEXPORTS\n  SHGetFolderPathA=shell32.SHGetFolderPathA @1\n  SHGetFolderPathW=shell32.SHGetFolderPathW @2\n"
    );
    assert_eq!(
        def_file(Cpu::Arm64),
        "; File generated automatically from shfolder.spec; do not edit!\n\nLIBRARY shfolder.dll\n\nEXPORTS\n  SHGetFolderPathA=shell32.SHGetFolderPathA @1\n  SHGetFolderPathW=shell32.SHGetFolderPathW @2\n"
    );
}
