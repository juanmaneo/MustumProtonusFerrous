//! Edge-case tests for the ported wkscli logic.
//!
//! These cover the NULL out-parameter branches, the workgroup buffer, the stub record, and the
//! export table. Not every expectation is oracle-derived; the ones that aren't say so in a
//! comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/lmerr.h, include/winerror.h, include/lmjoin.h,
    // include/winternl.h and include/winnt.h. The oracle rows print the same status values.
    assert_eq!(NERR_SUCCESS, 0);
    assert_eq!(ERROR_INVALID_PARAMETER, 87);
    assert_eq!(ERROR_NOT_SUPPORTED, 50);
    assert_eq!(NET_SETUP_WORKGROUP_NAME, 2);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn workgroup_name_is_nul_terminated() {
    // Not oracle-derived beyond row 03: the NUL is the terminator that sizeof(workgroupW) counts,
    // so the buffer holds 10 units. The oracle prints the name and alloc_units=10.
    let mut name = None;
    let mut join_type = 0;
    net_get_join_information(Some(&mut name), Some(&mut join_type));
    let name = name.expect("the name is set on success");
    assert_eq!(name.len(), 10);
    assert_eq!(name.last(), Some(&0));
    assert_eq!(String::from_utf16(&name[..9]).unwrap(), "Workgroup");
    assert!(!name[..9].contains(&0));
}

#[test]
fn join_information_success_overwrites_a_previous_name_and_type() {
    // Not oracle-derived: the harness only starts from NULL out-parameters, so a pre-set value
    // is an edge case of the port's out-parameter contract.
    let mut name = Some(vec![0x61]);
    let mut join_type = 0;
    assert_eq!(
        net_get_join_information(Some(&mut name), Some(&mut join_type)),
        NERR_SUCCESS
    );
    assert_eq!(name.as_ref().map(Vec::len), Some(10));
    assert_eq!(join_type, NET_SETUP_WORKGROUP_NAME);
}

#[test]
fn join_information_error_leaves_out_parameters_alone() {
    // Not oracle-derived: rows 05 and 06 check one NULL each, not a pre-set value and not both
    // NULL together.
    let mut name = Some(vec![0x61]);
    let mut join_type = 0;
    assert_eq!(
        net_get_join_information(None, Some(&mut join_type)),
        ERROR_INVALID_PARAMETER
    );
    assert_eq!(join_type, 0);
    assert_eq!(
        net_get_join_information(Some(&mut name), None),
        ERROR_INVALID_PARAMETER
    );
    assert_eq!(name, Some(vec![0x61]));
    assert_eq!(
        net_get_join_information(None, None),
        ERROR_INVALID_PARAMETER
    );
}

#[test]
fn wine_spec_unimplemented_stub_passes_its_names_through() {
    // Not oracle-derived: the oracle only calls the stub with the wkscli.dll module name.
    assert_eq!(
        wine_spec_unimplemented_stub("other.dll", "NetNotInSpec"),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "other.dll",
            function: "NetNotInSpec",
        }
    );
}

#[test]
fn every_stub_helper_names_a_stub_export() {
    // Not oracle-derived: each helper's function name must be a @ stub entry in the spec table.
    let helpers = [
        net_add_alternate_computer_name().function,
        net_enumerate_computer_names().function,
        net_get_joinable_ous().function,
        net_join_domain().function,
        net_remove_alternate_computer_name().function,
        net_rename_machine_in_domain().function,
        net_set_primary_computer_name().function,
        net_unjoin_domain().function,
        net_validate_name().function,
        net_wksta_get_info().function,
        net_wksta_set_info().function,
        net_wksta_statistics_get().function,
        net_wksta_transport_add().function,
        net_wksta_transport_del().function,
        net_wksta_user_enum().function,
        net_wksta_user_get_info().function,
        net_wksta_user_set_info().function,
    ];
    for name in helpers {
        assert_eq!(export_kind(name), Some(ExportKind::Stub), "{name}");
    }
}

#[test]
fn export_table_lists_the_spec_exports_in_order() {
    // Not oracle-derived: wkscli.spec has 23 entries, 6 of them @ stdcall and 17 of them @ stub.
    assert_eq!(EXPORTS.len(), 23);
    assert_eq!(
        EXPORTS
            .iter()
            .filter(|(_, kind)| *kind == ExportKind::Stdcall)
            .count(),
        6
    );
    assert_eq!(
        EXPORTS
            .iter()
            .filter(|(_, kind)| *kind == ExportKind::Stub)
            .count(),
        17
    );
    assert_eq!(
        EXPORTS[0],
        ("NetAddAlternateComputerName", ExportKind::Stub)
    );
    assert_eq!(EXPORTS[22], ("NetWkstaUserSetInfo", ExportKind::Stub));
}

#[test]
fn export_kind_is_case_sensitive_and_rejects_unknown_names() {
    // Not oracle-derived: the spec names and the lookup boundary.
    assert_eq!(export_kind("NetUseAdd"), Some(ExportKind::Stdcall));
    assert_eq!(
        export_kind("NetGetJoinInformation"),
        Some(ExportKind::Stdcall)
    );
    assert_eq!(export_kind("NetJoinDomain"), Some(ExportKind::Stub));
    assert_eq!(export_kind("netuseadd"), None);
    assert_eq!(export_kind("NetUseAdd "), None);
    assert_eq!(export_kind("NetNotInSpec"), None);
}
