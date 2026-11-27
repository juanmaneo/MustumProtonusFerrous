//! Edge checks for constants and the common exception record fields.

use super::*;

#[test]
fn exception_constants_and_module_name_are_pinned() {
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    assert_eq!(MODULE_NAME, "d3dx9_27.dll");
}

#[test]
fn every_stub_name_is_present_once() {
    let exports = stub_exports();
    let mut sorted = exports.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(exports.len(), 68);
    assert_eq!(sorted.len(), exports.len());
}
