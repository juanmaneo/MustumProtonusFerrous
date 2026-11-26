//! Edge-case tests for the ported traffic logic.
//!
//! These cover the NULL and non-NULL out-parameter branches, the handle width, and the stub and
//! export tables. Not every expectation is oracle-derived; the ones that aren't say so in a
//! comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winbase.h, include/winerror.h, include/winternl.h
    // and include/winnt.h. The oracle rows are 64-bit, so the handle width is checked here too.
    assert_eq!(ERROR_CALL_NOT_IMPLEMENTED, 120);
    assert_eq!(INVALID_HANDLE_VALUE, usize::MAX);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn register_without_buffer_leaves_no_trace() {
    // Not oracle-derived: the NULL `buffer` branch. The C body only writes through a non-NULL
    // PHANDLE, and the return value does not depend on the buffer.
    assert_eq!(tc_register_client(None), ERROR_CALL_NOT_IMPLEMENTED);
}

#[test]
fn register_overwrites_any_prior_buffer_value() {
    // Not oracle-derived: the write does not depend on the value already in the buffer. The
    // oracle covers the sentinel 0x55. These cases cover a zero and an already-invalid handle.
    for prior in [0, 0x55, INVALID_HANDLE_VALUE] {
        let mut buffer: Handle = prior;
        assert_eq!(
            tc_register_client(Some(&mut buffer)),
            ERROR_CALL_NOT_IMPLEMENTED
        );
        assert_eq!(buffer, INVALID_HANDLE_VALUE);
    }
}

#[test]
fn deregister_returns_not_implemented_for_every_handle_shape() {
    // Not oracle-derived: the handle is never read, so NULL, the invalid handle and a plain
    // value all give the same return. The oracle covers NULL and 0x1234.
    assert_eq!(tc_deregister_client(), ERROR_CALL_NOT_IMPLEMENTED);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the helpers pass their fixed names to the generic record, so the
    // record for other names is the same apart from the names.
    let other = wine_spec_unimplemented_stub("other.dll", "Foo");
    assert_eq!(other.module, "other.dll");
    assert_eq!(other.function, "Foo");
    assert_eq!(other.code, EXCEPTION_WINE_STUB);
    assert_eq!(other.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(other.number_parameters, 2);

    let empty = wine_spec_unimplemented_stub("", "");
    assert_eq!(empty.module, "");
    assert_eq!(empty.function, "");
}

#[test]
fn every_stub_helper_names_its_own_export() {
    // Not oracle-derived: each helper's function name must match its EXPORTS entry, and each
    // stub name must be a `Stub` in the spec table. The oracle checks the same pairs.
    let stubs: [(StubException, &str); 19] = [
        (tc_add_class_map(), "TcAddClassMap"),
        (tc_add_filter(), "TcAddFilter"),
        (tc_add_flow(), "TcAddFlow"),
        (tc_close_interface(), "TcCloseInterface"),
        (tc_delete_filter(), "TcDeleteFilter"),
        (tc_delete_flow(), "TcDeleteFlow"),
        (tc_enumerate_flows(), "TcEnumerateFlows"),
        (tc_enumerate_interfaces(), "TcEnumerateInterfaces"),
        (tc_get_flow_name_a(), "TcGetFlowNameA"),
        (tc_get_flow_name_w(), "TcGetFlowNameW"),
        (tc_modify_flow(), "TcModifyFlow"),
        (tc_open_interface_a(), "TcOpenInterfaceA"),
        (tc_open_interface_w(), "TcOpenInterfaceW"),
        (tc_query_flow_a(), "TcQueryFlowA"),
        (tc_query_flow_w(), "TcQueryFlowW"),
        (tc_query_interface(), "TcQueryInterface"),
        (tc_set_flow_a(), "TcSetFlowA"),
        (tc_set_flow_w(), "TcSetFlowW"),
        (tc_set_interface(), "TcSetInterface"),
    ];
    for (record, name) in stubs {
        assert_eq!(record.function, name);
        assert_eq!(record.module, "traffic.dll");
        assert_eq!(export_kind(name), Some(ExportKind::Stub));
    }
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the 21 lines of dlls/traffic/traffic.spec, in order.
    assert_eq!(EXPORTS.len(), 21);
    let names: Vec<&str> = EXPORTS.iter().map(|(n, _)| *n).collect();
    let mut sorted = names.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), 21);
    assert_eq!(names.first(), Some(&"TcAddClassMap"));
    assert_eq!(names.last(), Some(&"TcSetInterface"));

    assert_eq!(export_kind("TcRegisterClient"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("TcDeregisterClient"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("TcSetInterface"), Some(ExportKind::Stub));
    assert_eq!(export_kind("tcregisterclient"), None);
    assert_eq!(export_kind("TcNoSuchExport"), None);
    assert_eq!(export_kind(""), None);
}
