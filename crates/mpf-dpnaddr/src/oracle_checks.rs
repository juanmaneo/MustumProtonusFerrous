//! Oracle-derived test vectors for the ported dpnaddr logic.
//!
//! The vectors come from `target/canon/dpnaddr/harness.c`. It includes verbatim copies of
//! winebuild's `assign_ordinals` (`tools/winebuild/parser.c`), `get_args_size`
//! (`tools/winebuild/utils.c`) and `get_target_ptr_size` (`tools/tools.h`), and reads the verbatim
//! `dlls/dpnaddr/dpnaddr.spec`. It was built with `gcc -std=c11 -fsanitize=address,undefined -g`.
//! All 18 printed rows ran clean under both sanitizers, so none are excluded.

use super::*;

/// `get_args_size` rows 09 to 18: `(args as printed, cpu, args, bytes)`.
const ARG_ROWS: [(&str, Cpu, &[ArgType], u32); 10] = [
    ("-", Cpu::I386, &[], 0),
    ("ptr", Cpu::I386, &[ArgType::Ptr], 4),
    ("ptr,ptr", Cpu::I386, &[ArgType::Ptr, ArgType::Ptr], 8),
    ("int64", Cpu::I386, &[ArgType::Int64], 8),
    ("double", Cpu::I386, &[ArgType::Double], 8),
    ("int128", Cpu::I386, &[ArgType::Int128], 16),
    ("int128", Cpu::X86_64, &[ArgType::Int128], 8),
    (
        "word,sword,segptr,segstr,long,str,wstr,float",
        Cpu::I386,
        &[
            ArgType::Word,
            ArgType::SWord,
            ArgType::SegPtr,
            ArgType::SegStr,
            ArgType::Long,
            ArgType::Str,
            ArgType::WStr,
            ArgType::Float,
        ],
        32,
    ),
    (
        "int64,long",
        Cpu::X86_64,
        &[ArgType::Int64, ArgType::Long],
        16,
    ),
    (
        "int128,ptr",
        Cpu::X86_64,
        &[ArgType::Int128, ArgType::Ptr],
        16,
    ),
];

#[test]
fn oracle_spec_export_rows() {
    // row 01 spec name=DirectPlay8AddressCreate args=3 -> nb_args=3
    assert_eq!(FORWARDS[0].name, "DirectPlay8AddressCreate");
    assert_eq!(FORWARDS[0].args.len(), 3);
    // row 02 spec link_name=dpnet.DirectPlay8Create -> forward=1 flags=0x1000
    assert_eq!(
        format!("{}.{}", FORWARDS[0].target_dll, FORWARDS[0].target_name),
        "dpnet.DirectPlay8Create"
    );
    // row 03 spec split link_name -> target_dll=dpnet target_name=DirectPlay8Create
    assert_eq!(FORWARDS[0].target_dll, "dpnet");
    assert_eq!(FORWARDS[0].target_name, "DirectPlay8Create");
}

#[test]
fn oracle_assign_ordinals_row() {
    // row 04 assign_ordinals explicit=- -> base=1 limit=1 ordinal=1
    assert_eq!(FORWARDS[0].ordinal, 1);
}

#[test]
fn oracle_data_only_rows() {
    // row 05 data_only_check data_only=1 forward=1 -> accepted=1 error=""
    assert!(data_only_accepts(true));
    // row 06 data_only_check data_only=1 forward=0 -> accepted=0
    // error="Only forwarded entry points are allowed in data-only mode"
    assert!(!data_only_accepts(false));
}

#[test]
fn oracle_spec_stack_bytes_rows() {
    // row 07 get_args_size cpu=i386 args=spec -> 12
    assert_eq!(FORWARDS[0].stack_bytes(Cpu::I386), 12);
    // row 08 get_args_size cpu=x86_64 args=spec -> 24
    assert_eq!(FORWARDS[0].stack_bytes(Cpu::X86_64), 24);
}

#[test]
fn oracle_get_args_size_rows() {
    for (i, &(label, cpu, args, bytes)) in ARG_ROWS.iter().enumerate() {
        let row = i + 9;
        assert_eq!(
            stdcall_arg_bytes(args, cpu),
            bytes,
            "row {row:02} args={label}"
        );
    }
}
