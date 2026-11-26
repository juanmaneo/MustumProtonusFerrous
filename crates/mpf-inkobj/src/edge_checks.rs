//! Edge-case tests for the ported inkobj logic.
//!
//! `DllGetClassObject` has no branches, so the edge cases are the boundary IDs (NULL, all-zero
//! and all-0xff) and the spec export table. Expectations that are not oracle-derived say so in
//! a comment.

use super::*;

const ZERO: Guid = [0; 16];
const ONES: Guid = [0xff; 16];

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: the value from include/winerror.h, `_HRESULT_TYPEDEF_(0x80040111)`.
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE, 0x8004_0111u32 as i32);
}

#[test]
fn boundary_ids_all_return_the_constant() {
    // Six of these nine combinations are oracle rows (01/02, 03, 05, 13, 14, 15). The other three,
    // (NULL, all-0xff), (all-zero, all-0xff) and (all-0xff, all-zero), are not oracle-derived.
    // The C body reads neither ID for its result, so the same value is expected for them.
    let ids: [Option<&Guid>; 3] = [None, Some(&ZERO), Some(&ONES)];
    for rclsid in ids {
        for riid in ids {
            assert_eq!(
                dll_get_class_object(rclsid, riid),
                CLASS_E_CLASSNOTAVAILABLE
            );
        }
    }
}

#[test]
fn export_table_matches_spec() {
    // Not oracle-derived: the four export lines of dlls/inkobj/inkobj.spec, in order.
    assert_eq!(
        export_kind("DllGetClassObject"),
        Some(ExportKind::Implemented)
    );
    for name in [
        "DllCanUnloadNow",
        "DllRegisterServer",
        "DllUnregisterServer",
    ] {
        assert_eq!(export_kind(name), Some(ExportKind::PrivateWithoutBody));
    }
    assert_eq!(export_kind("dllgetclassobject"), None);
    assert_eq!(export_kind(""), None);
}
