//! Oracle-derived test vectors for the ported mapistub stub records.
//!
//! The vectors were captured by compiling the verbatim Wine C body of
//! `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`) with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/mapistub/`. The harness calls it once
//! for each of the 100 `stub` exports in `dlls/mapistub/mapistub.spec`, in spec order, with the
//! module name from `dlls/mapistub/Makefile.in`. The C loop never exits, so the harness's
//! `RtlRaiseException` shim records the exception and longjmps back out. ASan and UBSan reported
//! nothing, so no row is excluded.

use super::*;

/// One row per stub export, in spec order: the helper and the `function` value the C harness
/// recorded. Every row has code 0x8000_0100, flags 0x01, two parameters and module `mapistub.dll`.
const ORACLE_ROWS: [(fn() -> StubException, StubFunction); 100] = [
    (ordinal_8, StubFunction::Ordinal(8)), // row 01
    (pr_provider_init, StubFunction::Name("PRProviderInit")), // row 02
    (launchwizard, StubFunction::Name("LAUNCHWIZARD")), // row 03
    (launch_wizard_at_20, StubFunction::Name("LaunchWizard@20")), // row 04
    (
        dll_get_class_object,
        StubFunction::Name("DllGetClassObject"),
    ), // row 05
    (mapi_open_form_mgr, StubFunction::Name("MAPIOpenFormMgr")), // row 06
    (
        mapi_open_form_mgr_at_8,
        StubFunction::Name("MAPIOpenFormMgr@8"),
    ), // row 07
    (
        sc_generate_muid_at_4,
        StubFunction::Name("ScGenerateMuid@4"),
    ), // row 08
    (
        hr_alloc_advise_sink_at_12,
        StubFunction::Name("HrAllocAdviseSink@12"),
    ), // row 09
    (
        sc_bin_from_hex_bounded_at_12,
        StubFunction::Name("ScBinFromHexBounded@12"),
    ), // row 10
    (
        build_display_table_at_40,
        StubFunction::Name("BuildDisplayTable@40"),
    ), // row 11
    (mapi_init_idle_at_4, StubFunction::Name("MAPIInitIdle@4")), // row 12
    (
        mapi_deinit_idle_at_0,
        StubFunction::Name("MAPIDeinitIdle@0"),
    ), // row 13
    (
        install_filter_hook_at_4,
        StubFunction::Name("InstallFilterHook@4"),
    ), // row 14
    (
        ftg_register_idle_routine_at_20,
        StubFunction::Name("FtgRegisterIdleRoutine@20"),
    ), // row 15
    (
        enable_idle_routine_at_8,
        StubFunction::Name("EnableIdleRoutine@8"),
    ), // row 16
    (
        deregister_idle_routine_at_4,
        StubFunction::Name("DeregisterIdleRoutine@4"),
    ), // row 17
    (
        change_idle_routine_at_28,
        StubFunction::Name("ChangeIdleRoutine@28"),
    ), // row 18
    (create_table_at_36, StubFunction::Name("CreateTable@36")), // row 19
    (
        wrap_store_entry_id_at_24,
        StubFunction::Name("WrapStoreEntryID@24"),
    ), // row 20
    (hr_add_columns_at_16, StubFunction::Name("HrAddColumns@16")), // row 21
    (
        hr_add_columns_ex_at_20,
        StubFunction::Name("HrAddColumnsEx@20"),
    ), // row 22
    (ft_adc_ft_at_20, StubFunction::Name("FtAdcFt@20")), // row 23
    (ft_div_ft_bogus_at_20, StubFunction::Name("FtDivFtBogus@20")), // row 24
    (
        hr_sz_from_entry_id_at_12,
        StubFunction::Name("HrSzFromEntryID@12"),
    ), // row 25
    (
        hr_entry_id_from_sz_at_12,
        StubFunction::Name("HrEntryIDFromSz@12"),
    ), // row 26
    (hr_compose_eid_at_28, StubFunction::Name("HrComposeEID@28")), // row 27
    (
        hr_decompose_eid_at_28,
        StubFunction::Name("HrDecomposeEID@28"),
    ), // row 28
    (
        hr_compose_msg_id_at_24,
        StubFunction::Name("HrComposeMsgID@24"),
    ), // row 29
    (
        hr_decompose_msg_id_at_24,
        StubFunction::Name("HrDecomposeMsgID@24"),
    ), // row 30
    (
        open_tnef_stream_at_28,
        StubFunction::Name("OpenTnefStream@28"),
    ), // row 31
    (open_tnef_stream, StubFunction::Name("OpenTnefStream")), // row 32
    (
        open_tnef_stream_ex_at_32,
        StubFunction::Name("OpenTnefStreamEx@32"),
    ), // row 33
    (open_tnef_stream_ex, StubFunction::Name("OpenTnefStreamEx")), // row 34
    (
        get_tnef_stream_codepage_at_12,
        StubFunction::Name("GetTnefStreamCodepage@12"),
    ), // row 35
    (
        get_tnef_stream_codepage,
        StubFunction::Name("GetTnefStreamCodepage"),
    ), // row 36
    (
        unkobj_sc_allocate_at_12,
        StubFunction::Name("UNKOBJ_ScAllocate@12"),
    ), // row 37
    (
        unkobj_sc_allocate_more_at_16,
        StubFunction::Name("UNKOBJ_ScAllocateMore@16"),
    ), // row 38
    (unkobj_free_at_8, StubFunction::Name("UNKOBJ_Free@8")), // row 39
    (
        unkobj_free_rows_at_8,
        StubFunction::Name("UNKOBJ_FreeRows@8"),
    ), // row 40
    (
        unkobj_sc_co_allocate_at_12,
        StubFunction::Name("UNKOBJ_ScCOAllocate@12"),
    ), // row 41
    (
        unkobj_sc_co_reallocate_at_12,
        StubFunction::Name("UNKOBJ_ScCOReallocate@12"),
    ), // row 42
    (unkobj_co_free_at_8, StubFunction::Name("UNKOBJ_COFree@8")), // row 43
    (
        unkobj_sc_sz_from_ids_alloc_at_20,
        StubFunction::Name("UNKOBJ_ScSzFromIdsAlloc@20"),
    ), // row 44
    (
        sc_count_notifications_at_12,
        StubFunction::Name("ScCountNotifications@12"),
    ), // row 45
    (
        sc_copy_notifications_at_16,
        StubFunction::Name("ScCopyNotifications@16"),
    ), // row 46
    (
        sc_reloc_notifications_at_20,
        StubFunction::Name("ScRelocNotifications@20"),
    ), // row 47
    (
        f_bad_rglp_name_id_at_8,
        StubFunction::Name("FBadRglpNameID@8"),
    ), // row 48
    (rtf_sync_at_12, StubFunction::Name("RTFSync@12")), // row 49
    (rtf_sync, StubFunction::Name("RTFSync")), // row 50
    (
        wrap_compressed_rtf_stream_at_12,
        StubFunction::Name("WrapCompressedRTFStream@12"),
    ), // row 51
    (
        wrap_compressed_rtf_stream,
        StubFunction::Name("WrapCompressedRTFStream"),
    ), // row 52
    (
        __validate_parameters_at_8,
        StubFunction::Name("__ValidateParameters@8"),
    ), // row 53
    (
        __cpp_validate_parameters_at_8,
        StubFunction::Name("__CPPValidateParameters@8"),
    ), // row 54
    (
        f_bad_sort_order_set_at_4,
        StubFunction::Name("FBadSortOrderSet@4"),
    ), // row 55
    (
        f_bad_restriction_at_4,
        StubFunction::Name("FBadRestriction@4"),
    ), // row 56
    (
        sc_unc_from_local_path_at_12,
        StubFunction::Name("ScUNCFromLocalPath@12"),
    ), // row 57
    (
        sc_local_path_from_unc_at_12,
        StubFunction::Name("ScLocalPathFromUNC@12"),
    ), // row 58
    (
        hr_i_storage_from_stream_at_16,
        StubFunction::Name("HrIStorageFromStream@16"),
    ), // row 59
    (
        hr_validate_ipm_subtree_at_20,
        StubFunction::Name("HrValidateIPMSubtree@20"),
    ), // row 60
    (
        open_i_msg_session_at_12,
        StubFunction::Name("OpenIMsgSession@12"),
    ), // row 61
    (
        close_i_msg_session_at_4,
        StubFunction::Name("CloseIMsgSession@4"),
    ), // row 62
    (
        open_i_msg_on_i_stg_at_44,
        StubFunction::Name("OpenIMsgOnIStg@44"),
    ), // row 63
    (
        set_attrib_i_msg_on_i_stg_at_16,
        StubFunction::Name("SetAttribIMsgOnIStg@16"),
    ), // row 64
    (
        get_attrib_i_msg_on_i_stg_at_12,
        StubFunction::Name("GetAttribIMsgOnIStg@12"),
    ), // row 65
    (
        map_storage_s_code_at_4,
        StubFunction::Name("MapStorageSCode@4"),
    ), // row 66
    (sc_mapix_from_cmc, StubFunction::Name("ScMAPIXFromCMC")), // row 67
    (sc_mapix_from_smapi, StubFunction::Name("ScMAPIXFromSMAPI")), // row 68
    (encode_id_at_12, StubFunction::Name("EncodeID@12")), // row 69
    (f_decode_id_at_12, StubFunction::Name("FDecodeID@12")), // row 70
    (cch_of_encoding_at_4, StubFunction::Name("CchOfEncoding@4")), // row 71
    (bmapi_send_mail, StubFunction::Name("BMAPISendMail")), // row 72
    (bmapi_save_mail, StubFunction::Name("BMAPISaveMail")), // row 73
    (bmapi_read_mail, StubFunction::Name("BMAPIReadMail")), // row 74
    (bmapi_get_read_mail, StubFunction::Name("BMAPIGetReadMail")), // row 75
    (bmapi_find_next, StubFunction::Name("BMAPIFindNext")), // row 76
    (bmapi_address, StubFunction::Name("BMAPIAddress")), // row 77
    (bmapi_get_address, StubFunction::Name("BMAPIGetAddress")), // row 78
    (bmapi_details, StubFunction::Name("BMAPIDetails")), // row 79
    (bmapi_resolve_name, StubFunction::Name("BMAPIResolveName")), // row 80
    (cmc_act_on, StubFunction::Name("cmc_act_on")), // row 81
    (cmc_free, StubFunction::Name("cmc_free")), // row 82
    (cmc_list, StubFunction::Name("cmc_list")), // row 83
    (cmc_logoff, StubFunction::Name("cmc_logoff")), // row 84
    (cmc_logon, StubFunction::Name("cmc_logon")), // row 85
    (cmc_look_up, StubFunction::Name("cmc_look_up")), // row 86
    (cmc_read, StubFunction::Name("cmc_read")), // row 87
    (cmc_send, StubFunction::Name("cmc_send")), // row 88
    (cmc_send_documents, StubFunction::Name("cmc_send_documents")), // row 89
    (
        hr_dispatch_notifications_at_4,
        StubFunction::Name("HrDispatchNotifications@4"),
    ), // row 90
    (
        hr_validate_parameters_at_8,
        StubFunction::Name("HrValidateParameters@8"),
    ), // row 91
    (
        sc_create_conversation_index_at_16,
        StubFunction::Name("ScCreateConversationIndex@16"),
    ), // row 92
    (
        hr_get_omi_providers_flags,
        StubFunction::Name("HrGetOmiProvidersFlags"),
    ), // row 93
    (
        hr_get_omi_providers_flags_at_8,
        StubFunction::Name("HrGetOmiProvidersFlags@8"),
    ), // row 94
    (
        hr_set_omi_providers_flags_invalid,
        StubFunction::Name("HrSetOmiProvidersFlagsInvalid"),
    ), // row 95
    (
        hr_set_omi_providers_flags_invalid_at_4,
        StubFunction::Name("HrSetOmiProvidersFlagsInvalid@4"),
    ), // row 96
    (get_outlook_version, StubFunction::Name("GetOutlookVersion")), // row 97
    (
        get_outlook_version_at_0,
        StubFunction::Name("GetOutlookVersion@0"),
    ), // row 98
    (fix_mapi, StubFunction::Name("FixMAPI")), // row 99
    (fix_mapi_at_0, StubFunction::Name("FixMAPI@0")), // row 100
];

#[test]
fn oracle_stub_rows() {
    for (row, &(stub, function)) in ORACLE_ROWS.iter().enumerate() {
        assert_eq!(
            stub(),
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "mapistub.dll",
                function,
            },
            "row {:02}",
            row + 1
        );
    }
}

#[test]
fn export_table_stub_entries_are_the_oracle_rows() {
    let table: Vec<StubFunction> = EXPORTS
        .iter()
        .filter(|(_, _, kind)| *kind == ExportKind::Stub)
        .map(|&(ordinal, name, _)| match name {
            Some(name) => StubFunction::Name(name),
            None => StubFunction::Ordinal(ordinal),
        })
        .collect();
    let rows: Vec<StubFunction> = ORACLE_ROWS.iter().map(|&(_, function)| function).collect();
    assert_eq!(table, rows);
}
