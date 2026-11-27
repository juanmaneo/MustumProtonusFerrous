/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = setupapi.dll` in
/// `dlls/setupapi/Makefile.in`.
const SETUPAPI_DLL_NAME: &str = "setupapi.dll";

/// The record that `__wine_spec_unimplemented_stub` raises.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`. The C function raises
/// the record in a loop and never returns, so the port returns the record instead.
pub fn wine_spec_unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        // The module and the function name.
        number_parameters: 2,
        module,
        function,
    }
}

/// `AcquireSCMLock`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn acquire_scm_lock() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "AcquireSCMLock")
}

/// `AddMiniIconToList`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn add_mini_icon_to_list() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "AddMiniIconToList")
}

/// `AddTagToGroupOrderListEntry`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn add_tag_to_group_order_list_entry() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "AddTagToGroupOrderListEntry")
}

/// `AppendStringToMultiSz`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn append_string_to_multi_sz() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "AppendStringToMultiSz")
}

/// `CMP_Init_Detection`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cmp_init_detection() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CMP_Init_Detection")
}

/// `CMP_RegisterNotification`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cmp_register_notification() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CMP_RegisterNotification")
}

/// `CMP_Report_LogOn`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cmp_report_log_on() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CMP_Report_LogOn")
}

/// `CMP_UnregisterNotification`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cmp_unregister_notification() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CMP_UnregisterNotification")
}

/// `CMP_WaitServices`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cmp_wait_services() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CMP_WaitServices")
}

/// `CM_Add_Empty_Log_Conf`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_add_empty_log_conf() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Add_Empty_Log_Conf")
}

/// `CM_Add_Empty_Log_Conf_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_add_empty_log_conf_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Add_Empty_Log_Conf_Ex")
}

/// `CM_Add_IDA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_add_ida() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Add_IDA")
}

/// `CM_Add_IDW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_add_idw() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Add_IDW")
}

/// `CM_Add_ID_ExA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_add_id_ex_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Add_ID_ExA")
}

/// `CM_Add_ID_ExW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_add_id_ex_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Add_ID_ExW")
}

/// `CM_Add_Range`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_add_range() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Add_Range")
}

/// `CM_Add_Res_Des`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_add_res_des() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Add_Res_Des")
}

/// `CM_Add_Res_Des_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_add_res_des_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Add_Res_Des_Ex")
}

/// `CM_Create_DevNode_ExA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_create_dev_node_ex_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Create_DevNode_ExA")
}

/// `CM_Create_DevNode_ExW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_create_dev_node_ex_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Create_DevNode_ExW")
}

/// `CM_Create_Range_List`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_create_range_list() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Create_Range_List")
}

/// `CM_Delete_Class_Key`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_delete_class_key() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Delete_Class_Key")
}

/// `CM_Delete_Class_Key_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_delete_class_key_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Delete_Class_Key_Ex")
}

/// `CM_Delete_DevNode_Key`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_delete_dev_node_key() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Delete_DevNode_Key")
}

/// `CM_Delete_DevNode_Key_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_delete_dev_node_key_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Delete_DevNode_Key_Ex")
}

/// `CM_Delete_Range`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_delete_range() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Delete_Range")
}

/// `CM_Detect_Resource_Conflict`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_detect_resource_conflict() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Detect_Resource_Conflict")
}

/// `CM_Detect_Resource_Conflict_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_detect_resource_conflict_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Detect_Resource_Conflict_Ex")
}

/// `CM_Disable_DevNode`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_disable_dev_node() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Disable_DevNode")
}

/// `CM_Disable_DevNode_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_disable_dev_node_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Disable_DevNode_Ex")
}

/// `CM_Dup_Range_List`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_dup_range_list() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Dup_Range_List")
}

/// `CM_Enable_DevNode`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_enable_dev_node() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Enable_DevNode")
}

/// `CM_Enable_DevNode_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_enable_dev_node_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Enable_DevNode_Ex")
}

/// `CM_Find_Range`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_find_range() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Find_Range")
}

/// `CM_First_Range`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_first_range() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_First_Range")
}

/// `CM_Free_Log_Conf`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_free_log_conf() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Free_Log_Conf")
}

/// `CM_Free_Log_Conf_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_free_log_conf_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Free_Log_Conf_Ex")
}

/// `CM_Free_Log_Conf_Handle`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_free_log_conf_handle() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Free_Log_Conf_Handle")
}

/// `CM_Free_Range_List`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_free_range_list() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Free_Range_List")
}

/// `CM_Free_Res_Des`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_free_res_des() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Free_Res_Des")
}

/// `CM_Free_Res_Des_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_free_res_des_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Free_Res_Des_Ex")
}

/// `CM_Free_Res_Des_Handle`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_free_res_des_handle() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Free_Res_Des_Handle")
}

/// `CM_Get_Class_NameA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_class_name_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_Class_NameA")
}

/// `CM_Get_Class_NameW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_class_name_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_Class_NameW")
}

/// `CM_Get_Class_Name_ExA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_class_name_ex_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_Class_Name_ExA")
}

/// `CM_Get_Class_Name_ExW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_class_name_ex_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_Class_Name_ExW")
}

/// `CM_Get_Depth`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_depth() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_Depth")
}

/// `CM_Get_Depth_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_depth_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_Depth_Ex")
}

/// `CM_Get_Device_Interface_Alias_ExA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_device_interface_alias_ex_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_Device_Interface_Alias_ExA")
}

/// `CM_Get_Device_Interface_Alias_ExW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_device_interface_alias_ex_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_Device_Interface_Alias_ExW")
}

/// `CM_Get_First_Log_Conf`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_first_log_conf() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_First_Log_Conf")
}

/// `CM_Get_First_Log_Conf_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_first_log_conf_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_First_Log_Conf_Ex")
}

/// `CM_Get_Global_State`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_global_state() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_Global_State")
}

/// `CM_Get_Global_State_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_global_state_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_Global_State_Ex")
}

/// `CM_Get_HW_Prof_FlagsA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_hw_prof_flags_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_HW_Prof_FlagsA")
}

/// `CM_Get_HW_Prof_FlagsW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_hw_prof_flags_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_HW_Prof_FlagsW")
}

/// `CM_Get_HW_Prof_Flags_ExA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_hw_prof_flags_ex_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_HW_Prof_Flags_ExA")
}

/// `CM_Get_HW_Prof_Flags_ExW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_hw_prof_flags_ex_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_HW_Prof_Flags_ExW")
}

/// `CM_Get_Hardware_Profile_InfoA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_hardware_profile_info_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_Hardware_Profile_InfoA")
}

/// `CM_Get_Hardware_Profile_InfoW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_hardware_profile_info_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_Hardware_Profile_InfoW")
}

/// `CM_Get_Hardware_Profile_Info_ExA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_hardware_profile_info_ex_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_Hardware_Profile_Info_ExA")
}

/// `CM_Get_Hardware_Profile_Info_ExW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_hardware_profile_info_ex_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_Hardware_Profile_Info_ExW")
}

/// `CM_Get_Log_Conf_Priority`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_log_conf_priority() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_Log_Conf_Priority")
}

/// `CM_Get_Log_Conf_Priority_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_log_conf_priority_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_Log_Conf_Priority_Ex")
}

/// `CM_Get_Next_Log_Conf`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_next_log_conf() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_Next_Log_Conf")
}

/// `CM_Get_Next_Log_Conf_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_next_log_conf_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_Next_Log_Conf_Ex")
}

/// `CM_Get_Next_Res_Des`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_next_res_des() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_Next_Res_Des")
}

/// `CM_Get_Next_Res_Des_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_next_res_des_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_Next_Res_Des_Ex")
}

/// `CM_Get_Res_Des_Data`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_res_des_data() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_Res_Des_Data")
}

/// `CM_Get_Res_Des_Data_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_res_des_data_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_Res_Des_Data_Ex")
}

/// `CM_Get_Res_Des_Data_Size`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_res_des_data_size() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_Res_Des_Data_Size")
}

/// `CM_Get_Res_Des_Data_Size_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_res_des_data_size_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_Res_Des_Data_Size_Ex")
}

/// `CM_Get_Version_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_get_version_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Get_Version_Ex")
}

/// `CM_Intersect_Range_List`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_intersect_range_list() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Intersect_Range_List")
}

/// `CM_Invert_Range_List`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_invert_range_list() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Invert_Range_List")
}

/// `CM_Is_Dock_Station_Present`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_is_dock_station_present() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Is_Dock_Station_Present")
}

/// `CM_Merge_Range_List`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_merge_range_list() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Merge_Range_List")
}

/// `CM_Modify_Res_Des`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_modify_res_des() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Modify_Res_Des")
}

/// `CM_Modify_Res_Des_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_modify_res_des_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Modify_Res_Des_Ex")
}

/// `CM_Move_DevNode`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_move_dev_node() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Move_DevNode")
}

/// `CM_Move_DevNode_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_move_dev_node_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Move_DevNode_Ex")
}

/// `CM_Next_Range`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_next_range() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Next_Range")
}

/// `CM_Query_And_Remove_SubTreeA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_query_and_remove_sub_tree_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Query_And_Remove_SubTreeA")
}

/// `CM_Query_And_Remove_SubTreeW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_query_and_remove_sub_tree_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Query_And_Remove_SubTreeW")
}

/// `CM_Query_And_Remove_SubTree_ExA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_query_and_remove_sub_tree_ex_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Query_And_Remove_SubTree_ExA")
}

/// `CM_Query_And_Remove_SubTree_ExW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_query_and_remove_sub_tree_ex_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Query_And_Remove_SubTree_ExW")
}

/// `CM_Query_Arbitrator_Free_Data`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_query_arbitrator_free_data() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Query_Arbitrator_Free_Data")
}

/// `CM_Query_Arbitrator_Free_Data_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_query_arbitrator_free_data_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Query_Arbitrator_Free_Data_Ex")
}

/// `CM_Query_Arbitrator_Free_Size`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_query_arbitrator_free_size() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Query_Arbitrator_Free_Size")
}

/// `CM_Query_Arbitrator_Free_Size_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_query_arbitrator_free_size_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Query_Arbitrator_Free_Size_Ex")
}

/// `CM_Query_Remove_SubTree`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_query_remove_sub_tree() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Query_Remove_SubTree")
}

/// `CM_Query_Remove_SubTree_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_query_remove_sub_tree_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Query_Remove_SubTree_Ex")
}

/// `CM_Register_Device_Driver`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_register_device_driver() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Register_Device_Driver")
}

/// `CM_Register_Device_Driver_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_register_device_driver_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Register_Device_Driver_Ex")
}

/// `CM_Register_Device_InterfaceA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_register_device_interface_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Register_Device_InterfaceA")
}

/// `CM_Register_Device_InterfaceW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_register_device_interface_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Register_Device_InterfaceW")
}

/// `CM_Register_Device_Interface_ExA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_register_device_interface_ex_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Register_Device_Interface_ExA")
}

/// `CM_Register_Device_Interface_ExW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_register_device_interface_ex_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Register_Device_Interface_ExW")
}

/// `CM_Remove_SubTree`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_remove_sub_tree() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Remove_SubTree")
}

/// `CM_Remove_SubTree_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_remove_sub_tree_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Remove_SubTree_Ex")
}

/// `CM_Remove_Unmarked_Children`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_remove_unmarked_children() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Remove_Unmarked_Children")
}

/// `CM_Remove_Unmarked_Children_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_remove_unmarked_children_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Remove_Unmarked_Children_Ex")
}

/// `CM_Request_Eject_PC`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_request_eject_pc() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Request_Eject_PC")
}

/// `CM_Reset_Children_Marks`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_reset_children_marks() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Reset_Children_Marks")
}

/// `CM_Reset_Children_Marks_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_reset_children_marks_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Reset_Children_Marks_Ex")
}

/// `CM_Run_Detection`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_run_detection() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Run_Detection")
}

/// `CM_Run_Detection_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_run_detection_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Run_Detection_Ex")
}

/// `CM_Set_DevNode_Problem`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_set_dev_node_problem() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Set_DevNode_Problem")
}

/// `CM_Set_DevNode_Problem_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_set_dev_node_problem_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Set_DevNode_Problem_Ex")
}

/// `CM_Set_DevNode_Registry_PropertyA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_set_dev_node_registry_property_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Set_DevNode_Registry_PropertyA")
}

/// `CM_Set_DevNode_Registry_PropertyW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_set_dev_node_registry_property_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Set_DevNode_Registry_PropertyW")
}

/// `CM_Set_DevNode_Registry_Property_ExA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_set_dev_node_registry_property_ex_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Set_DevNode_Registry_Property_ExA")
}

/// `CM_Set_DevNode_Registry_Property_ExW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_set_dev_node_registry_property_ex_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Set_DevNode_Registry_Property_ExW")
}

/// `CM_Set_HW_Prof`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_set_hw_prof() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Set_HW_Prof")
}

/// `CM_Set_HW_Prof_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_set_hw_prof_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Set_HW_Prof_Ex")
}

/// `CM_Set_HW_Prof_FlagsA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_set_hw_prof_flags_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Set_HW_Prof_FlagsA")
}

/// `CM_Set_HW_Prof_FlagsW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_set_hw_prof_flags_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Set_HW_Prof_FlagsW")
}

/// `CM_Set_HW_Prof_Flags_ExA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_set_hw_prof_flags_ex_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Set_HW_Prof_Flags_ExA")
}

/// `CM_Set_HW_Prof_Flags_ExW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_set_hw_prof_flags_ex_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Set_HW_Prof_Flags_ExW")
}

/// `CM_Setup_DevNode`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_setup_dev_node() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Setup_DevNode")
}

/// `CM_Setup_DevNode_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_setup_dev_node_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Setup_DevNode_Ex")
}

/// `CM_Test_Range_Available`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_test_range_available() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Test_Range_Available")
}

/// `CM_Uninstall_DevNode`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_uninstall_dev_node() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Uninstall_DevNode")
}

/// `CM_Uninstall_DevNode_Ex`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_uninstall_dev_node_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Uninstall_DevNode_Ex")
}

/// `CM_Unregister_Device_InterfaceA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_unregister_device_interface_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Unregister_Device_InterfaceA")
}

/// `CM_Unregister_Device_InterfaceW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_unregister_device_interface_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Unregister_Device_InterfaceW")
}

/// `CM_Unregister_Device_Interface_ExA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_unregister_device_interface_ex_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Unregister_Device_Interface_ExA")
}

/// `CM_Unregister_Device_Interface_ExW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn cm_unregister_device_interface_ex_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CM_Unregister_Device_Interface_ExW")
}

/// `CenterWindowRelativeToParent`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn center_window_relative_to_parent() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "CenterWindowRelativeToParent")
}

/// `ConcatenatePaths`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn concatenate_paths() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "ConcatenatePaths")
}

/// `DelimStringToMultiSz`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn delim_string_to_multi_sz() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "DelimStringToMultiSz")
}

/// `DestroyTextFileReadBuffer`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn destroy_text_file_read_buffer() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "DestroyTextFileReadBuffer")
}

/// `DriverStoreEnumDriverPackageA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn driver_store_enum_driver_package_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "DriverStoreEnumDriverPackageA")
}

/// `DriverStoreEnumDriverPackageW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn driver_store_enum_driver_package_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "DriverStoreEnumDriverPackageW")
}

/// `ExtensionPropSheetPageProc`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn extension_prop_sheet_page_proc() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "ExtensionPropSheetPageProc")
}

/// `FreeStringArray`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn free_string_array() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "FreeStringArray")
}

/// `GetCurrentDriverSigningPolicy`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn get_current_driver_signing_policy() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "GetCurrentDriverSigningPolicy")
}

/// `GetNewInfName`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn get_new_inf_name() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "GetNewInfName")
}

/// `GetSetFileTimestamp`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn get_set_file_timestamp() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "GetSetFileTimestamp")
}

/// `GetVersionInfoFromImage`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn get_version_info_from_image() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "GetVersionInfoFromImage")
}

/// `InfIsFromOemLocation`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn inf_is_from_oem_location() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "InfIsFromOemLocation")
}

/// `InstallStop`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn install_stop() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "InstallStop")
}

/// `InstallStopEx`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn install_stop_ex() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "InstallStopEx")
}

/// `LookUpStringInTable`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn look_up_string_in_table() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "LookUpStringInTable")
}

/// `MemoryInitialize`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn memory_initialize() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "MemoryInitialize")
}

/// `MultiSzFromSearchControl`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn multi_sz_from_search_control() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "MultiSzFromSearchControl")
}

/// `MyGetFileTitle`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn my_get_file_title() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "MyGetFileTitle")
}

/// `OutOfMemory`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn out_of_memory() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "OutOfMemory")
}

/// `QueryMultiSzValueToArray`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn query_multi_sz_value_to_array() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "QueryMultiSzValueToArray")
}

/// `ReadAsciiOrUnicodeTextFile`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn read_ascii_or_unicode_text_file() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "ReadAsciiOrUnicodeTextFile")
}

/// `RetrieveServiceConfig`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn retrieve_service_config() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "RetrieveServiceConfig")
}

/// `SearchForInfFile`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn search_for_inf_file() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SearchForInfFile")
}

/// `SetArrayToMultiSzValue`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn set_array_to_multi_sz_value() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetArrayToMultiSzValue")
}

/// `SetupAddInstallSectionToDiskSpaceListW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_add_install_section_to_disk_space_list_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupAddInstallSectionToDiskSpaceListW")
}

/// `SetupAddSectionToDiskSpaceListA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_add_section_to_disk_space_list_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupAddSectionToDiskSpaceListA")
}

/// `SetupAddSectionToDiskSpaceListW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_add_section_to_disk_space_list_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupAddSectionToDiskSpaceListW")
}

/// `SetupAdjustDiskSpaceListA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_adjust_disk_space_list_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupAdjustDiskSpaceListA")
}

/// `SetupAdjustDiskSpaceListW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_adjust_disk_space_list_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupAdjustDiskSpaceListW")
}

/// `SetupCancelTemporarySourceList`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_cancel_temporary_source_list() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupCancelTemporarySourceList")
}

/// `SetupDefaultQueueCallback`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_default_queue_callback() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDefaultQueueCallback")
}

/// `SetupDiAskForOEMDisk`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_ask_for_oem_disk() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiAskForOEMDisk")
}

/// `SetupDiCancelDriverInfoSearch`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_cancel_driver_info_search() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiCancelDriverInfoSearch")
}

/// `SetupDiChangeState`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_change_state() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiChangeState")
}

/// `SetupDiDeleteDeviceRegKey`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_delete_device_reg_key() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiDeleteDeviceRegKey")
}

/// `SetupDiGetClassDevPropertySheetsA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_get_class_dev_property_sheets_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiGetClassDevPropertySheetsA")
}

/// `SetupDiGetClassDevPropertySheetsW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_get_class_dev_property_sheets_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiGetClassDevPropertySheetsW")
}

/// `SetupDiGetClassImageListExA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_get_class_image_list_ex_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiGetClassImageListExA")
}

/// `SetupDiGetClassImageListExW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_get_class_image_list_ex_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiGetClassImageListExW")
}

/// `SetupDiGetClassInstallParamsA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_get_class_install_params_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiGetClassInstallParamsA")
}

/// `SetupDiGetClassInstallParamsW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_get_class_install_params_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiGetClassInstallParamsW")
}

/// `SetupDiGetDeviceInfoListClass`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_get_device_info_list_class() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiGetDeviceInfoListClass")
}

/// `SetupDiGetDeviceInterfaceAlias`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_get_device_interface_alias() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiGetDeviceInterfaceAlias")
}

/// `SetupDiGetDriverInstallParamsA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_get_driver_install_params_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiGetDriverInstallParamsA")
}

/// `SetupDiGetDriverInstallParamsW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_get_driver_install_params_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiGetDriverInstallParamsW")
}

/// `SetupDiGetHwProfileFriendlyNameA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_get_hw_profile_friendly_name_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiGetHwProfileFriendlyNameA")
}

/// `SetupDiGetHwProfileFriendlyNameExA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_get_hw_profile_friendly_name_ex_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiGetHwProfileFriendlyNameExA")
}

/// `SetupDiGetHwProfileFriendlyNameExW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_get_hw_profile_friendly_name_ex_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiGetHwProfileFriendlyNameExW")
}

/// `SetupDiGetHwProfileFriendlyNameW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_get_hw_profile_friendly_name_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiGetHwProfileFriendlyNameW")
}

/// `SetupDiGetHwProfileList`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_get_hw_profile_list() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiGetHwProfileList")
}

/// `SetupDiGetHwProfileListExA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_get_hw_profile_list_ex_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiGetHwProfileListExA")
}

/// `SetupDiGetHwProfileListExW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_get_hw_profile_list_ex_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiGetHwProfileListExW")
}

/// `SetupDiGetSelectedDevice`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_get_selected_device() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiGetSelectedDevice")
}

/// `SetupDiGetWizardPage`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_get_wizard_page() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiGetWizardPage")
}

/// `SetupDiInstallClassExA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_install_class_ex_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiInstallClassExA")
}

/// `SetupDiInstallClassExW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_install_class_ex_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiInstallClassExW")
}

/// `SetupDiMoveDuplicateDevice`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_move_duplicate_device() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiMoveDuplicateDevice")
}

/// `SetupDiSelectDevice`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_select_device() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiSelectDevice")
}

/// `SetupDiSelectOEMDrv`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_select_oem_drv() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiSelectOEMDrv")
}

/// `SetupDiSetDriverInstallParamsA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_set_driver_install_params_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiSetDriverInstallParamsA")
}

/// `SetupDiSetDriverInstallParamsW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_set_driver_install_params_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiSetDriverInstallParamsW")
}

/// `SetupDiSetSelectedDriverA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_set_selected_driver_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiSetSelectedDriverA")
}

/// `SetupDiSetSelectedDriverW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_set_selected_driver_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiSetSelectedDriverW")
}

/// `SetupDiUnremoveDevice`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_di_unremove_device() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupDiUnremoveDevice")
}

/// `SetupFreeSourceListA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_free_source_list_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupFreeSourceListA")
}

/// `SetupFreeSourceListW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_free_source_list_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupFreeSourceListW")
}

/// `SetupGetBackupInformationA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_get_backup_information_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupGetBackupInformationA")
}

/// `SetupGetBackupInformationW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_get_backup_information_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupGetBackupInformationW")
}

/// `SetupGetInfSections`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_get_inf_sections() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupGetInfSections")
}

/// `SetupGetSourceFileSizeA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_get_source_file_size_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupGetSourceFileSizeA")
}

/// `SetupGetSourceFileSizeW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_get_source_file_size_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupGetSourceFileSizeW")
}

/// `SetupInstallServicesFromInfSectionExA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_install_services_from_inf_section_ex_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupInstallServicesFromInfSectionExA")
}

/// `SetupInstallServicesFromInfSectionExW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_install_services_from_inf_section_ex_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupInstallServicesFromInfSectionExW")
}

/// `SetupQueryFileLogA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_query_file_log_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupQueryFileLogA")
}

/// `SetupQueryFileLogW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_query_file_log_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupQueryFileLogW")
}

/// `SetupQuerySourceListA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_query_source_list_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupQuerySourceListA")
}

/// `SetupQuerySourceListW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_query_source_list_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupQuerySourceListW")
}

/// `SetupRemoveFileLogEntryA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_remove_file_log_entry_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupRemoveFileLogEntryA")
}

/// `SetupRemoveFileLogEntryW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_remove_file_log_entry_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupRemoveFileLogEntryW")
}

/// `SetupRemoveFromDiskSpaceListA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_remove_from_disk_space_list_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupRemoveFromDiskSpaceListA")
}

/// `SetupRemoveFromDiskSpaceListW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_remove_from_disk_space_list_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupRemoveFromDiskSpaceListW")
}

/// `SetupRemoveFromSourceListA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_remove_from_source_list_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupRemoveFromSourceListA")
}

/// `SetupRemoveFromSourceListW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_remove_from_source_list_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupRemoveFromSourceListW")
}

/// `SetupRemoveInstallSectionFromDiskSpaceListA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_remove_install_section_from_disk_space_list_a() -> StubException {
    wine_spec_unimplemented_stub(
        SETUPAPI_DLL_NAME,
        "SetupRemoveInstallSectionFromDiskSpaceListA",
    )
}

/// `SetupRemoveInstallSectionFromDiskSpaceListW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_remove_install_section_from_disk_space_list_w() -> StubException {
    wine_spec_unimplemented_stub(
        SETUPAPI_DLL_NAME,
        "SetupRemoveInstallSectionFromDiskSpaceListW",
    )
}

/// `SetupRemoveSectionFromDiskSpaceListA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_remove_section_from_disk_space_list_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupRemoveSectionFromDiskSpaceListA")
}

/// `SetupRemoveSectionFromDiskSpaceListW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_remove_section_from_disk_space_list_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupRemoveSectionFromDiskSpaceListW")
}

/// `SetupScanFileQueue`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_scan_file_queue() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupScanFileQueue")
}

/// `SetupSetDirectoryIdExA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_set_directory_id_ex_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupSetDirectoryIdExA")
}

/// `SetupSetDirectoryIdExW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_set_directory_id_ex_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupSetDirectoryIdExW")
}

/// `SetupSetPlatformPathOverrideA`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_set_platform_path_override_a() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupSetPlatformPathOverrideA")
}

/// `SetupSetPlatformPathOverrideW`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn setup_set_platform_path_override_w() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "SetupSetPlatformPathOverrideW")
}

/// `ShouldDeviceBeExcluded`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn should_device_be_excluded() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "ShouldDeviceBeExcluded")
}

/// `StringTableEnum`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn string_table_enum() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "StringTableEnum")
}

/// `VerifyCatalogFile`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn verify_catalog_file() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "VerifyCatalogFile")
}

/// `VerifyFile`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn verify_file() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "VerifyFile")
}

/// `pSetupAccessRunOnceNodeList`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn p_setup_access_run_once_node_list() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "pSetupAccessRunOnceNodeList")
}

/// `pSetupAddMiniIconToList`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn p_setup_add_mini_icon_to_list() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "pSetupAddMiniIconToList")
}

/// `pSetupAddTagToGroupOrderListEntry`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn p_setup_add_tag_to_group_order_list_entry() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "pSetupAddTagToGroupOrderListEntry")
}

/// `pSetupAppendStringToMultiSz`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn p_setup_append_string_to_multi_sz() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "pSetupAppendStringToMultiSz")
}

/// `pSetupDestroyRunOnceNodeList`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn p_setup_destroy_run_once_node_list() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "pSetupDestroyRunOnceNodeList")
}

/// `pSetupDirectoryIdToPath`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn p_setup_directory_id_to_path() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "pSetupDirectoryIdToPath")
}

/// `pSetupGetOsLoaderDriveAndPath`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn p_setup_get_os_loader_drive_and_path() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "pSetupGetOsLoaderDriveAndPath")
}

/// `pSetupGetVersionDatum`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn p_setup_get_version_datum() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "pSetupGetVersionDatum")
}

/// `pSetupGuidFromString`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn p_setup_guid_from_string() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "pSetupGuidFromString")
}

/// `pSetupIsGuidNull`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn p_setup_is_guid_null() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "pSetupIsGuidNull")
}

/// `pSetupMakeSurePathExists`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn p_setup_make_sure_path_exists() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "pSetupMakeSurePathExists")
}

/// `pSetupSetSystemSourceFlags`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn p_setup_set_system_source_flags() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "pSetupSetSystemSourceFlags")
}

/// `pSetupStringFromGuid`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn p_setup_string_from_guid() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "pSetupStringFromGuid")
}

/// `pSetupStringTableEnum`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn p_setup_string_table_enum() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "pSetupStringTableEnum")
}

/// `pSetupVerifyCatalogFile`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn p_setup_verify_catalog_file() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "pSetupVerifyCatalogFile")
}

/// `pSetupVerifyQueuedCatalogs`, exported as `@ stub` in `dlls/setupapi/setupapi.spec`.
pub fn p_setup_verify_queued_catalogs() -> StubException {
    wine_spec_unimplemented_stub(SETUPAPI_DLL_NAME, "pSetupVerifyQueuedCatalogs")
}
