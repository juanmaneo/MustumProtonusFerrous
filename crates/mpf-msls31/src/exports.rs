use crate::stub::{self, StubException};

/// The helper that raises the stub record of one export.
type StubHelper = fn() -> StubException;

/// The `msls31.spec` export table, in spec order. Every export is `@ stub`, so each name maps to
/// the helper in `stub.rs` that raises its record.
pub const EXPORTS: [(&str, StubHelper); 79] = [
    (
        "LsAppendRunToCurrentSubline",
        stub::ls_append_run_to_current_subline,
    ),
    ("LsCompressSubline", stub::ls_compress_subline),
    ("LsCreateContext", stub::ls_create_context),
    ("LsCreateLine", stub::ls_create_line),
    ("LsCreateSubline", stub::ls_create_subline),
    ("LsDestroyContext", stub::ls_destroy_context),
    ("LsDestroyLine", stub::ls_destroy_line),
    ("LsDestroySubline", stub::ls_destroy_subline),
    ("LsDisplayLine", stub::ls_display_line),
    ("LsDisplaySubline", stub::ls_display_subline),
    ("LsEnumLine", stub::ls_enum_line),
    ("LsEnumSubline", stub::ls_enum_subline),
    ("LsExpandSubline", stub::ls_expand_subline),
    (
        "LsFetchAppendToCurrentSubline",
        stub::ls_fetch_append_to_current_subline,
    ),
    (
        "LsFetchAppendToCurrentSublineResume",
        stub::ls_fetch_append_to_current_subline_resume,
    ),
    ("LsFindNextBreakSubline", stub::ls_find_next_break_subline),
    ("LsFindPrevBreakSubline", stub::ls_find_prev_break_subline),
    ("LsFinishCurrentSubline", stub::ls_finish_current_subline),
    ("LsForceBreakSubline", stub::ls_force_break_subline),
    ("LsGetHihLsimethods", stub::ls_get_hih_lsimethods),
    ("LsGetLineDur", stub::ls_get_line_dur),
    ("LsGetMinDurBreaks", stub::ls_get_min_dur_breaks),
    ("LsGetReverseLsimethods", stub::ls_get_reverse_lsimethods),
    ("LsGetRubyLsimethods", stub::ls_get_ruby_lsimethods),
    (
        "LsGetSpecialEffectsSubline",
        stub::ls_get_special_effects_subline,
    ),
    (
        "LsGetTatenakayokoLsimethods",
        stub::ls_get_tatenakayoko_lsimethods,
    ),
    ("LsGetWarichuLsimethods", stub::ls_get_warichu_lsimethods),
    ("LsLwMultDivR", stub::ls_lw_mult_div_r),
    ("LsMatchPresSubline", stub::ls_match_pres_subline),
    ("LsModifyLineHeight", stub::ls_modify_line_height),
    ("LsPointUV2FromPointUV1", stub::ls_point_uv2_from_point_uv1),
    ("LsPointXYFromPointUV", stub::ls_point_xy_from_point_uv),
    ("LsQueryCpPpointSubline", stub::ls_query_cp_ppoint_subline),
    ("LsQueryFLineEmpty", stub::ls_query_f_line_empty),
    ("LsQueryLineCpPpoint", stub::ls_query_line_cp_ppoint),
    ("LsQueryLineDup", stub::ls_query_line_dup),
    ("LsQueryLinePointPcp", stub::ls_query_line_point_pcp),
    ("LsQueryPointPcpSubline", stub::ls_query_point_pcp_subline),
    ("LsQueryTextCellDetails", stub::ls_query_text_cell_details),
    (
        "LsResetRMInCurrentSubline",
        stub::ls_reset_rm_in_current_subline,
    ),
    ("LsSetBreakSubline", stub::ls_set_break_subline),
    ("LsSetBreaking", stub::ls_set_breaking),
    ("LsSetCompression", stub::ls_set_compression),
    ("LsSetDoc", stub::ls_set_doc),
    ("LsSetExpansion", stub::ls_set_expansion),
    ("LsSetModWidthPairs", stub::ls_set_mod_width_pairs),
    ("LsSqueezeSubline", stub::ls_squeeze_subline),
    ("LsTruncateSubline", stub::ls_truncate_subline),
    ("LsdnDistribute", stub::lsdn_distribute),
    ("LsdnFinishByOneChar", stub::lsdn_finish_by_one_char),
    ("LsdnFinishByPen", stub::lsdn_finish_by_pen),
    ("LsdnFinishBySubline", stub::lsdn_finish_by_subline),
    ("LsdnFinishDelete", stub::lsdn_finish_delete),
    ("LsdnFinishDeleteAll", stub::lsdn_finish_delete_all),
    ("LsdnFinishRegular", stub::lsdn_finish_regular),
    (
        "LsdnFinishRegularAddAdvancePen",
        stub::lsdn_finish_regular_add_advance_pen,
    ),
    ("LsdnGetCurTabInfo", stub::lsdn_get_cur_tab_info),
    ("LsdnGetDup", stub::lsdn_get_dup),
    ("LsdnGetFormatDepth", stub::lsdn_get_format_depth),
    ("LsdnModifyParaEnding", stub::lsdn_modify_para_ending),
    ("LsdnQueryObjDimRange", stub::lsdn_query_obj_dim_range),
    ("LsdnQueryPenNode", stub::lsdn_query_pen_node),
    ("LsdnResetObjDim", stub::lsdn_reset_obj_dim),
    ("LsdnResetPenNode", stub::lsdn_reset_pen_node),
    ("LsdnResolvePrevTab", stub::lsdn_resolve_prev_tab),
    ("LsdnSetAbsBaseLine", stub::lsdn_set_abs_base_line),
    ("LsdnSetRigidDup", stub::lsdn_set_rigid_dup),
    ("LsdnSkipCurTab", stub::lsdn_skip_cur_tab),
    ("LsdnSubmitSublines", stub::lsdn_submit_sublines),
    ("LssbFDoneDisplay", stub::lssb_f_done_display),
    ("LssbFDonePresSubline", stub::lssb_f_done_pres_subline),
    ("LssbFIsSublineEmpty", stub::lssb_f_is_subline_empty),
    ("LssbGetDupSubline", stub::lssb_get_dup_subline),
    (
        "LssbGetDurTrailInSubline",
        stub::lssb_get_dur_trail_in_subline,
    ),
    (
        "LssbGetDurTrailWithPensInSubline",
        stub::lssb_get_dur_trail_with_pens_in_subline,
    ),
    (
        "LssbGetNumberDnodesInSubline",
        stub::lssb_get_number_dnodes_in_subline,
    ),
    ("LssbGetObjDimSubline", stub::lssb_get_obj_dim_subline),
    (
        "LssbGetPlsrunsFromSubline",
        stub::lssb_get_plsruns_from_subline,
    ),
    (
        "LssbGetVisibleDcpInSubline",
        stub::lssb_get_visible_dcp_in_subline,
    ),
];

/// The stub record for the export `name`, or `None` when `msls31.spec` does not export it.
/// Names are case-sensitive.
pub fn stub_exception(name: &str) -> Option<StubException> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, helper)| helper())
}
