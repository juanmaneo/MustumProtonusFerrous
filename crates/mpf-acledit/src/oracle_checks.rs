//! Oracle-derived test vectors for the ported acledit logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of
//! `dlls/acledit/main.c` (`FMExtensionProcW`), `libs/winecrt0/dll_main.c`
//! (default `DllMain`) and `libs/winecrt0/stub.c`
//! (`__wine_spec_unimplemented_stub`) with `gcc -fsanitize=address,undefined -g`
//! under `target/canon/acledit/`. The 20 printed rows were all kept: neither
//! sanitizer reported anything, so no vector is excluded.

use super::*;

#[test]
fn oracle_fm_extension_proc_w_rows() {
    // zero
    assert_eq!(fm_extension_proc_w(0, 0, 0), 0);
    // wm_user_mixed
    assert_eq!(fm_extension_proc_w(0x1234, 0x0400, 0x5678), 0);
    // max_event_neg1_lparam
    assert_eq!(fm_extension_proc_w(0x1, usize::MAX, -1), 0);
    // lparam_intptr_min
    assert_eq!(fm_extension_proc_w(0x2, 1, isize::MIN), 0);
    // lparam_intptr_max
    assert_eq!(fm_extension_proc_w(0x2, 0, isize::MAX), 0);
    // hwnd_uintptr_max
    assert_eq!(fm_extension_proc_w(usize::MAX, 2, 2), 0);
}

#[test]
fn oracle_dll_main_rows() {
    // detach_reason0
    assert_eq!(
        dll_main(0),
        DllMainOutcome {
            ret: true,
            disable_thread_library_calls: false
        }
    );
    // attach_reason1
    assert_eq!(
        dll_main(1),
        DllMainOutcome {
            ret: true,
            disable_thread_library_calls: true
        }
    );
    // attach_reserved_nonnull (inst and reserved do not change the outcome)
    assert_eq!(
        dll_main(DLL_PROCESS_ATTACH),
        DllMainOutcome {
            ret: true,
            disable_thread_library_calls: true
        }
    );
    // thread_attach_reason2
    assert_eq!(
        dll_main(2),
        DllMainOutcome {
            ret: true,
            disable_thread_library_calls: false
        }
    );
    // thread_detach_reason3
    assert_eq!(
        dll_main(3),
        DllMainOutcome {
            ret: true,
            disable_thread_library_calls: false
        }
    );
    // reason_99
    assert_eq!(
        dll_main(99),
        DllMainOutcome {
            ret: true,
            disable_thread_library_calls: false
        }
    );
    // reason_u32_max
    assert_eq!(
        dll_main(u32::MAX),
        DllMainOutcome {
            ret: true,
            disable_thread_library_calls: false
        }
    );
    // detach_reserved_nonnull (reserved does not change the outcome)
    assert_eq!(
        dll_main(0),
        DllMainOutcome {
            ret: true,
            disable_thread_library_calls: false
        }
    );
}

#[test]
fn oracle_stub_export_rows() {
    // edit_audit_info
    assert_eq!(
        export_stub("EditAuditInfo"),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            module: "acledit.dll",
            function: "EditAuditInfo"
        }
    );
    // edit_owner_info
    assert_eq!(
        export_stub("EditOwnerInfo"),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            module: "acledit.dll",
            function: "EditOwnerInfo"
        }
    );
    // edit_permission_info
    assert_eq!(
        export_stub("EditPermissionInfo"),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            module: "acledit.dll",
            function: "EditPermissionInfo"
        }
    );
    // sed_discretionary_acl_editor
    assert_eq!(
        export_stub("SedDiscretionaryAclEditor"),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            module: "acledit.dll",
            function: "SedDiscretionaryAclEditor"
        }
    );
    // sed_system_acl_editor
    assert_eq!(
        export_stub("SedSystemAclEditor"),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            module: "acledit.dll",
            function: "SedSystemAclEditor"
        }
    );
    // sed_take_ownership
    assert_eq!(
        export_stub("SedTakeOwnership"),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            module: "acledit.dll",
            function: "SedTakeOwnership"
        }
    );
}
