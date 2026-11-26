//! Edge-case tests for the modelled difxapi stubs.
//!
//! The stubs return fixed values, so the boundary cases are input extremes and
//! out-parameter handling. Each expectation that the oracle did not run says so
//! in a comment.

use super::*;

fn utf16(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

#[test]
fn constants_match_winerror_and_windef() {
    // Not oracle-derived: values read from target/repos/wine/include/winerror.h
    // (ERROR_UNSUPPORTED_TYPE is line 1333) and windef.h. The oracle shim uses the same numbers.
    assert_eq!(ERROR_SUCCESS, 0);
    assert_eq!(ERROR_UNSUPPORTED_TYPE, 1630);
    assert_eq!(FALSE, 0);
}

#[test]
fn clear_reboot_null_and_set_branches() {
    // The null branch is oracle-derived (rows I2 and U2 pass reboot=(null)).
    clear_reboot(None);
    // Not oracle-derived: the oracle wrote only 1 and -1. The other values follow
    // from the unconditional `*reboot = FALSE;` in main.c.
    for start in [i32::MIN, -1, 0, 1, i32::MAX] {
        let mut reboot = start;
        clear_reboot(Some(&mut reboot));
        assert_eq!(reboot, FALSE);
    }
}

#[test]
fn preinstall_ignores_inf_and_flags() {
    // Not oracle-derived for these inputs: the oracle used flags 0, 1, 0x80000000
    // and 0xffffffff. main.c ignores both arguments, so every combination returns success.
    for flags in [0, 0x1, 0x8, 0x20, 0x7fff_ffff, 0x8000_0000, u32::MAX] {
        assert_eq!(driver_package_preinstall_a(None, flags), ERROR_SUCCESS);
        assert_eq!(
            driver_package_preinstall_a(Some(&b"\xff\xfe\x00"[..]), flags),
            ERROR_SUCCESS
        );
        assert_eq!(driver_package_preinstall_w(None, flags), ERROR_SUCCESS);
        assert_eq!(
            driver_package_preinstall_w(Some(&utf16("\u{10ffff}")), flags),
            ERROR_SUCCESS
        );
    }
}

#[test]
fn install_and_uninstall_clear_reboot_for_any_info() {
    // Not oracle-derived: no oracle row pairs reboot with info set for install_w or uninstall_w.
    // main.c clears `*reboot` whatever `info` is, so the result does not depend on it.
    for info in [None, Some(&Opaque)] {
        let mut reboot = 7;
        assert_eq!(
            driver_package_install_a(Some(&b"a.inf"[..]), 0, info, Some(&mut reboot)),
            ERROR_SUCCESS
        );
        assert_eq!(reboot, FALSE);

        let mut reboot = 7;
        assert_eq!(
            driver_package_install_w(Some(&utf16("a.inf")), 0, info, Some(&mut reboot)),
            ERROR_SUCCESS
        );
        assert_eq!(reboot, FALSE);

        let mut reboot = 7;
        assert_eq!(
            driver_package_uninstall_a(Some(&b"a.inf"[..]), 0, info, Some(&mut reboot)),
            ERROR_SUCCESS
        );
        assert_eq!(reboot, FALSE);

        let mut reboot = 7;
        assert_eq!(
            driver_package_uninstall_w(Some(&utf16("a.inf")), 0, info, Some(&mut reboot)),
            ERROR_SUCCESS
        );
        assert_eq!(reboot, FALSE);
    }
}

#[test]
fn get_path_never_writes_dest_or_count() {
    // Not oracle-derived for these counts: the oracle used count_in 8 and 0. main.c
    // never stores through dest or count, so any count value and dest length is left alone.
    for start in [0u32, 1, 260, u32::MAX] {
        let mut count = start;
        let mut dest = [0x5a_u8; 4];
        assert_eq!(
            driver_package_get_path_a(Some(&b"z.inf"[..]), Some(&mut dest[..]), Some(&mut count)),
            ERROR_UNSUPPORTED_TYPE
        );
        assert_eq!(count, start);
        assert_eq!(dest, [0x5a; 4]);

        let mut wdest = [0x5a_u16; 4];
        assert_eq!(
            driver_package_get_path_w(
                Some(&utf16("z.inf")),
                Some(&mut wdest[..]),
                Some(&mut count)
            ),
            ERROR_UNSUPPORTED_TYPE
        );
        assert_eq!(count, start);
        assert_eq!(wdest, [0x5a; 4]);
    }
}

#[test]
fn get_path_accepts_empty_and_null_buffers() {
    // Not oracle-derived: the oracle used a non-empty dest or NULL dest. main.c does not
    // touch dest, so a zero-length buffer is also left alone.
    let mut count = 0;
    assert_eq!(
        driver_package_get_path_a(None, Some(&mut [][..]), Some(&mut count)),
        ERROR_UNSUPPORTED_TYPE
    );
    assert_eq!(
        driver_package_get_path_w(None, Some(&mut [][..]), None),
        ERROR_UNSUPPORTED_TYPE
    );
    assert_eq!(count, 0);
}

#[test]
fn log_callback_stubs_accept_every_pointer_combination() {
    // Not oracle-derived: the oracle used (set, set), (null, null) and (set, null).
    // The Opaque model has no invocation path, so these calls only need to return.
    for cb in [None, Some(&Opaque)] {
        for ctx in [None, Some(&Opaque)] {
            difxapi_set_log_callback_a(cb, ctx);
            difxapi_set_log_callback_w(cb, ctx);
            set_difx_log_callback_a(cb, ctx);
            set_difx_log_callback_w(cb, ctx);
        }
    }
}
