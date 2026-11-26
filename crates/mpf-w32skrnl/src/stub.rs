/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = w32skrnl.dll` in `dlls/w32skrnl/Makefile.in`.
const W32SKRNL_DLL_NAME: &str = "w32skrnl.dll";

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
        number_parameters: 2,
        module,
        function,
    }
}

/// `_kSetEnvironmentVariable`, a `@ stub` export.
pub fn _k_set_environment_variable() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_kSetEnvironmentVariable")
}

/// `_SzFromImte`, a `@ stub` export.
pub fn _sz_from_imte() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_SzFromImte")
}

/// `_DebugPrintf`, a `@ stub` export.
pub fn _debug_printf() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_DebugPrintf")
}

/// `_NtCloseSection`, a `@ stub` export.
pub fn _nt_close_section() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_NtCloseSection")
}

/// `_AllocKernelHeap`, a `@ stub` export.
pub fn _alloc_kernel_heap() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_AllocKernelHeap")
}

/// `_SelToFlat`, a `@ stub` export.
pub fn _sel_to_flat() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_SelToFlat")
}

/// `_kGetExeVersion`, a `@ stub` export.
pub fn _k_get_exe_version() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_kGetExeVersion")
}

/// `_ResizeKernelHeap`, a `@ stub` export.
pub fn _resize_kernel_heap() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_ResizeKernelHeap")
}

/// `_DbgBackTo32`, a `@ stub` export.
pub fn _dbg_back_to32() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_DbgBackTo32")
}

/// `_kGethInstance`, a `@ stub` export.
pub fn _k_geth_instance() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_kGethInstance")
}

/// `SelOffsetToLinear`, a `@ stub` export.
pub fn sel_offset_to_linear() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "SelOffsetToLinear")
}

/// `DebugPrintf`, a `@ stub` export.
pub fn debug_printf() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "DebugPrintf")
}

/// `_sprintf`, a `@ stub` export.
pub fn _sprintf() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_sprintf")
}

/// `_KIsDBCSLeadByte`, a `@ stub` export.
pub fn _k_is_dbcs_lead_byte() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_KIsDBCSLeadByte")
}

/// `_KGetDbgFlags32`, a `@ stub` export.
pub fn _k_get_dbg_flags32() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_KGetDbgFlags32")
}

/// `SelToFlat`, a `@ stub` export.
pub fn sel_to_flat() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "SelToFlat")
}

/// `_FreeKernelHeap`, a `@ stub` export.
pub fn _free_kernel_heap() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_FreeKernelHeap")
}

/// `WIN32SYSDLL`, a `@ stub` export.
pub fn win32_sysdll() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "WIN32SYSDLL")
}

/// `_KSilentException`, a `@ stub` export.
pub fn _k_silent_exception() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_KSilentException")
}

/// `_NtCreateSection`, a `@ stub` export.
pub fn _nt_create_section() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_NtCreateSection")
}

/// `_NtOpenSection`, a `@ stub` export.
pub fn _nt_open_section() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_NtOpenSection")
}

/// `_NtDupSection`, a `@ stub` export.
pub fn _nt_dup_section() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_NtDupSection")
}

/// `_GetSelModName`, a `@ stub` export.
pub fn _get_sel_mod_name() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_GetSelModName")
}

/// `_FGetDscr`, a `@ stub` export.
pub fn _f_get_dscr() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_FGetDscr")
}

/// `_NtFlushVirtualMemory`, a `@ stub` export.
pub fn _nt_flush_virtual_memory() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_NtFlushVirtualMemory")
}

/// `_snprintf`, a `@ stub` export.
pub fn _snprintf() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_snprintf")
}

/// `_NtUnmapViewOfSection`, a `@ stub` export.
pub fn _nt_unmap_view_of_section() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_NtUnmapViewOfSection")
}

/// `_PvAllocVirtMem`, a `@ stub` export.
pub fn _pv_alloc_virt_mem() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_PvAllocVirtMem")
}

/// `SetVirtMemProcess`, a `@ stub` export.
pub fn set_virt_mem_process() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "SetVirtMemProcess")
}

/// `_FFreeVirtMem`, a `@ stub` export.
pub fn _f_free_virt_mem() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_FFreeVirtMem")
}

/// `_FSetVirtProtect`, a `@ stub` export.
pub fn _f_set_virt_protect() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_FSetVirtProtect")
}

/// `_QueryVirtMem`, a `@ stub` export.
pub fn _query_virt_mem() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_QueryVirtMem")
}

/// `_FLockVirtMem`, a `@ stub` export.
pub fn _f_lock_virt_mem() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_FLockVirtMem")
}

/// `_FUnlockVirtMem`, a `@ stub` export.
pub fn _f_unlock_virt_mem() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_FUnlockVirtMem")
}

/// `_HHeapCreateHeap`, a `@ stub` export.
pub fn _h_heap_create_heap() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_HHeapCreateHeap")
}

/// `_FHeapDestroy`, a `@ stub` export.
pub fn _f_heap_destroy() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_FHeapDestroy")
}

/// `_PvAllocHeapMem`, a `@ stub` export.
pub fn _pv_alloc_heap_mem() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_PvAllocHeapMem")
}

/// `_PvResizeHeapMem`, a `@ stub` export.
pub fn _pv_resize_heap_mem() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_PvResizeHeapMem")
}

/// `_FFreeHeapMem`, a `@ stub` export.
pub fn _f_free_heap_mem() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_FFreeHeapMem")
}

/// `_PpdbGetProcess`, a `@ stub` export.
pub fn _ppdb_get_process() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_PpdbGetProcess")
}

/// `_ErcGetError`, a `@ stub` export.
pub fn _erc_get_error() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_ErcGetError")
}

/// `_SetError`, a `@ stub` export.
pub fn _set_error() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_SetError")
}

/// `_CbSearchPath`, a `@ stub` export.
pub fn _cb_search_path() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_CbSearchPath")
}

/// `_CbSearchBinPath`, a `@ stub` export.
pub fn _cb_search_bin_path() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_CbSearchBinPath")
}

/// `_FFindEnvVar`, a `@ stub` export.
pub fn _f_find_env_var() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_FFindEnvVar")
}

/// `_SzFileFromImte`, a `@ stub` export.
pub fn _sz_file_from_imte() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_SzFileFromImte")
}

/// `_ImteFromFileSz`, a `@ stub` export.
pub fn _imte_from_file_sz() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_ImteFromFileSz")
}

/// `_ImteFromSz`, a `@ stub` export.
pub fn _imte_from_sz() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_ImteFromSz")
}

/// `_FGetProcAddr`, a `@ stub` export.
pub fn _f_get_proc_addr() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_FGetProcAddr")
}

/// `_KLoadLibrary`, a `@ stub` export.
pub fn _k_load_library() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_KLoadLibrary")
}

/// `_KFreeLibrary`, a `@ stub` export.
pub fn _k_free_library() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_KFreeLibrary")
}

/// `_lmaUserBase`, a `@ stub` export.
pub fn _lma_user_base() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_lmaUserBase")
}

/// `_KrnOutputDebugString`, a `@ stub` export.
pub fn _krn_output_debug_string() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_KrnOutputDebugString")
}

/// `_RtlRaiseException`, a `@ stub` export.
pub fn _rtl_raise_exception() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_RtlRaiseException")
}

/// `_RtlUnwind`, a `@ stub` export.
pub fn _rtl_unwind() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_RtlUnwind")
}

/// `_RtlUnwind4`, a `@ stub` export.
pub fn _rtl_unwind4() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_RtlUnwind4")
}

/// `_DfhCreateFile`, a `@ stub` export.
pub fn _dfh_create_file() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_DfhCreateFile")
}

/// `_FCloseFile`, a `@ stub` export.
pub fn _f_close_file() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_FCloseFile")
}

/// `_CbWriteFile`, a `@ stub` export.
pub fn _cb_write_file() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_CbWriteFile")
}

/// `_LfoSetFilePos`, a `@ stub` export.
pub fn _lfo_set_file_pos() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_LfoSetFilePos")
}

/// `_CbReadFile`, a `@ stub` export.
pub fn _cb_read_file() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_CbReadFile")
}

/// `_SelOffsetToLinear`, a `@ stub` export.
pub fn _sel_offset_to_linear() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_SelOffsetToLinear")
}

/// `_ImteFromHModule`, a `@ stub` export.
pub fn _imte_from_h_module() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_ImteFromHModule")
}

/// `_HModuleFromImte`, a `@ stub` export.
pub fn _h_module_from_imte() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_HModuleFromImte")
}

/// `_ExitPEApp`, a `@ stub` export.
pub fn _exit_pe_app() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_ExitPEApp")
}

/// `_Dos3Call`, a `@ stub` export.
pub fn _dos3_call() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_Dos3Call")
}

/// `_GetPEInstanceData`, a `@ stub` export.
pub fn _get_pe_instance_data() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_GetPEInstanceData")
}

/// `_GetThreadTask`, a `@ stub` export.
pub fn _get_thread_task() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_GetThreadTask")
}

/// `_BaseAddrFromImte`, a `@ stub` export.
pub fn _base_addr_from_imte() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_BaseAddrFromImte")
}

/// `_GetTaskId`, a `@ stub` export.
pub fn _get_task_id() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_GetTaskId")
}

/// `_KGetProcessDebugPort`, a `@ stub` export.
pub fn _k_get_process_debug_port() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_KGetProcessDebugPort")
}

/// `_W32S_BackTo32`, a `@ stub` export.
pub fn _w32_s_back_to32() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_W32S_BackTo32")
}

/// `_KReadProcessMemory32`, a `@ stub` export.
pub fn _k_read_process_memory32() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_KReadProcessMemory32")
}

/// `_KWriteProcessMemory32`, a `@ stub` export.
pub fn _k_write_process_memory32() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_KWriteProcessMemory32")
}

/// `_KGetThreadContext32`, a `@ stub` export.
pub fn _k_get_thread_context32() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_KGetThreadContext32")
}

/// `_KSetThreadContext32`, a `@ stub` export.
pub fn _k_set_thread_context32() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_KSetThreadContext32")
}

/// `_KQueryPerformanceCounter`, a `@ stub` export.
pub fn _k_query_performance_counter() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_KQueryPerformanceCounter")
}

/// `_KGetExitCodeThread32`, a `@ stub` export.
pub fn _k_get_exit_code_thread32() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_KGetExitCodeThread32")
}

/// `_KGetExitCodeProcess32`, a `@ stub` export.
pub fn _k_get_exit_code_process32() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_KGetExitCodeProcess32")
}

/// `_KGetTaskPpdb32`, a `@ stub` export.
pub fn _k_get_task_ppdb32() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_KGetTaskPpdb32")
}

/// `_KGetThreadPtdb32`, a `@ stub` export.
pub fn _k_get_thread_ptdb32() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_KGetThreadPtdb32")
}

/// `_GetCurProcFIOData`, a `@ stub` export.
pub fn _get_cur_proc_fio_data() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_GetCurProcFIOData")
}

/// `_KUseObject32`, a `@ stub` export.
pub fn _k_use_object32() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_KUseObject32")
}

/// `_KUnuseObject32`, a `@ stub` export.
pub fn _k_unuse_object32() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_KUnuseObject32")
}

/// `_KErcGetError32`, a `@ stub` export.
pub fn _k_erc_get_error32() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_KErcGetError32")
}

/// `_KCreateProcessHeap`, a `@ stub` export.
pub fn _k_create_process_heap() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_KCreateProcessHeap")
}

/// `_KGetSystemInfo`, a `@ stub` export.
pub fn _k_get_system_info() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_KGetSystemInfo")
}

/// `_KGlobalMemStat`, a `@ stub` export.
pub fn _k_global_mem_stat() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_KGlobalMemStat")
}

/// `_GetHandleFromAddr`, a `@ stub` export.
pub fn _get_handle_from_addr() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_GetHandleFromAddr")
}

/// `_TlsNullSlot`, a `@ stub` export.
pub fn _tls_null_slot() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_TlsNullSlot")
}

/// `_FWorkingSetSize`, a `@ stub` export.
pub fn _f_working_set_size() -> StubException {
    wine_spec_unimplemented_stub(W32SKRNL_DLL_NAME, "_FWorkingSetSize")
}
