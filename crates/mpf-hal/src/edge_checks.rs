//! Edge-case tests for the ported hal logic.
//!
//! These cover the count boundaries of `ExTryToAcquireFastMutex`, the pointer arguments that the
//! C bodies never dereference, the stub records, and the `hal.spec` export table. Not every
//! expectation is oracle-derived. The ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    // Not oracle-derived: `MODULE = hal.dll` in dlls/hal/Makefile.in.
    assert_eq!(hal_clear_software_interrupt().module, "hal.dll");
}

#[test]
fn fast_mutex_only_count_one_is_free() {
    // Not oracle-derived: the C comparand is 1, so the values around it and the extremes stay held.
    for count in [i32::MIN, i32::MIN + 1, -1, 0, 2, i32::MAX - 1, i32::MAX] {
        let mut mutex = FastMutex { count };
        assert!(!ex_try_to_acquire_fast_mutex(&mut mutex));
        assert_eq!(mutex.count, count);
    }

    // Not oracle-derived: a second try on a taken mutex fails and leaves it at 0.
    let mut mutex = FastMutex { count: 1 };
    assert!(ex_try_to_acquire_fast_mutex(&mut mutex));
    assert!(!ex_try_to_acquire_fast_mutex(&mut mutex));
    assert_eq!(mutex.count, 0);
}

#[test]
fn spin_lock_irql_is_zero_for_any_start_value() {
    // Not oracle-derived: the C body overwrites *irql, so every starting value gives 0.
    for start in [0u8, 1, 0x7f, 0x80, u8::MAX] {
        let mut irql = start;
        ke_acquire_spin_lock(&mut irql);
        assert_eq!(irql, 0);
    }
    assert_eq!(kf_acquire_spin_lock(), 0);
}

#[test]
fn irql_functions_ignore_the_requested_level() {
    // Not oracle-derived: the C bodies only pass the level to FIXME.
    for level in [0u8, 1, 2, 15, 31, u8::MAX] {
        kf_lower_irql(level);
        assert_eq!(kf_raise_irql(level), 0);
    }
    assert_eq!(ke_get_current_irql(), 0);
}

#[test]
fn port_functions_never_touch_the_port() {
    // Not oracle-derived: the C bodies never dereference the port, so a null port is accepted.
    assert_eq!(read_port_uchar(std::ptr::null_mut()), 0);
    assert_eq!(read_port_ulong(std::ptr::null_mut()), 0);
    write_port_uchar(std::ptr::null_mut(), u8::MAX);
    write_port_ulong(std::ptr::null_mut(), u32::MAX);

    // Not oracle-derived: extreme values written to a live port leave the port unchanged.
    let mut cell = 0xa5u8;
    write_port_uchar(&mut cell, 0);
    assert_eq!(cell, 0xa5);
    let mut wide = 0xa5a5_a5a5u32;
    write_port_ulong(&mut wide, u32::MAX);
    assert_eq!(wide, 0xa5a5_a5a5);
}

#[test]
fn bus_functions_claim_there_is_no_bus() {
    // Not oracle-derived: the C bodies return 0 or FALSE for every input, and write nothing.
    assert_eq!(
        hal_get_bus_data(i32::MAX, u32::MAX, u32::MAX, std::ptr::null_mut(), u32::MAX),
        0
    );
    assert_eq!(
        hal_get_bus_data_by_offset(-1, 0, 0, std::ptr::null_mut(), u32::MAX, u32::MAX),
        0
    );
    assert!(!hal_translate_bus_address(
        -1,
        u32::MAX,
        i64::MIN,
        std::ptr::null_mut(),
        std::ptr::null_mut()
    ));
}

#[test]
fn stub_exception_carries_the_names_it_is_given() {
    // Not oracle-derived: the record keeps the module and function it is given, with the fixed
    // code, flags and parameter count.
    assert_eq!(
        wine_spec_unimplemented_stub("other.dll", "Other"),
        StubException {
            code: EXCEPTION_WINE_STUB,
            flags: EXCEPTION_NONCONTINUABLE,
            number_parameters: 2,
            module: "other.dll",
            function: "Other",
        }
    );
}

#[test]
fn export_table_matches_hal_spec() {
    // Not oracle-derived: counted from dlls/hal/hal.spec, which has 92 entries: 71 `@ stub`,
    // 20 functions defined in hal.c, and 1 forward.
    assert_eq!(EXPORTS.len(), 92);
    let count = |kind: ExportKind| EXPORTS.iter().filter(|(_, k)| *k == kind).count();
    assert_eq!(count(ExportKind::Stub), 71);
    assert_eq!(count(ExportKind::Function), 20);
    assert_eq!(count(ExportKind::Forward("ntoskrnl.exe.KeLowerIrql")), 1);
}

#[test]
fn export_kind_is_case_sensitive_and_rejects_unknown_names() {
    // Not oracle-derived: a sample of spec entries, and names the spec does not export.
    assert_eq!(export_kind("HalGetBusData"), Some(ExportKind::Function));
    assert_eq!(
        export_kind("HalClearSoftwareInterrupt"),
        Some(ExportKind::Stub)
    );
    assert_eq!(
        export_kind("KeLowerIrql"),
        Some(ExportKind::Forward("ntoskrnl.exe.KeLowerIrql"))
    );
    assert_eq!(export_kind("halgetbusdata"), None);
    assert_eq!(export_kind("NoSuchExport"), None);
    assert_eq!(export_kind(""), None);
}
