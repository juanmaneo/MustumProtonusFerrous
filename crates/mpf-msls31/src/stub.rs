/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = msls31.dll` in
/// `dlls/msls31/Makefile.in`.
const MSLS31_DLL_NAME: &str = "msls31.dll";

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

// One helper per `@ stub` export in `dlls/msls31/msls31.spec`, named after the export in
// snake_case. Each raises the record for its own export name.

pub fn ls_append_run_to_current_subline() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsAppendRunToCurrentSubline")
}

pub fn ls_compress_subline() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsCompressSubline")
}

pub fn ls_create_context() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsCreateContext")
}

pub fn ls_create_line() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsCreateLine")
}

pub fn ls_create_subline() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsCreateSubline")
}

pub fn ls_destroy_context() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsDestroyContext")
}

pub fn ls_destroy_line() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsDestroyLine")
}

pub fn ls_destroy_subline() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsDestroySubline")
}

pub fn ls_display_line() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsDisplayLine")
}

pub fn ls_display_subline() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsDisplaySubline")
}

pub fn ls_enum_line() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsEnumLine")
}

pub fn ls_enum_subline() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsEnumSubline")
}

pub fn ls_expand_subline() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsExpandSubline")
}

pub fn ls_fetch_append_to_current_subline() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsFetchAppendToCurrentSubline")
}

pub fn ls_fetch_append_to_current_subline_resume() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsFetchAppendToCurrentSublineResume")
}

pub fn ls_find_next_break_subline() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsFindNextBreakSubline")
}

pub fn ls_find_prev_break_subline() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsFindPrevBreakSubline")
}

pub fn ls_finish_current_subline() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsFinishCurrentSubline")
}

pub fn ls_force_break_subline() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsForceBreakSubline")
}

pub fn ls_get_hih_lsimethods() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsGetHihLsimethods")
}

pub fn ls_get_line_dur() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsGetLineDur")
}

pub fn ls_get_min_dur_breaks() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsGetMinDurBreaks")
}

pub fn ls_get_reverse_lsimethods() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsGetReverseLsimethods")
}

pub fn ls_get_ruby_lsimethods() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsGetRubyLsimethods")
}

pub fn ls_get_special_effects_subline() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsGetSpecialEffectsSubline")
}

pub fn ls_get_tatenakayoko_lsimethods() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsGetTatenakayokoLsimethods")
}

pub fn ls_get_warichu_lsimethods() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsGetWarichuLsimethods")
}

pub fn ls_lw_mult_div_r() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsLwMultDivR")
}

pub fn ls_match_pres_subline() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsMatchPresSubline")
}

pub fn ls_modify_line_height() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsModifyLineHeight")
}

pub fn ls_point_uv2_from_point_uv1() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsPointUV2FromPointUV1")
}

pub fn ls_point_xy_from_point_uv() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsPointXYFromPointUV")
}

pub fn ls_query_cp_ppoint_subline() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsQueryCpPpointSubline")
}

pub fn ls_query_f_line_empty() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsQueryFLineEmpty")
}

pub fn ls_query_line_cp_ppoint() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsQueryLineCpPpoint")
}

pub fn ls_query_line_dup() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsQueryLineDup")
}

pub fn ls_query_line_point_pcp() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsQueryLinePointPcp")
}

pub fn ls_query_point_pcp_subline() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsQueryPointPcpSubline")
}

pub fn ls_query_text_cell_details() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsQueryTextCellDetails")
}

pub fn ls_reset_rm_in_current_subline() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsResetRMInCurrentSubline")
}

pub fn ls_set_break_subline() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsSetBreakSubline")
}

pub fn ls_set_breaking() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsSetBreaking")
}

pub fn ls_set_compression() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsSetCompression")
}

pub fn ls_set_doc() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsSetDoc")
}

pub fn ls_set_expansion() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsSetExpansion")
}

pub fn ls_set_mod_width_pairs() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsSetModWidthPairs")
}

pub fn ls_squeeze_subline() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsSqueezeSubline")
}

pub fn ls_truncate_subline() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsTruncateSubline")
}

pub fn lsdn_distribute() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsdnDistribute")
}

pub fn lsdn_finish_by_one_char() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsdnFinishByOneChar")
}

pub fn lsdn_finish_by_pen() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsdnFinishByPen")
}

pub fn lsdn_finish_by_subline() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsdnFinishBySubline")
}

pub fn lsdn_finish_delete() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsdnFinishDelete")
}

pub fn lsdn_finish_delete_all() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsdnFinishDeleteAll")
}

pub fn lsdn_finish_regular() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsdnFinishRegular")
}

pub fn lsdn_finish_regular_add_advance_pen() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsdnFinishRegularAddAdvancePen")
}

pub fn lsdn_get_cur_tab_info() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsdnGetCurTabInfo")
}

pub fn lsdn_get_dup() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsdnGetDup")
}

pub fn lsdn_get_format_depth() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsdnGetFormatDepth")
}

pub fn lsdn_modify_para_ending() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsdnModifyParaEnding")
}

pub fn lsdn_query_obj_dim_range() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsdnQueryObjDimRange")
}

pub fn lsdn_query_pen_node() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsdnQueryPenNode")
}

pub fn lsdn_reset_obj_dim() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsdnResetObjDim")
}

pub fn lsdn_reset_pen_node() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsdnResetPenNode")
}

pub fn lsdn_resolve_prev_tab() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsdnResolvePrevTab")
}

pub fn lsdn_set_abs_base_line() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsdnSetAbsBaseLine")
}

pub fn lsdn_set_rigid_dup() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsdnSetRigidDup")
}

pub fn lsdn_skip_cur_tab() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsdnSkipCurTab")
}

pub fn lsdn_submit_sublines() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LsdnSubmitSublines")
}

pub fn lssb_f_done_display() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LssbFDoneDisplay")
}

pub fn lssb_f_done_pres_subline() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LssbFDonePresSubline")
}

pub fn lssb_f_is_subline_empty() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LssbFIsSublineEmpty")
}

pub fn lssb_get_dup_subline() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LssbGetDupSubline")
}

pub fn lssb_get_dur_trail_in_subline() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LssbGetDurTrailInSubline")
}

pub fn lssb_get_dur_trail_with_pens_in_subline() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LssbGetDurTrailWithPensInSubline")
}

pub fn lssb_get_number_dnodes_in_subline() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LssbGetNumberDnodesInSubline")
}

pub fn lssb_get_obj_dim_subline() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LssbGetObjDimSubline")
}

pub fn lssb_get_plsruns_from_subline() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LssbGetPlsrunsFromSubline")
}

pub fn lssb_get_visible_dcp_in_subline() -> StubException {
    wine_spec_unimplemented_stub(MSLS31_DLL_NAME, "LssbGetVisibleDcpInSubline")
}
