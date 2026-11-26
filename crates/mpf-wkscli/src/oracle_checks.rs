//! Oracle-derived test vectors for the ported wkscli logic.
//!
//! Vectors are the 26 rows printed by `target/canon/wkscli/oracle.c`. That harness compiles the
//! verbatim Wine bodies of `NetGetJoinInformation`, `NetUseAdd`, `NetUseDel`, `NetUseEnum`,
//! `NetUseGetInfo`, `NetWkstaTransportEnum` and `__wine_spec_unimplemented_stub` with
//! `gcc -fsanitize=address,undefined -g`. ASan and UBSan reported nothing, so no row is excluded.
//! The count is above 25 because `wkscli.spec` has 23 exports and `NetGetJoinInformation` has
//! four rows.

use super::*;

/// `Workgroup` and its terminating NUL, as UTF-16 code units. Row 03 prints the name and
/// `alloc_units=10`.
const WORKGROUP_UTF16: [u16; 10] = [0x57, 0x6f, 0x72, 0x6b, 0x67, 0x72, 0x6f, 0x75, 0x70, 0];

/// The record the harness printed for an `@ stub` row: code `0x80000100`, flags `0x1`, two
/// parameters, module `wkscli.dll`, and the export name as the function.
fn printed_stub(function: &'static str) -> StubException {
    StubException {
        code: 0x8000_0100,
        flags: 0x1,
        number_parameters: 2,
        module: "wkscli.dll",
        function,
    }
}

#[test]
fn oracle_stub_rows() {
    // row 01 stub NetAddAlternateComputerName
    assert_eq!(
        net_add_alternate_computer_name(),
        printed_stub("NetAddAlternateComputerName")
    );
    // row 02 stub NetEnumerateComputerNames
    assert_eq!(
        net_enumerate_computer_names(),
        printed_stub("NetEnumerateComputerNames")
    );
    // row 07 stub NetGetJoinableOUs
    assert_eq!(net_get_joinable_ous(), printed_stub("NetGetJoinableOUs"));
    // row 08 stub NetJoinDomain
    assert_eq!(net_join_domain(), printed_stub("NetJoinDomain"));
    // row 09 stub NetRemoveAlternateComputerName
    assert_eq!(
        net_remove_alternate_computer_name(),
        printed_stub("NetRemoveAlternateComputerName")
    );
    // row 10 stub NetRenameMachineInDomain
    assert_eq!(
        net_rename_machine_in_domain(),
        printed_stub("NetRenameMachineInDomain")
    );
    // row 11 stub NetSetPrimaryComputerName
    assert_eq!(
        net_set_primary_computer_name(),
        printed_stub("NetSetPrimaryComputerName")
    );
    // row 12 stub NetUnjoinDomain
    assert_eq!(net_unjoin_domain(), printed_stub("NetUnjoinDomain"));
    // row 17 stub NetValidateName
    assert_eq!(net_validate_name(), printed_stub("NetValidateName"));
    // row 18 stub NetWkstaGetInfo
    assert_eq!(net_wksta_get_info(), printed_stub("NetWkstaGetInfo"));
    // row 19 stub NetWkstaSetInfo
    assert_eq!(net_wksta_set_info(), printed_stub("NetWkstaSetInfo"));
    // row 20 stub NetWkstaStatisticsGet
    assert_eq!(
        net_wksta_statistics_get(),
        printed_stub("NetWkstaStatisticsGet")
    );
    // row 21 stub NetWkstaTransportAdd
    assert_eq!(
        net_wksta_transport_add(),
        printed_stub("NetWkstaTransportAdd")
    );
    // row 22 stub NetWkstaTransportDel
    assert_eq!(
        net_wksta_transport_del(),
        printed_stub("NetWkstaTransportDel")
    );
    // row 24 stub NetWkstaUserEnum
    assert_eq!(net_wksta_user_enum(), printed_stub("NetWkstaUserEnum"));
    // row 25 stub NetWkstaUserGetInfo
    assert_eq!(
        net_wksta_user_get_info(),
        printed_stub("NetWkstaUserGetInfo")
    );
    // row 26 stub NetWkstaUserSetInfo
    assert_eq!(
        net_wksta_user_set_info(),
        printed_stub("NetWkstaUserSetInfo")
    );
}

#[test]
fn oracle_net_get_join_information_rows() {
    // row 03 NetGetJoinInformation server=set name_ptr=set type_ptr=set -> ret=0 name=Workgroup
    // alloc_units=10 type=2
    let mut name = None;
    let mut join_type = 7;
    assert_eq!(
        net_get_join_information(Some(&mut name), Some(&mut join_type)),
        NERR_SUCCESS
    );
    assert_eq!(name, Some(WORKGROUP_UTF16.to_vec()));
    assert_eq!(join_type, NET_SETUP_WORKGROUP_NAME);

    // row 04 NetGetJoinInformation server=NULL name_ptr=set type_ptr=set -> ret=0 name=Workgroup
    // alloc_units=10 type=2. The Server argument is not a port parameter, so the call is the same.
    let mut name = None;
    let mut join_type = 7;
    assert_eq!(
        net_get_join_information(Some(&mut name), Some(&mut join_type)),
        NERR_SUCCESS
    );
    assert_eq!(name, Some(WORKGROUP_UTF16.to_vec()));
    assert_eq!(join_type, NET_SETUP_WORKGROUP_NAME);

    // row 05 NetGetJoinInformation server=set name_ptr=NULL type_ptr=set -> ret=87 name=untouched
    // alloc_units=0 type=7
    let mut join_type = 7;
    assert_eq!(
        net_get_join_information(None, Some(&mut join_type)),
        ERROR_INVALID_PARAMETER
    );
    assert_eq!(join_type, 7);

    // row 06 NetGetJoinInformation server=set name_ptr=set type_ptr=NULL -> ret=87 name=untouched
    // alloc_units=0 type=7
    let mut name = None;
    assert_eq!(
        net_get_join_information(Some(&mut name), None),
        ERROR_INVALID_PARAMETER
    );
    assert_eq!(name, None);
}

#[test]
fn oracle_fixed_status_rows() {
    // row 13 NetUseAdd level=3 bufptr=NULL parm_err=NULL -> ret=0
    assert_eq!(net_use_add(), 0);
    // row 14 NetUseDel force=1 -> ret=0
    assert_eq!(net_use_del(), 0);
    // row 15 NetUseEnum level=1 -> ret=50
    assert_eq!(net_use_enum(), 50);
    // row 16 NetUseGetInfo level=1 -> ret=50
    assert_eq!(net_use_get_info(), 50);
    // row 23 NetWkstaTransportEnum level=0 -> ret=50
    assert_eq!(net_wksta_transport_enum(), 50);
}
