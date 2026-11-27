//! Edge-case tests for the ported rsabase logic.
//!
//! These cover the ARM alignment branch, the int128 branch, the empty argument list, the lookup
//! misses, the `-private` filter and the `.def` suffix combinations. Expectations marked "not
//! oracle-derived" come from reading the reference source, not from a captured run.

use super::*;
use std::collections::HashSet;

#[test]
fn arm_pads_eight_byte_arguments_and_i386_does_not() {
    // Not oracle-derived: the `target.cpu == CPU_ARM` guard in get_args_size. One word leaves the
    // int64 at offset 4, which ARM pads to 8 and i386 does not. On ARM64 the word is pointer-sized,
    // so the offset is already 8 and nothing is padded.
    let args = [ArgType::Word, ArgType::Int64];
    assert_eq!(args_size(&args, Cpu::Arm), 16);
    assert_eq!(args_size(&args, Cpu::I386), 12);
    assert_eq!(args_size(&args, Cpu::Arm64), 16);
    // Two words put the ARM offset at 8 before the int64, so the padding is a no-op there.
    let args = [ArgType::Word, ArgType::Word, ArgType::Int64];
    assert_eq!(args_size(&args, Cpu::Arm), 16);
    assert_eq!(args_size(&args, Cpu::Arm64), 24);
}

#[test]
fn int128_takes_sixteen_bytes_except_on_x86_64() {
    // Not oracle-derived: the `ArgType::Int128 if cpu != Cpu::X86_64` guard. Oracle rows 06, 11,
    // 16 and 19 cover the same arms; this pins the two sides of the guard for the remaining CPUs.
    let args = [ArgType::Int128];
    assert_eq!(args_size(&args, Cpu::I386), 16);
    assert_eq!(args_size(&args, Cpu::Arm), 16);
    assert_eq!(args_size(&args, Cpu::Arm64), 16);
    assert_eq!(args_size(&args, Cpu::Arm64ec), 16);
    assert_eq!(args_size(&args, Cpu::X86_64), 8);
}

#[test]
fn empty_argument_list_is_zero_on_every_cpu() {
    // Not oracle-derived: the loop body never runs. Oracle rows 01, 21 and 22 check three of
    // these CPUs; this covers the other two.
    for cpu in [Cpu::I386, Cpu::X86_64, Cpu::Arm, Cpu::Arm64, Cpu::Arm64ec] {
        assert_eq!(args_size(&[], cpu), 0);
    }
}

#[test]
fn pointer_size_is_four_on_32_bit_and_eight_on_64_bit_cpus() {
    // Not oracle-derived: the `get_target_ptr_size` table in tools/tools.h. One pointer argument
    // on each CPU measures the same value the table holds.
    let ptr = [ArgType::Ptr];
    assert_eq!(args_size(&ptr, Cpu::I386), 4);
    assert_eq!(args_size(&ptr, Cpu::Arm), 4);
    assert_eq!(args_size(&ptr, Cpu::X86_64), 8);
    assert_eq!(args_size(&ptr, Cpu::Arm64), 8);
    assert_eq!(args_size(&ptr, Cpu::Arm64ec), 8);
}

#[test]
fn find_export_is_exact_and_case_sensitive() {
    // Not oracle-derived: lookup rules over the names in rsabase.spec.
    assert_eq!(find_export("CPAcquireContext").map(|e| e.ordinal), Some(1));
    assert_eq!(
        find_export("CPVerifySignature").map(|e| e.ordinal),
        Some(25)
    );
    assert_eq!(
        find_export("DllRegisterServer").map(|e| e.private),
        Some(true)
    );
    assert!(find_export("cpacquirecontext").is_none());
    assert!(find_export("rsaenh.CPAcquireContext").is_none());
    assert!(find_export("CPAcquireContext ").is_none());
    assert!(find_export("").is_none());
}

#[test]
fn importable_drops_the_two_private_exports() {
    // Not oracle-derived: the `-private` filter. The `.def` rows show the PRIVATE suffix on the
    // same two entries.
    let names: Vec<&str> = importable().map(|e| e.name).collect();
    assert_eq!(names.len(), 25);
    assert_eq!(names.first(), Some(&"CPAcquireContext"));
    assert_eq!(names.last(), Some(&"CPVerifySignature"));
    assert!(!names.contains(&"DllRegisterServer"));
    assert!(!names.contains(&"DllUnregisterServer"));
}

#[test]
fn ordinals_count_up_from_one_in_spec_order_and_names_are_unique() {
    // Not oracle-derived: the same numbering the `.def` rows show, checked as a table invariant.
    for (index, export) in EXPORTS.iter().enumerate() {
        assert_eq!(usize::from(export.ordinal), index + 1);
    }
    let names: HashSet<&str> = EXPORTS.iter().map(|e| e.name).collect();
    assert_eq!(names.len(), EXPORTS.len());
}

#[test]
fn def_line_suffixes_follow_cpu_and_private_flag() {
    // Not oracle-derived: the four combinations of the i386 `@N` suffix and the PRIVATE suffix.
    // The `.def` rows cover the same lines; this checks the two branches against each other.
    let public = find_export("CPGenRandom").unwrap();
    let private = find_export("DllUnregisterServer").unwrap();
    assert_eq!(
        def_line(public, Cpu::I386),
        "  CPGenRandom@12=rsaenh.CPGenRandom @12"
    );
    assert_eq!(
        def_line(public, Cpu::X86_64),
        "  CPGenRandom=rsaenh.CPGenRandom @12"
    );
    assert_eq!(
        def_line(private, Cpu::I386),
        "  DllUnregisterServer@0=rsaenh.DllUnregisterServer @27 PRIVATE"
    );
    assert_eq!(
        def_line(private, Cpu::Arm),
        "  DllUnregisterServer=rsaenh.DllUnregisterServer @27 PRIVATE"
    );
}
