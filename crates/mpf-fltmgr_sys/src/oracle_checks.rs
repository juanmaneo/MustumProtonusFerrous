//! Oracle-derived vectors for the ported fltmgr.sys logic.
//!
//! Captured by compiling the verbatim Wine C bodies of `dlls/fltmgr.sys/main.c`
//! with minimal ntdll and ntoskrnl shims that mirror `dlls/ntdll/sec.c`, under
//! `target/canon/fltmgr.sys/`, with `gcc -fsanitize=address,undefined -g`.
//! The harness printed 22 rows. ASan and UBSan flagged none, so none are excluded.

use super::*;

/// Oracle fields shared by the four successful default-descriptor rows.
fn assert_default_descriptor(sd: &SecurityDescriptor, access: u32) {
    assert_eq!(sd.revision, 1);
    assert_eq!(sd.control, 0x0004);
    assert_eq!(sd.alloc_len, 100);
    let dacl = sd.dacl.as_ref().expect("dacl");
    assert_eq!(dacl.revision, 2);
    assert_eq!(dacl.size, 60);
    assert_eq!(dacl.aces.len(), 2);

    let admins = &dacl.aces[0];
    assert_eq!(admins.mask, access);
    assert_eq!(admins.size(), 24);
    assert_eq!(admins.sid.revision, 1);
    assert_eq!(admins.sid.sub_authority, vec![32, 512]);

    let system = &dacl.aces[1];
    assert_eq!(system.mask, access);
    assert_eq!(system.size(), 20);
    assert_eq!(system.sid.revision, 1);
    assert_eq!(system.sid.sub_authority, vec![18]);
}

/// Runs the builder and records each requested size. `fail_at` is the 1-based
/// call that returns NULL, or `None` for all successes.
fn build_recording(
    access: u32,
    fail_at: Option<usize>,
) -> (Result<SecurityDescriptor, NtStatus>, Vec<u32>) {
    let mut sizes = Vec::new();
    let result = build_default_security_descriptor_with(access, |size| {
        sizes.push(size);
        Some(sizes.len()) != fail_at
    });
    (result, sizes)
}

// sd_access_0x0, sd_access_0x1f01ff, sd_access_0xffffffff, sd_access_0x80000000
#[test]
fn oracle_default_descriptor_success_rows() {
    for access in [0x0, 0x001f_01ff, 0xffff_ffff, 0x8000_0000] {
        let (result, sizes) = build_recording(access, None);
        assert_eq!(sizes, vec![16, 12, 100]);
        assert_default_descriptor(&result.expect("status 0"), access);
    }
}

// sd_fail_admin_sid_alloc: status 0xc000009a, sd_null=1, alloc=16
#[test]
fn oracle_default_descriptor_admin_sid_alloc_fails() {
    let (result, sizes) = build_recording(0x1, Some(1));
    assert_eq!(result, Err(STATUS_INSUFFICIENT_RESOURCES));
    assert_eq!(sizes, vec![16]);
}

// sd_fail_system_sid_alloc: status 0xc000009a, sd_null=1, alloc=16,12
#[test]
fn oracle_default_descriptor_system_sid_alloc_fails() {
    let (result, sizes) = build_recording(0x1, Some(2));
    assert_eq!(result, Err(STATUS_INSUFFICIENT_RESOURCES));
    assert_eq!(sizes, vec![16, 12]);
}

// sd_fail_descriptor_alloc: status 0xc0000017, sd_null=1, alloc=16,12,100
#[test]
fn oracle_default_descriptor_descriptor_alloc_fails() {
    let (result, sizes) = build_recording(0x1, Some(3));
    assert_eq!(result, Err(STATUS_NO_MEMORY));
    assert_eq!(sizes, vec![16, 12, 100]);
}

// driver_entry, register_filter_nonnull, register_filter_null,
// start_filtering, the nine NOT_SUPPORTED stubs, enumerate_volumes (x2)
#[test]
fn oracle_stub_rows() {
    assert_eq!(driver_entry(), 0x0000_0000);

    assert_eq!(flt_register_filter(true), (0x0000_0000, Some(0xdead_beaf)));
    assert_eq!(flt_register_filter(false), (0x0000_0000, None));

    assert_eq!(flt_start_filtering(), 0x0000_0000);

    assert_eq!(flt_allocate_context(), 0xC000_00BB);
    assert_eq!(flt_get_stream_context(), 0xC000_00BB);
    assert_eq!(flt_set_stream_context(), 0xC000_00BB);
    assert_eq!(flt_query_information_file(), 0xC000_00BB);
    assert_eq!(flt_get_file_name_information_unsafe(), 0xC000_00BB);
    assert_eq!(flt_read_file(), 0xC000_00BB);
    assert_eq!(flt_parse_file_name_information(), 0xC000_00BB);
    assert_eq!(flt_get_file_name_information(), 0xC000_00BB);
    assert_eq!(flt_create_communication_port(), 0xC000_00BB);

    // enumerate_volumes_nonnull: returned=0
    let mut returned = 0xdead_beef;
    assert_eq!(flt_enumerate_volumes(Some(&mut returned)), 0xC000_00BB);
    assert_eq!(returned, 0);

    // enumerate_volumes_null: nothing to write
    assert_eq!(flt_enumerate_volumes(None), 0xC000_00BB);
}
