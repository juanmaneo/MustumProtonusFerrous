//! Oracle-derived test vectors for the ported vdmdbg logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `VDMEnumProcessWOW` and
//! `VDMEnumTaskWOW` (`dlls/vdmdbg/vdmdbg.c`) and `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/vdmdbg/`, and running the harness. The 23 printed rows ran with no ASan or
//! UBSan reports, so none are excluded.

use super::*;

#[test]
fn oracle_stub_rows() {
    // row 01 stub VDMBreakThread: raised=1 code=0x80000100 flags=0x1 nparams=2 module=vdmdbg.dll function=VDMBreakThread
    assert_eq!(
        vdm_break_thread(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "vdmdbg.dll",
            function: "VDMBreakThread",
        }
    );

    // row 02 stub VDMDetectWOW: raised=1 code=0x80000100 flags=0x1 nparams=2 module=vdmdbg.dll function=VDMDetectWOW
    assert_eq!(
        vdm_detect_wow(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "vdmdbg.dll",
            function: "VDMDetectWOW",
        }
    );

    // row 03 stub VDMEnumTaskWOWEx: raised=1 code=0x80000100 flags=0x1 nparams=2 module=vdmdbg.dll function=VDMEnumTaskWOWEx
    assert_eq!(
        vdm_enum_task_wow_ex(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "vdmdbg.dll",
            function: "VDMEnumTaskWOWEx",
        }
    );

    // row 04 stub VDMGetModuleSelector: raised=1 code=0x80000100 flags=0x1 nparams=2 module=vdmdbg.dll function=VDMGetModuleSelector
    assert_eq!(
        vdm_get_module_selector(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "vdmdbg.dll",
            function: "VDMGetModuleSelector",
        }
    );

    // row 05 stub VDMGetPointer: raised=1 code=0x80000100 flags=0x1 nparams=2 module=vdmdbg.dll function=VDMGetPointer
    assert_eq!(
        vdm_get_pointer(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "vdmdbg.dll",
            function: "VDMGetPointer",
        }
    );

    // row 06 stub VDMGetSelectorModule: raised=1 code=0x80000100 flags=0x1 nparams=2 module=vdmdbg.dll function=VDMGetSelectorModule
    assert_eq!(
        vdm_get_selector_module(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "vdmdbg.dll",
            function: "VDMGetSelectorModule",
        }
    );

    // row 07 stub VDMGetThreadContext: raised=1 code=0x80000100 flags=0x1 nparams=2 module=vdmdbg.dll function=VDMGetThreadContext
    assert_eq!(
        vdm_get_thread_context(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "vdmdbg.dll",
            function: "VDMGetThreadContext",
        }
    );

    // row 08 stub VDMGetThreadSelectorEntry: raised=1 code=0x80000100 flags=0x1 nparams=2 module=vdmdbg.dll function=VDMGetThreadSelectorEntry
    assert_eq!(
        vdm_get_thread_selector_entry(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "vdmdbg.dll",
            function: "VDMGetThreadSelectorEntry",
        }
    );

    // row 09 stub VDMGlobalFirst: raised=1 code=0x80000100 flags=0x1 nparams=2 module=vdmdbg.dll function=VDMGlobalFirst
    assert_eq!(
        vdm_global_first(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "vdmdbg.dll",
            function: "VDMGlobalFirst",
        }
    );

    // row 10 stub VDMGlobalNext: raised=1 code=0x80000100 flags=0x1 nparams=2 module=vdmdbg.dll function=VDMGlobalNext
    assert_eq!(
        vdm_global_next(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "vdmdbg.dll",
            function: "VDMGlobalNext",
        }
    );

    // row 11 stub VDMKillWOW: raised=1 code=0x80000100 flags=0x1 nparams=2 module=vdmdbg.dll function=VDMKillWOW
    assert_eq!(
        vdm_kill_wow(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "vdmdbg.dll",
            function: "VDMKillWOW",
        }
    );

    // row 12 stub VDMModuleFirst: raised=1 code=0x80000100 flags=0x1 nparams=2 module=vdmdbg.dll function=VDMModuleFirst
    assert_eq!(
        vdm_module_first(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "vdmdbg.dll",
            function: "VDMModuleFirst",
        }
    );

    // row 13 stub VDMModuleNext: raised=1 code=0x80000100 flags=0x1 nparams=2 module=vdmdbg.dll function=VDMModuleNext
    assert_eq!(
        vdm_module_next(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "vdmdbg.dll",
            function: "VDMModuleNext",
        }
    );

    // row 14 stub VDMProcessException: raised=1 code=0x80000100 flags=0x1 nparams=2 module=vdmdbg.dll function=VDMProcessException
    assert_eq!(
        vdm_process_exception(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "vdmdbg.dll",
            function: "VDMProcessException",
        }
    );

    // row 15 stub VDMSetThreadContext: raised=1 code=0x80000100 flags=0x1 nparams=2 module=vdmdbg.dll function=VDMSetThreadContext
    assert_eq!(
        vdm_set_thread_context(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "vdmdbg.dll",
            function: "VDMSetThreadContext",
        }
    );

    // row 16 stub VDMStartTaskInWOW: raised=1 code=0x80000100 flags=0x1 nparams=2 module=vdmdbg.dll function=VDMStartTaskInWOW
    assert_eq!(
        vdm_start_task_in_wow(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "vdmdbg.dll",
            function: "VDMStartTaskInWOW",
        }
    );

    // row 17 stub VDMTerminateTaskWOW: raised=1 code=0x80000100 flags=0x1 nparams=2 module=vdmdbg.dll function=VDMTerminateTaskWOW
    assert_eq!(
        vdm_terminate_task_wow(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "vdmdbg.dll",
            function: "VDMTerminateTaskWOW",
        }
    );
}

#[test]
fn oracle_enum_rows() {
    // row 18 VDMEnumProcessWOW(fp=NULL, lparam=0) -> 0
    assert_eq!(vdm_enum_process_wow(), 0);
    // row 19 VDMEnumProcessWOW(fp=dummy, lparam=42) -> 0
    assert_eq!(vdm_enum_process_wow(), 0);
    // row 20 VDMEnumProcessWOW(fp=dummy, lparam=-1) -> 0
    assert_eq!(vdm_enum_process_wow(), 0);
    // row 21 VDMEnumTaskWOW(pid=0, fp=NULL, lparam=0) -> 0
    assert_eq!(vdm_enum_task_wow(), 0);
    // row 22 VDMEnumTaskWOW(pid=1234, fp=dummy, lparam=7) -> 0
    assert_eq!(vdm_enum_task_wow(), 0);
    // row 23 VDMEnumTaskWOW(pid=0xffffffff, fp=dummy, lparam=-1) -> 0
    assert_eq!(vdm_enum_task_wow(), 0);
}
