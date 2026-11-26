//! Oracle-derived test vectors for the ported w32skrnl logic.
//!
//! The stub rows are the `StubException` fields that `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`) raises for each `@ stub` export of `w32skrnl.spec`:
//! code `0x80000100`, flags `0x1`, two parameters, module `w32skrnl.dll`, and the export
//! name. The `GetWin32sDirectory` row is the concatenation of the system directory with
//! `\\win32s` from `dlls/w32skernel.c`.

use super::*;

#[test]
fn oracle_stub_rows() {
    // row 001 _kSetEnvironmentVariable
    assert_eq!(
        _k_set_environment_variable(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_kSetEnvironmentVariable",
        }
    );
    // row 002 _SzFromImte
    assert_eq!(
        _sz_from_imte(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_SzFromImte",
        }
    );
    // row 004 _DebugPrintf
    assert_eq!(
        _debug_printf(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_DebugPrintf",
        }
    );
    // row 005 _NtCloseSection
    assert_eq!(
        _nt_close_section(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_NtCloseSection",
        }
    );
    // row 006 _AllocKernelHeap
    assert_eq!(
        _alloc_kernel_heap(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_AllocKernelHeap",
        }
    );
    // row 007 _SelToFlat
    assert_eq!(
        _sel_to_flat(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_SelToFlat",
        }
    );
    // row 008 _kGetExeVersion
    assert_eq!(
        _k_get_exe_version(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_kGetExeVersion",
        }
    );
    // row 009 _ResizeKernelHeap
    assert_eq!(
        _resize_kernel_heap(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_ResizeKernelHeap",
        }
    );
    // row 010 _DbgBackTo32
    assert_eq!(
        _dbg_back_to32(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_DbgBackTo32",
        }
    );
    // row 011 _kGethInstance
    assert_eq!(
        _k_geth_instance(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_kGethInstance",
        }
    );
    // row 012 SelOffsetToLinear
    assert_eq!(
        sel_offset_to_linear(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "SelOffsetToLinear",
        }
    );
    // row 013 DebugPrintf
    assert_eq!(
        debug_printf(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "DebugPrintf",
        }
    );
    // row 015 _sprintf
    assert_eq!(
        _sprintf(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_sprintf",
        }
    );
    // row 016 _KIsDBCSLeadByte
    assert_eq!(
        _k_is_dbcs_lead_byte(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_KIsDBCSLeadByte",
        }
    );
    // row 018 _KGetDbgFlags32
    assert_eq!(
        _k_get_dbg_flags32(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_KGetDbgFlags32",
        }
    );
    // row 019 SelToFlat
    assert_eq!(
        sel_to_flat(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "SelToFlat",
        }
    );
    // row 020 _FreeKernelHeap
    assert_eq!(
        _free_kernel_heap(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_FreeKernelHeap",
        }
    );
    // row 021 WIN32SYSDLL
    assert_eq!(
        win32_sysdll(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "WIN32SYSDLL",
        }
    );
    // row 023 _KSilentException
    assert_eq!(
        _k_silent_exception(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_KSilentException",
        }
    );
    // row 024 _NtCreateSection
    assert_eq!(
        _nt_create_section(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_NtCreateSection",
        }
    );
    // row 026 _NtOpenSection
    assert_eq!(
        _nt_open_section(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_NtOpenSection",
        }
    );
    // row 027 _NtDupSection
    assert_eq!(
        _nt_dup_section(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_NtDupSection",
        }
    );
    // row 028 _GetSelModName
    assert_eq!(
        _get_sel_mod_name(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_GetSelModName",
        }
    );
    // row 029 _FGetDscr
    assert_eq!(
        _f_get_dscr(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_FGetDscr",
        }
    );
    // row 031 _NtFlushVirtualMemory
    assert_eq!(
        _nt_flush_virtual_memory(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_NtFlushVirtualMemory",
        }
    );
    // row 032 _snprintf
    assert_eq!(
        _snprintf(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_snprintf",
        }
    );
    // row 033 _NtUnmapViewOfSection
    assert_eq!(
        _nt_unmap_view_of_section(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_NtUnmapViewOfSection",
        }
    );
    // row 153 _PvAllocVirtMem
    assert_eq!(
        _pv_alloc_virt_mem(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_PvAllocVirtMem",
        }
    );
    // row 154 SetVirtMemProcess
    assert_eq!(
        set_virt_mem_process(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "SetVirtMemProcess",
        }
    );
    // row 155 _FFreeVirtMem
    assert_eq!(
        _f_free_virt_mem(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_FFreeVirtMem",
        }
    );
    // row 156 _FSetVirtProtect
    assert_eq!(
        _f_set_virt_protect(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_FSetVirtProtect",
        }
    );
    // row 157 _QueryVirtMem
    assert_eq!(
        _query_virt_mem(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_QueryVirtMem",
        }
    );
    // row 158 _FLockVirtMem
    assert_eq!(
        _f_lock_virt_mem(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_FLockVirtMem",
        }
    );
    // row 159 _FUnlockVirtMem
    assert_eq!(
        _f_unlock_virt_mem(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_FUnlockVirtMem",
        }
    );
    // row 171 _HHeapCreateHeap
    assert_eq!(
        _h_heap_create_heap(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_HHeapCreateHeap",
        }
    );
    // row 173 _FHeapDestroy
    assert_eq!(
        _f_heap_destroy(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_FHeapDestroy",
        }
    );
    // row 174 _PvAllocHeapMem
    assert_eq!(
        _pv_alloc_heap_mem(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_PvAllocHeapMem",
        }
    );
    // row 175 _PvResizeHeapMem
    assert_eq!(
        _pv_resize_heap_mem(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_PvResizeHeapMem",
        }
    );
    // row 176 _FFreeHeapMem
    assert_eq!(
        _f_free_heap_mem(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_FFreeHeapMem",
        }
    );
    // row 197 _PpdbGetProcess
    assert_eq!(
        _ppdb_get_process(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_PpdbGetProcess",
        }
    );
    // row 198 _ErcGetError
    assert_eq!(
        _erc_get_error(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_ErcGetError",
        }
    );
    // row 199 _SetError
    assert_eq!(
        _set_error(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_SetError",
        }
    );
    // row 224 _CbSearchPath
    assert_eq!(
        _cb_search_path(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_CbSearchPath",
        }
    );
    // row 225 _CbSearchBinPath
    assert_eq!(
        _cb_search_bin_path(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_CbSearchBinPath",
        }
    );
    // row 231 _FFindEnvVar
    assert_eq!(
        _f_find_env_var(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_FFindEnvVar",
        }
    );
    // row 252 _SzFileFromImte
    assert_eq!(
        _sz_file_from_imte(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_SzFileFromImte",
        }
    );
    // row 253 _ImteFromFileSz
    assert_eq!(
        _imte_from_file_sz(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_ImteFromFileSz",
        }
    );
    // row 254 _ImteFromSz
    assert_eq!(
        _imte_from_sz(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_ImteFromSz",
        }
    );
    // row 255 _FGetProcAddr
    assert_eq!(
        _f_get_proc_addr(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_FGetProcAddr",
        }
    );
    // row 256 _KLoadLibrary
    assert_eq!(
        _k_load_library(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_KLoadLibrary",
        }
    );
    // row 257 _KFreeLibrary
    assert_eq!(
        _k_free_library(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_KFreeLibrary",
        }
    );
    // row 272 _lmaUserBase
    assert_eq!(
        _lma_user_base(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_lmaUserBase",
        }
    );
    // row 326 _KrnOutputDebugString
    assert_eq!(
        _krn_output_debug_string(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_KrnOutputDebugString",
        }
    );
    // row 360 _RtlRaiseException
    assert_eq!(
        _rtl_raise_exception(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_RtlRaiseException",
        }
    );
    // row 361 _RtlUnwind
    assert_eq!(
        _rtl_unwind(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_RtlUnwind",
        }
    );
    // row 362 _RtlUnwind4
    assert_eq!(
        _rtl_unwind4(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_RtlUnwind4",
        }
    );
    // row 402 _DfhCreateFile
    assert_eq!(
        _dfh_create_file(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_DfhCreateFile",
        }
    );
    // row 403 _FCloseFile
    assert_eq!(
        _f_close_file(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_FCloseFile",
        }
    );
    // row 404 _CbWriteFile
    assert_eq!(
        _cb_write_file(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_CbWriteFile",
        }
    );
    // row 405 _LfoSetFilePos
    assert_eq!(
        _lfo_set_file_pos(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_LfoSetFilePos",
        }
    );
    // row 406 _CbReadFile
    assert_eq!(
        _cb_read_file(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_CbReadFile",
        }
    );
    // row 500 _SelOffsetToLinear
    assert_eq!(
        _sel_offset_to_linear(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_SelOffsetToLinear",
        }
    );
    // row 501 _ImteFromHModule
    assert_eq!(
        _imte_from_h_module(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_ImteFromHModule",
        }
    );
    // row 502 _HModuleFromImte
    assert_eq!(
        _h_module_from_imte(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_HModuleFromImte",
        }
    );
    // row 503 _ExitPEApp
    assert_eq!(
        _exit_pe_app(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_ExitPEApp",
        }
    );
    // row 505 _Dos3Call
    assert_eq!(
        _dos3_call(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_Dos3Call",
        }
    );
    // row 506 _GetPEInstanceData
    assert_eq!(
        _get_pe_instance_data(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_GetPEInstanceData",
        }
    );
    // row 507 _GetThreadTask
    assert_eq!(
        _get_thread_task(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_GetThreadTask",
        }
    );
    // row 508 _BaseAddrFromImte
    assert_eq!(
        _base_addr_from_imte(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_BaseAddrFromImte",
        }
    );
    // row 509 _GetTaskId
    assert_eq!(
        _get_task_id(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_GetTaskId",
        }
    );
    // row 510 _KGetProcessDebugPort
    assert_eq!(
        _k_get_process_debug_port(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_KGetProcessDebugPort",
        }
    );
    // row 511 _W32S_BackTo32
    assert_eq!(
        _w32_s_back_to32(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_W32S_BackTo32",
        }
    );
    // row 512 _KReadProcessMemory32
    assert_eq!(
        _k_read_process_memory32(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_KReadProcessMemory32",
        }
    );
    // row 513 _KWriteProcessMemory32
    assert_eq!(
        _k_write_process_memory32(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_KWriteProcessMemory32",
        }
    );
    // row 514 _KGetThreadContext32
    assert_eq!(
        _k_get_thread_context32(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_KGetThreadContext32",
        }
    );
    // row 515 _KSetThreadContext32
    assert_eq!(
        _k_set_thread_context32(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_KSetThreadContext32",
        }
    );
    // row 516 _KQueryPerformanceCounter
    assert_eq!(
        _k_query_performance_counter(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_KQueryPerformanceCounter",
        }
    );
    // row 518 _KGetExitCodeThread32
    assert_eq!(
        _k_get_exit_code_thread32(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_KGetExitCodeThread32",
        }
    );
    // row 519 _KGetExitCodeProcess32
    assert_eq!(
        _k_get_exit_code_process32(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_KGetExitCodeProcess32",
        }
    );
    // row 520 _KGetTaskPpdb32
    assert_eq!(
        _k_get_task_ppdb32(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_KGetTaskPpdb32",
        }
    );
    // row 521 _KGetThreadPtdb32
    assert_eq!(
        _k_get_thread_ptdb32(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_KGetThreadPtdb32",
        }
    );
    // row 600 _GetCurProcFIOData
    assert_eq!(
        _get_cur_proc_fio_data(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_GetCurProcFIOData",
        }
    );
    // row 601 _KUseObject32
    assert_eq!(
        _k_use_object32(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_KUseObject32",
        }
    );
    // row 602 _KUnuseObject32
    assert_eq!(
        _k_unuse_object32(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_KUnuseObject32",
        }
    );
    // row 603 _KErcGetError32
    assert_eq!(
        _k_erc_get_error32(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_KErcGetError32",
        }
    );
    // row 604 _KCreateProcessHeap
    assert_eq!(
        _k_create_process_heap(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_KCreateProcessHeap",
        }
    );
    // row 605 _KGetSystemInfo
    assert_eq!(
        _k_get_system_info(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_KGetSystemInfo",
        }
    );
    // row 606 _KGlobalMemStat
    assert_eq!(
        _k_global_mem_stat(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_KGlobalMemStat",
        }
    );
    // row 607 _GetHandleFromAddr
    assert_eq!(
        _get_handle_from_addr(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_GetHandleFromAddr",
        }
    );
    // row 608 _TlsNullSlot
    assert_eq!(
        _tls_null_slot(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_TlsNullSlot",
        }
    );
    // row 609 _FWorkingSetSize
    assert_eq!(
        _f_working_set_size(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "w32skrnl.dll",
            function: "_FWorkingSetSize",
        }
    );
}

#[test]
fn oracle_get_win32s_directory_row() {
    // row 014 _kGetWin32sDirectory@0 with system dir C:\\Windows\\System32
    assert_eq!(
        get_win32s_directory("C:\\Windows\\System32"),
        "C:\\Windows\\System32\\win32s"
    );
}
