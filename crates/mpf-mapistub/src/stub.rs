/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name winebuild passes to stubs: `MODULE = mapistub.dll` in `dlls/mapistub/Makefile.in`.
const MAPISTUB_DLL_NAME: &str = "mapistub.dll";

/// The `function` argument of `__wine_spec_unimplemented_stub`. Winebuild passes the export name
/// when the export has one, and the ordinal as an integer when it does not.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StubFunction {
    /// The export name as written in the spec, including any `@N` suffix.
    Name(&'static str),
    /// The ordinal of an export that has no name.
    Ordinal(u16),
}

/// The record that `__wine_spec_unimplemented_stub` raises.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: StubFunction,
}

/// `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`. The C function raises the
/// record in a loop and never returns, so the port returns the record instead.
pub fn wine_spec_unimplemented_stub(module: &'static str, function: StubFunction) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        // Two parameters: the module and the function.
        number_parameters: 2,
        module,
        function,
    }
}

/// The ordinal-only `stub` export at ordinal 8 in `dlls/mapistub/mapistub.spec`.
pub fn ordinal_8() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Ordinal(8))
}

/// `PRProviderInit`: a `stub` export at ordinal 24 in `dlls/mapistub/mapistub.spec`.
pub fn pr_provider_init() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("PRProviderInit"))
}

/// `LAUNCHWIZARD`: a `stub` export at ordinal 25 in `dlls/mapistub/mapistub.spec`.
pub fn launchwizard() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("LAUNCHWIZARD"))
}

/// `LaunchWizard@20`: a `stub` export at ordinal 26 in `dlls/mapistub/mapistub.spec`.
pub fn launch_wizard_at_20() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("LaunchWizard@20"))
}

/// `DllGetClassObject`: a `stub` export at ordinal 27 in `dlls/mapistub/mapistub.spec`.
pub fn dll_get_class_object() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("DllGetClassObject"))
}

/// `MAPIOpenFormMgr`: a `stub` export at ordinal 29 in `dlls/mapistub/mapistub.spec`.
pub fn mapi_open_form_mgr() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("MAPIOpenFormMgr"))
}

/// `MAPIOpenFormMgr@8`: a `stub` export at ordinal 30 in `dlls/mapistub/mapistub.spec`.
pub fn mapi_open_form_mgr_at_8() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("MAPIOpenFormMgr@8"))
}

/// `ScGenerateMuid@4`: a `stub` export at ordinal 35 in `dlls/mapistub/mapistub.spec`.
pub fn sc_generate_muid_at_4() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("ScGenerateMuid@4"))
}

/// `HrAllocAdviseSink@12`: a `stub` export at ordinal 36 in `dlls/mapistub/mapistub.spec`.
pub fn hr_alloc_advise_sink_at_12() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("HrAllocAdviseSink@12"),
    )
}

/// `ScBinFromHexBounded@12`: a `stub` export at ordinal 43 in `dlls/mapistub/mapistub.spec`.
pub fn sc_bin_from_hex_bounded_at_12() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("ScBinFromHexBounded@12"),
    )
}

/// `BuildDisplayTable@40`: a `stub` export at ordinal 46 in `dlls/mapistub/mapistub.spec`.
pub fn build_display_table_at_40() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("BuildDisplayTable@40"),
    )
}

/// `MAPIInitIdle@4`: a `stub` export at ordinal 49 in `dlls/mapistub/mapistub.spec`.
pub fn mapi_init_idle_at_4() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("MAPIInitIdle@4"))
}

/// `MAPIDeinitIdle@0`: a `stub` export at ordinal 50 in `dlls/mapistub/mapistub.spec`.
pub fn mapi_deinit_idle_at_0() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("MAPIDeinitIdle@0"))
}

/// `InstallFilterHook@4`: a `stub` export at ordinal 51 in `dlls/mapistub/mapistub.spec`.
pub fn install_filter_hook_at_4() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("InstallFilterHook@4"))
}

/// `FtgRegisterIdleRoutine@20`: a `stub` export at ordinal 52 in `dlls/mapistub/mapistub.spec`.
pub fn ftg_register_idle_routine_at_20() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("FtgRegisterIdleRoutine@20"),
    )
}

/// `EnableIdleRoutine@8`: a `stub` export at ordinal 53 in `dlls/mapistub/mapistub.spec`.
pub fn enable_idle_routine_at_8() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("EnableIdleRoutine@8"))
}

/// `DeregisterIdleRoutine@4`: a `stub` export at ordinal 54 in `dlls/mapistub/mapistub.spec`.
pub fn deregister_idle_routine_at_4() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("DeregisterIdleRoutine@4"),
    )
}

/// `ChangeIdleRoutine@28`: a `stub` export at ordinal 55 in `dlls/mapistub/mapistub.spec`.
pub fn change_idle_routine_at_28() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("ChangeIdleRoutine@28"),
    )
}

/// `CreateTable@36`: a `stub` export at ordinal 61 in `dlls/mapistub/mapistub.spec`.
pub fn create_table_at_36() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("CreateTable@36"))
}

/// `WrapStoreEntryID@24`: a `stub` export at ordinal 73 in `dlls/mapistub/mapistub.spec`.
pub fn wrap_store_entry_id_at_24() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("WrapStoreEntryID@24"))
}

/// `HrAddColumns@16`: a `stub` export at ordinal 81 in `dlls/mapistub/mapistub.spec`.
pub fn hr_add_columns_at_16() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("HrAddColumns@16"))
}

/// `HrAddColumnsEx@20`: a `stub` export at ordinal 82 in `dlls/mapistub/mapistub.spec`.
pub fn hr_add_columns_ex_at_20() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("HrAddColumnsEx@20"))
}

/// `FtAdcFt@20`: a `stub` export at ordinal 122 in `dlls/mapistub/mapistub.spec`.
pub fn ft_adc_ft_at_20() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("FtAdcFt@20"))
}

/// `FtDivFtBogus@20`: a `stub` export at ordinal 127 in `dlls/mapistub/mapistub.spec`.
pub fn ft_div_ft_bogus_at_20() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("FtDivFtBogus@20"))
}

/// `HrSzFromEntryID@12`: a `stub` export at ordinal 141 in `dlls/mapistub/mapistub.spec`.
pub fn hr_sz_from_entry_id_at_12() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("HrSzFromEntryID@12"))
}

/// `HrEntryIDFromSz@12`: a `stub` export at ordinal 142 in `dlls/mapistub/mapistub.spec`.
pub fn hr_entry_id_from_sz_at_12() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("HrEntryIDFromSz@12"))
}

/// `HrComposeEID@28`: a `stub` export at ordinal 143 in `dlls/mapistub/mapistub.spec`.
pub fn hr_compose_eid_at_28() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("HrComposeEID@28"))
}

/// `HrDecomposeEID@28`: a `stub` export at ordinal 144 in `dlls/mapistub/mapistub.spec`.
pub fn hr_decompose_eid_at_28() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("HrDecomposeEID@28"))
}

/// `HrComposeMsgID@24`: a `stub` export at ordinal 145 in `dlls/mapistub/mapistub.spec`.
pub fn hr_compose_msg_id_at_24() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("HrComposeMsgID@24"))
}

/// `HrDecomposeMsgID@24`: a `stub` export at ordinal 146 in `dlls/mapistub/mapistub.spec`.
pub fn hr_decompose_msg_id_at_24() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("HrDecomposeMsgID@24"))
}

/// `OpenTnefStream@28`: a `stub` export at ordinal 149 in `dlls/mapistub/mapistub.spec`.
pub fn open_tnef_stream_at_28() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("OpenTnefStream@28"))
}

/// `OpenTnefStream`: a `stub` export at ordinal 150 in `dlls/mapistub/mapistub.spec`.
pub fn open_tnef_stream() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("OpenTnefStream"))
}

/// `OpenTnefStreamEx@32`: a `stub` export at ordinal 151 in `dlls/mapistub/mapistub.spec`.
pub fn open_tnef_stream_ex_at_32() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("OpenTnefStreamEx@32"))
}

/// `OpenTnefStreamEx`: a `stub` export at ordinal 152 in `dlls/mapistub/mapistub.spec`.
pub fn open_tnef_stream_ex() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("OpenTnefStreamEx"))
}

/// `GetTnefStreamCodepage@12`: a `stub` export at ordinal 153 in `dlls/mapistub/mapistub.spec`.
pub fn get_tnef_stream_codepage_at_12() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("GetTnefStreamCodepage@12"),
    )
}

/// `GetTnefStreamCodepage`: a `stub` export at ordinal 154 in `dlls/mapistub/mapistub.spec`.
pub fn get_tnef_stream_codepage() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("GetTnefStreamCodepage"),
    )
}

/// `UNKOBJ_ScAllocate@12`: a `stub` export at ordinal 156 in `dlls/mapistub/mapistub.spec`.
pub fn unkobj_sc_allocate_at_12() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("UNKOBJ_ScAllocate@12"),
    )
}

/// `UNKOBJ_ScAllocateMore@16`: a `stub` export at ordinal 157 in `dlls/mapistub/mapistub.spec`.
pub fn unkobj_sc_allocate_more_at_16() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("UNKOBJ_ScAllocateMore@16"),
    )
}

/// `UNKOBJ_Free@8`: a `stub` export at ordinal 158 in `dlls/mapistub/mapistub.spec`.
pub fn unkobj_free_at_8() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("UNKOBJ_Free@8"))
}

/// `UNKOBJ_FreeRows@8`: a `stub` export at ordinal 159 in `dlls/mapistub/mapistub.spec`.
pub fn unkobj_free_rows_at_8() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("UNKOBJ_FreeRows@8"))
}

/// `UNKOBJ_ScCOAllocate@12`: a `stub` export at ordinal 160 in `dlls/mapistub/mapistub.spec`.
pub fn unkobj_sc_co_allocate_at_12() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("UNKOBJ_ScCOAllocate@12"),
    )
}

/// `UNKOBJ_ScCOReallocate@12`: a `stub` export at ordinal 161 in `dlls/mapistub/mapistub.spec`.
pub fn unkobj_sc_co_reallocate_at_12() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("UNKOBJ_ScCOReallocate@12"),
    )
}

/// `UNKOBJ_COFree@8`: a `stub` export at ordinal 162 in `dlls/mapistub/mapistub.spec`.
pub fn unkobj_co_free_at_8() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("UNKOBJ_COFree@8"))
}

/// `UNKOBJ_ScSzFromIdsAlloc@20`: a `stub` export at ordinal 163 in `dlls/mapistub/mapistub.spec`.
pub fn unkobj_sc_sz_from_ids_alloc_at_20() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("UNKOBJ_ScSzFromIdsAlloc@20"),
    )
}

/// `ScCountNotifications@12`: a `stub` export at ordinal 164 in `dlls/mapistub/mapistub.spec`.
pub fn sc_count_notifications_at_12() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("ScCountNotifications@12"),
    )
}

/// `ScCopyNotifications@16`: a `stub` export at ordinal 165 in `dlls/mapistub/mapistub.spec`.
pub fn sc_copy_notifications_at_16() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("ScCopyNotifications@16"),
    )
}

/// `ScRelocNotifications@20`: a `stub` export at ordinal 166 in `dlls/mapistub/mapistub.spec`.
pub fn sc_reloc_notifications_at_20() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("ScRelocNotifications@20"),
    )
}

/// `FBadRglpNameID@8`: a `stub` export at ordinal 178 in `dlls/mapistub/mapistub.spec`.
pub fn f_bad_rglp_name_id_at_8() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("FBadRglpNameID@8"))
}

/// `RTFSync@12`: a `stub` export at ordinal 183 in `dlls/mapistub/mapistub.spec`.
pub fn rtf_sync_at_12() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("RTFSync@12"))
}

/// `RTFSync`: a `stub` export at ordinal 184 in `dlls/mapistub/mapistub.spec`.
pub fn rtf_sync() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("RTFSync"))
}

/// `WrapCompressedRTFStream@12`: a `stub` export at ordinal 185 in `dlls/mapistub/mapistub.spec`.
pub fn wrap_compressed_rtf_stream_at_12() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("WrapCompressedRTFStream@12"),
    )
}

/// `WrapCompressedRTFStream`: a `stub` export at ordinal 186 in `dlls/mapistub/mapistub.spec`.
pub fn wrap_compressed_rtf_stream() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("WrapCompressedRTFStream"),
    )
}

/// `__ValidateParameters@8`: a `stub` export at ordinal 187 in `dlls/mapistub/mapistub.spec`.
pub fn __validate_parameters_at_8() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("__ValidateParameters@8"),
    )
}

/// `__CPPValidateParameters@8`: a `stub` export at ordinal 188 in `dlls/mapistub/mapistub.spec`.
pub fn __cpp_validate_parameters_at_8() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("__CPPValidateParameters@8"),
    )
}

/// `FBadSortOrderSet@4`: a `stub` export at ordinal 189 in `dlls/mapistub/mapistub.spec`.
pub fn f_bad_sort_order_set_at_4() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("FBadSortOrderSet@4"))
}

/// `FBadRestriction@4`: a `stub` export at ordinal 191 in `dlls/mapistub/mapistub.spec`.
pub fn f_bad_restriction_at_4() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("FBadRestriction@4"))
}

/// `ScUNCFromLocalPath@12`: a `stub` export at ordinal 192 in `dlls/mapistub/mapistub.spec`.
pub fn sc_unc_from_local_path_at_12() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("ScUNCFromLocalPath@12"),
    )
}

/// `ScLocalPathFromUNC@12`: a `stub` export at ordinal 193 in `dlls/mapistub/mapistub.spec`.
pub fn sc_local_path_from_unc_at_12() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("ScLocalPathFromUNC@12"),
    )
}

/// `HrIStorageFromStream@16`: a `stub` export at ordinal 194 in `dlls/mapistub/mapistub.spec`.
pub fn hr_i_storage_from_stream_at_16() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("HrIStorageFromStream@16"),
    )
}

/// `HrValidateIPMSubtree@20`: a `stub` export at ordinal 195 in `dlls/mapistub/mapistub.spec`.
pub fn hr_validate_ipm_subtree_at_20() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("HrValidateIPMSubtree@20"),
    )
}

/// `OpenIMsgSession@12`: a `stub` export at ordinal 196 in `dlls/mapistub/mapistub.spec`.
pub fn open_i_msg_session_at_12() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("OpenIMsgSession@12"))
}

/// `CloseIMsgSession@4`: a `stub` export at ordinal 197 in `dlls/mapistub/mapistub.spec`.
pub fn close_i_msg_session_at_4() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("CloseIMsgSession@4"))
}

/// `OpenIMsgOnIStg@44`: a `stub` export at ordinal 198 in `dlls/mapistub/mapistub.spec`.
pub fn open_i_msg_on_i_stg_at_44() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("OpenIMsgOnIStg@44"))
}

/// `SetAttribIMsgOnIStg@16`: a `stub` export at ordinal 199 in `dlls/mapistub/mapistub.spec`.
pub fn set_attrib_i_msg_on_i_stg_at_16() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("SetAttribIMsgOnIStg@16"),
    )
}

/// `GetAttribIMsgOnIStg@12`: a `stub` export at ordinal 200 in `dlls/mapistub/mapistub.spec`.
pub fn get_attrib_i_msg_on_i_stg_at_12() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("GetAttribIMsgOnIStg@12"),
    )
}

/// `MapStorageSCode@4`: a `stub` export at ordinal 201 in `dlls/mapistub/mapistub.spec`.
pub fn map_storage_s_code_at_4() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("MapStorageSCode@4"))
}

/// `ScMAPIXFromCMC`: a `stub` export at ordinal 202 in `dlls/mapistub/mapistub.spec`.
pub fn sc_mapix_from_cmc() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("ScMAPIXFromCMC"))
}

/// `ScMAPIXFromSMAPI`: a `stub` export at ordinal 203 in `dlls/mapistub/mapistub.spec`.
pub fn sc_mapix_from_smapi() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("ScMAPIXFromSMAPI"))
}

/// `EncodeID@12`: a `stub` export at ordinal 204 in `dlls/mapistub/mapistub.spec`.
pub fn encode_id_at_12() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("EncodeID@12"))
}

/// `FDecodeID@12`: a `stub` export at ordinal 205 in `dlls/mapistub/mapistub.spec`.
pub fn f_decode_id_at_12() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("FDecodeID@12"))
}

/// `CchOfEncoding@4`: a `stub` export at ordinal 206 in `dlls/mapistub/mapistub.spec`.
pub fn cch_of_encoding_at_4() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("CchOfEncoding@4"))
}

/// `BMAPISendMail`: a `stub` export at ordinal 220 in `dlls/mapistub/mapistub.spec`.
pub fn bmapi_send_mail() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("BMAPISendMail"))
}

/// `BMAPISaveMail`: a `stub` export at ordinal 221 in `dlls/mapistub/mapistub.spec`.
pub fn bmapi_save_mail() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("BMAPISaveMail"))
}

/// `BMAPIReadMail`: a `stub` export at ordinal 222 in `dlls/mapistub/mapistub.spec`.
pub fn bmapi_read_mail() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("BMAPIReadMail"))
}

/// `BMAPIGetReadMail`: a `stub` export at ordinal 223 in `dlls/mapistub/mapistub.spec`.
pub fn bmapi_get_read_mail() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("BMAPIGetReadMail"))
}

/// `BMAPIFindNext`: a `stub` export at ordinal 224 in `dlls/mapistub/mapistub.spec`.
pub fn bmapi_find_next() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("BMAPIFindNext"))
}

/// `BMAPIAddress`: a `stub` export at ordinal 225 in `dlls/mapistub/mapistub.spec`.
pub fn bmapi_address() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("BMAPIAddress"))
}

/// `BMAPIGetAddress`: a `stub` export at ordinal 226 in `dlls/mapistub/mapistub.spec`.
pub fn bmapi_get_address() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("BMAPIGetAddress"))
}

/// `BMAPIDetails`: a `stub` export at ordinal 227 in `dlls/mapistub/mapistub.spec`.
pub fn bmapi_details() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("BMAPIDetails"))
}

/// `BMAPIResolveName`: a `stub` export at ordinal 228 in `dlls/mapistub/mapistub.spec`.
pub fn bmapi_resolve_name() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("BMAPIResolveName"))
}

/// `cmc_act_on`: a `stub` export at ordinal 229 in `dlls/mapistub/mapistub.spec`.
pub fn cmc_act_on() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("cmc_act_on"))
}

/// `cmc_free`: a `stub` export at ordinal 230 in `dlls/mapistub/mapistub.spec`.
pub fn cmc_free() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("cmc_free"))
}

/// `cmc_list`: a `stub` export at ordinal 231 in `dlls/mapistub/mapistub.spec`.
pub fn cmc_list() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("cmc_list"))
}

/// `cmc_logoff`: a `stub` export at ordinal 232 in `dlls/mapistub/mapistub.spec`.
pub fn cmc_logoff() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("cmc_logoff"))
}

/// `cmc_logon`: a `stub` export at ordinal 233 in `dlls/mapistub/mapistub.spec`.
pub fn cmc_logon() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("cmc_logon"))
}

/// `cmc_look_up`: a `stub` export at ordinal 234 in `dlls/mapistub/mapistub.spec`.
pub fn cmc_look_up() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("cmc_look_up"))
}

/// `cmc_read`: a `stub` export at ordinal 236 in `dlls/mapistub/mapistub.spec`.
pub fn cmc_read() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("cmc_read"))
}

/// `cmc_send`: a `stub` export at ordinal 237 in `dlls/mapistub/mapistub.spec`.
pub fn cmc_send() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("cmc_send"))
}

/// `cmc_send_documents`: a `stub` export at ordinal 238 in `dlls/mapistub/mapistub.spec`.
pub fn cmc_send_documents() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("cmc_send_documents"))
}

/// `HrDispatchNotifications@4`: a `stub` export at ordinal 239 in `dlls/mapistub/mapistub.spec`.
pub fn hr_dispatch_notifications_at_4() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("HrDispatchNotifications@4"),
    )
}

/// `HrValidateParameters@8`: a `stub` export at ordinal 241 in `dlls/mapistub/mapistub.spec`.
pub fn hr_validate_parameters_at_8() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("HrValidateParameters@8"),
    )
}

/// `ScCreateConversationIndex@16`: a `stub` export at ordinal 244 in `dlls/mapistub/mapistub.spec`.
pub fn sc_create_conversation_index_at_16() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("ScCreateConversationIndex@16"),
    )
}

/// `HrGetOmiProvidersFlags`: a `stub` export at ordinal 246 in `dlls/mapistub/mapistub.spec`.
pub fn hr_get_omi_providers_flags() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("HrGetOmiProvidersFlags"),
    )
}

/// `HrGetOmiProvidersFlags@8`: a `stub` export at ordinal 247 in `dlls/mapistub/mapistub.spec`.
pub fn hr_get_omi_providers_flags_at_8() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("HrGetOmiProvidersFlags@8"),
    )
}

/// `HrSetOmiProvidersFlagsInvalid`: a `stub` export at ordinal 248 in `dlls/mapistub/mapistub.spec`.
pub fn hr_set_omi_providers_flags_invalid() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("HrSetOmiProvidersFlagsInvalid"),
    )
}

/// `HrSetOmiProvidersFlagsInvalid@4`: a `stub` export at ordinal 249 in `dlls/mapistub/mapistub.spec`.
pub fn hr_set_omi_providers_flags_invalid_at_4() -> StubException {
    wine_spec_unimplemented_stub(
        MAPISTUB_DLL_NAME,
        StubFunction::Name("HrSetOmiProvidersFlagsInvalid@4"),
    )
}

/// `GetOutlookVersion`: a `stub` export at ordinal 250 in `dlls/mapistub/mapistub.spec`.
pub fn get_outlook_version() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("GetOutlookVersion"))
}

/// `GetOutlookVersion@0`: a `stub` export at ordinal 251 in `dlls/mapistub/mapistub.spec`.
pub fn get_outlook_version_at_0() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("GetOutlookVersion@0"))
}

/// `FixMAPI`: a `stub` export at ordinal 252 in `dlls/mapistub/mapistub.spec`.
pub fn fix_mapi() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("FixMAPI"))
}

/// `FixMAPI@0`: a `stub` export at ordinal 253 in `dlls/mapistub/mapistub.spec`.
pub fn fix_mapi_at_0() -> StubException {
    wine_spec_unimplemented_stub(MAPISTUB_DLL_NAME, StubFunction::Name("FixMAPI@0"))
}
