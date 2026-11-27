use crate::{
    stub_export, wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_WINE_STUB, STUB_EXPORTS,
};

#[test]
fn non_stub_exports_have_no_record() {
    assert_eq!(stub_export("D3DXVec3Normalize"), None);
    assert_eq!(stub_export("D3DXComputeTangentFrameEx"), None);
    assert_eq!(stub_export(""), None);
    assert_eq!(stub_export("d3dxcomputetangent"), None);
    assert_eq!(stub_export("D3DXComputeTangent "), None);
}

#[test]
fn stub_table_has_unique_names() {
    assert_eq!(STUB_EXPORTS.len(), 61);
    for (i, name) in STUB_EXPORTS.iter().enumerate() {
        assert!(!STUB_EXPORTS[..i].contains(name));
    }
}

#[test]
fn raw_record_has_the_ntdll_fields() {
    assert_eq!(
        wine_spec_unimplemented_stub("d3dx9_24.dll", "D3DXRectPatchSize"),
        StubException {
            code: EXCEPTION_WINE_STUB,
            flags: EXCEPTION_NONCONTINUABLE,
            number_parameters: 2,
            module: "d3dx9_24.dll",
            function: "D3DXRectPatchSize",
        }
    );
}
