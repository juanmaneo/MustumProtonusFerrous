//! `DllGetActivationFactory` from `dlls/graphicscapture/main.c`.

use crate::session::{Interface, Session, IID_ACTIVATION_FACTORY, S_OK};

/// `CLASS_E_CLASSNOTAVAILABLE` from `include/winerror.h`.
pub const CLASS_E_CLASSNOTAVAILABLE: u32 = 0x8004_0111;

/// The class name that `widl` generates from `dlls/graphicscapture/classes.idl`.
const RUNTIME_CLASS_GRAPHICS_CAPTURE_SESSION: &str =
    "Windows.Graphics.Capture.GraphicsCaptureSession";

/// The part of a wide string that `wcscmp` compares: everything before the first NUL code unit,
/// or the whole slice when it has none.
fn wcscmp_view(buffer: &[u16]) -> &[u16] {
    let len = buffer
        .iter()
        .position(|&unit| unit == 0)
        .unwrap_or(buffer.len());
    &buffer[..len]
}

/// `DllGetActivationFactory`. `class_id` holds the `HSTRING` contents, where an empty slice stands
/// for a NULL `HSTRING`. `session` is the object behind `session_factory`.
pub fn dll_get_activation_factory(
    session: &mut Session,
    class_id: &[u16],
    factory: &mut Option<Interface>,
) -> u32 {
    let buffer = wcscmp_view(class_id);
    *factory = None;
    if buffer
        .iter()
        .copied()
        .eq(RUNTIME_CLASS_GRAPHICS_CAPTURE_SESSION.encode_utf16())
    {
        session.query_interface(&IID_ACTIVATION_FACTORY, factory);
    }
    if factory.is_some() {
        S_OK
    } else {
        CLASS_E_CLASSNOTAVAILABLE
    }
}
