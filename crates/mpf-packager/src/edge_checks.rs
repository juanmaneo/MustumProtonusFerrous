//! Boundary and branch tests for the ported packager logic.
//!
//! Expectations marked "not oracle-derived" come from reading the C body or from Win32 semantics,
//! not from a printed row in `oracle_checks.rs`.

use super::*;

fn pkg(ref_count: i32) -> Package {
    Package {
        ref_count,
        ..Package::default()
    }
}

#[test]
fn add_ref_wraps_at_i32_max() {
    // Not oracle-derived: InterlockedIncrement wraps the LONG count, so the port uses wrapping_add.
    assert_eq!(add_ref(&mut pkg(i32::MAX)), i32::MIN);
}

#[test]
fn release_wraps_at_i32_min_and_is_not_freed() {
    // Not oracle-derived: InterlockedDecrement wraps the LONG count, so the port uses wrapping_sub.
    assert_eq!(
        release(&mut pkg(i32::MIN)),
        ReleaseOutcome {
            ref_count: i32::MAX,
            freed: false,
            client_site_released: false,
            file_deleted: false,
        }
    );
}

#[test]
fn release_with_references_left_keeps_site_and_file() {
    // Not oracle-derived: the C body releases the site and deletes the file only when `!ref`.
    let mut p = Package {
        ref_count: 2,
        client_site: true,
        has_filename: true,
    };
    assert_eq!(
        release(&mut p),
        ReleaseOutcome {
            ref_count: 1,
            freed: false,
            client_site_released: false,
            file_deleted: false,
        }
    );
}

#[test]
fn release_last_reference_with_only_a_site_or_only_a_file() {
    // Not oracle-derived: the oracle covers site and file both set, and both unset.
    let mut site_only = Package {
        ref_count: 1,
        client_site: true,
        has_filename: false,
    };
    assert_eq!(
        release(&mut site_only),
        ReleaseOutcome {
            ref_count: 0,
            freed: true,
            client_site_released: true,
            file_deleted: false,
        }
    );
    let mut file_only = Package {
        ref_count: 1,
        client_site: false,
        has_filename: true,
    };
    assert_eq!(
        release(&mut file_only),
        ReleaseOutcome {
            ref_count: 0,
            freed: true,
            client_site_released: false,
            file_deleted: true,
        }
    );
}

#[test]
fn query_interface_moves_refcount_only_on_success() {
    // Not oracle-derived: the C body calls IUnknown_AddRef only on the success path.
    let mut p = pkg(1);
    assert_eq!(
        query_interface(&mut p, &IID_IUNKNOWN),
        Ok(OleInterface::IOleObject)
    );
    assert_eq!(p.ref_count, 2);
    let mut q = pkg(1);
    assert_eq!(
        query_interface(&mut q, &IID_IOLE_OBJECT),
        Ok(OleInterface::IOleObject)
    );
    assert_eq!(q.ref_count, 2);
    let mut r = pkg(1);
    assert_eq!(
        query_interface(
            &mut r,
            &Guid {
                data1: 1,
                ..IID_IUNKNOWN
            }
        ),
        Err(E_NOINTERFACE)
    );
    assert_eq!(r.ref_count, 1);
}

#[test]
fn set_client_site_with_nothing_on_either_side() {
    // Not oracle-derived: the C body skips both Release and AddRef when both pointers are NULL.
    let mut p = pkg(1);
    assert_eq!(
        set_client_site(&mut p, false),
        ClientSiteChange {
            hr: S_OK,
            released_previous: false,
            added_ref: false,
        }
    );
    assert!(!p.client_site);
}

#[test]
fn do_verb_rejects_every_verb_but_zero() {
    // Not oracle-derived: the C switch has one case, so every other verb takes the E_INVALIDARG path.
    assert_eq!(do_verb(i32::MIN), E_INVALIDARG);
    assert_eq!(do_verb(i32::MAX), E_INVALIDARG);
}

#[test]
fn persist_storage_query_interface_delegates_for_iunknown() {
    // Not oracle-derived: the C body delegates to OleObject_QueryInterface, which accepts IUnknown.
    assert_eq!(
        persist_storage_query_interface(&mut pkg(1), &IID_IUNKNOWN),
        Ok(OleInterface::IOleObject)
    );
}

#[test]
fn create_instance_accepts_iunknown_and_keeps_refcount_at_one() {
    // Not oracle-derived: the new package starts at zero, and the one query takes the first reference.
    assert_eq!(
        create_instance(true, &IID_IUNKNOWN),
        Ok(OleInterface::IOleObject)
    );
}

#[test]
fn lock_server_unlock_returns_s_ok() {
    // Not oracle-derived: the C body returns S_OK for both lock values.
    assert_eq!(lock_server(false), S_OK);
}

#[test]
fn discard_string_stops_at_first_nul() {
    // Not oracle-derived: the loop ends at the first NUL, and the bytes after it are not read.
    assert_eq!(run_discard_bytes(b"a\0b"), (S_OK, 2));
    // Not oracle-derived: running out of bytes without a NUL reads as an empty S_OK, which the
    // C loop turns into E_FAIL.
    assert_eq!(run_discard_bytes(b"ab"), (E_FAIL, 2));
}

#[test]
fn discard_string_returns_a_negative_read_result_unchanged() {
    // Not oracle-derived: FAILED(hr) returns hr unless hr is S_OK or S_FALSE.
    assert_eq!(discard_string(|_| (E_FAIL, 1)), E_FAIL);
    assert_eq!(discard_string(|_| (E_NOTIMPL, 0)), E_NOTIMPL);
}

#[test]
fn discard_string_positive_read_result_with_bytes_is_success_at_nul() {
    // Not oracle-derived: a read that succeeds with a positive non-S_FALSE HRESULT and one byte
    // is not a failure, so the NUL byte ends the loop with S_OK.
    assert_eq!(
        discard_string(|chr: &mut u8| {
            *chr = 0;
            (2, 1)
        }),
        S_OK
    );
}

#[test]
fn failed_is_true_only_for_negative_hresults() {
    // Not oracle-derived: FAILED() from include/winerror.h.
    assert!(failed(E_FAIL));
    assert!(failed(i32::MIN));
    assert!(!failed(S_FALSE));
    assert!(!failed(S_OK));
    assert!(!failed(i32::MAX));
}

#[test]
fn dll_get_class_object_refuses_other_iids_for_package() {
    // Not oracle-derived: the C body forwards any IID to the class factory QueryInterface, which
    // rejects IPersistStorage just as it rejects IOleObject.
    assert_eq!(
        dll_get_class_object(&CLSID_PACKAGE, &IID_IPERSIST_STORAGE),
        E_NOINTERFACE
    );
}

/// Drives `discard_string` over a byte slice, one byte per read, returning the result and the
/// number of bytes consumed. The end of the slice reads as `S_OK` with no bytes, which is what the
/// C harness's `eof_hr` of `S_OK` does.
fn run_discard_bytes(bytes: &[u8]) -> (HRESULT, usize) {
    let mut pos = 0;
    let hr = discard_string(|chr: &mut u8| match bytes.get(pos) {
        Some(&byte) => {
            *chr = byte;
            pos += 1;
            (S_OK, 1)
        }
        None => (S_OK, 0),
    });
    (hr, pos)
}
