//! Boundary and branch tests for the ported dpnaddr logic.
//!
//! The oracle rows in `oracle_checks.rs` cover the values. These tests cover the lookup edges,
//! the arithmetic rule and the build constants. Expectations that are not oracle-derived are
//! marked with a comment.

use super::*;

#[test]
fn forward_for_finds_the_only_entry() {
    assert_eq!(forward_for("DirectPlay8AddressCreate"), Some(FORWARDS[0]));
}

#[test]
fn forward_for_matches_the_whole_name_only() {
    // Not oracle-derived: the reference has no lookup function, so these are name-matching edges.
    assert_eq!(forward_for("DirectPlay8AddressCreate "), None);
    assert_eq!(forward_for(" DirectPlay8AddressCreate"), None);
    assert_eq!(forward_for("directplay8addresscreate"), None);
    assert_eq!(forward_for("DirectPlay8AddressCreat"), None);
    assert_eq!(forward_for("DirectPlay8AddressCreate2"), None);
    assert_eq!(forward_for(""), None);
    // The forward target is not an export of dpnaddr itself.
    assert_eq!(forward_for("DirectPlay8Create"), None);
    assert_eq!(forward_for("dpnet.DirectPlay8Create"), None);
}

#[test]
fn stack_bytes_is_pointer_size_per_argument() {
    // Not oracle-derived: a structural check that i386 gives 4 bytes per `ptr` parameter.
    assert_eq!(
        FORWARDS[0].stack_bytes(Cpu::I386),
        Cpu::I386.ptr_size() * FORWARDS[0].args.len() as u32
    );
}

#[test]
fn arg_bytes_are_zero_for_no_arguments() {
    // Not oracle-derived: the x86_64 empty list. The harness only ran the i386 empty list.
    assert_eq!(stdcall_arg_bytes(&[], Cpu::X86_64), 0);
    assert_eq!(stdcall_arg_bytes(&[], Cpu::I386), 0);
}

#[test]
fn arg_bytes_sum_each_argument() {
    // Not oracle-derived: a mixed list that the harness did not run. Sums are per argument.
    assert_eq!(
        stdcall_arg_bytes(&[ArgType::Int64, ArgType::Ptr, ArgType::Word], Cpu::I386),
        16
    );
    assert_eq!(
        stdcall_arg_bytes(
            &[ArgType::Ptr, ArgType::Int128, ArgType::Double],
            Cpu::X86_64
        ),
        24
    );
}

#[test]
fn pointer_size_is_four_on_i386_and_eight_on_x86_64() {
    // Not oracle-derived: the `get_target_ptr_size` values that oracle rows 07, 08 and 10 rely on.
    assert_eq!(Cpu::I386.ptr_size(), 4);
    assert_eq!(Cpu::X86_64.ptr_size(), 8);
}

#[test]
fn makefile_constants_match_makefile_in() {
    // Not oracle-derived: literals copied from `dlls/dpnaddr/Makefile.in`.
    assert_eq!(MODULE, "dpnaddr.dll");
    assert_eq!(PRODUCT_VERSION, [5, 3, 2600, 2180]);
    assert_eq!(FILE_DESCRIPTION, "Wine DpnAddr dll");
    const { assert!(DATA_ONLY) };
}
