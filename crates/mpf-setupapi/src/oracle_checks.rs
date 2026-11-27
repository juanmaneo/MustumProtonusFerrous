//! Oracle-derived test vectors for the ported setupapi logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of the 22 fixed-value functions
//! (`dlls/setupapi/stubs.c`), `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`) and
//! `parse_buffer` (`dlls/setupapi/parser.c`) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/setupapi/`, and running the harness. ASan and UBSan reported nothing, so no
//! vector is excluded. The 335 rows are 22 fixed-value rows, 241 `@ stub` rows and 72 parse rows.
//! Rows 333 to 335 are long, so [`oracle_long_parse_rows`] builds their inputs instead of
//! listing them.

use super::*;

/// A `@ stub` helper of `stub.rs`.
type StubHelper = fn() -> StubException;

/// The `(name, helper)` pairs of the 241 `@ stub` rows, in spec order.
const STUB_ROWS: &[(&str, StubHelper)] = &[
    ("AcquireSCMLock", acquire_scm_lock),
    ("AddMiniIconToList", add_mini_icon_to_list),
    (
        "AddTagToGroupOrderListEntry",
        add_tag_to_group_order_list_entry,
    ),
    ("AppendStringToMultiSz", append_string_to_multi_sz),
    ("CMP_Init_Detection", cmp_init_detection),
    ("CMP_RegisterNotification", cmp_register_notification),
    ("CMP_Report_LogOn", cmp_report_log_on),
    ("CMP_UnregisterNotification", cmp_unregister_notification),
    ("CMP_WaitServices", cmp_wait_services),
    ("CM_Add_Empty_Log_Conf", cm_add_empty_log_conf),
    ("CM_Add_Empty_Log_Conf_Ex", cm_add_empty_log_conf_ex),
    ("CM_Add_IDA", cm_add_ida),
    ("CM_Add_IDW", cm_add_idw),
    ("CM_Add_ID_ExA", cm_add_id_ex_a),
    ("CM_Add_ID_ExW", cm_add_id_ex_w),
    ("CM_Add_Range", cm_add_range),
    ("CM_Add_Res_Des", cm_add_res_des),
    ("CM_Add_Res_Des_Ex", cm_add_res_des_ex),
    ("CM_Create_DevNode_ExA", cm_create_dev_node_ex_a),
    ("CM_Create_DevNode_ExW", cm_create_dev_node_ex_w),
    ("CM_Create_Range_List", cm_create_range_list),
    ("CM_Delete_Class_Key", cm_delete_class_key),
    ("CM_Delete_Class_Key_Ex", cm_delete_class_key_ex),
    ("CM_Delete_DevNode_Key", cm_delete_dev_node_key),
    ("CM_Delete_DevNode_Key_Ex", cm_delete_dev_node_key_ex),
    ("CM_Delete_Range", cm_delete_range),
    ("CM_Detect_Resource_Conflict", cm_detect_resource_conflict),
    (
        "CM_Detect_Resource_Conflict_Ex",
        cm_detect_resource_conflict_ex,
    ),
    ("CM_Disable_DevNode", cm_disable_dev_node),
    ("CM_Disable_DevNode_Ex", cm_disable_dev_node_ex),
    ("CM_Dup_Range_List", cm_dup_range_list),
    ("CM_Enable_DevNode", cm_enable_dev_node),
    ("CM_Enable_DevNode_Ex", cm_enable_dev_node_ex),
    ("CM_Find_Range", cm_find_range),
    ("CM_First_Range", cm_first_range),
    ("CM_Free_Log_Conf", cm_free_log_conf),
    ("CM_Free_Log_Conf_Ex", cm_free_log_conf_ex),
    ("CM_Free_Log_Conf_Handle", cm_free_log_conf_handle),
    ("CM_Free_Range_List", cm_free_range_list),
    ("CM_Free_Res_Des", cm_free_res_des),
    ("CM_Free_Res_Des_Ex", cm_free_res_des_ex),
    ("CM_Free_Res_Des_Handle", cm_free_res_des_handle),
    ("CM_Get_Class_NameA", cm_get_class_name_a),
    ("CM_Get_Class_NameW", cm_get_class_name_w),
    ("CM_Get_Class_Name_ExA", cm_get_class_name_ex_a),
    ("CM_Get_Class_Name_ExW", cm_get_class_name_ex_w),
    ("CM_Get_Depth", cm_get_depth),
    ("CM_Get_Depth_Ex", cm_get_depth_ex),
    (
        "CM_Get_Device_Interface_Alias_ExA",
        cm_get_device_interface_alias_ex_a,
    ),
    (
        "CM_Get_Device_Interface_Alias_ExW",
        cm_get_device_interface_alias_ex_w,
    ),
    ("CM_Get_First_Log_Conf", cm_get_first_log_conf),
    ("CM_Get_First_Log_Conf_Ex", cm_get_first_log_conf_ex),
    ("CM_Get_Global_State", cm_get_global_state),
    ("CM_Get_Global_State_Ex", cm_get_global_state_ex),
    ("CM_Get_HW_Prof_FlagsA", cm_get_hw_prof_flags_a),
    ("CM_Get_HW_Prof_FlagsW", cm_get_hw_prof_flags_w),
    ("CM_Get_HW_Prof_Flags_ExA", cm_get_hw_prof_flags_ex_a),
    ("CM_Get_HW_Prof_Flags_ExW", cm_get_hw_prof_flags_ex_w),
    (
        "CM_Get_Hardware_Profile_InfoA",
        cm_get_hardware_profile_info_a,
    ),
    (
        "CM_Get_Hardware_Profile_InfoW",
        cm_get_hardware_profile_info_w,
    ),
    (
        "CM_Get_Hardware_Profile_Info_ExA",
        cm_get_hardware_profile_info_ex_a,
    ),
    (
        "CM_Get_Hardware_Profile_Info_ExW",
        cm_get_hardware_profile_info_ex_w,
    ),
    ("CM_Get_Log_Conf_Priority", cm_get_log_conf_priority),
    ("CM_Get_Log_Conf_Priority_Ex", cm_get_log_conf_priority_ex),
    ("CM_Get_Next_Log_Conf", cm_get_next_log_conf),
    ("CM_Get_Next_Log_Conf_Ex", cm_get_next_log_conf_ex),
    ("CM_Get_Next_Res_Des", cm_get_next_res_des),
    ("CM_Get_Next_Res_Des_Ex", cm_get_next_res_des_ex),
    ("CM_Get_Res_Des_Data", cm_get_res_des_data),
    ("CM_Get_Res_Des_Data_Ex", cm_get_res_des_data_ex),
    ("CM_Get_Res_Des_Data_Size", cm_get_res_des_data_size),
    ("CM_Get_Res_Des_Data_Size_Ex", cm_get_res_des_data_size_ex),
    ("CM_Get_Version_Ex", cm_get_version_ex),
    ("CM_Intersect_Range_List", cm_intersect_range_list),
    ("CM_Invert_Range_List", cm_invert_range_list),
    ("CM_Is_Dock_Station_Present", cm_is_dock_station_present),
    ("CM_Merge_Range_List", cm_merge_range_list),
    ("CM_Modify_Res_Des", cm_modify_res_des),
    ("CM_Modify_Res_Des_Ex", cm_modify_res_des_ex),
    ("CM_Move_DevNode", cm_move_dev_node),
    ("CM_Move_DevNode_Ex", cm_move_dev_node_ex),
    ("CM_Next_Range", cm_next_range),
    (
        "CM_Query_And_Remove_SubTreeA",
        cm_query_and_remove_sub_tree_a,
    ),
    (
        "CM_Query_And_Remove_SubTreeW",
        cm_query_and_remove_sub_tree_w,
    ),
    (
        "CM_Query_And_Remove_SubTree_ExA",
        cm_query_and_remove_sub_tree_ex_a,
    ),
    (
        "CM_Query_And_Remove_SubTree_ExW",
        cm_query_and_remove_sub_tree_ex_w,
    ),
    (
        "CM_Query_Arbitrator_Free_Data",
        cm_query_arbitrator_free_data,
    ),
    (
        "CM_Query_Arbitrator_Free_Data_Ex",
        cm_query_arbitrator_free_data_ex,
    ),
    (
        "CM_Query_Arbitrator_Free_Size",
        cm_query_arbitrator_free_size,
    ),
    (
        "CM_Query_Arbitrator_Free_Size_Ex",
        cm_query_arbitrator_free_size_ex,
    ),
    ("CM_Query_Remove_SubTree", cm_query_remove_sub_tree),
    ("CM_Query_Remove_SubTree_Ex", cm_query_remove_sub_tree_ex),
    ("CM_Register_Device_Driver", cm_register_device_driver),
    ("CM_Register_Device_Driver_Ex", cm_register_device_driver_ex),
    (
        "CM_Register_Device_InterfaceA",
        cm_register_device_interface_a,
    ),
    (
        "CM_Register_Device_InterfaceW",
        cm_register_device_interface_w,
    ),
    (
        "CM_Register_Device_Interface_ExA",
        cm_register_device_interface_ex_a,
    ),
    (
        "CM_Register_Device_Interface_ExW",
        cm_register_device_interface_ex_w,
    ),
    ("CM_Remove_SubTree", cm_remove_sub_tree),
    ("CM_Remove_SubTree_Ex", cm_remove_sub_tree_ex),
    ("CM_Remove_Unmarked_Children", cm_remove_unmarked_children),
    (
        "CM_Remove_Unmarked_Children_Ex",
        cm_remove_unmarked_children_ex,
    ),
    ("CM_Request_Eject_PC", cm_request_eject_pc),
    ("CM_Reset_Children_Marks", cm_reset_children_marks),
    ("CM_Reset_Children_Marks_Ex", cm_reset_children_marks_ex),
    ("CM_Run_Detection", cm_run_detection),
    ("CM_Run_Detection_Ex", cm_run_detection_ex),
    ("CM_Set_DevNode_Problem", cm_set_dev_node_problem),
    ("CM_Set_DevNode_Problem_Ex", cm_set_dev_node_problem_ex),
    (
        "CM_Set_DevNode_Registry_PropertyA",
        cm_set_dev_node_registry_property_a,
    ),
    (
        "CM_Set_DevNode_Registry_PropertyW",
        cm_set_dev_node_registry_property_w,
    ),
    (
        "CM_Set_DevNode_Registry_Property_ExA",
        cm_set_dev_node_registry_property_ex_a,
    ),
    (
        "CM_Set_DevNode_Registry_Property_ExW",
        cm_set_dev_node_registry_property_ex_w,
    ),
    ("CM_Set_HW_Prof", cm_set_hw_prof),
    ("CM_Set_HW_Prof_Ex", cm_set_hw_prof_ex),
    ("CM_Set_HW_Prof_FlagsA", cm_set_hw_prof_flags_a),
    ("CM_Set_HW_Prof_FlagsW", cm_set_hw_prof_flags_w),
    ("CM_Set_HW_Prof_Flags_ExA", cm_set_hw_prof_flags_ex_a),
    ("CM_Set_HW_Prof_Flags_ExW", cm_set_hw_prof_flags_ex_w),
    ("CM_Setup_DevNode", cm_setup_dev_node),
    ("CM_Setup_DevNode_Ex", cm_setup_dev_node_ex),
    ("CM_Test_Range_Available", cm_test_range_available),
    ("CM_Uninstall_DevNode", cm_uninstall_dev_node),
    ("CM_Uninstall_DevNode_Ex", cm_uninstall_dev_node_ex),
    (
        "CM_Unregister_Device_InterfaceA",
        cm_unregister_device_interface_a,
    ),
    (
        "CM_Unregister_Device_InterfaceW",
        cm_unregister_device_interface_w,
    ),
    (
        "CM_Unregister_Device_Interface_ExA",
        cm_unregister_device_interface_ex_a,
    ),
    (
        "CM_Unregister_Device_Interface_ExW",
        cm_unregister_device_interface_ex_w,
    ),
    (
        "CenterWindowRelativeToParent",
        center_window_relative_to_parent,
    ),
    ("ConcatenatePaths", concatenate_paths),
    ("DelimStringToMultiSz", delim_string_to_multi_sz),
    ("DestroyTextFileReadBuffer", destroy_text_file_read_buffer),
    (
        "DriverStoreEnumDriverPackageA",
        driver_store_enum_driver_package_a,
    ),
    (
        "DriverStoreEnumDriverPackageW",
        driver_store_enum_driver_package_w,
    ),
    ("ExtensionPropSheetPageProc", extension_prop_sheet_page_proc),
    ("FreeStringArray", free_string_array),
    (
        "GetCurrentDriverSigningPolicy",
        get_current_driver_signing_policy,
    ),
    ("GetNewInfName", get_new_inf_name),
    ("GetSetFileTimestamp", get_set_file_timestamp),
    ("GetVersionInfoFromImage", get_version_info_from_image),
    ("InfIsFromOemLocation", inf_is_from_oem_location),
    ("InstallStop", install_stop),
    ("InstallStopEx", install_stop_ex),
    ("LookUpStringInTable", look_up_string_in_table),
    ("MemoryInitialize", memory_initialize),
    ("MultiSzFromSearchControl", multi_sz_from_search_control),
    ("MyGetFileTitle", my_get_file_title),
    ("OutOfMemory", out_of_memory),
    ("QueryMultiSzValueToArray", query_multi_sz_value_to_array),
    (
        "ReadAsciiOrUnicodeTextFile",
        read_ascii_or_unicode_text_file,
    ),
    ("RetrieveServiceConfig", retrieve_service_config),
    ("SearchForInfFile", search_for_inf_file),
    ("SetArrayToMultiSzValue", set_array_to_multi_sz_value),
    (
        "SetupAddInstallSectionToDiskSpaceListW",
        setup_add_install_section_to_disk_space_list_w,
    ),
    (
        "SetupAddSectionToDiskSpaceListA",
        setup_add_section_to_disk_space_list_a,
    ),
    (
        "SetupAddSectionToDiskSpaceListW",
        setup_add_section_to_disk_space_list_w,
    ),
    ("SetupAdjustDiskSpaceListA", setup_adjust_disk_space_list_a),
    ("SetupAdjustDiskSpaceListW", setup_adjust_disk_space_list_w),
    (
        "SetupCancelTemporarySourceList",
        setup_cancel_temporary_source_list,
    ),
    ("SetupDefaultQueueCallback", setup_default_queue_callback),
    ("SetupDiAskForOEMDisk", setup_di_ask_for_oem_disk),
    (
        "SetupDiCancelDriverInfoSearch",
        setup_di_cancel_driver_info_search,
    ),
    ("SetupDiChangeState", setup_di_change_state),
    ("SetupDiDeleteDeviceRegKey", setup_di_delete_device_reg_key),
    (
        "SetupDiGetClassDevPropertySheetsA",
        setup_di_get_class_dev_property_sheets_a,
    ),
    (
        "SetupDiGetClassDevPropertySheetsW",
        setup_di_get_class_dev_property_sheets_w,
    ),
    (
        "SetupDiGetClassImageListExA",
        setup_di_get_class_image_list_ex_a,
    ),
    (
        "SetupDiGetClassImageListExW",
        setup_di_get_class_image_list_ex_w,
    ),
    (
        "SetupDiGetClassInstallParamsA",
        setup_di_get_class_install_params_a,
    ),
    (
        "SetupDiGetClassInstallParamsW",
        setup_di_get_class_install_params_w,
    ),
    (
        "SetupDiGetDeviceInfoListClass",
        setup_di_get_device_info_list_class,
    ),
    (
        "SetupDiGetDeviceInterfaceAlias",
        setup_di_get_device_interface_alias,
    ),
    (
        "SetupDiGetDriverInstallParamsA",
        setup_di_get_driver_install_params_a,
    ),
    (
        "SetupDiGetDriverInstallParamsW",
        setup_di_get_driver_install_params_w,
    ),
    (
        "SetupDiGetHwProfileFriendlyNameA",
        setup_di_get_hw_profile_friendly_name_a,
    ),
    (
        "SetupDiGetHwProfileFriendlyNameExA",
        setup_di_get_hw_profile_friendly_name_ex_a,
    ),
    (
        "SetupDiGetHwProfileFriendlyNameExW",
        setup_di_get_hw_profile_friendly_name_ex_w,
    ),
    (
        "SetupDiGetHwProfileFriendlyNameW",
        setup_di_get_hw_profile_friendly_name_w,
    ),
    ("SetupDiGetHwProfileList", setup_di_get_hw_profile_list),
    (
        "SetupDiGetHwProfileListExA",
        setup_di_get_hw_profile_list_ex_a,
    ),
    (
        "SetupDiGetHwProfileListExW",
        setup_di_get_hw_profile_list_ex_w,
    ),
    ("SetupDiGetSelectedDevice", setup_di_get_selected_device),
    ("SetupDiGetWizardPage", setup_di_get_wizard_page),
    ("SetupDiInstallClassExA", setup_di_install_class_ex_a),
    ("SetupDiInstallClassExW", setup_di_install_class_ex_w),
    ("SetupDiMoveDuplicateDevice", setup_di_move_duplicate_device),
    ("SetupDiSelectDevice", setup_di_select_device),
    ("SetupDiSelectOEMDrv", setup_di_select_oem_drv),
    (
        "SetupDiSetDriverInstallParamsA",
        setup_di_set_driver_install_params_a,
    ),
    (
        "SetupDiSetDriverInstallParamsW",
        setup_di_set_driver_install_params_w,
    ),
    ("SetupDiSetSelectedDriverA", setup_di_set_selected_driver_a),
    ("SetupDiSetSelectedDriverW", setup_di_set_selected_driver_w),
    ("SetupDiUnremoveDevice", setup_di_unremove_device),
    ("SetupFreeSourceListA", setup_free_source_list_a),
    ("SetupFreeSourceListW", setup_free_source_list_w),
    ("SetupGetBackupInformationA", setup_get_backup_information_a),
    ("SetupGetBackupInformationW", setup_get_backup_information_w),
    ("SetupGetInfSections", setup_get_inf_sections),
    ("SetupGetSourceFileSizeA", setup_get_source_file_size_a),
    ("SetupGetSourceFileSizeW", setup_get_source_file_size_w),
    (
        "SetupInstallServicesFromInfSectionExA",
        setup_install_services_from_inf_section_ex_a,
    ),
    (
        "SetupInstallServicesFromInfSectionExW",
        setup_install_services_from_inf_section_ex_w,
    ),
    ("SetupQueryFileLogA", setup_query_file_log_a),
    ("SetupQueryFileLogW", setup_query_file_log_w),
    ("SetupQuerySourceListA", setup_query_source_list_a),
    ("SetupQuerySourceListW", setup_query_source_list_w),
    ("SetupRemoveFileLogEntryA", setup_remove_file_log_entry_a),
    ("SetupRemoveFileLogEntryW", setup_remove_file_log_entry_w),
    (
        "SetupRemoveFromDiskSpaceListA",
        setup_remove_from_disk_space_list_a,
    ),
    (
        "SetupRemoveFromDiskSpaceListW",
        setup_remove_from_disk_space_list_w,
    ),
    (
        "SetupRemoveFromSourceListA",
        setup_remove_from_source_list_a,
    ),
    (
        "SetupRemoveFromSourceListW",
        setup_remove_from_source_list_w,
    ),
    (
        "SetupRemoveInstallSectionFromDiskSpaceListA",
        setup_remove_install_section_from_disk_space_list_a,
    ),
    (
        "SetupRemoveInstallSectionFromDiskSpaceListW",
        setup_remove_install_section_from_disk_space_list_w,
    ),
    (
        "SetupRemoveSectionFromDiskSpaceListA",
        setup_remove_section_from_disk_space_list_a,
    ),
    (
        "SetupRemoveSectionFromDiskSpaceListW",
        setup_remove_section_from_disk_space_list_w,
    ),
    ("SetupScanFileQueue", setup_scan_file_queue),
    ("SetupSetDirectoryIdExA", setup_set_directory_id_ex_a),
    ("SetupSetDirectoryIdExW", setup_set_directory_id_ex_w),
    (
        "SetupSetPlatformPathOverrideA",
        setup_set_platform_path_override_a,
    ),
    (
        "SetupSetPlatformPathOverrideW",
        setup_set_platform_path_override_w,
    ),
    ("ShouldDeviceBeExcluded", should_device_be_excluded),
    ("StringTableEnum", string_table_enum),
    ("VerifyCatalogFile", verify_catalog_file),
    ("VerifyFile", verify_file),
    (
        "pSetupAccessRunOnceNodeList",
        p_setup_access_run_once_node_list,
    ),
    ("pSetupAddMiniIconToList", p_setup_add_mini_icon_to_list),
    (
        "pSetupAddTagToGroupOrderListEntry",
        p_setup_add_tag_to_group_order_list_entry,
    ),
    (
        "pSetupAppendStringToMultiSz",
        p_setup_append_string_to_multi_sz,
    ),
    (
        "pSetupDestroyRunOnceNodeList",
        p_setup_destroy_run_once_node_list,
    ),
    ("pSetupDirectoryIdToPath", p_setup_directory_id_to_path),
    (
        "pSetupGetOsLoaderDriveAndPath",
        p_setup_get_os_loader_drive_and_path,
    ),
    ("pSetupGetVersionDatum", p_setup_get_version_datum),
    ("pSetupGuidFromString", p_setup_guid_from_string),
    ("pSetupIsGuidNull", p_setup_is_guid_null),
    ("pSetupMakeSurePathExists", p_setup_make_sure_path_exists),
    (
        "pSetupSetSystemSourceFlags",
        p_setup_set_system_source_flags,
    ),
    ("pSetupStringFromGuid", p_setup_string_from_guid),
    ("pSetupStringTableEnum", p_setup_string_table_enum),
    ("pSetupVerifyCatalogFile", p_setup_verify_catalog_file),
    ("pSetupVerifyQueuedCatalogs", p_setup_verify_queued_catalogs),
];

/// Rows 264 to 332: the input and the dump of each `PARSE` row, as Rust string literal bodies.
const PARSE_ROWS: &[(&str, &str)] = &[
    ("", "ok strings=none"),
    ("[Strings]\u{a}", "ok strings=0\n[Strings]"),
    ("[A]\u{a}[Strings]\u{a}", "ok strings=1\n[A]\n[Strings]"),
    (
        "[a]\u{a}k=v\u{a}[A]\u{a}k2=v2\u{a}",
        "ok strings=none\n[a]\n{k|v}\n{k2|v2}",
    ),
    (
        "[Version]\u{a}Signature=\"$CHICAGO$\"\u{a}",
        "ok strings=none\n[Version]\n{Signature|$CHICAGO$}",
    ),
    ("[A]\u{a}k=v\u{a}", "ok strings=none\n[A]\n{k|v}"),
    (
        "[A]\u{d}\u{a}k = v \u{d}\u{a}",
        "ok strings=none\n[A]\n{k|v}",
    ),
    ("[A]\u{a}k=v1,v2\u{a}", "ok strings=none\n[A]\n{k|v1|v2}"),
    ("[A]\u{a}k,v\u{a}", "ok strings=none\n[A]\n{-|k|v}"),
    (
        "[A]\u{a}single\u{a}",
        "ok strings=none\n[A]\n{single|single}",
    ),
    ("[A]\u{a}=v\u{a}", "ok strings=none\n[A]\n{|v}"),
    ("[A]\u{a}k=\u{a}", "ok strings=none\n[A]\n{k|}"),
    ("[A]\u{a}k=a,\u{a}", "ok strings=none\n[A]\n{k|a|}"),
    ("[A]\u{a};comment\u{a}  ; x\u{a}", "ok strings=none\n[A]"),
    ("[A] ; c\u{a}k ;c\u{a}", "ok strings=none\n[A]\n{k|k}"),
    ("[]\u{a}", "ok strings=none\n[]"),
    ("[ A ]\u{a}", "ok strings=none\n[ A ]"),
    (
        "[A]\u{a}k=v\u{a}[B]\u{a}k2=v2",
        "ok strings=none\n[A]\n{k|v}\n[B]\n{k2|v2}",
    ),
    ("x=1\u{a}", "err code=0xe0000000 line=1"),
    ("a\u{a}[S]\u{a}k=v", "err code=0xe0000000 line=1"),
    ("x\u{a}[Strings]\u{a}", "ok strings=0\n[Strings]"),
    ("x\u{a}[A]\u{a}k=v", "err code=0xe0000000 line=1"),
    ("[abc", "err code=0xe0000001 line=1"),
    ("[abc\u{a}]", "err code=0xe0000001 line=1"),
    ("[", "err code=0xe0000001 line=1"),
    ("[ab]cd\u{a}", "ok strings=none\n[ab]"),
    (
        "[A]\u{a}k=\"a\"\"b\"\u{a}",
        "ok strings=none\n[A]\n{k|a\"b}",
    ),
    ("[A]\u{a}k=\"a;b\"\u{a}", "ok strings=none\n[A]\n{k|a;b}"),
    ("[A]\u{a}k=\"abc", "ok strings=none\n[A]\n{k|abc}"),
    (
        "[A]\u{a}k=\"abc\u{a}d",
        "ok strings=none\n[A]\n{k|abc}\n{d|d}",
    ),
    ("[A]\u{a}k=\"a\"x\u{a}", "ok strings=none\n[A]\n{k|ax}"),
    ("[A]\u{a}k=\"", "ok strings=none\n[A]\n{k|}"),
    ("[A]\u{a}k=\"\"\u{a}", "ok strings=none\n[A]\n{k|}"),
    ("[A]\u{a}k=\"x\"\"\"\u{a}", "ok strings=none\n[A]\n{k|x\"}"),
    (
        "[A]\u{a}k=\"x\\\"y\"\u{a}",
        "ok strings=none\n[A]\n{k|x\\y}",
    ),
    ("[A]\u{a}\"a b\"=c\u{a}", "ok strings=none\n[A]\n{a b|c}"),
    ("[A]\u{a}ab\"cd\"=e\u{a}", "ok strings=none\n[A]\n{abcd|e}"),
    ("[A]\u{a}a  =b\u{a}", "ok strings=none\n[A]\n{a|b}"),
    ("[A]\u{a}k]x=y\u{a}", "ok strings=none\n[A]\n{k]x|y}"),
    ("[A]\u{a}k\\0x=v\u{a}", "ok strings=none\n[A]\n{k\\0x|v}"),
    ("[A]\u{a}k=a\\0b\u{a}", "ok strings=none\n[A]\n{k|a\\0b}"),
    ("[A]\u{a}k=   v\u{a}", "ok strings=none\n[A]\n{k|v}"),
    (
        "[A]\u{a}\u{9}k\u{9}=\u{9}v\u{9}\u{a}",
        "ok strings=none\n[A]\n{k|v}",
    ),
    ("[A]\u{a}k=a\\\u{a}b\u{a}", "ok strings=none\n[A]\n{k|ab}"),
    ("[A]\u{a}k\\\u{a}v=w\u{a}", "ok strings=none\n[A]\n{kv|w}"),
    ("[A]\u{a}k=a\\b\u{a}", "ok strings=none\n[A]\n{k|a\\b}"),
    ("[A]\u{a}k=a\\\\\u{a}", "ok strings=none\n[A]\n{k|a}"),
    ("[A]\u{a}k=a\\;c\u{a}", "ok strings=none\n[A]\n{k|a}"),
    ("[A]\u{a}k=a \\\u{a}b\u{a}", "ok strings=none\n[A]\n{k|ab}"),
    ("[A]\u{a}k=v ; c\u{a}", "ok strings=none\n[A]\n{k|v}"),
    ("[A]\u{a}k=a=b\u{a}", "ok strings=none\n[A]\n{k|a=b}"),
    ("[A]\u{a}k=\"a\" b\u{a}", "ok strings=none\n[A]\n{k|a b}"),
    ("[A]\u{a}k\\", "ok strings=none\n[A]\n{k|k}"),
    ("[A]\u{a}k=v\\", "ok strings=none\n[A]\n{k|v}"),
    ("[A]\u{a}k=v\u{1a}[B]\u{a}", "ok strings=none\n[A]\n{k|v}"),
    (
        "[A]\u{a}k=v\u{a}\u{1a}junk\u{a}",
        "ok strings=none\n[A]\n{k|v}",
    ),
    ("[A]\u{a}k=a\u{1}b\u{a}", "ok strings=none\n[A]\n{k|a?b}"),
    ("[A]\u{a}a\u{7f}b=c\u{a}", "ok strings=none\n[A]\n{a?b|c}"),
    (
        "[A]\u{a}k=\"a\u{9}b\"\u{a}",
        "ok strings=none\n[A]\n{k|a?b}",
    ),
    ("[A]\u{a}k=v\u{d}\u{a}", "ok strings=none\n[A]\n{k|v}"),
    ("\u{a}\u{a}[A]\u{a}", "ok strings=none\n[A]"),
    ("  ; only comment\u{a}", "ok strings=none"),
    ("a\u{a}b\u{a}[S]\u{a}k=v\u{a}", "err code=0xe0000000 line=1"),
    ("[A]\u{a}k=\\\u{a}v\u{a}", "ok strings=none\n[A]\n{k|v}"),
    ("[A]\u{a}k=a\\ b\u{a}", "ok strings=none\n[A]\n{k|a\\ b}"),
    ("[A]\u{a}k \\\u{a}v=w\u{a}", "ok strings=none\n[A]\n{kv|w}"),
    ("[A]\u{a}k\u{0}v\u{a}", "ok strings=none\n[A]\n{k v|k v}"),
    ("[A]\u{a}k=a\u{0}b\u{a}", "ok strings=none\n[A]\n{k|a b}"),
    ("[A]\u{a}k=a\u{0}\u{a}", "ok strings=none\n[A]\n{k|a}"),
];

/// Encodes text as the UTF-16 buffer that `parse_buffer` reads.
fn utf16(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

/// Prints a name or field as `put_units` in the harness does: printable ASCII as is, and any
/// other unit as `?`.
fn units_text(units: &[u16]) -> String {
    units
        .iter()
        .map(|&c| {
            if (0x20..=0x7e).contains(&c) {
                char::from(c as u8)
            } else {
                '?'
            }
        })
        .collect()
}

/// Renders a parse result as `parse_and_dump` in the harness does.
fn render(units: &[u16]) -> String {
    let file = match parse_inf_buffer(units) {
        Ok(file) => file,
        Err(err) => return format!("err code=0x{:x} line={}", err.code, err.line),
    };
    let mut out = match file.strings_section {
        Some(index) => format!("ok strings={index}"),
        None => "ok strings=none".to_string(),
    };
    for section in &file.sections {
        out.push_str(&format!("\n[{}]", units_text(&section.name)));
        for line in &section.lines {
            let key = match line.key_field {
                Some(field) => units_text(&file.fields[field]),
                None => "-".to_string(),
            };
            out.push_str(&format!("\n{{{key}"));
            for field in 0..line.nb_fields {
                out.push_str(&format!(
                    "|{}",
                    units_text(&file.fields[line.first_field + field])
                ));
            }
            out.push('}');
        }
    }
    out
}

#[test]
fn oracle_stub_rows() {
    for &(name, helper) in STUB_ROWS {
        assert_eq!(
            helper(),
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "setupapi.dll",
                function: name,
            },
            "row for {name}"
        );
    }
}

#[test]
fn oracle_fixed_rows() {
    // row 01 SetupInitializeFileLogW ret=-1 last_error=none
    assert_eq!(
        setup_initialize_file_log_w(),
        FixedReturn {
            value: -1,
            last_error: None
        }
    );
    // row 02 SetupInitializeFileLogA ret=-1 last_error=none
    assert_eq!(
        setup_initialize_file_log_a(),
        FixedReturn {
            value: -1,
            last_error: None
        }
    );
    // row 03 SetupTerminateFileLog ret=1 last_error=none
    assert_eq!(
        setup_terminate_file_log(),
        FixedReturn {
            value: true,
            last_error: None
        }
    );
    // row 04 RegistryDelnode ret=0 last_error=none
    assert_eq!(
        registry_delnode(),
        FixedReturn {
            value: false,
            last_error: None
        }
    );
    // row 05 SetupPromptReboot ret=0 last_error=none
    assert_eq!(
        setup_prompt_reboot(),
        FixedReturn {
            value: 0,
            last_error: None
        }
    );
    // row 06 SetupQueryDrivesInDiskSpaceListA ret=0 last_error=none
    assert_eq!(
        setup_query_drives_in_disk_space_list_a(),
        FixedReturn {
            value: false,
            last_error: None
        }
    );
    // row 07 SetupQueryDrivesInDiskSpaceListW ret=0 last_error=none
    assert_eq!(
        setup_query_drives_in_disk_space_list_w(),
        FixedReturn {
            value: false,
            last_error: None
        }
    );
    // row 08 SetupAddToSourceListA ret=1 last_error=none
    assert_eq!(
        setup_add_to_source_list_a(),
        FixedReturn {
            value: true,
            last_error: None
        }
    );
    // row 09 SetupAddToSourceListW ret=1 last_error=none
    assert_eq!(
        setup_add_to_source_list_w(),
        FixedReturn {
            value: true,
            last_error: None
        }
    );
    // row 10 SetupSetSourceListA ret=0 last_error=none
    assert_eq!(
        setup_set_source_list_a(),
        FixedReturn {
            value: false,
            last_error: None
        }
    );
    // row 11 SetupSetSourceListW ret=0 last_error=none
    assert_eq!(
        setup_set_source_list_w(),
        FixedReturn {
            value: false,
            last_error: None
        }
    );
    // row 12 SetupDiDestroyClassImageList ret=1 last_error=none
    assert_eq!(
        setup_di_destroy_class_image_list(),
        FixedReturn {
            value: true,
            last_error: None
        }
    );
    // row 13 SetupDiGetClassImageList ret=0 last_error=none
    assert_eq!(
        setup_di_get_class_image_list(),
        FixedReturn {
            value: false,
            last_error: None
        }
    );
    // row 14 SetupDiGetClassImageIndex ret=0 last_error=none
    assert_eq!(
        setup_di_get_class_image_index(),
        FixedReturn {
            value: false,
            last_error: None
        }
    );
    // row 15 SetupLogFileW ret=0 last_error=120
    assert_eq!(
        setup_log_file_w(),
        FixedReturn {
            value: false,
            last_error: Some(120)
        }
    );
    // row 16 SetupLogFileA ret=0 last_error=120
    assert_eq!(
        setup_log_file_a(),
        FixedReturn {
            value: false,
            last_error: Some(120)
        }
    );
    // row 17 SetupDiDestroyDriverInfoList ret=0 last_error=120
    assert_eq!(
        setup_di_destroy_driver_info_list(),
        FixedReturn {
            value: false,
            last_error: Some(120)
        }
    );
    // row 18 SetupDiDrawMiniIcon ret=0 last_error=120
    assert_eq!(
        setup_di_draw_mini_icon(),
        FixedReturn {
            value: 0,
            last_error: Some(120)
        }
    );
    // row 19 SetupDiGetClassBitmapIndex ret=0 last_error=120
    assert_eq!(
        setup_di_get_class_bitmap_index(),
        FixedReturn {
            value: false,
            last_error: Some(120)
        }
    );
    // row 20 SetupDiLoadClassIcon ret=0 last_error=120
    assert_eq!(
        setup_di_load_class_icon(),
        FixedReturn {
            value: false,
            last_error: Some(120)
        }
    );
    // row 21 SetupDiSetSelectedDevice ret=1 last_error=none
    assert_eq!(
        setup_di_set_selected_device(),
        FixedReturn {
            value: true,
            last_error: None
        }
    );
    // row 22 SetupDiGetClassRegistryPropertyW ret=0 last_error=120
    assert_eq!(
        setup_di_get_class_registry_property_w(),
        FixedReturn {
            value: false,
            last_error: Some(120)
        }
    );
}

#[test]
fn oracle_parse_rows() {
    for (row, &(input, dump)) in (264..).zip(PARSE_ROWS) {
        assert_eq!(render(&utf16(input)), dump, "row {row}");
    }
}

#[test]
fn oracle_long_parse_rows() {
    // Row 333: a 607-unit field is kept as its first 511 units.
    assert_eq!(
        render(&utf16(&format!("[A]\nk={}\n", "a".repeat(607)))),
        format!("ok strings=none\n[A]\n{{k|{}}}", "a".repeat(511))
    );
    // Row 334: a 259-unit section name is rejected on line 1.
    assert_eq!(
        render(&utf16(&format!("[{}]\n", "a".repeat(259)))),
        "err code=0xe0000002 line=1"
    );
    // Row 335: a 255-unit section name is the longest one accepted.
    assert_eq!(
        render(&utf16(&format!("[{}]\n", "a".repeat(255)))),
        format!("ok strings=none\n[{}]", "a".repeat(255))
    );
}
