//! Edge-case tests for the ported mfmediaengine logic.
//!
//! These cover the capacity limit of the array rule, the inclusive ends and NaN behaviour of the
//! time ranges, the order in which the factory checks its arguments, and the export table. Not
//! every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::com::*;
use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/winternl.h and include/winnt.h.
    assert_eq!(S_OK, 0);
    assert_eq!(E_NOTIMPL, 0x8000_4001);
    assert_eq!(E_NOINTERFACE, 0x8000_4002);
    assert_eq!(E_INVALIDARG, 0x8007_0057);
    assert_eq!(CLASS_E_NOAGGREGATION, 0x8004_0110);
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE, 0x8004_0111);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn array_capacity_has_a_largest_allocation() {
    // Not oracle-derived: the limit max_capacity = ~0 / size in mf_array_reserve. A count at the
    // limit is allowed, and one past it fails.
    const SIZE: usize = 16;
    let max = usize::MAX / SIZE;
    assert_eq!(array_reserve_capacity(0, max, SIZE), Some(max));
    assert_eq!(array_reserve_capacity(0, max + 1, SIZE), None);
    assert_eq!(array_reserve_capacity(max, max + 1, SIZE), None);
    assert_eq!(array_reserve_capacity(max, max, SIZE), Some(max));
}

#[test]
fn array_capacity_doubles_until_the_count_fits() {
    // Not oracle-derived: the doubling loop of mf_array_reserve, from a capacity below the
    // minimum of 4, and a count that is an exact power of two.
    assert_eq!(array_reserve_capacity(0, 0, 16), Some(0));
    assert_eq!(array_reserve_capacity(2, 3, 16), Some(4));
    assert_eq!(array_reserve_capacity(4, 9, 16), Some(16));
    assert_eq!(array_reserve_capacity(4, 8, 16), Some(8));
    assert_eq!(array_reserve_capacity(0, usize::MAX, 1), Some(usize::MAX));
}

#[test]
fn time_range_contains_time_is_inclusive_at_both_ends() {
    // Not oracle-derived: the comparisons in time_range_ContainsTime. A NaN time is never inside
    // a range, and an empty table contains no time.
    assert!(!TimeRange::default().contains_time(0.0));

    let mut t = TimeRange::default();
    t.add_range(1.0, 2.0);
    assert!(t.contains_time(1.0));
    assert!(t.contains_time(2.0));
    assert!(!t.contains_time(0.5));
    assert!(!t.contains_time(2.5));
    assert!(!t.contains_time(f64::NAN));
}

#[test]
fn time_range_start_and_end_reject_only_indexes_past_the_count() {
    // Not oracle-derived: the index check of time_range_GetStart and time_range_GetEnd.
    let mut t = TimeRange::default();
    t.add_range(1.0, 2.0);
    t.add_range(5.0, 6.0);
    assert_eq!(t.start(0), Ok(1.0));
    assert_eq!(t.end(0), Ok(2.0));
    assert_eq!(t.start(1), Ok(5.0));
    assert_eq!(t.end(1), Ok(6.0));
    assert_eq!(t.end(2), Err(E_INVALIDARG));
    assert_eq!(TimeRange::default().end(0), Err(E_INVALIDARG));
}

#[test]
fn add_range_keeps_the_first_match_and_leaves_later_overlaps() {
    // Not oracle-derived: time_range_AddRange returns at the first stored range that matches, so a
    // later range that also overlaps the new one is left unmerged.
    let mut t = TimeRange::default();
    t.add_range(1.0, 2.0);
    t.add_range(5.0, 6.0);
    t.add_range(0.5, 7.0);
    assert_eq!(t.ranges(), &[range(0.5, 7.0), range(5.0, 6.0)]);
}

#[test]
fn add_range_merges_by_an_end_inside_the_stored_range() {
    // Not oracle-derived: the intersection branch that takes the new range's end inside the
    // stored range, with the new start before it.
    let mut t = TimeRange::default();
    t.add_range(1.0, 2.0);
    t.add_range(0.0, 1.5);
    assert_eq!(t.ranges(), &[range(0.0, 2.0)]);
}

#[test]
fn add_range_min_and_max_follow_the_c_macros_with_nan() {
    // Not oracle-derived: min(a, b) is (a < b) ? a : b and max(a, b) is (a > b) ? a : b in
    // include/minwindef.h. A NaN operand therefore takes the second value, unlike f64::min and
    // f64::max, which ignore NaN.
    let mut t = TimeRange::default();
    t.add_range(1.0, 2.0);
    t.add_range(f64::NAN, 1.5);
    assert!(t.ranges()[0].start.is_nan());
    assert_eq!(t.ranges()[0].end, 2.0);

    let mut t = TimeRange::default();
    t.add_range(1.0, 2.0);
    t.add_range(1.5, f64::NAN);
    assert_eq!(t.ranges()[0].start, 1.0);
    assert!(t.ranges()[0].end.is_nan());
}

#[test]
fn gcd_handles_zero_and_the_largest_values() {
    // Not oracle-derived: a zero operand returns the other one, and the largest unsigned values.
    assert_eq!(get_gcd(7, 0), 7);
    assert_eq!(get_gcd(0, 5), 5);
    assert_eq!(get_gcd(u32::MAX, u32::MAX - 1), 1);
    assert_eq!(get_gcd(u32::MAX, u32::MAX), u32::MAX);
}

#[test]
fn mftime_converts_negative_ticks() {
    // Not oracle-derived: the sign of the C conversion, with a whole number of seconds.
    assert_eq!(mftime_to_seconds(-10_000_000), -1.0);
    assert_eq!(mftime_to_seconds(10_000_000), 1.0);
}

#[test]
fn class_factory_query_interface_accepts_iclassfactory_and_iunknown() {
    // Not oracle-derived: the two IIDs that classfactory_QueryInterface accepts, and a third IID
    // it refuses.
    assert_eq!(ClassFactory.query_interface(&IID_ICLASSFACTORY), Ok(()));
    assert_eq!(ClassFactory.query_interface(&IID_IUNKNOWN), Ok(()));
    assert_eq!(
        ClassFactory.query_interface(&IID_IMF_MEDIA_ENGINE_CLASS_FACTORY),
        Err(E_NOINTERFACE)
    );
}

#[test]
fn media_engine_factory_query_interface_accepts_its_two_interfaces_and_iunknown() {
    // Not oracle-derived: the three IIDs that media_engine_factory_QueryInterface accepts, and
    // IClassFactory, which it refuses.
    for iid in [
        IID_IMF_MEDIA_ENGINE_CLASS_FACTORY,
        IID_IMF_MEDIA_ENGINE_CLASS_FACTORY_EX,
        IID_IUNKNOWN,
    ] {
        assert_eq!(MediaEngineClassFactory.query_interface(&iid), Ok(()));
    }
    assert_eq!(
        MediaEngineClassFactory.query_interface(&IID_ICLASSFACTORY),
        Err(E_NOINTERFACE)
    );
}

#[test]
fn create_instance_checks_outer_before_riid() {
    // Not oracle-derived: classfactory_CreateInstance tests outer before it calls QueryInterface,
    // so a non-NULL outer gives CLASS_E_NOAGGREGATION whatever the riid is.
    assert_eq!(
        ClassFactory.create_instance(true, &IID_IUNKNOWN),
        Err(CLASS_E_NOAGGREGATION)
    );
    assert_eq!(
        ClassFactory.create_instance(false, &IID_ICLASSFACTORY),
        Err(E_NOINTERFACE)
    );
}

#[test]
fn lock_server_returns_s_ok_for_both_values() {
    // Not oracle-derived: classfactory_LockServer ignores dolock, so FALSE also gives S_OK.
    assert_eq!(ClassFactory.lock_server(false), S_OK);
}

#[test]
fn dll_get_class_object_refuses_other_clsids_before_checking_riid() {
    // Not oracle-derived: DllGetClassObject compares the CLSID first, so an unknown class gives
    // CLASS_E_CLASSNOTAVAILABLE even for an IID the factory would accept.
    assert_eq!(
        dll_get_class_object(&IID_IUNKNOWN, &IID_IUNKNOWN),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
    assert_eq!(
        dll_get_class_object(&CLSID_MF_MEDIA_ENGINE_CLASS_FACTORY, &IID_IUNKNOWN),
        Ok(ClassFactory)
    );
}

#[test]
fn export_table_lists_the_spec_in_order() {
    // Not oracle-derived: the three entries of dlls/mfmediaengine/mfmediaengine.spec, and the
    // case-sensitive lookup.
    let names: Vec<&str> = EXPORTS.iter().map(|(name, _)| *name).collect();
    assert_eq!(
        names,
        [
            "DllCanUnloadNow",
            "DllGetActivationFactory",
            "DllGetClassObject"
        ]
    );
    assert_eq!(export_kind("DllCanUnloadNow"), Some(ExportKind::Undefined));
    assert_eq!(
        export_kind("DllGetActivationFactory"),
        Some(ExportKind::Stub)
    );
    assert_eq!(export_kind("DllGetClassObject"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("dllgetclassobject"), None);
    assert_eq!(export_kind("DllUnknown"), None);
}

#[test]
fn stub_helper_stores_its_arguments_as_given() {
    // Not oracle-derived: wine_spec_unimplemented_stub stores the module and function names it is
    // given, and always sets the fixed code, flags and parameter count.
    assert_eq!(
        wine_spec_unimplemented_stub("other.dll", "Export"),
        StubException {
            code: EXCEPTION_WINE_STUB,
            flags: EXCEPTION_NONCONTINUABLE,
            number_parameters: 2,
            module: "other.dll",
            function: "Export",
        }
    );
}

fn range(start: f64, end: f64) -> Range {
    Range { start, end }
}
