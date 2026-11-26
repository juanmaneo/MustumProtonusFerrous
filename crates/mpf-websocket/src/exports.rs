/// How `dlls/websocket/websocket.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: an explicit C function in `websocket.c`.
    Stdcall,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `websocket.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 13] = [
    ("WebSocketAbortHandle", ExportKind::Stdcall),
    ("WebSocketBeginClientHandshake", ExportKind::Stub),
    ("WebSocketBeginServerHandshake", ExportKind::Stub),
    ("WebSocketCompleteAction", ExportKind::Stub),
    ("WebSocketCreateClientHandle", ExportKind::Stdcall),
    ("WebSocketCreateServerHandle", ExportKind::Stub),
    ("WebSocketDeleteHandle", ExportKind::Stdcall),
    ("WebSocketEndClientHandshake", ExportKind::Stub),
    ("WebSocketEndServerHandshake", ExportKind::Stub),
    ("WebSocketGetAction", ExportKind::Stub),
    ("WebSocketGetGlobalProperty", ExportKind::Stub),
    ("WebSocketReceive", ExportKind::Stub),
    ("WebSocketSend", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
