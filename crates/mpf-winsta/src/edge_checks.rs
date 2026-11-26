//! Edge-case tests for the ported winsta logic.
//!
//! These cover the last-error slot, the null and extreme arguments, the out-parameter writes,
//! and the export and stub tables. Not every expectation is oracle-derived; the ones that aren't
//! say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/winternl.h, include/winnt.h
    // and dlls/winsta/Makefile.in.
    assert_eq!(ERROR_CALL_NOT_IMPLEMENTED, 120);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    assert_eq!(WINSTA_DLL_NAME, "winsta.dll");
}

#[test]
fn last_error_slot_starts_where_it_is_told() {
    // Not oracle-derived: the default slot and with_last_error are port-only helpers.
    assert_eq!(Winsta::default().last_error(), 0);
    assert_eq!(Winsta::with_last_error(5).last_error(), 5);
}

#[test]
fn explicit_bodies_overwrite_an_earlier_last_error() {
    // Not oracle-derived: the oracle starts every call from 0. Here a stale value is in the slot,
    // and each body must overwrite it with ERROR_CALL_NOT_IMPLEMENTED.
    let mut w = Winsta::with_last_error(0xdead);
    assert_eq!(w.win_station_virtual_open(0, 0, 0), 0);
    assert_eq!(w.last_error(), 120);

    let mut w = Winsta::with_last_error(0xdead);
    assert_eq!(w.win_station_enumerate_w(0, 0, None), 0);
    assert_eq!(w.last_error(), 120);
}

#[test]
fn explicit_bodies_accept_null_and_extreme_arguments() {
    // Not oracle-derived: zero handles, u32::MAX, u64::MAX and absent out-parameters. The C
    // bodies never dereference a handle or an optional out-parameter, so all of these return
    // FALSE with ERROR_CALL_NOT_IMPLEMENTED.
    let mut w = Winsta::default();
    assert_eq!(w.win_station_query_information_a(0, 0, 0, 0, 0, None), 0);
    assert_eq!(w.last_error(), 120);
    assert_eq!(
        w.win_station_query_information_w(0, u32::MAX, u32::MAX, 0, u32::MAX, None),
        0
    );
    assert_eq!(
        w.win_station_register_console_notification(0, 0, u32::MAX),
        0
    );
    assert_eq!(w.win_station_un_register_console_notification(0, 0), 0);
    assert_eq!(w.win_station_get_process_sid(0, 0, u64::MAX, 0, None), 0);
    assert_eq!(w.win_station_enumerate_w(0, 0, None), 0);
    assert_eq!(w.last_error(), 120);
}

#[test]
fn out_parameters_other_than_process_count_are_left_alone() {
    // Not oracle-derived: the C bodies write only *process_count. Every other out-parameter keeps
    // its value, including u32::MAX.
    let mut w = Winsta::default();
    let mut ret_len = u32::MAX;
    assert_eq!(
        w.win_station_query_information_a(1, 1, 1, 1, 1, Some(&mut ret_len)),
        0
    );
    assert_eq!(ret_len, u32::MAX);

    let mut sid_size = u32::MAX;
    assert_eq!(
        w.win_station_get_process_sid(1, 1, 1, 1, Some(&mut sid_size)),
        0
    );
    assert_eq!(sid_size, u32::MAX);
}

#[test]
fn get_all_processes_zeroes_the_count_whatever_it_was() {
    // Not oracle-derived: the count is zeroed from its boundary values and from an arbitrary
    // value, and the level and info arguments do not change the result.
    let mut w = Winsta::default();
    for start in [0, 1, u32::MAX] {
        let mut count = start;
        assert_eq!(
            w.win_station_get_all_processes(0, u32::MAX, &mut count, 0),
            0
        );
        assert_eq!(count, 0);
        assert_eq!(w.last_error(), 120);
    }
}

#[test]
fn export_kind_classifies_names() {
    // Not oracle-derived: one explicit and one stub name from the spec, names with an underscore,
    // and the misses. Names are case-sensitive.
    assert_eq!(
        export_kind("WinStationEnumerateW"),
        Some(ExportKind::Stdcall)
    );
    assert_eq!(
        export_kind("WinStationQueryInformationA"),
        Some(ExportKind::Stdcall)
    );
    assert_eq!(
        export_kind("WinStationEnumerate_IndexedA"),
        Some(ExportKind::Stub)
    );
    assert_eq!(export_kind("_NWLogonSetAdmin"), Some(ExportKind::Stub));
    assert_eq!(export_kind("winstationenumeratew"), None);
    assert_eq!(export_kind("WinStationNotAnExport"), None);
    assert_eq!(export_kind(""), None);
}

#[test]
fn export_table_counts_match_the_spec() {
    // Not oracle-derived: counted from dlls/winsta/winsta.spec, 95 lines: 8 `@ stdcall` and
    // 87 `@ stub`.
    assert_eq!(EXPORTS.len(), 95);
    assert_eq!(
        EXPORTS
            .iter()
            .filter(|(_, k)| *k == ExportKind::Stdcall)
            .count(),
        8
    );
    assert_eq!(
        EXPORTS
            .iter()
            .filter(|(_, k)| *k == ExportKind::Stub)
            .count(),
        87
    );
}

#[test]
fn stub_table_follows_the_export_table_order() {
    // Not oracle-derived: the helper table must hold exactly the `@ stub` exports, in spec order.
    let stub_names: Vec<&str> = EXPORTS
        .iter()
        .filter(|(_, k)| *k == ExportKind::Stub)
        .map(|(n, _)| *n)
        .collect();
    let table_names: Vec<&str> = STUB_EXPORTS.iter().map(|(n, _)| *n).collect();
    assert_eq!(table_names, stub_names);
}

#[test]
fn stub_helpers_raise_their_own_export_name() {
    // Not oracle-derived: every winsta export has a name, so `function` is always the export
    // name. The ordinal form that winebuild uses for unnamed exports does not occur here.
    for &(name, helper) in STUB_EXPORTS {
        let record = helper();
        assert_eq!(record.module, "winsta.dll");
        assert_eq!(record.function, name);
    }
}

#[test]
fn stub_record_echoes_its_arguments() {
    // Not oracle-derived: the record for a module and function outside the table.
    assert_eq!(
        wine_spec_unimplemented_stub("other.dll", "Thing"),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "other.dll",
            function: "Thing",
        }
    );
}
