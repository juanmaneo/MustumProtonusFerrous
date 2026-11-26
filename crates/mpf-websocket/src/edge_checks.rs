//! Boundary and branch tests. Expectations here are not oracle-derived unless stated.

use super::*;

#[test]
fn e_notimpl_value() {
    // From include/winerror.h, not oracle-derived.
    assert_eq!(E_NOTIMPL as u32, 0x8000_4001);
}

#[test]
fn export_table_matches_spec() {
    // Counts taken from websocket.spec by hand, not oracle-derived.
    assert_eq!(EXPORTS.len(), 13);
    let stubs = EXPORTS.iter().filter(|e| e.1 == ExportKind::Stub).count();
    assert_eq!(stubs, 10);
    assert_eq!(
        export_kind("WebSocketAbortHandle"),
        Some(ExportKind::Stdcall)
    );
    assert_eq!(
        export_kind("WebSocketCreateClientHandle"),
        Some(ExportKind::Stdcall)
    );
    assert_eq!(
        export_kind("WebSocketDeleteHandle"),
        Some(ExportKind::Stdcall)
    );
    assert_eq!(export_kind("WebSocketSend"), Some(ExportKind::Stub));
    assert_eq!(export_kind("websocketsend"), None);
    assert_eq!(export_kind(""), None);
}

#[test]
fn every_stub_export_has_matching_record() {
    // Not oracle-derived: each Stub export name must equal the function in its record.
    let records = [
        web_socket_begin_client_handshake(),
        web_socket_begin_server_handshake(),
        web_socket_complete_action(),
        web_socket_create_server_handle(),
        web_socket_end_client_handshake(),
        web_socket_end_server_handshake(),
        web_socket_get_action(),
        web_socket_get_global_property(),
        web_socket_receive(),
        web_socket_send(),
    ];
    let names: Vec<&str> = EXPORTS
        .iter()
        .filter(|e| e.1 == ExportKind::Stub)
        .map(|e| e.0)
        .collect();
    let got: Vec<&str> = records.iter().map(|r| r.function).collect();
    assert_eq!(got, names);
    assert!(records.iter().all(|r| r.module == "websocket.dll"));
}

#[test]
fn generic_stub_record() {
    // Not oracle-derived: constants of the record.
    let r = wine_spec_unimplemented_stub("a.dll", "F");
    assert_eq!(r.code, EXCEPTION_WINE_STUB);
    assert_eq!(r.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(r.number_parameters, 2);
}

#[test]
fn create_client_handle_leaves_handle_untouched() {
    // Not oracle-derived beyond the oracle rows: extreme count.
    let mut handle = usize::MAX;
    assert_eq!(
        web_socket_create_client_handle(true, u32::MAX, Some(&mut handle)),
        E_NOTIMPL
    );
    assert_eq!(handle, usize::MAX);
}
